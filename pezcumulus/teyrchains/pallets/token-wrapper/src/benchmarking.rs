//! Benchmarking setup for pezpallet-token-wrapper

#![cfg(feature = "runtime-benchmarks")]

use super::*;
#[allow(unused)]
use crate::Pallet as TokenWrapper;
use pezframe_benchmarking::v2::*;
use pezframe_support::traits::Currency;
use pezframe_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn wrap() {
		let caller: T::AccountId = whitelisted_caller();
		let pezpallet_account = Pallet::<T>::account_id();
		let amount = 10_000u32.into();

		// Fund both caller and pallet account
		let funding = <T::Currency as Currency<T::AccountId>>::minimum_balance()
			.saturating_mul(1000u32.into());

		T::Currency::make_free_balance_be(&caller, funding);
		T::Currency::make_free_balance_be(&pezpallet_account, funding);

		// Create asset
		let _ =
			T::Assets::create(T::WrapperAssetId::get(), pezpallet_account.clone(), true, 1u32.into());

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), amount);

		// Verify
		assert!(T::Assets::balance(T::WrapperAssetId::get(), &caller) >= amount);
	}

	#[benchmark]
	fn unwrap() {
		let caller: T::AccountId = whitelisted_caller();
		let pezpallet_account = Pallet::<T>::account_id();
		let amount = 10_000u32.into();

		// Fund both accounts
		let funding = <T::Currency as Currency<T::AccountId>>::minimum_balance()
			.saturating_mul(1000u32.into());

		T::Currency::make_free_balance_be(&caller, funding);
		T::Currency::make_free_balance_be(&pezpallet_account, funding);

		// Create asset
		let _ =
			T::Assets::create(T::WrapperAssetId::get(), pezpallet_account.clone(), true, 1u32.into());

		// Wrap first
		let _ = Pallet::<T>::wrap(RawOrigin::Signed(caller.clone()).into(), amount);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), amount);

		// Verify
		assert_eq!(T::Assets::balance(T::WrapperAssetId::get(), &caller), 0u32.into());
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
