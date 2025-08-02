#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as PezTreasury;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn initialize_treasury() {
		// Setup - Treasury'yi reset et (clean state için)
		crate::TreasuryStartBlock::<T>::kill();
		crate::HalvingInfo::<T>::kill();
		crate::NextReleaseMonth::<T>::kill();

		#[extrinsic_call]
		initialize_treasury(RawOrigin::Root);

		// Verify treasury was initialized
		assert!(crate::TreasuryStartBlock::<T>::get().is_some());
		let halving_info = crate::HalvingInfo::<T>::get();
		assert_eq!(halving_info.current_period, 0);
		assert!(halving_info.monthly_amount > Zero::zero());
	}

	#[benchmark]
	fn force_genesis_distribution() {
		// Setup - account'ları temizle ve clean state sağla
		let treasury_account = PezTreasury::<T>::treasury_account_id();
		let presale_account = T::PresaleAccount::get();
		let founder_account = T::FounderAccount::get();

		// Existing balances'ları temizle (benchmark için clean state)
		// Not: Bu benchmark ortamı için güvenlidir
		let _ = T::Currency::make_free_balance_be(&treasury_account, Zero::zero());
		let _ = T::Currency::make_free_balance_be(&presale_account, Zero::zero());
		let _ = T::Currency::make_free_balance_be(&founder_account, Zero::zero());

		#[extrinsic_call]
		force_genesis_distribution(RawOrigin::Root);

		// Verify distribution happened
		assert!(T::Currency::free_balance(&treasury_account) > Zero::zero());
		assert!(T::Currency::free_balance(&presale_account) > Zero::zero());
		assert!(T::Currency::free_balance(&founder_account) > Zero::zero());
	}

	#[benchmark]
	fn release_monthly_funds() {
		// Setup - initialize treasury first ve clean state
		crate::TreasuryStartBlock::<T>::kill();
		crate::HalvingInfo::<T>::kill();
		crate::NextReleaseMonth::<T>::kill();
		crate::MonthlyReleases::<T>::remove_all(None);

		// Treasury'yi initialize et
		let _ = PezTreasury::<T>::do_genesis_distribution();
		let _ = PezTreasury::<T>::do_initialize_treasury();
		
		// Fast forward time to make release possible
		let current_block = frame_system::Pallet::<T>::block_number();
		let target_block = current_block + crate::BLOCKS_PER_MONTH.into() + 1u32.into();
		frame_system::Pallet::<T>::set_block_number(target_block);

		#[extrinsic_call]
		release_monthly_funds(RawOrigin::Root);

		// Verify funds were released to pots
		let incentive_balance = PezTreasury::<T>::get_incentive_pot_balance();
		let government_balance = PezTreasury::<T>::get_government_pot_balance();
		assert!(incentive_balance > Zero::zero());
		assert!(government_balance > Zero::zero());
	}

	impl_benchmark_test_suite!(PezTreasury, crate::mock::new_test_ext(), crate::mock::Test);
}