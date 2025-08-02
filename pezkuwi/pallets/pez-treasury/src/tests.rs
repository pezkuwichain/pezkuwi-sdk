use crate::{mock::*, Error, Event, TREASURY_ALLOCATION, PRESALE_ALLOCATION, FOUNDER_ALLOCATION, BLOCKS_PER_MONTH};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::{traits::BadOrigin, BuildStorage};

#[test]
fn genesis_distribution_works() {
	new_test_ext().execute_with(|| {
		let treasury_account = PezTreasury::treasury_account_id();
		let presale_account = PresaleAccount::get();
		let founder_account = FounderAccount::get();

		// Check balances after genesis distribution
		assert_eq!(Balances::free_balance(&treasury_account), TREASURY_ALLOCATION);
		assert_eq!(Balances::free_balance(&presale_account), PRESALE_ALLOCATION);
		assert_eq!(Balances::free_balance(&founder_account), FOUNDER_ALLOCATION);

		// Check total supply
		let total_issued = Balances::free_balance(&treasury_account) +
			Balances::free_balance(&presale_account) +
			Balances::free_balance(&founder_account);
		
		assert_eq!(total_issued, TREASURY_ALLOCATION + PRESALE_ALLOCATION + FOUNDER_ALLOCATION);
	});
}

#[test]
fn treasury_initialization_works() {
	new_test_ext().execute_with(|| {
		// Treasury should already be initialized in new_test_ext
		let halving_info = PezTreasury::get_current_halving_info();
		
		assert_eq!(halving_info.current_period, 0);
		assert!(halving_info.monthly_amount > 0);
		assert_eq!(halving_info.total_released, 0);
	});
}

#[test]
fn cannot_initialize_treasury_twice() {
	new_test_ext().execute_with(|| {
		// Try to initialize again
		assert_noop!(
			PezTreasury::initialize_treasury(RuntimeOrigin::root()),
			Error::<Test>::TreasuryAlreadyInitialized
		);
	});
}

#[test]
fn monthly_release_too_early_fails() {
	// Create a fresh test environment without automatic initialization
	let mut ext = sp_io::TestExternalities::new(
		frame_system::GenesisConfig::<Test>::default()
			.build_storage()
			.unwrap()
	);
	
	ext.execute_with(|| {
		// Manually do genesis distribution and treasury initialization
		assert_ok!(PezTreasury::force_genesis_distribution(RuntimeOrigin::root()));
		assert_ok!(PezTreasury::initialize_treasury(RuntimeOrigin::root()));
		
		// Now we're at the exact initialization point - no time has passed
		// Try to release funds immediately (should fail)
		assert_noop!(
			PezTreasury::release_monthly_funds(RuntimeOrigin::root()),
			Error::<Test>::ReleaseTooEarly
		);
	});
}

#[test]
fn monthly_release_works() {
	new_test_ext().execute_with(|| {
		// Advance time by one month
		advance_blocks(BLOCKS_PER_MONTH as u64);
		
		let incentive_pot_before = PezTreasury::get_incentive_pot_balance();
		let government_pot_before = PezTreasury::get_government_pot_balance();
		
		// Release monthly funds
		assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
		
		let incentive_pot_after = PezTreasury::get_incentive_pot_balance();
		let government_pot_after = PezTreasury::get_government_pot_balance();
		
		// Check that funds were transferred to pots
		assert!(incentive_pot_after > incentive_pot_before);
		assert!(government_pot_after > government_pot_before);
		
		// Check ratio (should be roughly 75% / 25%)
		let incentive_increase = incentive_pot_after - incentive_pot_before;
		let government_increase = government_pot_after - government_pot_before;
		let total_increase = incentive_increase + government_increase;
		
		// Incentive should be 75% of total
		let expected_incentive = total_increase * 75 / 100;
		assert_eq!(incentive_increase, expected_incentive);
	});
}

