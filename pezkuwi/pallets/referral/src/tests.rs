use super::*;
use crate::{mock::*, Error, Event, ReferralCount, PendingReferrals};
use pallet_identity_kyc::types::OnKycApproved;
use frame_support::{assert_noop, assert_ok};

type ReferralPallet = Pallet<Test>;

#[test]
fn initiate_referral_works() {
	new_test_ext().execute_with(|| {
		// Eylem: 1 numaralı kullanıcı, 2 numaralı kullanıcıyı davet eder.
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(1), 2));

		// Doğrulama: Bekleyen referanslar listesine doğru kayıt atılır.
		assert_eq!(ReferralPallet::pending_referrals(2), Some(1));
		// Doğru olay yayınlanır.
		System::assert_last_event(Event::ReferralInitiated { referrer: 1, referred: 2 }.into());
	});
}

#[test]
fn initiate_referral_fails_for_self_referral() {
	new_test_ext().execute_with(|| {
		// Eylem & Doğrulama: Kullanıcı kendini davet edemez.
		assert_noop!(
			ReferralPallet::initiate_referral(RuntimeOrigin::signed(1), 1),
			Error::<Test>::SelfReferral
		);
	});
}

#[test]
fn initiate_referral_fails_if_already_referred() {
	new_test_ext().execute_with(|| {
		// Kurulum: 2 numaralı kullanıcı, 1 tarafından zaten davet edilmiş.
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(1), 2));

		// Eylem & Doğrulama: 3 numaralı kullanıcı, zaten davet edilmiş olan 2'yi davet edemez.
		assert_noop!(
			ReferralPallet::initiate_referral(RuntimeOrigin::signed(3), 2),
			Error::<Test>::AlreadyReferred
		);
	});
}

#[test]
fn on_kyc_approved_hook_works_when_referral_exists() {
	new_test_ext().execute_with(|| {
		// Kurulum: 1 numaralı kullanıcı 2'yi davet eder.
		let referrer = 1;
		let referred = 2;

		// Test senaryosunu kuran en önemli adım: Bekleyen referansı oluştur!
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer), referred));
		
		// KYC'nin onaylanmış gibi davranması için mock'u hazırlıyoruz.
		// Aslında mock'umuz her zaman Approved döndürdüğü için bu adıma gerek yok,
		// ama gerçek senaryoda state'i böyle kurardık.
		// IdentityKyc::set_kyc_status_for_account(referred, KycLevel::Approved);
		
		// Eylemden önce kullanıcının KYC'sini onaylanmış olarak ayarlayalım.
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred, pallet_identity_kyc::types::KycLevel::Approved);

		// Eylem: KYC paleti, 2 numaralı kullanıcının KYC'sinin onaylandığını bildirir.
		ReferralPallet::on_kyc_approved(&referred);

		// Doğrulama
		// 1. Bekleyen referans kaydı silinir.
		assert_eq!(PendingReferrals::<Test>::get(referred), None);
		// 2. Davet edenin referans sayısı 1 artar.
		assert_eq!(ReferralCount::<Test>::get(referrer), 1);
		// 3. Kalıcı referans bilgisi oluşturulur.
		assert!(Referrals::<Test>::contains_key(referred));
		let referral_info = Referrals::<Test>::get(referred).unwrap();
		assert_eq!(referral_info.referrer, referrer);
		// 4. Doğru olay yayınlanır.
		System::assert_last_event(
			Event::ReferralConfirmed { referrer, referred, new_referrer_count: 1 }.into(),
		);
	});
}

#[test]
fn on_kyc_approved_hook_does_nothing_when_no_referral() {
	new_test_ext().execute_with(|| {
		// Kurulum: Hiçbir referans durumu yok.
		let user_without_referral = 5;

		// Eylem: KYC onayı gelir.
		ReferralPallet::on_kyc_approved(&user_without_referral);

		// Doğrulama: Hiçbir depolama değişmez ve olay yayınlanmaz.
		// (Bu testi basit tutmak için olay sayısını kontrol edebiliriz)
		assert_eq!(ReferralCount::<Test>::iter().count(), 0);
		assert_eq!(Referrals::<Test>::iter().count(), 0);
	});
}

