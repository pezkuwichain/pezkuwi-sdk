//! Benchmarking setup for pallet-referral

use super::*;
#[allow(unused_imports)]
use crate::Pallet as Referral;
use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;

benchmarks! {
	initiate_referral {
		let referrer: T::AccountId = account("referrer", 0, 0);
		let referred: T::AccountId = account("referred", 0, 1);
		
		// `referred` hesabının daha önce davet edilmediğinden emin olalım.
		PendingReferrals::<T>::remove(&referred);
		Referrals::<T>::remove(&referred);

	}: _(RawOrigin::Signed(referrer.clone()), referred.clone())
	verify {
		assert_eq!(PendingReferrals::<T>::get(&referred), Some(referrer));
	}

	impl_benchmark_test_suite!(Referral, crate::mock::new_test_ext(), crate::mock::Test);
}