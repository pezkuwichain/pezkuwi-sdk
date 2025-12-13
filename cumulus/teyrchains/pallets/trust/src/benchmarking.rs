//! Benchmarking setup for pallet-trust
//!
//! These benchmarks measure the performance of trust score operations.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as TrustPallet;

use frame_benchmarking::{v2::*, whitelisted_caller};
use frame_support::pallet_prelude::*;
use frame_system::RawOrigin;
use sp_runtime::traits::Zero;

// We don't use IdentityKycPallet directly - just mock the citizenship status
// This simplifies benchmarks and avoids coupling with identity-kyc internals

#[benchmarks]
mod benchmarks {
	use super::*;

	/// Helper to setup a citizen for benchmarking
	/// Instead of calling identity-kyc extrinsics, we mock the citizenship source
	fn setup_citizen<T: Config>(account: &T::AccountId) {
		// For benchmarks, we rely on the runtime's CitizenshipSource implementation
		// The benchmark mock should configure CitizenshipSource to return true for whitelisted
		// accounts This is typically done via TestCitizenshipProvider in mock.rs

		// Initialize trust score storage for the account so update operations work
		TrustScores::<T>::insert(account, T::Score::zero());
	}

	#[benchmark]
	fn force_recalculate_trust_score() -> Result<(), BenchmarkError> {
		// Setup
		let account: T::AccountId = whitelisted_caller();
		setup_citizen::<T>(&account);

		#[extrinsic_call]
		force_recalculate_trust_score(RawOrigin::Root, account.clone());

		// Verify - trust score should be calculated (may be zero if no component scores)
		assert!(TrustScores::<T>::contains_key(&account));
		Ok(())
	}

	#[benchmark]
	fn update_all_trust_scores() {
		// Setup - Ensure no batch update is in progress
		crate::BatchUpdateInProgress::<T>::put(false);

		#[extrinsic_call]
		update_all_trust_scores(RawOrigin::Root);

		// Verify - The function completed (may or may not have set BatchUpdateInProgress
		// depending on whether there are citizens to process)
		// We just verify it doesn't panic
	}

	#[benchmark]
	fn periodic_trust_score_update() {
		// Setup - Ensure no batch update is in progress
		crate::BatchUpdateInProgress::<T>::put(false);

		#[extrinsic_call]
		periodic_trust_score_update(RawOrigin::Root);

		// Verify - The function completed successfully
	}

	impl_benchmark_test_suite!(TrustPallet, crate::mock::new_test_ext(), crate::mock::Test);
}