// ============================================================================
// Referral Score Calculation Tests (3 tests)
// ============================================================================

#[test]
fn referral_score_tier_0_to_5() {
	use crate::types::ReferralScoreProvider;

	new_test_ext().execute_with(|| {
		let referrer = 1;

		// 0 referrals = 0 score
		assert_eq!(ReferralPallet::get_referral_score(&referrer), 0);

		// Simulate 1 referral
		ReferralCount::<Test>::insert(&referrer, 1);
		assert_eq!(ReferralPallet::get_referral_score(&referrer), 4); // 1 * 4

		// 5 referrals = 20 score
		ReferralCount::<Test>::insert(&referrer, 5);
		assert_eq!(ReferralPallet::get_referral_score(&referrer), 20); // 5 * 4
	});
}

#[test]
fn referral_score_tier_6_to_20() {
	use crate::types::ReferralScoreProvider;

	new_test_ext().execute_with(|| {
		let referrer = 1;

		// 6 referrals: 20 + (1 * 2) = 22
		ReferralCount::<Test>::insert(&referrer, 6);
		assert_eq!(ReferralPallet::get_referral_score(&referrer), 22);

		// 10 referrals: 20 + (5 * 2) = 30
		ReferralCount::<Test>::insert(&referrer, 10);
		assert_eq!(ReferralPallet::get_referral_score(&referrer), 30);

		// 20 referrals: 20 + (15 * 2) = 50
		ReferralCount::<Test>::insert(&referrer, 20);
		assert_eq!(ReferralPallet::get_referral_score(&referrer), 50);
	});
}

#[test]
fn referral_score_capped_at_50() {
	use crate::types::ReferralScoreProvider;

	new_test_ext().execute_with(|| {
		let referrer = 1;

		// 21+ referrals capped at 50
		ReferralCount::<Test>::insert(&referrer, 21);
		assert_eq!(ReferralPallet::get_referral_score(&referrer), 50);

		// Even 100 referrals = 50
		ReferralCount::<Test>::insert(&referrer, 100);
		assert_eq!(ReferralPallet::get_referral_score(&referrer), 50);
	});
}

// ============================================================================
// InviterProvider Trait Tests (2 tests)
// ============================================================================

#[test]
fn get_inviter_returns_correct_referrer() {
	use crate::types::InviterProvider;

	new_test_ext().execute_with(|| {
		let referrer = 1;
		let referred = 2;

		// Setup referral
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer), referred));
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred, pallet_identity_kyc::types::KycLevel::Approved);
		ReferralPallet::on_kyc_approved(&referred);

		// Verify InviterProvider trait
		let inviter = ReferralPallet::get_inviter(&referred);
		assert_eq!(inviter, Some(referrer));
	});
}

#[test]
fn get_inviter_returns_none_for_non_referred() {
	use crate::types::InviterProvider;

	new_test_ext().execute_with(|| {
		let user_without_referral = 99;

		// User was not referred by anyone
		let inviter = ReferralPallet::get_inviter(&user_without_referral);
		assert_eq!(inviter, None);
	});
}

// ============================================================================
// Edge Cases and Storage Tests (3 tests)
// ============================================================================

#[test]
fn multiple_referrals_for_same_referrer() {
	new_test_ext().execute_with(|| {
		let referrer = 1;
		let referred1 = 2;
		let referred2 = 3;
		let referred3 = 4;

		// Setup multiple referrals
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer), referred1));
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer), referred2));
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer), referred3));

		// Approve all KYCs
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred1, pallet_identity_kyc::types::KycLevel::Approved);
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred2, pallet_identity_kyc::types::KycLevel::Approved);
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred3, pallet_identity_kyc::types::KycLevel::Approved);

		ReferralPallet::on_kyc_approved(&referred1);
		ReferralPallet::on_kyc_approved(&referred2);
		ReferralPallet::on_kyc_approved(&referred3);

		// Verify count
		assert_eq!(ReferralCount::<Test>::get(referrer), 3);
	});
}

