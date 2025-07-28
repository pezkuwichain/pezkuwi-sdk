//! Benchmarking setup for pallet-staking-score

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use crate::{Config, Pallet, StakingStartBlock};

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

	impl_benchmark_test_suite!(
		StakingScore,
		crate::mock::ExtBuilder::default().build(),
		crate::mock::Test,
	);
}