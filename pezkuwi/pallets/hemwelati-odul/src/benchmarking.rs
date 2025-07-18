//! Hemwelati Odul Palleti - Pezkuwichain

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement},
        sp_runtime::traits::Hash,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::BlakeTwo256;
    use sp_std::vec::Vec;
    use sp_trie::LayoutV0;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct RewardInfo<AccountId, Balance> {
        pub account: AccountId,
        pub amount: Balance,
    }

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct RewardPeriod<Hash> {
        pub root: Hash,
        pub completed: bool,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn reward_periods)]
    pub(super) type RewardPeriods<T: Config> = StorageMap<_, Blake2_128Concat, u32, RewardPeriod<T::Hash>, OptionQuery>;

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
        #[pallet::weight(10_000)]
        pub fn start_new_period(origin: OriginFor<T>, root: T::Hash) -> DispatchResult {
            ensure_root(origin)?;

            let period_id = NextPeriodId::<T>::get();
            RewardPeriods::<T>::insert(period_id, RewardPeriod { root, completed: false });
            NextPeriodId::<T>::put(period_id + 1);

            Self::deposit_event(Event::NewPeriodStarted(period_id));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn complete_period(origin: OriginFor<T>, period_id: u32) -> DispatchResult {
            ensure_root(origin)?;

            RewardPeriods::<T>::try_mutate(period_id, |period| -> DispatchResult {
                let p = period.as_mut().ok_or(Error::<T>::PeriodNotFound)?;
                p.completed = true;
                Ok(())
            })?;

            Self::deposit_event(Event::PeriodCompleted(period_id));
            Ok(())
        }

        #[pallet::weight(10_000)]
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

            let leaf = (account.clone(), amount.clone()).using_encoded(|b| BlakeTwo256::hash(b));

            let proof_valid = sp_trie::verify_trie_proof::<LayoutV0<BlakeTwo256>, _, _, _>(
                &period.root,
                proof.clone(),
                vec![(leaf.as_ref(), Some(&leaf.encode()))],
            ).is_ok();

            ensure!(proof_valid, Error::<T>::InvalidProof);

            T::Currency::transfer(&Self::account_id(), &account, amount, ExistenceRequirement::KeepAlive)?;

            ClaimedRewards::<T>::insert(period_id, &who, true);
            Self::deposit_event(Event::RewardClaimed(account, amount));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            PALLET_ID.into_account_truncating()
        }
    }

    const PALLET_ID: frame_support::PalletId = frame_support::PalletId(*b"pz/rewrd");
}

// Benchmarking module
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use super::*;
    use frame_benchmarking::{benchmarks, whitelisted_caller};
    use frame_system::RawOrigin;

    benchmarks! {
        start_new_period {
            let caller: T::AccountId = whitelisted_caller();
            let root = T::Hash::default();
        }: _(RawOrigin::Root, root)

        complete_period {
            let caller: T::AccountId = whitelisted_caller();
            let root = T::Hash::default();
            Pallet::<T>::start_new_period(RawOrigin::Root.into(), root.clone()).unwrap();
            let period_id = NextPeriodId::<T>::get() - 1;
        }: _(RawOrigin::Root, period_id)

        claim_reward {
            let caller: T::AccountId = whitelisted_caller();
            let root = T::Hash::default();
            Pallet::<T>::start_new_period(RawOrigin::Root.into(), root.clone()).unwrap();
            let period_id = NextPeriodId::<T>::get() - 1;
            // Dummy claim info
            let account = caller.clone();
            let amount: BalanceOf<T> = 100u32.into();
            let proof = Vec::new();
        }: _(RawOrigin::Signed(caller.clone()), period_id, account, amount, proof)
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