#[test]
fn referral_info_stores_block_number() {
	new_test_ext().execute_with(|| {
		let referrer = 1;
		let referred = 2;
		let block_number = 42;

		System::set_block_number(block_number);

		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer), referred));
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred, pallet_identity_kyc::types::KycLevel::Approved);
		ReferralPallet::on_kyc_approved(&referred);

		// Verify stored block number
		let info = Referrals::<Test>::get(referred).unwrap();
		assert_eq!(info.created_at, block_number);
		assert_eq!(info.referrer, referrer);
	});
}

#[test]
fn events_emitted_correctly() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let referrer = 1;
		let referred = 2;

		// Initiate referral - should emit ReferralInitiated
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer), referred));

		let events = System::events();
		assert!(events.iter().any(|e| matches!(
			e.event,
			RuntimeEvent::Referral(Event::ReferralInitiated { .. })
		)));

		// Approve KYC - should emit ReferralConfirmed
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred, pallet_identity_kyc::types::KycLevel::Approved);
		ReferralPallet::on_kyc_approved(&referred);

		let events = System::events();
		assert!(events.iter().any(|e| matches!(
			e.event,
			RuntimeEvent::Referral(Event::ReferralConfirmed { .. })
		)));
	});
}

// ============================================================================
// Integration Tests (2 tests)
// ============================================================================

#[test]
fn complete_referral_flow_integration() {
	use crate::types::{InviterProvider, ReferralScoreProvider};

	new_test_ext().execute_with(|| {
		let referrer = 1;
		let referred = 2;

		// Step 1: Initiate referral
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer), referred));
		assert_eq!(PendingReferrals::<Test>::get(referred), Some(referrer));

		// Step 2: KYC approval triggers confirmation
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred, pallet_identity_kyc::types::KycLevel::Approved);
		ReferralPallet::on_kyc_approved(&referred);

		// Step 3: Verify all storage updates
		assert_eq!(PendingReferrals::<Test>::get(referred), None);
		assert_eq!(ReferralCount::<Test>::get(referrer), 1);
		assert!(Referrals::<Test>::contains_key(referred));

		// Step 4: Verify trait implementations
		assert_eq!(ReferralPallet::get_inviter(&referred), Some(referrer));
		assert_eq!(ReferralPallet::get_referral_score(&referrer), 4); // 1 * 4
	});
}

#[test]
fn storage_consistency_multiple_operations() {
	new_test_ext().execute_with(|| {
		let referrer1 = 1;
		let referrer2 = 2;
		let referred1 = 10;
		let referred2 = 11;
		let referred3 = 12;

		// Referrer1 refers 2 people
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer1), referred1));
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer1), referred2));

		// Referrer2 refers 1 person
		assert_ok!(ReferralPallet::initiate_referral(RuntimeOrigin::signed(referrer2), referred3));

		// Approve all
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred1, pallet_identity_kyc::types::KycLevel::Approved);
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred2, pallet_identity_kyc::types::KycLevel::Approved);
		pallet_identity_kyc::KycStatuses::<Test>::insert(referred3, pallet_identity_kyc::types::KycLevel::Approved);

		ReferralPallet::on_kyc_approved(&referred1);
		ReferralPallet::on_kyc_approved(&referred2);
		ReferralPallet::on_kyc_approved(&referred3);

		// Verify independent counts
		assert_eq!(ReferralCount::<Test>::get(referrer1), 2);
		assert_eq!(ReferralCount::<Test>::get(referrer2), 1);

		// Verify all referrals stored
		assert!(Referrals::<Test>::contains_key(referred1));
		assert!(Referrals::<Test>::contains_key(referred2));
		assert!(Referrals::<Test>::contains_key(referred3));

		// Verify correct referrer stored
		assert_eq!(Referrals::<Test>::get(referred1).unwrap().referrer, referrer1);
		assert_eq!(Referrals::<Test>::get(referred2).unwrap().referrer, referrer1);
		assert_eq!(Referrals::<Test>::get(referred3).unwrap().referrer, referrer2);
	});
}