#[test]
fn cannot_release_same_month_twice() {
	new_test_ext().execute_with(|| {
		// Get the current month that should be released
		let current_month = PezTreasury::next_release_month();
		
		// Advance time by exactly one month 
		advance_blocks(BLOCKS_PER_MONTH as u64);
		
		// First release should work
		assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
		
		// Verify that the month was actually released
		assert!(crate::MonthlyReleases::<Test>::contains_key(current_month));
		
		// Try to somehow "trick" the system to release the same month again
		// This should fail because we stored the release in MonthlyReleases
		// We need to reset next_release_month to test this scenario
		crate::NextReleaseMonth::<Test>::put(current_month);
		
		// Now try to release the same month again - should fail
		assert_noop!(
			PezTreasury::release_monthly_funds(RuntimeOrigin::root()),
			Error::<Test>::MonthlyReleaseAlreadyDone
		);
	});
}

#[test]
fn halving_works_after_48_months() {
	new_test_ext().execute_with(|| {
		let initial_halving_info = PezTreasury::get_current_halving_info();
		let initial_monthly_amount = initial_halving_info.monthly_amount;
		
		// Advance time by 48 months (4 years)
		advance_blocks((BLOCKS_PER_MONTH * 48) as u64);
		
		// Release funds to trigger halving
		assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
		
		let new_halving_info = PezTreasury::get_current_halving_info();
		
		// Check that we're in period 1 and monthly amount is halved
		assert_eq!(new_halving_info.current_period, 1);
		assert_eq!(new_halving_info.monthly_amount, initial_monthly_amount / 2);
	});
}

#[test]
fn multiple_monthly_releases_work() {
	new_test_ext().execute_with(|| {
		let mut total_released = 0u128;
		
		// Release funds for 3 months
		for _month in 0..3 {
			advance_blocks(BLOCKS_PER_MONTH as u64);
			
			let before_balance = PezTreasury::get_incentive_pot_balance() + 
				PezTreasury::get_government_pot_balance();
			
			assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
			
			let after_balance = PezTreasury::get_incentive_pot_balance() + 
				PezTreasury::get_government_pot_balance();
			
			let released_this_month = after_balance - before_balance;
			total_released += released_this_month;
		}
		
		// Check that total released matches halving_info
		let halving_info = PezTreasury::get_current_halving_info();
		assert_eq!(halving_info.total_released, total_released);
	});
}

#[test]
fn events_are_emitted() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		// Advance time and release funds
		advance_blocks(BLOCKS_PER_MONTH as u64);
		assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
		
		// Check events
		let events = System::events();
		assert!(events.iter().any(|event| {
			matches!(
				event.event,
				RuntimeEvent::PezTreasury(Event::MonthlyFundsReleased { .. })
			)
		}));
	});
}

#[test]
fn pot_account_ids_are_different() {
	new_test_ext().execute_with(|| {
		let treasury_account = PezTreasury::treasury_account_id();
		let incentive_pot = PezTreasury::incentive_pot_account_id();
		let government_pot = PezTreasury::government_pot_account_id();
		
		// All accounts should be different
		assert_ne!(treasury_account, incentive_pot);
		assert_ne!(treasury_account, government_pot);
		assert_ne!(incentive_pot, government_pot);
	});
}

#[test]
fn only_root_can_call_functions() {
	new_test_ext().execute_with(|| {
		// Non-root user cannot call functions
		assert_noop!(
			PezTreasury::initialize_treasury(RuntimeOrigin::signed(1)),
			BadOrigin
		);
		
		advance_blocks(BLOCKS_PER_MONTH as u64);
		assert_noop!(
			PezTreasury::release_monthly_funds(RuntimeOrigin::signed(1)),
			BadOrigin
		);
		
		assert_noop!(
			PezTreasury::force_genesis_distribution(RuntimeOrigin::signed(1)),
			BadOrigin
		);
	});
}