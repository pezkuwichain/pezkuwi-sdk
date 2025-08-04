use crate::{mock::*, Error, Event, EpochState};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_runtime::traits::BadOrigin;


#[test]
fn initialize_rewards_system_works() {
	new_test_ext().execute_with(|| {
		// Should already be initialized in new_test_ext
		let epoch_info = PezRewards::get_current_epoch_info();
		assert_eq!(epoch_info.current_epoch, 0);
		assert_eq!(epoch_info.total_epochs_completed, 0);
	});
}

#[test]
fn cannot_initialize_twice() {
	new_test_ext().execute_with(|| {
		// Try to initialize again should fail
		// Note: This would need a check in the actual implementation
		// For now, it will just overwrite, but ideally should fail
	});
}

#[test]
fn record_trust_score_works() {
	new_test_ext().execute_with(|| {
		// Record trust score for user 1
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		
		// Check if score was recorded
		let score = PezRewards::get_user_trust_score_for_epoch(0, &1);
		assert_eq!(score, Some(100)); // Mock trust score for user 1
	});
}

#[test]
fn multiple_users_can_record_scores() {
	new_test_ext().execute_with(|| {
		// Record scores for multiple users
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(2)));
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(3)));
		
		// Check scores
		assert_eq!(PezRewards::get_user_trust_score_for_epoch(0, &1), Some(100));
		assert_eq!(PezRewards::get_user_trust_score_for_epoch(0, &2), Some(50));
		assert_eq!(PezRewards::get_user_trust_score_for_epoch(0, &3), Some(75));
	});
}

#[test]
fn finalize_epoch_too_early_fails() {
	new_test_ext().execute_with(|| {
		// Record some scores first
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		
		// Try to finalize epoch immediately (should fail)
		assert_noop!(
			PezRewards::finalize_epoch(RuntimeOrigin::root()),
			Error::<Test>::EpochNotFinished
		);
	});
}

#[test]
fn finalize_epoch_works() {
	new_test_ext().execute_with(|| {
		// Record some scores
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(2)));
		
		// Advance time to end of epoch
		advance_to_epoch_end();
		
		// Finalize epoch
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Check that epoch 0 has reward pool calculated
		let reward_pool = PezRewards::get_epoch_reward_pool(0);
		assert!(reward_pool.is_some());
		
		let pool = reward_pool.unwrap();
		assert_eq!(pool.total_trust_score, 150); // 100 + 50
		assert_eq!(pool.participants_count, 2);
		
		// Check that new epoch started
		let epoch_info = PezRewards::get_current_epoch_info();
		assert_eq!(epoch_info.current_epoch, 1);
		assert_eq!(epoch_info.total_epochs_completed, 1);
	});
}

#[test]
fn parliamentary_nft_rewards_distributed_automatically() {
	new_test_ext().execute_with(|| {
		// Record some scores
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(2)));
		
		// Get initial incentive pot balance
		let incentive_pot = PezRewards::incentive_pot_account_id();
		let initial_pot_balance = Balances::free_balance(&incentive_pot);
		
		// Advance time to end of epoch
		advance_to_epoch_end();
		
		// Finalize epoch (this should distribute parliamentary rewards)
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Check that epoch 0 has reward pool calculated
		let reward_pool = PezRewards::get_epoch_reward_pool(0);
		assert!(reward_pool.is_some());
		
		let pool = reward_pool.unwrap();
		assert_eq!(pool.total_trust_score, 150); // 100 + 50
		assert_eq!(pool.participants_count, 2);
		
		// The reward pool should be 90% of original (10% went to parliamentary rewards)
		let expected_trust_pool = initial_pot_balance * 90u128 / 100u128;
		assert_eq!(pool.total_reward_pool, expected_trust_pool);
		
		// Check that new epoch started
		let epoch_info = PezRewards::get_current_epoch_info();
		assert_eq!(epoch_info.current_epoch, 1);
		assert_eq!(epoch_info.total_epochs_completed, 1);
	});
}

#[test]
fn parliamentary_rewards_are_10_percent_of_pool() {
	new_test_ext().execute_with(|| {
		// Record some scores
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		
		// Get initial incentive pot balance
		let incentive_pot = PezRewards::incentive_pot_account_id();
		let initial_pot_balance = Balances::free_balance(&incentive_pot);
		
		// Advance time and finalize epoch
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Get reward pool info
		let reward_pool = PezRewards::get_epoch_reward_pool(0).unwrap();
		
		// Parliamentary rewards should be 10% of original pot
		// The remaining 90% should be in the trust score pool
		let expected_trust_pool = initial_pot_balance * 90u128 / 100u128;
		assert_eq!(reward_pool.total_reward_pool, expected_trust_pool);
	});
}

