use crate::{mock::*, Error, Event, PresaleStatus};
use frame_support::{assert_noop, assert_ok, traits::fungibles::Inspect};

#[test]
fn create_presale_works() {
	new_test_ext().execute_with(|| {
		create_assets();

		// Mint reward tokens to Alice (presale owner)
		mint_assets(1, 1, 100_000_000_000_000_000_000); // 100,000 PEZ

		// Alice creates a presale
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, // wUSDT payment asset
			1, // PEZ reward asset
			10_000_000_000_000_000_000, // 10,000 PEZ tokens for sale (10^12 decimals)
			100, // 100 blocks duration
			false, // public presale
			10_000_000, // min 10 USDT (10^6 decimals)
			1_000_000_000, // max 1000 USDT
			5_000_000_000, // soft cap 5,000 USDT
			10_000_000_000, // hard cap 10,000 USDT
			false, // no vesting
			0,
			0,
			0,
			24, // 24 blocks grace period
			5, // 5% refund fee
			2, // 2% grace refund fee
		));

		// Check presale created
		let presale = Presale::presales(0).unwrap();
		assert_eq!(presale.owner, 1);
		assert_eq!(presale.payment_asset, 2);
		assert_eq!(presale.reward_asset, 1);
		assert_eq!(presale.tokens_for_sale, 10_000_000_000_000_000_000);
		assert_eq!(presale.duration, 100);

		// Check event
		System::assert_last_event(Event::PresaleCreated { presale_id: 0, owner: 1 }.into());

		// Check NextPresaleId incremented
		assert_eq!(Presale::next_presale_id(), 1);
	});
}

#[test]
fn create_multiple_presales_works() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 1_000_000_000_000_000_000_000);
		mint_assets(1, 2, 1_000_000_000_000_000_000_000);

		// Alice creates first presale
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Bob creates second presale
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(2),
			2, 1, 20_000_000_000_000_000_000, 200, false,
			20_000_000, 2_000_000_000, 10_000_000_000, 20_000_000_000,
			false, 0, 0, 0, 48, 10, 5,
		));

		// Check both presales exist
		assert!(Presale::presales(0).is_some());
		assert!(Presale::presales(1).is_some());

		// Check owners
		assert_eq!(Presale::presales(0).unwrap().owner, 1);
		assert_eq!(Presale::presales(1).unwrap().owner, 2);

		// Check NextPresaleId
		assert_eq!(Presale::next_presale_id(), 2);
	});
}

#[test]
fn contribute_works() {
	new_test_ext().execute_with(|| {
		create_assets();

		// Setup: Alice creates presale
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Mint wUSDT to Bob
		mint_assets(2, 2, 1_000_000_000); // 1000 USDT

		// Bob contributes 100 USDT
		let contribution = 100_000_000;
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, contribution));

		// Check contribution tracked
		assert_eq!(Presale::contributions(0, 2), contribution);

		// Check total raised
		assert_eq!(Presale::total_raised(0), contribution);

		// Check contributors list
		let contributors = Presale::contributors(0);
		assert_eq!(contributors.len(), 1);
		assert_eq!(contributors[0], 2);

		// Check wUSDT transferred to presale treasury
		let treasury = presale_treasury(0);
		let balance = Assets::balance(2, treasury);
		assert_eq!(balance, contribution);

		// Check event
		System::assert_last_event(
			Event::Contributed { presale_id: 0, who: 2, amount: contribution }.into(),
		);
	});
}

#[test]
fn contribute_multiple_times_works() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		mint_assets(2, 2, 1_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// First contribution
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 50_000_000));
		assert_eq!(Presale::contributions(0, 2), 50_000_000);

		// Second contribution
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 30_000_000));
		assert_eq!(Presale::contributions(0, 2), 80_000_000);

		// Contributors list should still have only 1 entry
		assert_eq!(Presale::contributors(0).len(), 1);

		// Total raised should be sum
		assert_eq!(Presale::total_raised(0), 80_000_000);
	});
}

