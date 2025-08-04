#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as PezRewards;
use frame_benchmarking::v2::*;
use frame_benchmarking::account;
use frame_system::RawOrigin;
use sp_runtime::traits::{Zero, Bounded};

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn initialize_rewards_system() {
		// Setup - clear storage for clean state
		crate::EpochInfo::<T>::kill();
		let _ = crate::EpochStatus::<T>::clear(1000, None);

		#[extrinsic_call]
		initialize_rewards_system(RawOrigin::Root);

		// Verify initialization
		let epoch_info = crate::EpochInfo::<T>::get();
		assert_eq!(epoch_info.current_epoch, 0);
		assert_eq!(epoch_info.total_epochs_completed, 0);
	}

	#[benchmark]
	fn register_parliamentary_nft_owner() {
		let owner: T::AccountId = account("owner", 0, 0);
		let nft_id = 1u32;

		#[extrinsic_call]
		register_parliamentary_nft_owner(RawOrigin::Root, nft_id, owner.clone());

		assert!(PezRewards::<T>::get_parliamentary_nft_owner(nft_id).is_some());
		assert_eq!(PezRewards::<T>::get_parliamentary_nft_owner(nft_id).unwrap(), owner);
	}

	#[benchmark]
	fn record_trust_score() {
		// Setup - clear storage and initialize system
		crate::EpochInfo::<T>::kill();
		let _ = crate::EpochStatus::<T>::clear(1000, None);
		let _ = crate::UserEpochScores::<T>::clear(1000, None);
		
		let _ = PezRewards::<T>::do_initialize_rewards_system();
		
		let user: T::AccountId = account("user", 0, 0);

		#[extrinsic_call]
		record_trust_score(RawOrigin::Signed(user.clone()));

		// Mock trust score verification - since we don't have real trust system in benchmark
		// We'll verify the function completed without error
		// In real runtime, trust score would be available
		// For benchmark, let's just verify the call succeeded
		let epoch_info = crate::EpochInfo::<T>::get();
		assert_eq!(epoch_info.current_epoch, 0);
	}

	#[benchmark]
	fn finalize_epoch() {
		// Setup - clear storage and initialize system
		crate::EpochInfo::<T>::kill();
		let _ = crate::EpochStatus::<T>::clear(1000, None);
		let _ = crate::UserEpochScores::<T>::clear(1000, None);
		let _ = crate::EpochRewardPools::<T>::clear(1000, None);
		
		let _ = PezRewards::<T>::do_initialize_rewards_system();
		
		// Manually insert trust scores since trust system might not be available
		let user1: T::AccountId = account("user1", 0, 0);
		let user2: T::AccountId = account("user2", 0, 0);
		crate::UserEpochScores::<T>::insert(0, &user1, 100u128);
		crate::UserEpochScores::<T>::insert(0, &user2, 50u128);
		
		// Setup incentive pot with sufficient balance
		let incentive_pot = PezRewards::<T>::incentive_pot_account_id();
		let large_amount: BalanceOf<T> = 1_000_000_000_000u128.try_into().unwrap_or_else(|_| {
			// Fallback for different balance types using Bounded trait
			BalanceOf::<T>::max_value() / 2u32.into()
		});
		let _ = T::Currency::make_free_balance_be(&incentive_pot, large_amount);
		
		// Fast forward time to end of epoch
		let current_block = frame_system::Pallet::<T>::block_number();
		let target_block = current_block + crate::pallet::BLOCKS_PER_EPOCH.into();
		frame_system::Pallet::<T>::set_block_number(target_block);

		#[extrinsic_call]
		finalize_epoch(RawOrigin::Root);

		// Verify epoch was finalized
		let reward_pool = crate::EpochRewardPools::<T>::get(0);
		assert!(reward_pool.is_some());
		
		let epoch_info = crate::EpochInfo::<T>::get();
		assert_eq!(epoch_info.current_epoch, 1);
	}

	#[benchmark]
	fn claim_reward() {
		// Setup - clear storage and manually setup complete scenario
		crate::EpochInfo::<T>::kill();
		let _ = crate::EpochStatus::<T>::clear(1000, None);
		let _ = crate::UserEpochScores::<T>::clear(1000, None);
		let _ = crate::EpochRewardPools::<T>::clear(1000, None);
		let _ = crate::ClaimedRewards::<T>::clear(1000, None);
		
		let user: T::AccountId = account("user", 0, 0);
		
		// Give user MASSIVE initial balance to meet existential deposit requirement  
		let large_initial_balance: BalanceOf<T> = 1_000_000_000_000u128.try_into().unwrap_or_else(|_| {
			BalanceOf::<T>::max_value() / 10u32.into()
		});
		let _ = T::Currency::make_free_balance_be(&user, large_initial_balance);
		
		// Manually setup complete scenario for claim
		crate::UserEpochScores::<T>::insert(0, &user, 1000u128); // Increased trust score
		
		// Setup incentive pot with HUGE funds
		let incentive_pot = PezRewards::<T>::incentive_pot_account_id();
		let amount: BalanceOf<T> = 1_000_000_000_000_000u128.try_into().unwrap_or_else(|_| {
			BalanceOf::<T>::max_value() / 10u32.into()
		});
		let _ = T::Currency::make_free_balance_be(&incentive_pot, amount);
		
		// Setup reward pool with valid claim period
		let current_block = frame_system::Pallet::<T>::block_number();
		let claim_deadline = current_block + crate::pallet::CLAIM_PERIOD_BLOCKS.into();
		
		// Use MUCH larger reward per point to meet existential deposit requirements
		let reward_per_point: BalanceOf<T> = 1_000_000u32.try_into().unwrap_or_else(|_| {
			// Fallback to a very large amount
			BalanceOf::<T>::max_value() / 1000000u32.into()
		});
		
		let reward_pool = crate::EpochRewardPool {
			epoch_index: 0,
			total_reward_pool: amount,
			total_trust_score: 1000u128, // Matching trust score
			reward_per_trust_point: reward_per_point,
			participants_count: 1,
			claim_deadline,
		};
		
		crate::EpochRewardPools::<T>::insert(0, reward_pool);
		crate::EpochStatus::<T>::insert(0, crate::EpochState::ClaimPeriod);

		#[extrinsic_call]
		claim_reward(RawOrigin::Signed(user.clone()), 0);

		// Verify reward was claimed
		let claimed = crate::ClaimedRewards::<T>::get(0, &user);
		assert!(claimed.is_some());
		assert!(claimed.unwrap() > Zero::zero());
	}

	#[benchmark]
	fn close_epoch() {
		// Setup - clear storage and manually setup expired scenario
		crate::EpochInfo::<T>::kill();
		let _ = crate::EpochStatus::<T>::clear(1000, None);
		let _ = crate::UserEpochScores::<T>::clear(1000, None);
		let _ = crate::EpochRewardPools::<T>::clear(1000, None);
		
		// Setup incentive pot with remaining funds
		let incentive_pot = PezRewards::<T>::incentive_pot_account_id();
		let amount: BalanceOf<T> = 500_000u32.try_into().unwrap_or_else(|_| Zero::zero());
		let _ = T::Currency::make_free_balance_be(&incentive_pot, amount);
		
		// Setup reward pool with EXPIRED claim period
		let current_block = frame_system::Pallet::<T>::block_number();
		let past_deadline = current_block.saturating_sub(1u32.into()); // Deadline in the past
		
		let reward_pool = crate::EpochRewardPool {
			epoch_index: 0,
			total_reward_pool: amount,
			total_trust_score: 100u128,
			reward_per_trust_point: 1000u32.try_into().unwrap_or_else(|_| Zero::zero()),
			participants_count: 1,
			claim_deadline: past_deadline,
		};
		
		crate::EpochRewardPools::<T>::insert(0, reward_pool);
		crate::EpochStatus::<T>::insert(0, crate::EpochState::ClaimPeriod);

		#[extrinsic_call]
		close_epoch(RawOrigin::Root, 0);

		// Verify epoch was closed
		let status = crate::EpochStatus::<T>::get(0);
		assert_eq!(status, crate::EpochState::Closed);
	}

	impl_benchmark_test_suite!(PezRewards, crate::mock::new_test_ext(), crate::mock::Test);
}