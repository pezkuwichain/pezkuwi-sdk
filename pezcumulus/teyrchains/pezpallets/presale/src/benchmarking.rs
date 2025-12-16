//! Benchmarking setup for pezpallet-presale
//!
//! Complete benchmarks for all presale operations including:
//! - create_presale, cancel_presale, add_to_whitelist
//! - contribute, refund, claim_vested
//! - finalize_presale (with O(N) contributor loop)
//! - refund_cancelled_presale, batch_refund_failed_presale

#![cfg(feature = "runtime-benchmarks")]

use super::*;
#[allow(unused)]
use crate::Pezpallet as Presale;
use pezframe_benchmarking::v2::*;
use pezframe_support::traits::fungibles::{Create, Mutate};
use pezframe_system::RawOrigin;

/// Helper trait for benchmark asset setup
pub trait BenchmarkHelper<AssetId, AccountId> {
	/// Create an asset ID from seed
	fn create_asset_id(seed: u32) -> AssetId;
	/// Setup assets for benchmarking (create and mint)
	fn setup_assets(
		payment_asset: AssetId,
		reward_asset: AssetId,
		admin: &AccountId,
		accounts: &[AccountId],
		payment_amount: u128,
		reward_amount: u128,
	);
}

impl<AssetId: From<u32>, AccountId> BenchmarkHelper<AssetId, AccountId> for () {
	fn create_asset_id(seed: u32) -> AssetId {
		seed.into()
	}
	fn setup_assets(
		_payment_asset: AssetId,
		_reward_asset: AssetId,
		_admin: &AccountId,
		_accounts: &[AccountId],
		_payment_amount: u128,
		_reward_amount: u128,
	) {
		// Default implementation does nothing
		// Runtime should provide actual implementation
	}
}

#[benchmarks(
	where
		T::AssetId: From<u32>,
		T::Assets: Create<T::AccountId> + Mutate<T::AccountId>,
)]
mod benchmarks {
	use super::*;
	use pezframe_support::traits::{fungibles::Create, Get};

	fn get_asset_id<T: Config>(seed: u32) -> T::AssetId
	where
		T::AssetId: From<u32>,
	{
		seed.into()
	}

	/// Setup assets for presale benchmarking
	/// Creates payment and reward assets, mints to necessary accounts
	fn setup_benchmark_assets<T: Config>(
		caller: &T::AccountId,
		presale_treasury: &T::AccountId,
	) -> (T::AssetId, T::AssetId)
	where
		T::AssetId: From<u32>,
		T::Assets: Create<T::AccountId> + Mutate<T::AccountId>,
	{
		let payment_asset = get_asset_id::<T>(1);
		let reward_asset = get_asset_id::<T>(2);

		// Create assets if they don't exist (ignore errors if already created)
		let min_balance: T::Balance = 1u128.into();
		let _ = T::Assets::create(payment_asset.clone(), caller.clone(), true, min_balance);
		let _ = T::Assets::create(reward_asset.clone(), caller.clone(), true, min_balance);

		// Mint payment tokens to caller for contributions
		let payment_amount: T::Balance = 100_000_000u128.into();
		let _ = T::Assets::mint_into(payment_asset.clone(), caller, payment_amount);

		// Mint payment tokens to platform accounts for fee distribution
		let _ = T::Assets::mint_into(
			payment_asset.clone(),
			&T::PlatformTreasury::get(),
			payment_amount,
		);
		let _ = T::Assets::mint_into(
			payment_asset.clone(),
			&T::StakingRewardPool::get(),
			payment_amount,
		);

		// Mint reward tokens to presale treasury for distribution
		let reward_amount: T::Balance = 10_000_000_000u128.into();
		let _ = T::Assets::mint_into(reward_asset.clone(), presale_treasury, reward_amount);

		(payment_asset, reward_asset)
	}