#[test]
fn contribute_to_different_presales_works() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 1_000_000_000_000_000_000_000);
		mint_assets(2, 2, 2_000_000_000); // 2000 USDT

		// Create two presales
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 15_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Bob contributes to both presales
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 100_000_000));
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 1, 200_000_000));

		// Check contributions tracked separately
		assert_eq!(Presale::contributions(0, 2), 100_000_000);
		assert_eq!(Presale::contributions(1, 2), 200_000_000);

		// Check total raised per presale
		assert_eq!(Presale::total_raised(0), 100_000_000);
		assert_eq!(Presale::total_raised(1), 200_000_000);

		// Check balances in separate treasuries
		assert_eq!(Assets::balance(2, presale_treasury(0)), 100_000_000);
		assert_eq!(Assets::balance(2, presale_treasury(1)), 200_000_000);
	});
}

#[test]
fn contribute_below_min_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		mint_assets(2, 2, 1_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Try to contribute less than minimum (10 USDT)
		assert_noop!(
			Presale::contribute(RuntimeOrigin::signed(2), 0, 5_000_000),
			Error::<Test>::ContributionTooLow
		);
	});
}

#[test]
fn contribute_above_max_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		mint_assets(2, 2, 5_000_000_000); // 5000 USDT

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Try to contribute more than maximum (1000 USDT)
		assert_noop!(
			Presale::contribute(RuntimeOrigin::signed(2), 0, 2_000_000_000),
			Error::<Test>::ContributionTooHigh
		);
	});
}

#[test]
fn contribute_exceeding_hard_cap_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		mint_assets(2, 2, 15_000_000_000); // 15,000 USDT

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000, // Soft cap: 5,000 USDT, Hard cap: 10,000 USDT
			false, 0, 0, 0, 24, 5, 2,
		));

		// Alice contributes 9,000 USDT
		mint_assets(2, 3, 10_000_000_000);
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(3), 0, 9_000_000_000));

		// Bob tries to contribute 2,000 USDT (would exceed 10,000 cap)
		assert_noop!(
			Presale::contribute(RuntimeOrigin::signed(2), 0, 2_000_000_000),
			Error::<Test>::HardCapExceeded
		);
	});
}

#[test]
fn contribute_after_presale_ended_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		mint_assets(2, 2, 1_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Move past presale end (block 1 + 100 = 101)
		System::set_block_number(102);

		assert_noop!(
			Presale::contribute(RuntimeOrigin::signed(2), 0, 100_000_000),
			Error::<Test>::PresaleEnded
		);
	});
}

#[test]
fn finalize_presale_works() {
	new_test_ext().execute_with(|| {
		create_assets();

		// Setup: Alice creates presale with PEZ rewards
		mint_assets(1, 1, 100_000_000_000_000_000_000); // 100,000 PEZ
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Mint PEZ to presale treasury for distribution
		let treasury = presale_treasury(0);
		mint_assets(1, treasury, 100_000_000_000_000_000_000);

		// Bob and Charlie contribute
		mint_assets(2, 2, 1_000_000_000);
		mint_assets(2, 3, 1_000_000_000);

		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 100_000_000)); // 100 USDT
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(3), 0, 200_000_000)); // 200 USDT

		// Move to end of presale
		System::set_block_number(101);

		// Finalize presale
		assert_ok!(Presale::finalize_presale(RuntimeOrigin::signed(1), 0));

		// Check presale status changed to Finalized
		let presale = Presale::presales(0).unwrap();
		assert!(matches!(presale.status, PresaleStatus::Finalized));

		// Check Bob received correct PEZ (100 USDT * 20 = 2000 PEZ)
		// Need to account for 6 decimals USDT -> 12 decimals PEZ conversion
		let bob_pez = Assets::balance(1, 2);
		assert_eq!(bob_pez, 2_000_000_000_000_000);

		// Check Charlie received correct PEZ (200 USDT * 20 = 4000 PEZ)
		let charlie_pez = Assets::balance(1, 3);
		assert_eq!(charlie_pez, 4_000_000_000_000_000);

		// Check event
		System::assert_last_event(
			Event::PresaleFinalized { presale_id: 0, total_raised: 300_000_000 }.into(),
		);
	});
}