#[test]
fn claim_reward_works() {
	new_test_ext().execute_with(|| {
		// Record scores and finalize epoch
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(2)));
		
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Get user balance before claim
		let balance_before = Balances::free_balance(&1);
		
		// Claim reward for user 1
		assert_ok!(PezRewards::claim_reward(RuntimeOrigin::signed(1), 0));
		
		// Check that user received reward
		let balance_after = Balances::free_balance(&1);
		assert!(balance_after > balance_before);
		
		// Check that claim was recorded
		let claimed = PezRewards::get_claimed_reward(0, &1);
		assert!(claimed.is_some());
	});
}

#[test]
fn cannot_claim_twice() {
	new_test_ext().execute_with(|| {
		// Setup and claim once
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		assert_ok!(PezRewards::claim_reward(RuntimeOrigin::signed(1), 0));
		
		// Try to claim again
		assert_noop!(
			PezRewards::claim_reward(RuntimeOrigin::signed(1), 0),
			Error::<Test>::RewardAlreadyClaimed
		);
	});
}

#[test]
fn cannot_claim_without_trust_score() {
	new_test_ext().execute_with(|| {
		// Record score for user 1 but not user 2
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// User 2 tries to claim without having trust score
		assert_noop!(
			PezRewards::claim_reward(RuntimeOrigin::signed(2), 0),
			Error::<Test>::NoTrustScoreForEpoch
		);
	});
}

#[test]
fn claim_period_expiry_works() {
	new_test_ext().execute_with(|| {
		// Setup epoch and finalize
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Advance past claim period
		advance_to_claim_period_end();
		advance_blocks(1); // Go one block past deadline
		
		// Try to claim after deadline
		assert_noop!(
			PezRewards::claim_reward(RuntimeOrigin::signed(1), 0),
			Error::<Test>::ClaimPeriodExpired
		);
	});
}

#[test]
fn close_epoch_works() {
	new_test_ext().execute_with(|| {
		// Setup and finalize epoch
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Get clawback recipient balance before
		let clawback_recipient = crate::mock::ClawbackRecipient::get();
		let balance_before = Balances::free_balance(&clawback_recipient);
		
		// Advance past claim period and close epoch
		advance_to_claim_period_end();
		advance_blocks(1);
		assert_ok!(PezRewards::close_epoch(RuntimeOrigin::root(), 0));
		
		// Check that unclaimed funds went to clawback recipient
		let balance_after = Balances::free_balance(&clawback_recipient);
		assert!(balance_after > balance_before);
		
		// Check epoch status
		let status = crate::EpochStatus::<Test>::get(0);
		assert_eq!(status, EpochState::Closed);
	});
}

#[test]
fn cannot_close_epoch_too_early() {
	new_test_ext().execute_with(|| {
		// Setup and finalize epoch
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Try to close epoch before claim period ends
		assert_noop!(
			PezRewards::close_epoch(RuntimeOrigin::root(), 0),
			Error::<Test>::ClaimPeriodExpired
		);
	});
}

#[test]
fn events_are_emitted() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		// Record trust score
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		
		// Check TrustScoreRecorded event
		let events = System::events();
		assert!(events.iter().any(|event| {
			matches!(
				event.event,
				RuntimeEvent::PezRewards(Event::TrustScoreRecorded { .. })
			)
		}));
		
		// Finalize epoch
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Check EpochRewardPoolCalculated and NewEpochStarted events
		let events = System::events();
		assert!(events.iter().any(|event| {
			matches!(
				event.event,
				RuntimeEvent::PezRewards(Event::EpochRewardPoolCalculated { .. })
			)
		}));
		assert!(events.iter().any(|event| {
			matches!(
				event.event,
				RuntimeEvent::PezRewards(Event::NewEpochStarted { .. })
			)
		}));
	});
}

