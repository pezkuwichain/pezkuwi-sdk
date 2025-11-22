//! Benchmarking setup for pallet-presale

use super::*;
#[allow(unused)]
use crate::Pallet as Presale;
use frame_benchmarking::v2::*;
use frame_support::traits::{fungibles, fungibles::Mutate, Currency, Get, tokens::Preservation};
use frame_system::RawOrigin;
use log::info;
use pallet_balances::Pallet as Balances;
use sp_runtime::traits::{AccountIdConversion, StaticLookup};

#[benchmarks(
	where
		<T as pallet_assets::Config>::AssetId: From<u32>,
		T: pallet_balances::Config,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_presale() {
		let caller: T::AccountId = whitelisted_caller();
		let payment_asset: <T as pallet_assets::Config>::AssetId = 1000u32.into();
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
		
		        // Touch the presale treasury account to create the AssetAccount before transfer
		        let presale_treasury_account = Presale::<T>::presale_account_id(0);
		        <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
		            reward_asset.clone(),
		            &presale_treasury_account,
		            1u128.try_into().ok().unwrap(),
		        ).unwrap();
		
		
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
			5_000_000_000u128, // soft_cap
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
		let payment_asset: <T as pallet_assets::Config>::AssetId = 1000u32.into();
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
			RawOrigin::Signed(owner.clone()).into(),
			payment_asset.clone(),
			reward_asset.clone(),
			10_000_000_000u128, // tokens_for_sale
			100u32.into(),
			false,
			10_000_000u128,
			1_000_000_000u128,
			5_000_000_000u128, // soft_cap
			10_000_000_000u128,
			false, 0u8, 0u32.into(), 0u32.into(), 24u32.into(), 5u8, 2u8,
		);

		// Fund presale treasury with reward tokens (required for contributions)
		let pallet_treasury = T::PalletId::get().into_sub_account_truncating(0u32);
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset,
			&pallet_treasury,
			100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

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
	fn finalize_presale(n: Linear<25, 100>) { // Min 25 to reach 5B soft_cap (25 * 200M = 5B)
		let owner: T::AccountId = account("owner", 0, 0);
		let payment_asset: <T as pallet_assets::Config>::AssetId = 1000u32.into();
		let reward_asset: <T as pallet_assets::Config>::AssetId = 1u32.into(); // Unique ID for benchmark
		let presale_treasury_account = Presale::<T>::presale_account_id(0);

		// Fund owner and presale treasury with native currency for existential deposits
		let existential_deposit = Balances::<T>::minimum_balance();
		let _ = <Balances<T> as Currency<T::AccountId>>::make_free_balance_be(&owner, existential_deposit * 1000u32.into());
		let _ = <Balances<T> as Currency<T::AccountId>>::make_free_balance_be(&presale_treasury_account, existential_deposit * 1000u32.into());


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
		// Mint enough reward tokens to owner for tokens_for_sale transfer
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset.clone(),
			&owner,
			1_000_000_000_000_000_000_000_000u128.try_into().ok().unwrap(), // 100 quintillion * 1000 = 100 sextillion
		)
		.ok();

		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(owner.clone()).into(),
			payment_asset.clone(),
			reward_asset.clone(),
			1_000_000_000_000u128, // tokens_for_sale (increased significantly)
			100u32.into(),
			false,
			10_000_000u128,
			1_000_000_000u128,
			5_000_000_000u128, // soft_cap (n=25 * 200M = 5B)
			            25_000_000_000u128, // hard_cap (n=100 * 200M = 20B, rounded to 25B)
						false, 0u8, 0u32.into(), 0u32.into(), 24u32.into(), 5u8, 2u8,
					);
			
					// MANUALLY transfer tokens_for_sale to bypass internal transfer issues in benchmark env
					let tokens_for_sale = 1_000_000_000_000u128;
							<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
								reward_asset.clone(),
								&owner,
								&presale_treasury_account,
								tokens_for_sale.try_into().ok().unwrap(),
								Preservation::Preserve,
							).unwrap();			
			
		let presale_treasury_account = Presale::<T>::presale_account_id(0);
		let treasury_balance_after_creation = <pallet_assets::Pallet<T> as fungibles::Inspect<T::AccountId>>::balance(reward_asset.clone(), &presale_treasury_account);
		info!("create_presale sonrası presale hazine bakiyesi: {:?}", treasury_balance_after_creation);


		// create_presale automatically transfers tokens_for_sale from owner to treasury

		// Fund platform treasury and stakers pool ONCE (before loop) with sufficient amounts
		let treasury_account = T::PlatformTreasury::get();
		let stakers_account = T::StakingRewardPool::get();

		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			payment_asset.clone(),
			&treasury_account,
			1_000_000_000_000u128.try_into().ok().unwrap(), // 100B for all contributions
		)
		.unwrap();
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			payment_asset.clone(),
			&stakers_account,
			1_000_000_000_000u128.try_into().ok().unwrap(), // 100B for all contributions
		)
		.unwrap();

		// Create n contributors - each contributes a fixed amount
		// soft_cap = 5_000_000_000
		// Each contribution: 200_000_000
		// With n >= 25, total will reach soft_cap (25 * 200_000_000 = 5_000_000_000)
		let contribution_amount = 200_000_000u128;
		for i in 0..n {
			let contributor: T::AccountId = account("contributor", i, 0);

			// Fund contributor with payment asset + large buffer for existential deposit
			<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
				payment_asset.clone(),
				&contributor,
				(contribution_amount + 1_000_000_000_000u128).try_into().ok().unwrap(), // 1B buffer
			)
			.unwrap();

			// Contribute - MUST succeed
			Presale::<T>::contribute(RawOrigin::Signed(contributor).into(), 0u32, contribution_amount).unwrap();
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
		let payment_asset: <T as pallet_assets::Config>::AssetId = 1000u32.into();
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
			RawOrigin::Signed(owner.clone()).into(),
			payment_asset.clone(),
			reward_asset.clone(),
			10_000_000_000u128,
			100u32.into(),
			false,
			10_000_000u128,
			1_000_000_000u128,
			5_000_000_000u128, // soft_cap
			10_000_000_000u128,
			false, 0u8, 0u32.into(), 0u32.into(), 24u32.into(), 5u8, 2u8,
		);

		// Fund presale treasury with reward tokens
		let pallet_treasury = T::PalletId::get().into_sub_account_truncating(0u32);
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			reward_asset,
			&pallet_treasury,
			100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
		)
		.ok();

		// Fund caller with enough for contribution + large existential buffer
		// amount (100M) + buffer (1B) = 1.1B to ensure account stays alive through all operations
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			payment_asset.clone(),
			&caller,
			(amount + 1_000_000_000u128).try_into().ok().unwrap(),
		)
		.unwrap();

		// Fund platform treasury and stakers pool with large buffers
		let treasury = T::PlatformTreasury::get();
		let stakers_pool = T::StakingRewardPool::get();
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			payment_asset.clone(),
			&treasury,
			1_000_000_000u128.try_into().ok().unwrap(),
		)
		.unwrap();
		<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
			payment_asset.clone(),
			&stakers_pool,
			1_000_000_000u128.try_into().ok().unwrap(),
		)
		.unwrap();

		// Contribute - MUST succeed (will transfer 98M to presale treasury, distribute 2M fee from caller)
		Presale::<T>::contribute(RawOrigin::Signed(caller.clone()).into(), 0u32, amount).unwrap();

		// Treasury now has 98M from contribution
		// Refund will calculate: net = 98M, fee = 4.9M, refund_amount = 93.1M
		// Treasury will distribute 4.9M as: 2.45M treasury + 1.225M burn + 1.225M stakers
		// After refund, treasury should have: 98M - 93.1M - 4.9M = 0M (account can be destroyed with Expendable)

		// Refund is for Active presales, so don't finalize

		#[extrinsic_call]
		refund(RawOrigin::Signed(caller.clone()), 0u32);

		let contribution_info = Contributions::<T>::get(0, &caller).unwrap();
		assert!(contribution_info.refunded);
	}

	#[benchmark]
	fn cancel_presale() {
		let owner: T::AccountId = whitelisted_caller();
		let payment_asset: <T as pallet_assets::Config>::AssetId = 1000u32.into();
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
			5_000_000_000u128, // soft_cap
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
		let payment_asset: <T as pallet_assets::Config>::AssetId = 1000u32.into();
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
			5_000_000_000u128, // soft_cap
			10_000_000_000u128,
			false, 0u8, 0u32.into(), 0u32.into(), 24u32.into(), 5u8, 2u8,
		);

		#[extrinsic_call]
		add_to_whitelist(RawOrigin::Signed(owner), 0u32, user);

		// Verify user was added (would need to check storage)
	}

	impl_benchmark_test_suite!(Presale, crate::mock::new_test_ext(), crate::mock::Test);
}
