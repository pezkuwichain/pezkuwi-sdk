//! Tests for pezpallet-staking-score.
//! All tests use receive_staking_details to populate CachedStakingDetails,
//! mirroring the real People Chain architecture.

use crate::{mock::*, Error, Event, StakingScoreProvider, StakingSource, MONTH_IN_BLOCKS, UNITS};
use pezframe_support::{assert_noop, assert_ok};

const USER_STASH: AccountId = 10;

// ============================================================================
// Basic Score Calculation
// ============================================================================

#[test]
fn zero_stake_should_return_zero_score() {
	ExtBuilder::default().build_and_execute(|| {
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 0);
	});
}

#[test]
fn score_is_calculated_correctly_without_time_tracking() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			50 * UNITS,
			0,
			0
		));

		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 20);
	});
}

#[test]
fn start_score_tracking_works_and_enables_duration_multiplier() {
	ExtBuilder::default().build_and_execute(|| {
		let initial_block = 10u64;
		System::set_block_number(initial_block);

		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			500 * UNITS,
			0,
			0
		));

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));

		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 40);

		// After 4 months: 40 * 1.4 = 56
		let target_block_4m = initial_block + (4 * MONTH_IN_BLOCKS) as u64;
		System::set_block_number(target_block_4m);

		let (score_4m, duration_4m) = StakingScore::get_staking_score(&USER_STASH);
		assert_eq!(duration_4m, target_block_4m - initial_block);
		assert_eq!(score_4m, 56);

		// After 13 months: 40 * 2.0 = 80
		let target_block_13m = initial_block + (13 * MONTH_IN_BLOCKS) as u64;
		System::set_block_number(target_block_13m);

		let (score_13m, duration_13m) = StakingScore::get_staking_score(&USER_STASH);
		assert_eq!(duration_13m, target_block_13m - initial_block);
		assert_eq!(score_13m, 80);
	});
}

#[test]
fn get_staking_score_works_without_explicit_tracking() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			751 * UNITS,
			0,
			0
		));

		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 50);

		// Even after time passes, score stays the same without tracking
		System::set_block_number(1_000_000_000);
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 50);
	});
}

// ============================================================================
// Amount-Based Scoring Tiers
// ============================================================================

#[test]
fn amount_score_boundary_100_hez() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			100 * UNITS,
			0,
			0
		));

		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 20);
	});
}

#[test]
fn amount_score_boundary_250_hez() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			250 * UNITS,
			0,
			0
		));

		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 30);
	});
}

#[test]
fn amount_score_boundary_750_hez() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			750 * UNITS,
			0,
			0
		));

		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 40);
	});
}

#[test]
fn score_capped_at_100() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			1000 * UNITS,
			0,
			0
		));

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));

		// After 12+ months: 50 * 2.0 = 100 (capped)
		System::set_block_number((12 * MONTH_IN_BLOCKS + 1) as u64);

		let (score, _) = StakingScore::get_staking_score(&USER_STASH);
		assert_eq!(score, 100);
	});
}

// ============================================================================
// Duration Multiplier Tests
// ============================================================================

#[test]
fn duration_multiplier_1_month() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			500 * UNITS,
			0,
			0
		));

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));

		System::set_block_number((MONTH_IN_BLOCKS + 1) as u64);

		// 40 * 1.2 = 48
		let (score, _) = StakingScore::get_staking_score(&USER_STASH);
		assert_eq!(score, 48);
	});
}

#[test]
fn duration_multiplier_6_months() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			500 * UNITS,
			0,
			0
		));

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));

		System::set_block_number((6 * MONTH_IN_BLOCKS + 1) as u64);

		// 40 * 1.7 = 68
		let (score, _) = StakingScore::get_staking_score(&USER_STASH);
		assert_eq!(score, 68);
	});
}

#[test]
fn duration_multiplier_progression() {
	ExtBuilder::default().build_and_execute(|| {
		let base_block = 100u64;
		System::set_block_number(base_block);

		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			100 * UNITS,
			0,
			0
		));

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));

		// Start: 20 * 1.0 = 20
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 20);

		// After 3 months: 20 * 1.4 = 28
		System::set_block_number(base_block + (3 * MONTH_IN_BLOCKS) as u64);
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 28);

		// After 12 months: 20 * 2.0 = 40
		System::set_block_number(base_block + (12 * MONTH_IN_BLOCKS) as u64);
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 40);
	});
}

// ============================================================================
// start_score_tracking Extrinsic Tests
// ============================================================================

#[test]
fn start_tracking_fails_without_stake() {
	ExtBuilder::default().build_and_execute(|| {
		assert_noop!(
			StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)),
			Error::<Test>::NoStakeFound
		);
	});
}

#[test]
fn start_tracking_fails_if_already_started() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			100 * UNITS,
			0,
			0
		));

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));

		assert_noop!(
			StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)),
			Error::<Test>::TrackingAlreadyStarted
		);
	});
}

