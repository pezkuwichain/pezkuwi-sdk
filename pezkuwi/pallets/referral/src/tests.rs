use super::*;
use crate::{mock::*, Error, Event, ReferralCount, PendingReferrals};
use crate::types::OnKycApproved;
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