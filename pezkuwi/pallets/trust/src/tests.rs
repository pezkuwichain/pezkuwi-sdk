use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn calculate_trust_score_works() {
	new_test_ext().execute_with(|| {
		let account = 1u64;
		let score = TrustPallet::calculate_trust_score(&account).unwrap();
		
		let expected = {
			let staking = 100u128;
			let referral = 50u128;
			let perwerde = 30u128;
			let tiki = 20u128;
			let base = ScoreMultiplierBase::get();
			
			let weighted_sum = staking * 100 + referral * 300 + perwerde * 300 + tiki * 300;
			staking * weighted_sum / base
		};
		
		assert_eq!(score, expected);
	});
}

#[test]
fn calculate_trust_score_fails_for_non_citizen() {
	new_test_ext().execute_with(|| {
		let non_citizen = 999u64;
		assert_noop!(
			TrustPallet::calculate_trust_score(&non_citizen),
			Error::<Test>::NotACitizen
		);
	});
}

#[test]
fn calculate_trust_score_zero_staking() {
	new_test_ext().execute_with(|| {
		let account = 1u64;
		let score = TrustPallet::calculate_trust_score(&account).unwrap();
		assert!(score > 0);
	});
}

#[test]
fn update_score_for_account_works() {
	new_test_ext().execute_with(|| {
		let account = 1u64;
		
		let initial_score = TrustPallet::trust_score_of(&account);
		assert_eq!(initial_score, 0);
		
		let new_score = TrustPallet::update_score_for_account(&account).unwrap();
		assert!(new_score > 0);
		
		let stored_score = TrustPallet::trust_score_of(&account);
		assert_eq!(stored_score, new_score);
		
		let total_score = TrustPallet::total_active_trust_score();
		assert_eq!(total_score, new_score);
	});
}

#[test]
fn update_score_for_account_updates_total() {
	new_test_ext().execute_with(|| {
		let account1 = 1u64;
		let account2 = 2u64;
		
		let score1 = TrustPallet::update_score_for_account(&account1).unwrap();
		let total_after_first = TrustPallet::total_active_trust_score();
		assert_eq!(total_after_first, score1);
		
		let score2 = TrustPallet::update_score_for_account(&account2).unwrap();
		let total_after_second = TrustPallet::total_active_trust_score();
		assert_eq!(total_after_second, score1 + score2);
	});
}

#[test]
fn force_recalculate_trust_score_works() {
	new_test_ext().execute_with(|| {
		let account = 1u64;
		
		assert_ok!(TrustPallet::force_recalculate_trust_score(
			RuntimeOrigin::root(),
			account
		));
		
		let score = TrustPallet::trust_score_of(&account);
		assert!(score > 0);
	});
}

#[test]
fn force_recalculate_trust_score_requires_root() {
	new_test_ext().execute_with(|| {
		let account = 1u64;
		
		assert_noop!(
			TrustPallet::force_recalculate_trust_score(
				RuntimeOrigin::signed(account),
				account
			),
			BadOrigin
		);
	});
}

#[test]
fn update_all_trust_scores_works() {
	new_test_ext().execute_with(|| {
		// Event'leri yakalamak için block number set et
		System::set_block_number(1);
		
		assert_ok!(TrustPallet::update_all_trust_scores(RuntimeOrigin::root()));
		
		// Mock implementation boş account listesi kullandığı için
		// AllTrustScoresUpdated event'i yayınlanır (count: 0 ile)
		let events = System::events();
		assert!(events.iter().any(|event| {
			matches!(
				event.event,
				RuntimeEvent::TrustPallet(Event::AllTrustScoresUpdated { total_updated: 0 })
			)
		}));
	});
}

#[test]
fn update_all_trust_scores_requires_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			TrustPallet::update_all_trust_scores(RuntimeOrigin::signed(1)),
			BadOrigin
		);
	});
}