#[test]
fn finalize_presale_before_end_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Try to finalize immediately
		assert_noop!(
			Presale::finalize_presale(RuntimeOrigin::signed(1), 0),
			Error::<Test>::PresaleNotEnded
		);
	});
}

#[test]
fn finalize_presale_non_owner_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		System::set_block_number(101);

		// Bob tries to finalize Alice's presale
		assert_noop!(
			Presale::finalize_presale(RuntimeOrigin::signed(2), 0),
			Error::<Test>::NotPresaleOwner
		);
	});
}

#[test]
fn refund_works() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		mint_assets(2, 2, 1_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Bob contributes
		let contribution = 100_000_000; // 100 USDT
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, contribution));

		// Bob requests refund (not in grace period)
		System::set_block_number(30);

		let initial_balance = Assets::balance(2, 2);
		assert_ok!(Presale::refund(RuntimeOrigin::signed(2), 0));

		// Check refund with 5% fee
		let fee = contribution * 5 / 100; // 5 USDT fee
		let refund_amount = contribution - fee;

		// Check Bob's balance increased
		assert_eq!(Assets::balance(2, 2), initial_balance + refund_amount);

		// Check contribution removed
		assert_eq!(Presale::contributions(0, 2), 0);

		// Check total raised decreased
		assert_eq!(Presale::total_raised(0), 0);

		// Check event
		System::assert_last_event(
			Event::Refunded { presale_id: 0, who: 2, amount: contribution }.into(),
		);
	});
}

#[test]
fn refund_in_grace_period_lower_fee() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		mint_assets(2, 2, 1_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0,
			24, // 24 blocks grace period (block 1 + 24 = 25)
			5,  // 5% regular refund fee
			2,  // 2% grace refund fee
		));

		let contribution = 100_000_000; // 100 USDT
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, contribution));

		// Refund within grace period (block < 25)
		System::set_block_number(20);

		let initial_balance = Assets::balance(2, 2);
		assert_ok!(Presale::refund(RuntimeOrigin::signed(2), 0));

		// Should use grace period fee (2%)
		let grace_fee = contribution * 2 / 100; // 2 USDT fee
		let refund_amount = contribution - grace_fee;

		assert_eq!(Assets::balance(2, 2), initial_balance + refund_amount);
	});
}

#[test]
fn refund_with_no_contribution_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Bob tries to refund without contributing
		assert_noop!(
			Presale::refund(RuntimeOrigin::signed(2), 0),
			Error::<Test>::NoContribution
		);
	});
}

#[test]
fn cancel_presale_works() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		mint_assets(2, 2, 1_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Bob contributes
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 100_000_000));

		// Owner cancels presale
		assert_ok!(Presale::cancel_presale(RuntimeOrigin::signed(1), 0));

		// Check status changed
		let presale = Presale::presales(0).unwrap();
		assert!(matches!(presale.status, PresaleStatus::Cancelled));

		// Check event
		System::assert_last_event(Event::PresaleCancelled { presale_id: 0 }.into());
	});
}

#[test]
fn cancel_presale_non_owner_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Bob tries to cancel Alice's presale (not root)
		assert_noop!(
			Presale::cancel_presale(RuntimeOrigin::signed(2), 0),
			Error::<Test>::NotPresaleOwner
		);
	});
}

#[test]
fn emergency_cancel_by_root_works() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Root can cancel any presale (emergency)
		assert_ok!(Presale::cancel_presale(RuntimeOrigin::root(), 0));

		let presale = Presale::presales(0).unwrap();
		assert!(matches!(presale.status, PresaleStatus::Cancelled));
	});
}

