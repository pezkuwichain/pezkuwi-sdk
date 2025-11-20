//! Benchmarking setup for pallet-presale

use super::*;

#[allow(unused)]
use crate::Pallet as Presale;
use frame_benchmarking::v2::*;
use frame_support::traits::{fungibles::Mutate, Get};
use frame_system::RawOrigin;
use sp_runtime::traits::{AccountIdConversion, StaticLookup};

#[benchmarks(
	where
		<T as pallet_assets::Config>::AssetId: From<u32>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_presale() {
		let caller: T::AccountId = whitelisted_caller();
		let payment_asset: <T as pallet_assets::Config>::AssetId = 2u32.into();
		let reward_asset: <T as pallet_assets::Config>::AssetId = 1u32.into();

		// Create assets first
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			reward_asset.clone().into(),
			T::Lookup::unlookup(caller.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			payment_asset.clone().into(),
			T::Lookup::unlookup(caller.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);

		// Fund caller with reward tokens for presale
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset.clone(),
			&caller,
			100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		#[extrinsic_call]
		create_presale(
			RawOrigin::Signed(caller),
			payment_asset,
			reward_asset,
			10_000_000_000u128, // tokens_for_sale
			100u32.into(),  // duration
			false,      // is_whitelist
			10_000_000u128, // min_contribution
			1_000_000_000u128, // max_contribution
			10_000_000_000u128, // hard_cap
			false,      // enable_vesting
			0u8,        // vesting_immediate_percent
			0u32.into(),    // vesting_duration_blocks
			0u32.into(),    // vesting_cliff_blocks
			24u32.into(),   // grace_period_blocks
			5u8,        // refund_fee_percent
			2u8,        // grace_refund_fee_percent
		);

		assert_eq!(NextPresaleId::<T>::get(), 1);
	}

	#[benchmark]
	fn contribute() {
		let caller: T::AccountId = whitelisted_caller();
		let owner: T::AccountId = account("owner", 0, 0);
		let payment_asset: <T as pallet_assets::Config>::AssetId = 2u32.into();
		let reward_asset: <T as pallet_assets::Config>::AssetId = 1u32.into();
		let amount: u128 = 100_000_000; // 100 USDT

		// Create assets first
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			reward_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			payment_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);

		// Setup: Owner creates presale with tokens_for_sale
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset.clone(),
			&owner,
			100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(owner).into(),
			payment_asset.clone(),
			reward_asset,
			10_000_000_000u128, // tokens_for_sale
			100u32.into(),
			false,
			10_000_000u128,
			1_000_000_000u128,
			10_000_000_000u128,
			false, 0u8, 0u32.into(), 0u32.into(), 24u32.into(), 5u8, 2u8,
		);

		// Fund caller with payment tokens
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			payment_asset.clone(),
			&caller,
			1_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		// Fund platform treasury and stakers pool with payment asset (needed for distribute_platform_fee)
		let treasury = T::PlatformTreasury::get();
		let stakers_pool = T::StakingRewardPool::get();
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			payment_asset.clone(),
			&treasury,
			1u128.try_into().ok().unwrap(),
		)
		.ok();
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			payment_asset,
			&stakers_pool,
			1u128.try_into().ok().unwrap(),
		)
		.ok();

		#[extrinsic_call]
		contribute(RawOrigin::Signed(caller.clone()), 0u32, amount);

		let contribution_info = Contributions::<T>::get(0, &caller).unwrap();
		assert_eq!(contribution_info.amount, amount);
	}

	#[benchmark]
	fn finalize_presale(n: Linear<1, 100>) {
		let owner: T::AccountId = account("owner", 0, 0);
		let payment_asset: <T as pallet_assets::Config>::AssetId = 2u32.into();
		let reward_asset: <T as pallet_assets::Config>::AssetId = 1u32.into();

		// Create assets first
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			reward_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			payment_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);

		// Setup: Owner creates presale
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset.clone(),
			&owner,
			100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(owner.clone()).into(),
			payment_asset.clone(),
			reward_asset.clone(),
			10_000_000_000u128, // tokens_for_sale
			100u32.into(),
			false,
			10_000_000u128,
			1_000_000_000u128,
			10_000_000_000u128,
			false, 0u8, 0u32.into(), 0u32.into(), 24u32.into(), 5u8, 2u8,
		);

		// Fund presale treasury with reward tokens
		let treasury = T::PalletId::get().into_sub_account_truncating(0u32);
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset,
			&treasury,
			100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		// Create n contributors
		for i in 0..n {
			let contributor: T::AccountId = account("contributor", i, 0);

			// Fund contributor
			<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
				payment_asset.clone(),
				&contributor,
				1_000_000_000u128.try_into().ok().unwrap(),
			)
			.ok();

			// Contribute
			let _ = Presale::<T>::contribute(RawOrigin::Signed(contributor).into(), 0u32, 100_000_000);
		}

		// Move to end of presale
		let current_block = frame_system::Pallet::<T>::block_number();
		frame_system::Pallet::<T>::set_block_number(current_block + 101u32.into());

		#[extrinsic_call]
		finalize_presale(RawOrigin::Root, 0u32);

		let presale = Presales::<T>::get(0).unwrap();
		assert!(matches!(presale.status, PresaleStatus::Finalized));
	}

	#[benchmark]
	fn refund() {
		let caller: T::AccountId = whitelisted_caller();
		let owner: T::AccountId = account("owner", 0, 0);
		let payment_asset: <T as pallet_assets::Config>::AssetId = 2u32.into();
		let reward_asset: <T as pallet_assets::Config>::AssetId = 1u32.into();
		let amount: u128 = 100_000_000;

		// Create assets first
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			reward_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			payment_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);

		// Setup: Owner creates presale
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset.clone(),
			&owner,
			100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(owner).into(),
			payment_asset.clone(),
			reward_asset,
			10_000_000_000u128,
			100u32.into(),
			false,
			10_000_000u128,
			1_000_000_000u128,
			10_000_000_000u128,
			false, 0u8, 0u32.into(), 0u32.into(), 24u32.into(), 5u8, 2u8,
		);

		// Fund caller and contribute
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			payment_asset,
			&caller,
			1_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		let _ = Presale::<T>::contribute(RawOrigin::Signed(caller.clone()).into(), 0u32, amount);

		#[extrinsic_call]
		refund(RawOrigin::Signed(caller.clone()), 0u32);

		let contribution_info = Contributions::<T>::get(0, &caller).unwrap();
		assert!(contribution_info.refunded);
	}

	#[benchmark]
	fn cancel_presale() {
		let owner: T::AccountId = whitelisted_caller();
		let payment_asset: <T as pallet_assets::Config>::AssetId = 2u32.into();
		let reward_asset: <T as pallet_assets::Config>::AssetId = 1u32.into();

		// Create assets first
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			reward_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			payment_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);

		// Fund owner
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset.clone(),
			&owner,
			100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		// Create presale
		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(owner.clone()).into(),
			payment_asset,
			reward_asset,
			10_000_000_000u128,
			100u32.into(),
			false,
			10_000_000u128,
			1_000_000_000u128,
			10_000_000_000u128,
			false, 0u8, 0u32.into(), 0u32.into(), 24u32.into(), 5u8, 2u8,
		);

		#[extrinsic_call]
		cancel_presale(RawOrigin::Root, 0u32);

		let presale = Presales::<T>::get(0).unwrap();
		assert!(matches!(presale.status, PresaleStatus::Cancelled));
	}

	#[benchmark]
	fn add_to_whitelist() {
		let owner: T::AccountId = whitelisted_caller();
		let user: T::AccountId = account("user", 0, 0);
		let payment_asset: <T as pallet_assets::Config>::AssetId = 2u32.into();
		let reward_asset: <T as pallet_assets::Config>::AssetId = 1u32.into();

		// Create assets first
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			reward_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);
		let _ = pallet_assets::Pallet::<T>::force_create(
			frame_system::RawOrigin::Root.into(),
			payment_asset.clone().into(),
			T::Lookup::unlookup(owner.clone()),
			true,
			1u128.try_into().ok().unwrap(),
		);

		// Fund owner
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset.clone(),
			&owner,
			100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		// Create whitelist presale
		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(owner.clone()).into(),
			payment_asset,
			reward_asset,
			10_000_000_000u128,
			100u32.into(),
			true, // whitelist enabled
			10_000_000u128,
			1_000_000_000u128,
			10_000_000_000u128,
			false, 0u8, 0u32.into(), 0u32.into(), 24u32.into(), 5u8, 2u8,
		);

		#[extrinsic_call]
		add_to_whitelist(RawOrigin::Signed(owner), 0u32, user);

		// Verify user was added (would need to check storage)
	}

	impl_benchmark_test_suite!(Presale, crate::mock::new_test_ext(), crate::mock::Test);
}
