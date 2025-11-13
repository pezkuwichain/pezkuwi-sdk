//! Benchmarking setup for pallet-referral

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Referral;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn initiate_referral() {
		let referrer: T::AccountId = account("referrer", 0, 0);
		let referred: T::AccountId = account("referred", 0, 1);

		// Ensure the `referred` account has not been referred before
		PendingReferrals::<T>::remove(&referred);
		Referrals::<T>::remove(&referred);

		#[extrinsic_call]
		initiate_referral(RawOrigin::Signed(referrer.clone()), referred.clone());

		assert_eq!(PendingReferrals::<T>::get(&referred), Some(referrer));
	}

	impl_benchmark_test_suite!(Referral, crate::mock::new_test_ext(), crate::mock::Test);
}