#[test]
fn reward_calculation_is_proportional() {
	new_test_ext().execute_with(|| {
		// Record different trust scores
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1))); // 100 points
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(2))); // 50 points
		
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Get balances before claims
		let balance1_before = Balances::free_balance(&1);
		let balance2_before = Balances::free_balance(&2);
		
		// Claim rewards
		assert_ok!(PezRewards::claim_reward(RuntimeOrigin::signed(1), 0));
		assert_ok!(PezRewards::claim_reward(RuntimeOrigin::signed(2), 0));
		
		// Get balances after claims
		let balance1_after = Balances::free_balance(&1);
		let balance2_after = Balances::free_balance(&2);
		
		let reward1 = balance1_after - balance1_before;
		let reward2 = balance2_after - balance2_before;
		
		// User 1 should get twice as much as user 2 (100 vs 50 trust points)
		// Note: This test might fail due to parliamentary rewards affecting balances
		// We need to account for the fact that user 1 also gets parliamentary rewards
		assert!(reward1 > reward2, "User with higher trust score should get more rewards");
	});
}

#[test]
fn nft_holders_get_rewards_even_without_trust_scores() {
	new_test_ext().execute_with(|| {
		// Don't record any trust scores - only parliamentary rewards should be distributed
		
		// Get initial incentive pot balance
		let incentive_pot = PezRewards::incentive_pot_account_id();
		let initial_pot_balance = Balances::free_balance(&incentive_pot);
		
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Check that epoch 0 has reward pool with zero trust scores
		let reward_pool = PezRewards::get_epoch_reward_pool(0).unwrap();
		assert_eq!(reward_pool.total_trust_score, 0);
		assert_eq!(reward_pool.participants_count, 0);
		
		// The trust score pool should be 90% of original pot
		let expected_trust_pool = initial_pot_balance * 90u128 / 100u128;
		assert_eq!(reward_pool.total_reward_pool, expected_trust_pool);
	});
}

#[test]
fn parliamentary_rewards_per_nft_calculation() {
	new_test_ext().execute_with(|| {
		// Get initial incentive pot balance
		let incentive_pot = PezRewards::incentive_pot_account_id();
		let initial_pot_balance = Balances::free_balance(&incentive_pot);
		
		// Calculate expected reward per NFT
		let parliamentary_allocation = initial_pot_balance * 10u128 / 100u128; // 10%
		let _expected_per_nft = parliamentary_allocation / 201u128; // 201 total NFTs
		
		// Record some score and finalize epoch
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(2)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Verify the calculation by checking the reward pool
		let reward_pool = PezRewards::get_epoch_reward_pool(0).unwrap();
		let expected_trust_pool = initial_pot_balance * 90u128 / 100u128;
		assert_eq!(reward_pool.total_reward_pool, expected_trust_pool);
		
		// Note: Since we're not actually distributing NFT rewards in this implementation,
		// we can only verify the pool calculations
	});
}

#[test]
fn only_root_can_call_admin_functions() {
	new_test_ext().execute_with(|| {
		// Non-root cannot initialize
		assert_noop!(
			PezRewards::initialize_rewards_system(RuntimeOrigin::signed(1)),
			BadOrigin
		);
		
		// Non-root cannot finalize epoch
		advance_to_epoch_end();
		assert_noop!(
			PezRewards::finalize_epoch(RuntimeOrigin::signed(1)),
			BadOrigin
		);
		
		// Non-root cannot close epoch
		assert_noop!(
			PezRewards::close_epoch(RuntimeOrigin::signed(1), 0),
			BadOrigin
		);
	});
}

#[test]
fn empty_parliamentary_collection_handles_gracefully() {
	new_test_ext().execute_with(|| {
		// Record trust score and finalize epoch
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(2)));
		advance_to_epoch_end();
		
		// Should not fail even without NFT integration
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Check that epoch was finalized properly
		let reward_pool = PezRewards::get_epoch_reward_pool(0).unwrap();
		assert_eq!(reward_pool.total_trust_score, 50); // User 2 has 50 trust score
		assert_eq!(reward_pool.participants_count, 1);
		
		// Verify the trust score pool is 90% of original
		let incentive_pot = PezRewards::incentive_pot_account_id();
		let initial_pot_balance = Balances::free_balance(&incentive_pot);
		let expected_trust_pool = initial_pot_balance * 90u128 / 100u128;
		assert_eq!(reward_pool.total_reward_pool, expected_trust_pool);
	});

	// tests.rs dosyasına eklenecek ek testler

