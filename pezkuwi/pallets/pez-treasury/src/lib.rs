#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::{
    traits::{
        fungibles::{Inspect, Mutate},
        tokens::Preservation,
        Get,
    },
    PalletId,
};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::traits::{AccountIdConversion, Saturating, Zero};

#[frame_support::pallet]
pub mod pallet {
    use super::{*, weights::WeightInfo};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    // use sp_runtime::traits::CheckedDiv;

    pub const HALVING_PERIOD_MONTHS: u32 = 48; // 4 yıl = 48 ay
    pub const BLOCKS_PER_MONTH: u32 = 432_000; // ~30 gün * 24 saat * 60 dakika * 10 blok/dakika
    pub const HALVING_PERIOD_BLOCKS: u32 = HALVING_PERIOD_MONTHS * BLOCKS_PER_MONTH;

    pub const TOTAL_SUPPLY: u128 = 5_000_000_000 * 1_000_000_000_000; // 5 milyar PEZ (12 decimal)
    pub const TREASURY_ALLOCATION: u128 = 4_812_500_000 * 1_000_000_000_000; // %96.25
    pub const PRESALE_ALLOCATION: u128 = 93_750_000 * 1_000_000_000_000; // %1.875
    pub const FOUNDER_ALLOCATION: u128 = 93_750_000 * 1_000_000_000_000; // %1.875

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + TypeInfo {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Assets: Mutate<Self::AccountId>;
        type WeightInfo: weights::WeightInfo;

        #[pallet::constant]
        type PezAssetId: Get<<Self::Assets as Inspect<Self::AccountId>>::AssetId>;

        #[pallet::constant]
        type TreasuryPalletId: Get<PalletId>;

        #[pallet::constant]
        type IncentivePotId: Get<PalletId>;

        #[pallet::constant]
        type GovernmentPotId: Get<PalletId>;

        #[pallet::constant]
        type PresaleAccount: Get<Self::AccountId>;

        #[pallet::constant]
        type FounderAccount: Get<Self::AccountId>;

        type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    }