#[test]
fn whitelist_presale_works() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		mint_assets(2, 2, 1_000_000_000);

		// Create whitelist presale
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100,
			true, // whitelist enabled
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Bob tries to contribute (not whitelisted)
		assert_noop!(
			Presale::contribute(RuntimeOrigin::signed(2), 0, 100_000_000),
			Error::<Test>::NotWhitelisted
		);

		// Owner adds Bob to whitelist
		assert_ok!(Presale::add_to_whitelist(RuntimeOrigin::signed(1), 0, 2));

		// Now Bob can contribute
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 100_000_000));
	});
}

#[test]
fn add_to_whitelist_non_owner_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);

		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, true,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Charlie tries to add Bob to Alice's presale whitelist
		assert_noop!(
			Presale::add_to_whitelist(RuntimeOrigin::signed(3), 0, 2),
			Error::<Test>::NotPresaleOwner
		);
	});
}

// ========== SOFT CAP TESTS ==========

#[test]
fn finalize_presale_soft_cap_reached_success() {
	new_test_ext().execute_with(|| {
		create_assets();

		// Setup: Alice creates presale
		// Soft cap: 5,000 USDT, Hard cap: 10,000 USDT
		mint_assets(1, 1, 100_000_000_000_000_000_000); // 100,000 PEZ
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Mint PEZ to presale treasury
		let treasury = presale_treasury(0);
		mint_assets(1, treasury, 100_000_000_000_000_000_000);

		// Contributors exceed soft cap
		mint_assets(2, 2, 3_000_000_000); // Bob: 3,000 USDT
		mint_assets(2, 3, 3_000_000_000); // Charlie: 3,000 USDT

		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 3_000_000_000));
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(3), 0, 3_000_000_000));

		// Total raised: 6,000 USDT > soft cap (5,000 USDT) ✅
		assert_eq!(Presale::total_raised(0), 6_000_000_000);

		// Move past presale end
		System::set_block_number(102);

		// Root finalizes presale
		assert_ok!(Presale::finalize_presale(RuntimeOrigin::root(), 0));

		// Check presale status is Finalized (went through Successful)
		let presale = Presale::presales(0).unwrap();
		assert!(matches!(presale.status, PresaleStatus::Finalized));

		// Check contributors received tokens
		// Total raised: 6,000 USDT
		// Tokens for sale: 10,000 PEZ (10^12 decimals)
		// Bob's share: (3,000 / 6,000) * 10,000 = 5,000 PEZ
		// Charlie's share: (3,000 / 6,000) * 10,000 = 5,000 PEZ
		assert!(Assets::balance(1, 2) > 0); // Bob received PEZ
		assert!(Assets::balance(1, 3) > 0); // Charlie received PEZ
	});
}

#[test]
fn finalize_presale_soft_cap_not_reached_fails() {
	new_test_ext().execute_with(|| {
		create_assets();

		// Setup: Alice creates presale
		// Soft cap: 5,000 USDT, Hard cap: 10,000 USDT
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Contributors below soft cap
		mint_assets(2, 2, 2_000_000_000); // Bob: 2,000 USDT
		mint_assets(2, 3, 2_000_000_000); // Charlie: 2,000 USDT

		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 2_000_000_000));
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(3), 0, 2_000_000_000));

		// Total raised: 4,000 USDT < soft cap (5,000 USDT) ❌
		assert_eq!(Presale::total_raised(0), 4_000_000_000);

		// Move past presale end
		System::set_block_number(102);

		// Root finalizes presale
		assert_ok!(Presale::finalize_presale(RuntimeOrigin::root(), 0));

		// Check presale status is Failed (soft cap not reached)
		let presale = Presale::presales(0).unwrap();
		assert!(matches!(presale.status, PresaleStatus::Failed));

		// Check contributors did NOT receive tokens (presale failed)
		assert_eq!(Assets::balance(1, 2), 0); // Bob received nothing
		assert_eq!(Assets::balance(1, 3), 0); // Charlie received nothing
	});
}