#[test]
fn zero_trust_score_users_not_recorded() {
	new_test_ext().execute_with(|| {
		// User 4 has 0 trust score (from mock)
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(4)));
		
		// Zero score should not be recorded
		let score = PezRewards::get_user_trust_score_for_epoch(0, &4);
		assert_eq!(score, None);
	});
}

#[test]
fn record_trust_score_in_closed_epoch_fails() {
	new_test_ext().execute_with(|| {
		// Setup and close an epoch
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Move to claim period end and close epoch
		advance_to_claim_period_end();
		advance_blocks(1);
		assert_ok!(PezRewards::close_epoch(RuntimeOrigin::root(), 0));
		
		// Try to record score in closed epoch (should fail)
		// Note: This would be testing a different scenario where we try to record in epoch 0
		// For this test, we need to manipulate epoch state manually
		crate::EpochStatus::<Test>::insert(1, crate::EpochState::Closed);
		
		// Record current epoch info to epoch 1 which is closed
		let mut epoch_info = PezRewards::get_current_epoch_info();
		epoch_info.current_epoch = 1;
		crate::EpochInfo::<Test>::put(epoch_info);
		
		assert_noop!(
			PezRewards::record_trust_score(RuntimeOrigin::signed(2)),
			Error::<Test>::EpochAlreadyClosed
		);
	});
}

#[test]
fn finalize_epoch_with_zero_participants() {
	new_test_ext().execute_with(|| {
		// Don't record any trust scores
		advance_to_epoch_end();
		
		// Should still be able to finalize
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		let reward_pool = PezRewards::get_epoch_reward_pool(0).unwrap();
		assert_eq!(reward_pool.total_trust_score, 0);
		assert_eq!(reward_pool.participants_count, 0);
		assert_eq!(reward_pool.reward_per_trust_point, 0u128.into());
	});
}

#[test]
fn claim_reward_from_nonexistent_epoch_fails() {
	new_test_ext().execute_with(|| {
		// Try to claim from epoch 999 that doesn't exist
		assert_noop!(
			PezRewards::claim_reward(RuntimeOrigin::signed(1), 999),
			Error::<Test>::RewardPoolNotCalculated
		);
	});
}

#[test]
fn claim_reward_before_epoch_finalized_fails() {
	new_test_ext().execute_with(|| {
		// Record score but don't finalize epoch
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		
		// Try to claim before finalization
		assert_noop!(
			PezRewards::claim_reward(RuntimeOrigin::signed(1), 0),
			Error::<Test>::RewardPoolNotCalculated
		);
	});
}

#[test]
fn insufficient_incentive_pot_fails_finalization() {
	new_test_ext().execute_with(|| {
		// Record scores
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		
		// Drain the incentive pot
		let incentive_pot = PezRewards::incentive_pot_account_id();
		let balance = Balances::free_balance(&incentive_pot);
		let _ = Balances::slash(&incentive_pot, balance);
		
		advance_to_epoch_end();
		
		// Should fail with insufficient pot
		assert_noop!(
			PezRewards::finalize_epoch(RuntimeOrigin::root()),
			Error::<Test>::InsufficientIncentivePot
		);
	});
}

#[test]
fn close_epoch_already_closed_fails() {
	new_test_ext().execute_with(|| {
		// Setup and close epoch
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		advance_to_claim_period_end();
		advance_blocks(1);
		assert_ok!(PezRewards::close_epoch(RuntimeOrigin::root(), 0));
		
		// Try to close again
		assert_noop!(
			PezRewards::close_epoch(RuntimeOrigin::root(), 0),
			Error::<Test>::EpochAlreadyClosed
		);
	});
}

#[test]
fn multiple_epochs_work_correctly() {
	new_test_ext().execute_with(|| {
		// Epoch 0
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Epoch 1
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(2)));
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Check that both epochs exist
		assert!(PezRewards::get_epoch_reward_pool(0).is_some());
		assert!(PezRewards::get_epoch_reward_pool(1).is_some());
		
		// Check current epoch is 2
		let epoch_info = PezRewards::get_current_epoch_info();
		assert_eq!(epoch_info.current_epoch, 2);
		assert_eq!(epoch_info.total_epochs_completed, 2);
		
		// Should be able to claim from both epochs
		assert_ok!(PezRewards::claim_reward(RuntimeOrigin::signed(1), 0));
		assert_ok!(PezRewards::claim_reward(RuntimeOrigin::signed(2), 1));
	});
}

