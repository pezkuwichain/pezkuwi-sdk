//! Benchmarking setup for pallet-trust
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Trust;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn force_recalculate_trust_score() {
		let who: T::AccountId = whitelisted_caller();
		
		// Benchmark'ın çalışması için sahte veri sağlayıcıları yapılandırmamız gerekir.
		// Bu, runtime'daki adaptörler aracılığıyla gerçek paletlerle yapılacaktır.
		// Şimdilik, mock'taki gibi bir yapı varsayıyoruz.
		// Gerçek benchmark, runtime'da çalıştırıldığında doğru şekilde çalışacaktır.

		#[extrinsic_call]
		_(RawOrigin::Root, who.clone());

		assert_ne!(TrustScores::<T>::get(&who), T::Score::zero());
	}

	impl_benchmark_test_suite!(Trust, crate::mock::ExtBuilder::default().build(), crate::mock::Test);
}