#[test]
fn batch_refund_failed_presale_works() {
	new_test_ext().execute_with(|| {
		create_assets();

		// Setup: Alice creates presale
		mint_assets(1, 1, 100_000_000_000_000_000_000);
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		// Fund presale treasury with wUSDT for refunds
		let treasury = presale_treasury(0);
		mint_assets(2, treasury, 10_000_000_000); // Mint enough for refunds

		// Contributors below soft cap
		mint_assets(2, 2, 2_000_000_000); // Bob
		mint_assets(2, 3, 2_000_000_000); // Charlie

		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 2_000_000_000));
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(3), 0, 2_000_000_000));

		// Record initial balances
		let bob_initial = Assets::balance(2, 2);
		let charlie_initial = Assets::balance(2, 3);

		// Move past presale end and finalize (will set status to Failed)
		System::set_block_number(102);
		assert_ok!(Presale::finalize_presale(RuntimeOrigin::root(), 0));

		// Check status is Failed
		let presale = Presale::presales(0).unwrap();
		assert!(matches!(presale.status, PresaleStatus::Failed));

		// Anyone can call batch_refund_failed_presale
		assert_ok!(Presale::batch_refund_failed_presale(
			RuntimeOrigin::signed(4), // Random account (not owner)
			0, // presale_id
			0, // start_index
			10, // batch_size (refund up to 10 contributors)
		));

		// Check contributors got full refunds (NO FEE for failed presale)
		assert_eq!(Assets::balance(2, 2), bob_initial + 2_000_000_000); // Full refund
		assert_eq!(Assets::balance(2, 3), charlie_initial + 2_000_000_000); // Full refund

		// Check contributions marked as refunded
		let bob_contribution = Presale::contributions(0, 2).unwrap();
		assert!(bob_contribution.refunded);
		assert_eq!(bob_contribution.refund_fee_paid, 0); // No fee!
	});
}

#[test]
fn batch_refund_successful_presale_fails() {
	new_test_ext().execute_with(|| {
		create_assets();

		mint_assets(1, 1, 100_000_000_000_000_000_000);
		assert_ok!(Presale::create_presale(
			RuntimeOrigin::signed(1),
			2, 1, 10_000_000_000_000_000_000, 100, false,
			10_000_000, 1_000_000_000, 5_000_000_000, 10_000_000_000,
			false, 0, 0, 0, 24, 5, 2,
		));

		let treasury = presale_treasury(0);
		mint_assets(1, treasury, 100_000_000_000_000_000_000);
		mint_assets(2, treasury, 10_000_000_000);

		// Exceed soft cap
		mint_assets(2, 2, 6_000_000_000);
		assert_ok!(Presale::contribute(RuntimeOrigin::signed(2), 0, 6_000_000_000));

		// Finalize (will succeed because soft cap reached)
		System::set_block_number(102);
		assert_ok!(Presale::finalize_presale(RuntimeOrigin::root(), 0));

		// Try to batch refund a successful presale (should fail)
		assert_noop!(
			Presale::batch_refund_failed_presale(
				RuntimeOrigin::signed(4),
				0,
				0,
				10,
			),
			Error::<Test>::PresaleNotFailed
		);
	});
}

#[test]
fn create_presale_with_soft_cap_greater_than_hard_cap_fails() {
	new_test_ext().execute_with(|| {
		create_assets();
		mint_assets(1, 1, 100_000_000_000_000_000_000);

		// Try to create presale with soft_cap > hard_cap (invalid)
		assert_noop!(
			Presale::create_presale(
				RuntimeOrigin::signed(1),
				2, 1, 10_000_000_000_000_000_000, 100, false,
				10_000_000,
				1_000_000_000,
				15_000_000_000, // soft_cap: 15,000 USDT
				10_000_000_000, // hard_cap: 10,000 USDT (INVALID!)
				false, 0, 0, 0, 24, 5, 2,
			),
			Error::<Test>::InvalidTokensForSale
		);
	});
}
