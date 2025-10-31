//! Benchmarking setup for pallet-token-wrapper

use super::*;
#[allow(unused)]
use crate::Pallet as TokenWrapper;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::traits::Currency;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn wrap() {
        let caller: T::AccountId = whitelisted_caller();
        let pallet_account = Pallet::<T>::account_id();
        let amount = 10_000u32.into();

        // Fund both caller and pallet account
        let funding = <T::Currency as Currency<T::AccountId>>::minimum_balance()
            .saturating_mul(1000u32.into());
        
        T::Currency::make_free_balance_be(&caller, funding);
        T::Currency::make_free_balance_be(&pallet_account, funding);
        
        // Create asset
        let _ = T::Assets::create(
            T::WrapperAssetId::get(),
            pallet_account.clone(),
            true,
            1u32.into()
        );

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), amount);

        // Verify
        assert!(T::Assets::balance(T::WrapperAssetId::get(), &caller) >= amount);
    }

    #[benchmark]
    fn unwrap() {
        let caller: T::AccountId = whitelisted_caller();
        let pallet_account = Pallet::<T>::account_id();
        let amount = 10_000u32.into();

        // Fund both accounts
        let funding = <T::Currency as Currency<T::AccountId>>::minimum_balance()
            .saturating_mul(1000u32.into());
        
        T::Currency::make_free_balance_be(&caller, funding);
        T::Currency::make_free_balance_be(&pallet_account, funding);
        
        // Create asset
        let _ = T::Assets::create(
            T::WrapperAssetId::get(),
            pallet_account.clone(),
            true,
            1u32.into()
        );
        
        // Wrap first
        let _ = Pallet::<T>::wrap(RawOrigin::Signed(caller.clone()).into(), amount);

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), amount);

        // Verify
        assert_eq!(
            T::Assets::balance(T::WrapperAssetId::get(), &caller),
            0u32.into()
        );
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}