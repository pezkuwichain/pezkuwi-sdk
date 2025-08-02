use crate::{mock::*, Error, Event, EpochState};
use frame_support::{assert_noop, assert_ok, traits::Get};
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
		let clawback_recipient = ClawbackRecipient::get();
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
		assert_eq!(reward1, reward2 * 2);
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