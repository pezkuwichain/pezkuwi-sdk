#![cfg_attr(not(feature = "std"), no_std)]

//! # Pallet Presale
//!
//! PEZ token presale pallet for PezkuwiChain.
//!
//! ## Overview
//!
//! This pallet manages the PEZ token presale:
//! - Accepts wUSDT (Asset ID: 2) contributions
//! - Tracks contributor amounts
//! - Distributes PEZ (Asset ID: 1) after 45-day period
//! - Conversion rate: 1 wUSDT = 100 PEZ
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `start_presale` - Start the presale (sudo only)
//! - `contribute` - Contribute wUSDT to presale
//! - `finalize_presale` - End presale and distribute PEZ (sudo only)
//! - `emergency_pause` - Pause presale in emergency (sudo only)
//! - `emergency_unpause` - Unpause presale (sudo only)

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{fungibles::{Inspect, Mutate}, tokens::Preservation},
        PalletId,
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AccountIdConversion, Zero, CheckedMul, CheckedDiv};
    use sp_std::vec::Vec;
    use codec::{Encode, Decode, MaxEncodedLen};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The presale pallet id, used for deriving its sovereign account.
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// wUSDT asset ID (should be 2)
        #[pallet::constant]
        type WUsdtAssetId: Get<<Self as pallet_assets::Config>::AssetId>;

        /// PEZ asset ID (should be 1)
        #[pallet::constant]
        type PezAssetId: Get<<Self as pallet_assets::Config>::AssetId>;

        /// Conversion rate: 1 wUSDT = X PEZ (e.g., 100)
        #[pallet::constant]
        type ConversionRate: Get<u128>;

        /// Presale duration in blocks (45 days)
        #[pallet::constant]
        type PresaleDuration: Get<BlockNumberFor<Self>>;

        /// Maximum number of contributors
        #[pallet::constant]
        type MaxContributors: Get<u32>;

        /// Weight information for extrinsics in this pallet.
        type PresaleWeightInfo: crate::weights::WeightInfo;
    }

    /// Contributions mapping: AccountId => wUSDT amount (6 decimals)
    #[pallet::storage]
    #[pallet::getter(fn contributions)]
    pub type Contributions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        u128,
        ValueQuery,
    >;

    /// List of all contributors
    #[pallet::storage]
    #[pallet::getter(fn contributors)]
    pub type Contributors<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxContributors>, ValueQuery>;

    /// Is presale currently active
    #[pallet::storage]
    #[pallet::getter(fn presale_active)]
    pub type PresaleActive<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// Block number when presale started
    #[pallet::storage]
    #[pallet::getter(fn presale_start_block)]
    pub type PresaleStartBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, OptionQuery>;

    /// Total wUSDT raised (6 decimals)
    #[pallet::storage]
    #[pallet::getter(fn total_raised)]
    pub type TotalRaised<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// Emergency pause flag
    #[pallet::storage]
    #[pallet::getter(fn paused)]
    pub type Paused<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Presale started. [end_block]
        PresaleStarted { end_block: BlockNumberFor<T> },
        /// User contributed wUSDT. [who, amount]
        Contributed { who: T::AccountId, amount: u128 },
        /// Presale finalized. [total_raised]
        PresaleFinalized { total_raised: u128 },
        /// PEZ distributed to contributor. [who, pez_amount]
        Distributed { who: T::AccountId, pez_amount: u128 },
        /// Emergency pause activated
        EmergencyPaused,
        /// Emergency pause deactivated
        EmergencyUnpaused,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Presale is not currently active
        PresaleNotActive,
        /// Presale period has ended
        PresaleEnded,
        /// Presale period has not ended yet
        PresaleNotEnded,
        /// Presale has already been finalized
        AlreadyFinalized,
        /// Contribution amount must be greater than zero
        ZeroContribution,
        /// Transfer of wUSDT failed
        TransferFailed,
        /// Arithmetic overflow
        ArithmeticOverflow,
        /// Presale is paused
        PresalePaused,
        /// Presale already started
        AlreadyStarted,
        /// Insufficient PEZ balance in treasury
        InsufficientPezBalance,
        /// Too many contributors (reached MaxContributors limit)
        TooManyContributors,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Start the presale (sudo only)
        ///
        /// This will set the start block and activate the presale for 45 days.
        #[pallet::call_index(0)]
        #[pallet::weight(T::PresaleWeightInfo::start_presale())]
        pub fn start_presale(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(!PresaleActive::<T>::get(), Error::<T>::AlreadyStarted);

            let current_block = <frame_system::Pallet<T>>::block_number();
            let end_block = current_block + T::PresaleDuration::get();

            PresaleActive::<T>::put(true);
            PresaleStartBlock::<T>::put(current_block);
            Paused::<T>::put(false);

            Self::deposit_event(Event::PresaleStarted { end_block });
            Ok(())
        }

        /// Contribute wUSDT to the presale
        ///
        /// User sends wUSDT to the presale treasury and their contribution is tracked.
        ///
        /// # Arguments
        ///
        /// * `amount` - Amount of wUSDT to contribute (with 6 decimals)
        #[pallet::call_index(1)]
        #[pallet::weight(T::PresaleWeightInfo::contribute())]
        pub fn contribute(
            origin: OriginFor<T>,
            #[pallet::compact] amount: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Checks
            ensure!(PresaleActive::<T>::get(), Error::<T>::PresaleNotActive);
            ensure!(!Paused::<T>::get(), Error::<T>::PresalePaused);
            ensure!(amount > Zero::zero(), Error::<T>::ZeroContribution);

            // Check if presale ended
            let current_block = <frame_system::Pallet<T>>::block_number();
            let start_block = PresaleStartBlock::<T>::get()
                .ok_or(Error::<T>::PresaleNotActive)?;
            let end_block = start_block + T::PresaleDuration::get();
            ensure!(current_block < end_block, Error::<T>::PresaleEnded);

            // Transfer wUSDT from user to pallet treasury
            let treasury = Self::account_id();
            let asset_id = T::WUsdtAssetId::get();
            let amount_balance = amount.try_into()
                .map_err(|_| Error::<T>::ArithmeticOverflow)?;

            // Use pallet_assets to transfer
            <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                asset_id,
                &who,
                &treasury,
                amount_balance,
                Preservation::Preserve,
            )?;

            // Track contribution
            let current_contribution = Contributions::<T>::get(&who);
            if current_contribution == Zero::zero() {
                // New contributor
                Contributors::<T>::try_mutate(|contributors| -> DispatchResult {
                    contributors.try_push(who.clone())
                        .map_err(|_| Error::<T>::TooManyContributors)?;
                    Ok(())
                })?;
            }

            let new_total = current_contribution
                .checked_add(amount)
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            Contributions::<T>::insert(&who, new_total);

            let new_raised = TotalRaised::<T>::get()
                .checked_add(amount)
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            TotalRaised::<T>::put(new_raised);

            Self::deposit_event(Event::Contributed { who, amount });
            Ok(())
        }

        /// Finalize presale and distribute PEZ (sudo only)
        ///
        /// After 45 days, this distributes PEZ tokens to all contributors
        /// based on their wUSDT contributions.
        #[pallet::call_index(2)]
        #[pallet::weight(T::PresaleWeightInfo::finalize_presale(Contributors::<T>::get().len() as u32))]
        pub fn finalize_presale(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(PresaleActive::<T>::get(), Error::<T>::PresaleNotActive);

            // Check if presale period ended
            let current_block = <frame_system::Pallet<T>>::block_number();
            let start_block = PresaleStartBlock::<T>::get()
                .ok_or(Error::<T>::PresaleNotActive)?;
            let end_block = start_block + T::PresaleDuration::get();
            ensure!(current_block >= end_block, Error::<T>::PresaleNotEnded);

            let treasury = Self::account_id();
            let total_raised = TotalRaised::<T>::get();
            let pez_asset_id = T::PezAssetId::get();

            // Distribute PEZ to all contributors
            for contributor in Contributors::<T>::get().iter() {
                let wusdt_amount = Contributions::<T>::get(contributor);
                if wusdt_amount == Zero::zero() {
                    continue;
                }

                // Calculate PEZ amount
                let pez_amount = Self::calculate_pez(wusdt_amount)?;
                let pez_balance = pez_amount.try_into()
                    .map_err(|_| Error::<T>::ArithmeticOverflow)?;

                // Transfer PEZ from treasury to contributor
                <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                    pez_asset_id.clone(),
                    &treasury,
                    contributor,
                    pez_balance,
                    Preservation::Preserve,
                )?;

                Self::deposit_event(Event::Distributed {
                    who: contributor.clone(),
                    pez_amount,
                });
            }

            PresaleActive::<T>::put(false);
            Self::deposit_event(Event::PresaleFinalized { total_raised });
            Ok(())
        }

        /// Emergency pause (sudo only)
        #[pallet::call_index(3)]
        #[pallet::weight(T::PresaleWeightInfo::emergency_pause())]
        pub fn emergency_pause(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            Paused::<T>::put(true);
            Self::deposit_event(Event::EmergencyPaused);
            Ok(())
        }

        /// Emergency unpause (sudo only)
        #[pallet::call_index(4)]
        #[pallet::weight(T::PresaleWeightInfo::emergency_unpause())]
        pub fn emergency_unpause(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            Paused::<T>::put(false);
            Self::deposit_event(Event::EmergencyUnpaused);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get the account ID of the presale pallet.
        ///
        /// This is the treasury where wUSDT is collected and PEZ is distributed from.
        pub fn account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }

        /// Calculate PEZ amount from wUSDT amount
        ///
        /// Formula:
        /// - wUSDT has 6 decimals (1 USDT = 1_000_000 units)
        /// - PEZ has 12 decimals (1 PEZ = 1_000_000_000_000 units)
        /// - Rate: 1 wUSDT = 100 PEZ (configurable)
        ///
        /// Calculation:
        /// 1. Convert wUSDT to USD: wusdt_amount / 1_000_000
        /// 2. Apply conversion rate: USD * rate
        /// 3. Convert to PEZ decimals: result * 1_000_000_000_000
        ///
        /// Example:
        /// - Input: 100 wUSDT (100_000_000 units with 6 decimals)
        /// - USD: 100
        /// - PEZ units: 100 * 100 = 10_000
        /// - PEZ with decimals: 10_000 * 1_000_000_000_000 = 10_000_000_000_000_000
        fn calculate_pez(wusdt_amount: u128) -> Result<u128, Error<T>> {
            let rate = T::ConversionRate::get();

            // Step 1: wUSDT to USD (remove 6 decimals)
            // Step 2: Apply rate
            let pez_units = wusdt_amount
                .checked_mul(rate)
                .ok_or(Error::<T>::ArithmeticOverflow)?
                .checked_div(1_000_000)
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            // Step 3: Add 12 decimals
            let pez_with_decimals = pez_units
                .checked_mul(1_000_000_000_000)
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            Ok(pez_with_decimals)
        }

        /// Get time remaining in blocks
        pub fn get_time_remaining() -> BlockNumberFor<T> {
            if !PresaleActive::<T>::get() {
                return Zero::zero();
            }

            if let Some(start_block) = PresaleStartBlock::<T>::get() {
                let current_block = <frame_system::Pallet<T>>::block_number();
                let end_block = start_block + T::PresaleDuration::get();

                if current_block >= end_block {
                    Zero::zero()
                } else {
                    end_block - current_block
                }
            } else {
                Zero::zero()
            }
        }
    }
}
