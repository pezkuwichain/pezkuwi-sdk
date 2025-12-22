//! Benchmarking setup for pezpallet-staking-score

use crate::{Call, Config, Pezpallet, StakingStartBlock};
use pezframe_benchmarking::v2::*;
use pezframe_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn start_score_tracking() {
		let caller: T::AccountId = whitelisted_caller();

		// Mock staking provider kullanıyoruz, gerçek staking setup'ı yapmıyoruz
		// Runtime'da conditional olarak MockStakingInfoProvider kullanılacak

		// Ölçümden önce, bu kullanıcının daha önce takibi başlatmadığından emin olalım.
		StakingStartBlock::<T>::remove(&caller);

		// EYLEM: Bu bloğun içindeki extrinsic çağrısının ne kadar sürdüğünü ölçüyoruz.
		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()));

		// DOĞRULAMA: Mock provider kullanıldığında bu başarılı olmalı
		assert!(StakingStartBlock::<T>::get(&caller).is_some());
	}

	// Benchmark test suite is in tests.rs with mock runtime
}