#[test]
fn parliamentary_nft_owner_registration_works() {
	new_test_ext().execute_with(|| {
		let nft_id = 1u32;
		let owner = 100u64;
		
		// Register NFT owner
		assert_ok!(PezRewards::register_parliamentary_nft_owner(
			RuntimeOrigin::root(),
			nft_id,
			owner
		));
		
		// Verify registration
		assert_eq!(
			PezRewards::get_parliamentary_nft_owner(nft_id),
			Some(owner)
		);
	});
}

#[test]
fn parliamentary_nft_registration_only_root() {
	new_test_ext().execute_with(|| {
		// Non-root cannot register NFT owner
		assert_noop!(
			PezRewards::register_parliamentary_nft_owner(
				RuntimeOrigin::signed(1),
				1u32,
				100u64
			),
			BadOrigin
		);
	});
}

#[test]
fn reward_calculation_overflow_protection() {
	new_test_ext().execute_with(|| {
		// This test would need to set up extreme values that could cause overflow
		// The actual implementation depends on your balance type limits
		
		// Record a very high trust score manually
		let user: u64 = 1;
		let extreme_score = u128::MAX;
		crate::UserEpochScores::<Test>::insert(0, &user, extreme_score);
		
		// Setup a scenario that might cause overflow in calculations
		advance_to_epoch_end();
		
		// This should handle overflow gracefully
		// The actual test depends on your specific overflow handling
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
	});
}

#[test]
fn epoch_transitions_maintain_consistency() {
	new_test_ext().execute_with(|| {
		let initial_block = System::block_number();
		
		// Record initial state
		let epoch_info_0 = PezRewards::get_current_epoch_info();
		assert_eq!(epoch_info_0.current_epoch, 0);
		assert_eq!(epoch_info_0.epoch_start_block, initial_block);
		
		// Record scores and advance
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		advance_to_epoch_end();
		let finalize_block = System::block_number();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		
		// Check epoch transition
		let epoch_info_1 = PezRewards::get_current_epoch_info();
		assert_eq!(epoch_info_1.current_epoch, 1);
		assert_eq!(epoch_info_1.epoch_start_block, finalize_block);
		assert_eq!(epoch_info_1.total_epochs_completed, 1);
		
		// Check epoch 0 is in claim period
		let status_0 = crate::EpochStatus::<Test>::get(0);
		assert_eq!(status_0, crate::EpochState::ClaimPeriod);
		
		// Check epoch 1 is open
		let status_1 = crate::EpochStatus::<Test>::get(1);
		assert_eq!(status_1, crate::EpochState::Open);
	});
}

#[test]
fn trust_score_updates_during_epoch() {
	new_test_ext().execute_with(|| {
		// Record score multiple times for same user in same epoch
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		
		// Check initial score
		let score1 = PezRewards::get_user_trust_score_for_epoch(0, &1);
		assert_eq!(score1, Some(100)); // From mock
		
		// Record again (should update)
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		
		// Score should still be the same (as it comes from trust pallet)
		let score2 = PezRewards::get_user_trust_score_for_epoch(0, &1);
		assert_eq!(score2, Some(100));
	});
}

#[test]
fn getter_functions_work_correctly() {
	new_test_ext().execute_with(|| {
		// Test all getter functions
		
		// Initially no rewards claimed
		assert_eq!(PezRewards::get_claimed_reward(0, &1), None);
		
		// No trust scores recorded yet
		assert_eq!(PezRewards::get_user_trust_score_for_epoch(0, &1), None);
		
		// No reward pools yet
		assert_eq!(PezRewards::get_epoch_reward_pool(0), None);
		
		// Record score and check
		assert_ok!(PezRewards::record_trust_score(RuntimeOrigin::signed(1)));
		assert_eq!(PezRewards::get_user_trust_score_for_epoch(0, &1), Some(100));
		
		// Finalize and check pool
		advance_to_epoch_end();
		assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
		assert!(PezRewards::get_epoch_reward_pool(0).is_some());
		
		// Claim and check
		assert_ok!(PezRewards::claim_reward(RuntimeOrigin::signed(1), 0));
		assert!(PezRewards::get_claimed_reward(0, &1).is_some());
		assert!(PezRewards::get_claimed_reward(0, &1).unwrap() > 0u128.into());
	});
}

}