	/// Create a presale with standard parameters
	fn create_test_presale<T: Config>(
		caller: &T::AccountId,
		payment_asset: T::AssetId,
		reward_asset: T::AssetId,
		is_whitelist: bool,
		enable_vesting: bool,
	) -> PresaleId
	where
		T::AssetId: From<u32>,
	{
		let presale_id = NextPresaleId::<T>::get();

		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(caller.clone()).into(),
			payment_asset,
			reward_asset,
			10_000_000_000u128, // tokens_for_sale (10M)
			1000u32.into(),     // duration (long enough for tests)
			is_whitelist,
			100u128,         // min_contribution
			10_000_000u128,  // max_contribution
			1_000_000u128,   // soft_cap
			100_000_000u128, // hard_cap
			enable_vesting,
			if enable_vesting { 20u8 } else { 0u8 }, // 20% immediate if vesting
			if enable_vesting { 100u32.into() } else { 0u32.into() }, // vesting_duration
			if enable_vesting { 10u32.into() } else { 0u32.into() }, // cliff
			10u32.into(),                            // grace_period_blocks
			5u8,                                     // refund_fee_percent
			2u8,                                     // grace_refund_fee_percent
		);

		presale_id
	}

	#[benchmark]
	fn create_presale() {
		let caller: T::AccountId = whitelisted_caller();
		let payment_asset = get_asset_id::<T>(1);
		let reward_asset = get_asset_id::<T>(2);

		#[extrinsic_call]
		create_presale(
			RawOrigin::Signed(caller),
			payment_asset,
			reward_asset,
			1_000_000u128, // tokens_for_sale
			100u32.into(), // duration
			false,         // is_whitelist
			100u128,       // min_contribution
			10_000u128,    // max_contribution
			500_000u128,   // soft_cap
			1_000_000u128, // hard_cap
			false,         // enable_vesting
			0u8,           // vesting_immediate_percent
			0u32.into(),   // vesting_duration_blocks
			0u32.into(),   // vesting_cliff_blocks
			10u32.into(),  // grace_period_blocks
			5u8,           // refund_fee_percent
			10u8,          // grace_refund_fee_percent
		);

		// Verify presale was created
		assert!(crate::Presales::<T>::contains_key(0));
	}

	#[benchmark]
	fn cancel_presale() {
		let caller: T::AccountId = whitelisted_caller();
		let payment_asset = get_asset_id::<T>(1);
		let reward_asset = get_asset_id::<T>(2);

		// Create a presale first
		let presale_id =
			create_test_presale::<T>(&caller, payment_asset, reward_asset, false, false);

		#[extrinsic_call]
		cancel_presale(RawOrigin::Root, presale_id);

		// Verify presale was cancelled
		let presale = crate::Presales::<T>::get(presale_id).unwrap();
		assert_eq!(presale.status, PresaleStatus::Cancelled);
	}

	#[benchmark]
	fn add_to_whitelist() {
		let owner: T::AccountId = whitelisted_caller();
		let user: T::AccountId = account("user", 0, 0);
		let payment_asset = get_asset_id::<T>(1);
		let reward_asset = get_asset_id::<T>(2);

		// Create a whitelist presale
		let presale_id = create_test_presale::<T>(&owner, payment_asset, reward_asset, true, false);

		#[extrinsic_call]
		add_to_whitelist(RawOrigin::Signed(owner), presale_id, user.clone());

		// Verify user was whitelisted
		assert!(crate::WhitelistedAccounts::<T>::get(presale_id, &user));
	}

	#[benchmark]
	fn contribute() {
		let caller: T::AccountId = whitelisted_caller();
		// Get next presale ID before creating
		let presale_id = NextPresaleId::<T>::get();
		let presale_treasury = Presale::<T>::presale_account_id(presale_id);

		// Setup assets
		let (payment_asset, reward_asset) = setup_benchmark_assets::<T>(&caller, &presale_treasury);

		// Create presale (will get the presale_id we calculated)
		let _ = create_test_presale::<T>(&caller, payment_asset, reward_asset, false, false);

		let amount: u128 = 10_000u128;

		#[extrinsic_call]
		contribute(RawOrigin::Signed(caller.clone()), presale_id, amount);

		// Verify contribution was recorded
		assert!(crate::Contributions::<T>::get(presale_id, &caller).is_some());
		assert!(crate::TotalRaised::<T>::get(presale_id) > 0);
	}

	#[benchmark]
	fn refund() {
		let caller: T::AccountId = whitelisted_caller();
		// Get next presale ID before creating
		let presale_id = NextPresaleId::<T>::get();
		let presale_treasury = Presale::<T>::presale_account_id(presale_id);

		// Setup assets
		let (payment_asset, reward_asset) = setup_benchmark_assets::<T>(&caller, &presale_treasury);

		// Create presale (will get the presale_id we calculated)
		let _ = create_test_presale::<T>(&caller, payment_asset, reward_asset, false, false);

		// Make a contribution first
		let amount: u128 = 10_000u128;
		let _ =
			Presale::<T>::contribute(RawOrigin::Signed(caller.clone()).into(), presale_id, amount);

		// Verify contribution exists
		assert!(crate::Contributions::<T>::get(presale_id, &caller).is_some());

		#[extrinsic_call]
		refund(RawOrigin::Signed(caller.clone()), presale_id);

		// Verify refund was processed
		let contribution = crate::Contributions::<T>::get(presale_id, &caller).unwrap();
		assert!(contribution.refunded);
	}

	#[benchmark]
	fn claim_vested() {
		let caller: T::AccountId = whitelisted_caller();
		// Get next presale ID before creating
		let presale_id = NextPresaleId::<T>::get();
		let presale_treasury = Presale::<T>::presale_account_id(presale_id);

		// Setup assets
		let (payment_asset, reward_asset) = setup_benchmark_assets::<T>(&caller, &presale_treasury);

		// Mint EXTRA reward tokens to presale treasury to prevent account death
		let extra_reward: T::Balance = 100_000_000_000u128.into();
		let _ = T::Assets::mint_into(reward_asset.clone(), &presale_treasury, extra_reward);

		// Create presale WITH vesting (will get the presale_id we calculated)
		let _ = create_test_presale::<T>(&caller, payment_asset, reward_asset, false, true);

		// Make a contribution
		let amount: u128 = 1_000_000u128; // Large enough to reach soft cap
		let _ =
			Presale::<T>::contribute(RawOrigin::Signed(caller.clone()).into(), presale_id, amount);

		// Advance blocks past presale end
		pezframe_system::Pezpallet::<T>::set_block_number(2000u32.into());

		// Finalize presale (requires root)
		let _ = Presale::<T>::finalize_presale(RawOrigin::Root.into(), presale_id);

		// Advance past cliff period
		pezframe_system::Pezpallet::<T>::set_block_number(3000u32.into());

		#[extrinsic_call]
		claim_vested(RawOrigin::Signed(caller.clone()), presale_id);

		// Verify claim was recorded
		let claimed = crate::VestingClaimed::<T>::get(presale_id, &caller);
		assert!(claimed > 0);
	}

	#[benchmark]
	fn refund_cancelled_presale() {
		let caller: T::AccountId = whitelisted_caller();
		// Get next presale ID before creating
		let presale_id = NextPresaleId::<T>::get();
		let presale_treasury = Presale::<T>::presale_account_id(presale_id);

		// Setup assets
		let (payment_asset, reward_asset) = setup_benchmark_assets::<T>(&caller, &presale_treasury);

		// Create presale (will get the presale_id we calculated)
		let _ =
			create_test_presale::<T>(&caller, payment_asset.clone(), reward_asset, false, false);

		// Make a contribution
		let amount: u128 = 10_000u128;
		let _ =
			Presale::<T>::contribute(RawOrigin::Signed(caller.clone()).into(), presale_id, amount);

		// Mint payment tokens to presale treasury for refund
		let refund_amount: T::Balance = 100_000u128.into();
		let _ = T::Assets::mint_into(payment_asset, &presale_treasury, refund_amount);

		// Cancel the presale
		let _ = Presale::<T>::cancel_presale(RawOrigin::Root.into(), presale_id);

		#[extrinsic_call]
		refund_cancelled_presale(RawOrigin::Signed(caller.clone()), presale_id);

		// Verify refund was processed
		let contribution = crate::Contributions::<T>::get(presale_id, &caller).unwrap();
		assert!(contribution.refunded);
	}

	/// Benchmark finalize_presale with variable number of contributors
	/// This is O(N) complexity - critical for proper weight calculation
	#[benchmark]
	fn finalize_presale(n: Linear<1, 100>) {
		let caller: T::AccountId = whitelisted_caller();
		// Get next presale ID before creating
		let presale_id = NextPresaleId::<T>::get();
		let presale_treasury = Presale::<T>::presale_account_id(presale_id);

		// Setup assets with enough for many contributors
		let (payment_asset, reward_asset) = setup_benchmark_assets::<T>(&caller, &presale_treasury);

		// Create presale (will get the presale_id we calculated)
		let _ = create_test_presale::<T>(
			&caller,
			payment_asset.clone(),
			reward_asset.clone(),
			false,
			false,
		);

		// Add n contributors
		for i in 0..n {
			let contributor: T::AccountId = account("contributor", i, 0);

			// Mint payment tokens to contributor
			let contribution_amount: T::Balance = 50_000u128.into();
			let _ = T::Assets::mint_into(payment_asset.clone(), &contributor, contribution_amount);

			// Make contribution
			let _ = Presale::<T>::contribute(
				RawOrigin::Signed(contributor).into(),
				presale_id,
				10_000u128,
			);
		}

		// Advance blocks past presale end
		pezframe_system::Pezpallet::<T>::set_block_number(2000u32.into());

		#[extrinsic_call]
		finalize_presale(RawOrigin::Root, presale_id);

		// Verify presale was finalized
		let presale = crate::Presales::<T>::get(presale_id).unwrap();
		assert!(
			presale.status == PresaleStatus::Finalized || presale.status == PresaleStatus::Failed
		);
	}

	/// Benchmark batch_refund_failed_presale with variable batch size
	/// This is also O(N) complexity
	#[benchmark]
	fn batch_refund_failed_presale(n: Linear<1, 100>) {
		let caller: T::AccountId = whitelisted_caller();
		// Get next presale ID before creating
		let presale_id = NextPresaleId::<T>::get();
		let presale_treasury = Presale::<T>::presale_account_id(presale_id);

		// Setup assets
		let (payment_asset, reward_asset) = setup_benchmark_assets::<T>(&caller, &presale_treasury);

		// Create presale with HIGH soft cap (will fail)
		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(caller.clone()).into(),
			payment_asset.clone(),
			reward_asset,
			10_000_000_000u128, // tokens_for_sale
			1000u32.into(),     // duration
			false,
			100u128,               // min_contribution
			10_000_000u128,        // max_contribution
			1_000_000_000_000u128, // soft_cap (very high - will fail)
			2_000_000_000_000u128, // hard_cap
			false,
			0u8,
			0u32.into(),
			0u32.into(),
			10u32.into(),
			5u8,
			2u8,
		);

		// Add n contributors (small amounts that won't reach soft cap)
		for i in 0..n {
			let contributor: T::AccountId = account("contributor", i, 0);

			// Mint payment tokens to contributor
			let contribution_amount: T::Balance = 50_000u128.into();
			let _ = T::Assets::mint_into(payment_asset.clone(), &contributor, contribution_amount);

			// Make small contribution
			let _ = Presale::<T>::contribute(
				RawOrigin::Signed(contributor).into(),
				presale_id,
				1_000u128,
			);
		}

		// Mint payment tokens to presale treasury for refunds
		let refund_pool: T::Balance = (n as u128 * 10_000u128).into();
		let _ = T::Assets::mint_into(payment_asset.clone(), &presale_treasury, refund_pool);

		// Advance blocks past presale end
		pezframe_system::Pezpallet::<T>::set_block_number(2000u32.into());

		// Finalize presale (will mark as Failed due to soft cap not reached)
		let _ = Presale::<T>::finalize_presale(RawOrigin::Root.into(), presale_id);

		// Verify presale failed
		let presale = crate::Presales::<T>::get(presale_id).unwrap();
		assert_eq!(presale.status, PresaleStatus::Failed);

		#[extrinsic_call]
		batch_refund_failed_presale(RawOrigin::Signed(caller), presale_id, 0, n);

		// Verify refunds were processed
		let first_contributor: T::AccountId = account("contributor", 0, 0);
		let contribution = crate::Contributions::<T>::get(presale_id, &first_contributor);
		if let Some(c) = contribution {
			assert!(c.refunded);
		}
	}

	impl_benchmark_test_suite!(Presale, crate::mock::new_test_ext(), crate::mock::Test);
}