    pub type BalanceOf<T> =
        <<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::storage]
    #[pallet::getter(fn halving_info)]
    pub type HalvingInfo<T: Config> = StorageValue<_, HalvingData<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn monthly_releases)]
    pub type MonthlyReleases<T: Config> =
        StorageMap<_, Blake2_128Concat, u32, MonthlyRelease<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_release_month)]
    pub type NextReleaseMonth<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn treasury_start_block)]
    pub type TreasuryStartBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, OptionQuery>;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct HalvingData<T: Config> {
        pub current_period: u32,
        pub period_start_block: BlockNumberFor<T>,
        pub monthly_amount: BalanceOf<T>,
        pub total_released: BalanceOf<T>,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct MonthlyRelease<T: Config> {
        pub month_index: u32,
        pub release_block: BlockNumberFor<T>,
        pub amount_released: BalanceOf<T>,
        pub incentive_amount: BalanceOf<T>,
        pub government_amount: BalanceOf<T>,
    }

    impl<T: Config> Default for HalvingData<T> {
        fn default() -> Self {
            Self {
                current_period: 0,
                period_start_block: Zero::zero(),
                monthly_amount: Zero::zero(),
                total_released: Zero::zero(),
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TreasuryInitialized {
            start_block: BlockNumberFor<T>,
            initial_monthly_amount: BalanceOf<T>,
        },
        MonthlyFundsReleased {
            month_index: u32,
            total_amount: BalanceOf<T>,
            incentive_amount: BalanceOf<T>,
            government_amount: BalanceOf<T>,
        },
        NewHalvingPeriod {
            period: u32,
            new_monthly_amount: BalanceOf<T>,
        },
        GenesisDistributionCompleted {
            treasury_amount: BalanceOf<T>,
            presale_amount: BalanceOf<T>,
            founder_amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        TreasuryAlreadyInitialized,
        TreasuryNotInitialized,
        MonthlyReleaseAlreadyDone,
        InsufficientTreasuryBalance,
        InvalidHalvingPeriod,
        ReleaseTooEarly,
    }

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub initialize_treasury: bool,
        #[serde(skip)]
        pub _phantom: core::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            if self.initialize_treasury {
                let _ = Pallet::<T>::do_initialize_treasury();
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::initialize_treasury())]
        pub fn initialize_treasury(origin: OriginFor<T>) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            Self::do_initialize_treasury()
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::release_monthly_funds())]
        pub fn release_monthly_funds(origin: OriginFor<T>) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            Self::do_monthly_release()
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::force_genesis_distribution())]
        pub fn force_genesis_distribution(origin: OriginFor<T>) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            Self::do_genesis_distribution()
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn treasury_account_id() -> T::AccountId {
            T::TreasuryPalletId::get().into_account_truncating()
        }

        pub fn incentive_pot_account_id() -> T::AccountId {
            T::IncentivePotId::get().into_account_truncating()
        }

        pub fn government_pot_account_id() -> T::AccountId {
            T::GovernmentPotId::get().into_account_truncating()
        }

        pub fn do_genesis_distribution() -> DispatchResult {
            let treasury_account = Self::treasury_account_id();
            let presale_account = T::PresaleAccount::get();
            let founder_account = T::FounderAccount::get();

            let treasury_amount: BalanceOf<T> =
                TREASURY_ALLOCATION.try_into().map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;
            let presale_amount: BalanceOf<T> =
                PRESALE_ALLOCATION.try_into().map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;
            let founder_amount: BalanceOf<T> =
                FOUNDER_ALLOCATION.try_into().map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;

            T::Assets::mint_into(T::PezAssetId::get(), &treasury_account, treasury_amount)?;
            T::Assets::mint_into(T::PezAssetId::get(), &presale_account, presale_amount)?;
            T::Assets::mint_into(T::PezAssetId::get(), &founder_account, founder_amount)?;

            Self::deposit_event(Event::GenesisDistributionCompleted {
                treasury_amount,
                presale_amount,
                founder_amount,
            });

            Ok(())
        }

        pub fn do_initialize_treasury() -> DispatchResult {
            ensure!(
                TreasuryStartBlock::<T>::get().is_none(),
                Error::<T>::TreasuryAlreadyInitialized
            );

            let current_block = frame_system::Pallet::<T>::block_number();

            let treasury_balance = TREASURY_ALLOCATION;
            let first_period_total =
                treasury_balance.checked_div(2).ok_or(Error::<T>::InvalidHalvingPeriod)?;
            let monthly_amount = first_period_total
                .checked_div(HALVING_PERIOD_MONTHS.into())
                .ok_or(Error::<T>::InvalidHalvingPeriod)?;

            let monthly_amount_balance: BalanceOf<T> =
                monthly_amount.try_into().map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;

            let halving_data = HalvingData {
                current_period: 0,
                period_start_block: current_block,
                monthly_amount: monthly_amount_balance,
                total_released: Zero::zero(),
            };

            TreasuryStartBlock::<T>::put(current_block);
            HalvingInfo::<T>::put(halving_data);
            NextReleaseMonth::<T>::put(0);

            Self::deposit_event(Event::TreasuryInitialized {
                start_block: current_block,
                initial_monthly_amount: monthly_amount_balance,
            });

            Ok(())
        }

        pub fn do_monthly_release() -> DispatchResult {
            ensure!(
                TreasuryStartBlock::<T>::get().is_some(),
                Error::<T>::TreasuryNotInitialized
            );

            let current_block = frame_system::Pallet::<T>::block_number();
            let start_block = TreasuryStartBlock::<T>::get().unwrap();
            let next_month = NextReleaseMonth::<T>::get();

            ensure!(
                !MonthlyReleases::<T>::contains_key(next_month),
                Error::<T>::MonthlyReleaseAlreadyDone
            );

            let blocks_passed = current_block.saturating_sub(start_block);
            let months_passed: u32 = (blocks_passed / BLOCKS_PER_MONTH.into()).try_into().unwrap_or(0);

            // 0. ayı serbest bırakmak için months_passed >= 1 olmalı (next_month + 1)
            // 1. ayı serbest bırakmak için months_passed >= 2 olmalı
            ensure!(months_passed >= next_month + 1, Error::<T>::ReleaseTooEarly);

            let mut halving_data = HalvingInfo::<T>::get();

            let current_period_passed_months =
                months_passed.saturating_sub(halving_data.current_period * HALVING_PERIOD_MONTHS);

            if current_period_passed_months >= HALVING_PERIOD_MONTHS {
                halving_data.current_period += 1;
                halving_data.monthly_amount = halving_data.monthly_amount / 2u32.into();
                halving_data.period_start_block = current_block;

                Self::deposit_event(Event::NewHalvingPeriod {
                    period: halving_data.current_period,
                    new_monthly_amount: halving_data.monthly_amount,
                });
            }

            let monthly_amount = halving_data.monthly_amount;
            let incentive_amount = monthly_amount * 75u32.into() / 100u32.into();
            let government_amount = monthly_amount.saturating_sub(incentive_amount);

            let treasury_account = Self::treasury_account_id();
            let incentive_pot = Self::incentive_pot_account_id();
            let government_pot = Self::government_pot_account_id();

            T::Assets::transfer(
                T::PezAssetId::get(),
                &treasury_account,
                &incentive_pot,
                incentive_amount,
                Preservation::Preserve,
            )
            .map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;

            T::Assets::transfer(
                T::PezAssetId::get(),
                &treasury_account,
                &government_pot,
                government_amount,
                Preservation::Preserve,
            )
            .map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;

            halving_data.total_released = halving_data.total_released.saturating_add(monthly_amount);
            HalvingInfo::<T>::put(halving_data);

            let release_info = MonthlyRelease {
                month_index: next_month,
                release_block: current_block,
                amount_released: monthly_amount,
                incentive_amount,
                government_amount,
            };

            MonthlyReleases::<T>::insert(next_month, release_info);
            NextReleaseMonth::<T>::put(next_month + 1);

            Self::deposit_event(Event::MonthlyFundsReleased {
                month_index: next_month,
                total_amount: monthly_amount,
                incentive_amount,
                government_amount,
            });

            Ok(())
        }

        pub fn get_current_halving_info() -> HalvingData<T> {
            HalvingInfo::<T>::get()
        }

        pub fn get_incentive_pot_balance() -> BalanceOf<T> {
            let pot_account = Self::incentive_pot_account_id();
            T::Assets::balance(T::PezAssetId::get(), &pot_account)
        }

        pub fn get_government_pot_balance() -> BalanceOf<T> {
            let pot_account = Self::government_pot_account_id();
            T::Assets::balance(T::PezAssetId::get(), &pot_account)
        }
    }
}