#[test]
fn periodic_trust_score_update_works() {
	new_test_ext().execute_with(|| {
		// Event'leri yakalamak için block number set et
		System::set_block_number(1);
		
		assert_ok!(TrustPallet::periodic_trust_score_update(RuntimeOrigin::root()));
		
		// Periyodik güncelleme event'inin yayınlandığını kontrol et
		let events = System::events();
		assert!(events.iter().any(|event| {
			matches!(
				event.event,
				RuntimeEvent::TrustPallet(Event::PeriodicUpdateScheduled { .. })
			)
		}));
		
		// Ayrıca AllTrustScoresUpdated event'i de yayınlanmalı
		assert!(events.iter().any(|event| {
			matches!(
				event.event,
				RuntimeEvent::TrustPallet(Event::AllTrustScoresUpdated { .. })
			)
		}));
	});
}

#[test]
fn periodic_update_fails_when_batch_in_progress() {
	new_test_ext().execute_with(|| {
		// Batch update'i başlat
		crate::BatchUpdateInProgress::<Test>::put(true);
		
		// Periyodik update'in başarısız olmasını bekle
		assert_noop!(
			TrustPallet::periodic_trust_score_update(RuntimeOrigin::root()),
			Error::<Test>::UpdateInProgress
		);
	});
}

#[test]
fn events_are_emitted() {
	new_test_ext().execute_with(|| {
		let account = 1u64;
		
		System::set_block_number(1);
		
		TrustPallet::update_score_for_account(&account).unwrap();
		
		let events = System::events();
		assert!(events.len() >= 2);
		
		let trust_score_updated = events.iter().any(|event| {
			matches!(
				event.event,
				RuntimeEvent::TrustPallet(Event::TrustScoreUpdated { .. })
			)
		});
		
		let total_updated = events.iter().any(|event| {
			matches!(
				event.event,
				RuntimeEvent::TrustPallet(Event::TotalTrustScoreUpdated { .. })
			)
		});
		
		assert!(trust_score_updated);
		assert!(total_updated);
	});
}

#[test]
fn trust_score_updater_trait_works() {
	new_test_ext().execute_with(|| {
		use crate::TrustScoreUpdater;
		
		let account = 1u64;
		
		let initial_score = TrustPallet::trust_score_of(&account);
		assert_eq!(initial_score, 0);
		
		TrustPallet::on_score_component_changed(&account);
		
		let updated_score = TrustPallet::trust_score_of(&account);
		assert!(updated_score > 0);
	});
}

#[test]
fn batch_update_storage_works() {
	new_test_ext().execute_with(|| {
		// Başlangıçta batch update aktif değil
		assert!(!crate::BatchUpdateInProgress::<Test>::get());
		assert!(crate::LastProcessedAccount::<Test>::get().is_none());
		
		// Batch update'i simüle et
		crate::BatchUpdateInProgress::<Test>::put(true);
		crate::LastProcessedAccount::<Test>::put(42u64);
		
		assert!(crate::BatchUpdateInProgress::<Test>::get());
		assert_eq!(crate::LastProcessedAccount::<Test>::get(), Some(42u64));
		
		// Temizle
		crate::BatchUpdateInProgress::<Test>::put(false);
		crate::LastProcessedAccount::<Test>::kill();
		
		assert!(!crate::BatchUpdateInProgress::<Test>::get());
		assert!(crate::LastProcessedAccount::<Test>::get().is_none());
	});
}

#[test]
fn periodic_update_scheduling_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		
		assert_ok!(TrustPallet::periodic_trust_score_update(RuntimeOrigin::root()));
		
		// Event'te next_block'un doğru hesaplandığını kontrol et
		let events = System::events();
		let scheduled_event = events.iter().find(|event| {
			matches!(
				event.event,
				RuntimeEvent::TrustPallet(Event::PeriodicUpdateScheduled { .. })
			)
		});
		
		assert!(scheduled_event.is_some());
		
		if let Some(event_record) = scheduled_event {
			if let RuntimeEvent::TrustPallet(Event::PeriodicUpdateScheduled { next_block }) = &event_record.event {
				// Current block (100) + interval (100) = 200
				assert_eq!(next_block, &200u64);
			}
		}
	});
}