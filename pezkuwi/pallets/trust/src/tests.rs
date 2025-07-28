//! Tests for pallet-trust.
use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn non_citizen_gets_error() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Trust::calculate_trust_score(&1),
			Error::<Test>::NotACitizen
		);
	});
}

#[test]
fn zero_stake_returns_zero_score() {
	ExtBuilder {
		citizens: BTreeMap::from([(1, true)]),
		..Default::default()
	}
	.build()
	.execute_with(|| {
		assert_eq!(Trust::calculate_trust_score(&1).unwrap(), 0);
	});
}

#[test]
fn calculation_works_correctly() {
	// Test verilerini hazırlayalım
	let mut builder = ExtBuilder::default();
	builder.citizens.insert(1, true); // Kullanıcı 1 bir vatandaş
	builder.staking_scores.insert(1, (40, 100)); // Staking puanı: 40
	builder.referral_scores.insert(1, 20); // Referral puanı: 20
	builder.perwerde_scores.insert(1, 30); // Perwerde puanı: 30
	builder.tiki_scores.insert(1, 50); // Tiki puanı: 50

	builder.build().execute_with(|| {
		// Formül: Staking * ( (Staking*0.1) + (Referral*0.3) + (Perwerde*0.3) + (Tiki*0.3) )
		// = 40 * ( (40*0.1) + (20*0.3) + (30*0.3) + (50*0.3) )
		// = 40 * ( 4 + 6 + 9 + 15 )
		// = 40 * 34
		// = 1360
		let expected_score = 1360;

		assert_ok!(Trust::update_score_for_account(&1));
		assert_eq!(Trust::trust_score_of(&1), expected_score);
		assert_eq!(Trust::total_active_trust_score(), expected_score);
	});
}

#[test]
fn total_score_updates_correctly() {
	// İki vatandaş hazırlayalım
	let mut builder = ExtBuilder::default();
	builder.citizens.insert(1, true);
	builder.citizens.insert(2, true);
	builder.staking_scores.insert(1, (40, 100)); // Puanı 1360
	builder.referral_scores.insert(1, 20);
	builder.perwerde_scores.insert(1, 30);
	builder.tiki_scores.insert(1, 50);
	builder.staking_scores.insert(2, (10, 100)); // Puanı 10 * ((10*0.1)+(0*0.3)+(0*0.3)+(0*0.3)) = 10 * 1 = 10

	builder.build().execute_with(|| {
		assert_ok!(Trust::update_score_for_account(&1));
		assert_eq!(Trust::total_active_trust_score(), 1360);

		assert_ok!(Trust::update_score_for_account(&2));
		assert_eq!(Trust::total_active_trust_score(), 1360 + 10); // 1370

		// Kullanıcı 1'in puanını sıfırlayalım (stake'i sıfırlayarak)
		MockStakingScoreProvider { scores: BTreeMap::from([(1, (0, 0))]) }.assimilate_storage(&mut Default::default()).unwrap();
		assert_ok!(Trust::update_score_for_account(&1));
		assert_eq!(Trust::trust_score_of(&1), 0);
		assert_eq!(Trust::total_active_trust_score(), 10); // Sadece kullanıcı 2'nin puanı kalmalı
	});
}