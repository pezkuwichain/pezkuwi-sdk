//kk/pallets/hemwelati-odul/src/lib.rs (v4)

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "std")]
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement}, // PalletId struct removed from here
        // PalletId struct is referenced directly via frame_support::PalletId
        sp_runtime::traits::{Hash, AccountIdConversion},
    };
    use frame_system::pallet_prelude::*;
    use sp_core::H256;
    use sp_runtime::traits::BlakeTwo256;
    use sp_std::vec::Vec;
    use sp_trie::LayoutV0;
    use crate::weights::WeightInfo;


    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct RewardInfo<AccountId, Balance> {
        pub account: AccountId,
        pub amount: Balance,
    }

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct RewardPeriod {
        pub root: H256,
        pub completed: bool,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn reward_periods)]
    pub(super) type RewardPeriods<T: Config> = StorageMap<_, Blake2_128Concat, u32, RewardPeriod, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn claimed_rewards)]
    pub(super) type ClaimedRewards<T: Config> = StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_period_id)]
    pub(super) type NextPeriodId<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        InvalidProof,
        AlreadyClaimed,
        PeriodNotFound,
        PeriodCompleted,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RewardClaimed(T::AccountId, BalanceOf<T>),
        PeriodCompleted(u32),
        NewPeriodStarted(u32),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::start_new_period())]
        #[pallet::call_index(0)] // Added call_index
        pub fn start_new_period(origin: OriginFor<T>, root: H256) -> DispatchResult {
            ensure_root(origin)?;

            let period_id = NextPeriodId::<T>::get();
            RewardPeriods::<T>::insert(period_id, RewardPeriod { root, completed: false });
            NextPeriodId::<T>::put(period_id + 1);

            Self::deposit_event(Event::NewPeriodStarted(period_id));
            Ok(())
        }

        #[pallet::weight(T::WeightInfo::complete_period())]
        #[pallet::call_index(1)] // Added call_index
        pub fn complete_period(origin: OriginFor<T>, period_id: u32) -> DispatchResult {
            ensure_root(origin)?;

            RewardPeriods::<T>::try_mutate(period_id, |period_option| -> DispatchResult {
                let p = period_option.as_mut().ok_or(Error::<T>::PeriodNotFound)?;
                p.completed = true;
                Ok(())
            })?;

            Self::deposit_event(Event::PeriodCompleted(period_id));
            Ok(())
        }

        #[pallet::weight(T::WeightInfo::claim_reward())]
        #[pallet::call_index(2)] // Added call_index
        pub fn claim_reward(
            origin: OriginFor<T>,
            period_id: u32,
            account: T::AccountId,
            amount: BalanceOf<T>,
            proof: Vec<Vec<u8>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!ClaimedRewards::<T>::get(period_id, &who), Error::<T>::AlreadyClaimed);

            let period = RewardPeriods::<T>::get(period_id).ok_or(Error::<T>::PeriodNotFound)?;
            ensure!(!period.completed, Error::<T>::PeriodCompleted);

            let leaf_data = (account.clone(), amount.clone()).encode();
            let leaf = BlakeTwo256::hash(&leaf_data);

            let key_value_pairs_for_trie = [(leaf.as_ref(), Some(leaf.as_ref()))];

            sp_trie::verify_trie_proof::<LayoutV0<BlakeTwo256>, _, _, _>(
                &period.root,
                &proof,
                &key_value_pairs_for_trie,
            ).map_err(|_| Error::<T>::InvalidProof)?;

            T::Currency::transfer(&Self::account_id(), &account, amount.clone(), ExistenceRequirement::KeepAlive)?;

            ClaimedRewards::<T>::insert(period_id, &who, true);
            Self::deposit_event(Event::RewardClaimed(account, amount));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            PALLET_ID.into_account_truncating()
        }

        pub fn get_account_from_target_event(_event: &<T as crate::pallet::Config>::RuntimeEvent) -> Option<T::AccountId> {
            None
        }
    }

    // Changed: PALLET_ID type explicitly to frame_support::PalletId
    const PALLET_ID: frame_support::PalletId = frame_support::PalletId(*b"pz/rewrd");
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use super::*;
    use crate::pallet::{Pallet as HemwelatiOdulPallet, BalanceOf, NextPeriodId, RewardPeriods, ClaimedRewards}; // Added more imports from super::pallet
    use frame_benchmarking::{benchmarks, whitelisted_caller, account};
    use frame_system::RawOrigin;
    use frame_support::traits::Currency;
    use sp_core::H256;
    use sp_std::vec; // For Vec::new()
    use sp_runtime::traits::BlakeTwo256; // For leaf hashing in benchmark
    use frame_support::pallet_prelude::Encode; // For .encode() in benchmark


    benchmarks! {
        start_new_period {
            let root = H256::default();
        }: _(RawOrigin::Root, root)
        verify {
            let period_id = NextPeriodId::<T>::get() - 1;
            assert!(RewardPeriods::<T>::get(period_id).is_some());
        }

        complete_period {
            let root = H256::default();
            HemwelatiOdulPallet::<T>::start_new_period(RawOrigin::Root.into(), root.clone()).unwrap();
            let period_id = NextPeriodId::<T>::get() - 1;
        }: _(RawOrigin::Root, period_id)
        verify {
             assert!(RewardPeriods::<T>::get(period_id).unwrap().completed);
        }

        claim_reward {
            let caller: T::AccountId = whitelisted_caller();
            T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::from(1_000_000u32));

            let account_to_claim = caller.clone();
            let amount_to_claim: BalanceOf<T> = 100u32.into();

            let leaf_data = (account_to_claim.clone(), amount_to_claim.clone()).encode();
            let leaf_hash = BlakeTwo256::hash(&leaf_data);
            let root = leaf_hash;

            let pallet_account = HemwelatiOdulPallet::<T>::account_id();
            T::Currency::make_free_balance_be(&pallet_account, amount_to_claim.clone() + BalanceOf::<T>::from(1u32));

            HemwelatiOdulPallet::<T>::start_new_period(RawOrigin::Root.into(), root).unwrap();
            let period_id = NextPeriodId::<T>::get() - 1;
            let proof: Vec<Vec<u8>> = vec![];

        }: _(RawOrigin::Signed(caller.clone()), period_id, account_to_claim.clone(), amount_to_claim.clone(), proof)
        verify {
            assert!(ClaimedRewards::<T>::get(period_id, &caller));
        }
    }

    impl_benchmark_test_suite!(HemwelatiOdulPallet, crate::mock::new_test_ext(), crate::mock::TestRuntime);
}