#[test]
fn start_tracking_emits_event() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			100 * UNITS,
			0,
			0
		));

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));

		let events = System::events();
		assert!(events.iter().any(|event| {
			matches!(event.event, RuntimeEvent::StakingScore(Event::ScoreTrackingStarted { .. }))
		}));
	});
}

#[test]
fn start_tracking_works_with_only_asset_hub_stake() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		// Only Asset Hub stake, no Relay Chain stake
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::AssetHub,
			500 * UNITS,
			3,
			0
		));

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 40);
	});
}

// ============================================================================
// receive_staking_details Tests
// ============================================================================

#[test]
fn receive_staking_details_requires_root() {
	ExtBuilder::default().build_and_execute(|| {
		assert_noop!(
			StakingScore::receive_staking_details(
				RuntimeOrigin::signed(USER_STASH),
				USER_STASH,
				StakingSource::RelayChain,
				100 * UNITS,
				0,
				0
			),
			pezsp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn receive_staking_details_emits_event() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::AssetHub,
			500 * UNITS,
			2,
			1
		));

		let events = System::events();
		assert!(events.iter().any(|event| {
			matches!(event.event, RuntimeEvent::StakingScore(Event::StakingDetailsReceived { .. }))
		}));
	});
}

#[test]
fn receive_staking_details_overwrites_same_source() {
	ExtBuilder::default().build_and_execute(|| {
		// First: 100 HEZ from Relay
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			100 * UNITS,
			0,
			0
		));
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 20);

		// Update same source to 300 HEZ
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			300 * UNITS,
			0,
			0
		));
		// 300 HEZ is in 250-750 tier = 40 points
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 40);
	});
}

// ============================================================================
// Dual-Source Aggregation Tests (NEW)
// ============================================================================

#[test]
fn relay_and_asset_hub_stake_aggregated() {
	ExtBuilder::default().build_and_execute(|| {
		// Relay Chain: 200 HEZ
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			200 * UNITS,
			0,
			0
		));

		// Asset Hub: 300 HEZ
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::AssetHub,
			300 * UNITS,
			1,
			0
		));

		// Total: 500 HEZ -> 250-750 tier -> 40 points
		let (score, _) = StakingScore::get_staking_score(&USER_STASH);
		assert_eq!(score, 40);
	});
}

#[test]
fn single_source_update_changes_aggregate() {
	ExtBuilder::default().build_and_execute(|| {
		// Relay: 100 HEZ -> <=100 tier -> 20 points
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			100 * UNITS,
			0,
			0
		));
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 20);

		// Add Asset Hub: 60 HEZ -> total 160 HEZ -> 101-250 tier -> 30 points
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::AssetHub,
			60 * UNITS,
			0,
			0
		));
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 30);
	});
}

#[test]
fn dual_source_with_duration_multiplier() {
	ExtBuilder::default().build_and_execute(|| {
		let base_block = 100u64;
		System::set_block_number(base_block);

		// Relay: 200 HEZ + Asset Hub: 300 HEZ = 500 HEZ -> 40 base
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			200 * UNITS,
			0,
			0
		));
		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::AssetHub,
			300 * UNITS,
			1,
			0
		));

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 40);

		// After 6 months: 40 * 1.7 = 68
		System::set_block_number(base_block + (6 * MONTH_IN_BLOCKS) as u64);
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 68);
	});
}

// ============================================================================
// Multiple Users and Edge Cases
// ============================================================================

#[test]
fn multiple_users_independent_scores() {
	ExtBuilder::default().build_and_execute(|| {
		let user1 = USER_STASH;
		let user2 = 20;

		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			user1,
			StakingSource::RelayChain,
			100 * UNITS,
			0,
			0
		));

		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			user2,
			StakingSource::AssetHub,
			500 * UNITS,
			2,
			0
		));

		// User2 starts tracking
		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(user2)));

		assert_eq!(StakingScore::get_staking_score(&user1).0, 20);
		assert_eq!(StakingScore::get_staking_score(&user2).0, 40);

		// Advance time
		System::set_block_number((3 * MONTH_IN_BLOCKS) as u64);

		// User1 unchanged (no tracking)
		assert_eq!(StakingScore::get_staking_score(&user1).0, 20);

		// User2 increased (40 * 1.4 = 56)
		assert_eq!(StakingScore::get_staking_score(&user2).0, 56);
	});
}

#[test]
fn duration_returned_correctly() {
	ExtBuilder::default().build_and_execute(|| {
		let start_block = 100u64;
		System::set_block_number(start_block);

		assert_ok!(StakingScore::receive_staking_details(
			RuntimeOrigin::root(),
			USER_STASH,
			StakingSource::RelayChain,
			100 * UNITS,
			0,
			0
		));

		// Without tracking, duration should be 0
		let (_, duration) = StakingScore::get_staking_score(&USER_STASH);
		assert_eq!(duration, 0);

		assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));

		// After 5 months
		let target_block = start_block + (5 * MONTH_IN_BLOCKS) as u64;
		System::set_block_number(target_block);

		let (_, duration) = StakingScore::get_staking_score(&USER_STASH);
		assert_eq!(duration, target_block - start_block);
	});
}
