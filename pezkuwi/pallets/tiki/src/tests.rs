use crate::{mock::*, Error, Event, Tiki as TikiEnum};
use frame_support::{assert_noop, assert_ok};
use crate::TikiScoreProvider;

type TikiPallet = crate::Pallet<Test>;

#[test]
fn grant_tiki_works() {
	new_test_ext().execute_with(|| {
		let user_account = 2; // Koleksiyon sahibi 1 olduğu için başka bir hesap kullanalım.
		let tiki_to_grant = TikiEnum::Serok;

		// Rolün başlangıçta sahibi olmamalı
		assert_eq!(TikiPallet::tiki_holder(&tiki_to_grant), None);
        // Kullanıcının başlangıçta bu role sahip olmaması gerekir
        assert!(TikiPallet::tikis_of(user_account).into_iter().find(|&t| t == tiki_to_grant).is_none());

		// grant_tiki fonksiyonunu çağırıyoruz.
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), user_account, tiki_to_grant.clone()));

		// Rolün yeni sahibini ve tersine arama haritasını kontrol et
		assert_eq!(TikiPallet::tiki_holder(&tiki_to_grant), Some(user_account));
        assert!(TikiPallet::tikis_of(user_account).into_iter().find(|&t| t == tiki_to_grant).is_some());

		// Event'in doğru atıldığını kontrol et
		System::assert_last_event(
			Event::TikiGranted { who: user_account, tiki: tiki_to_grant }.into(),
		);
	});
}

#[test]
fn grant_tiki_fails_if_role_is_taken() {
	new_test_ext().execute_with(|| {
		let user_1 = 2;
		let user_2 = 3;
		let tiki_to_grant = TikiEnum::Serok;

		// Rolü ilk kullanıcıya ver
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), user_1, tiki_to_grant.clone()));

		// Aynı rolü ikinci kullanıcıya vermeye çalış, 'RoleAlreadyTaken' hatası bekliyoruz.
		assert_noop!(
			TikiPallet::grant_tiki(RuntimeOrigin::root(), user_2, tiki_to_grant.clone()),
			Error::<Test>::RoleAlreadyTaken
		);
	});
}

#[test]
fn revoke_tiki_works() {
	new_test_ext().execute_with(|| {
		let user_account = 2;
		let tiki_to_revoke = TikiEnum::Wezir;

		// Önce Tiki'yi verelim
		assert_ok!(TikiPallet::grant_tiki(
			RuntimeOrigin::root(),
			user_account,
			tiki_to_revoke.clone()
		));
		assert_eq!(TikiPallet::tiki_holder(&tiki_to_revoke), Some(user_account));
        assert!(TikiPallet::tikis_of(user_account).into_iter().find(|&t| t == tiki_to_revoke).is_some());


		// Şimdi Tiki'yi geri alalım
		assert_ok!(TikiPallet::revoke_tiki(
			RuntimeOrigin::root(),
			user_account,
			tiki_to_revoke.clone()
		));
		// Her iki haritanın da temizlendiğini kontrol et
		assert_eq!(TikiPallet::tiki_holder(&tiki_to_revoke), None);
        assert!(TikiPallet::tikis_of(user_account).into_iter().find(|&t| t == tiki_to_revoke).is_none());

		// Event'i kontrol et
		System::assert_last_event(
			Event::TikiRevoked { who: user_account, tiki: tiki_to_revoke }.into(),
		);
	});
}

#[test]
fn revoke_tiki_fails_for_non_holder() {
	new_test_ext().execute_with(|| {
		let holder_account = 2;
		let another_account = 3;
		let tiki = TikiEnum::Dadger;

		// Tiki'yi 'holder_account'a verelim
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), holder_account, tiki.clone()));

		// 'another_account'dan rolü geri almaya çalışalım, hata vermeli
		assert_noop!(
			TikiPallet::revoke_tiki(RuntimeOrigin::root(), another_account, tiki.clone()),
			Error::<Test>::NotTheHolder
		);

		// Rolün hala asıl sahibinde olduğunu doğrulayalım
		assert_eq!(TikiPallet::tiki_holder(&tiki), Some(holder_account));
	});
}

#[test]
fn tiki_score_works_for_single_role() {
	new_test_ext().execute_with(|| {
		let user = 2;
		let tiki = TikiEnum::Serok; // Puan: 200

		// Başlangıçta puan 0 olmalı
		assert_eq!(TikiPallet::get_tiki_score(&user), 0);

		// Tiki'yi verelim
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), user, tiki.clone()));

		// Puanı kontrol edelim
		assert_eq!(TikiPallet::get_tiki_score(&user), 200);
	});
}

#[test]
fn tiki_score_works_for_multiple_roles() {
	new_test_ext().execute_with(|| {
		let user = 2;
		let tiki1 = TikiEnum::Wezir;   // Puan: 125
		let tiki2 = TikiEnum::Dadger;  // Puan: 150
		let expected_total_score = 125 + 150;

		// İki Tiki'yi de verelim
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), user, tiki1.clone()));
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), user, tiki2.clone()));

		// Toplam puanı kontrol edelim
		assert_eq!(TikiPallet::get_tiki_score(&user), expected_total_score);
	});
}

#[test]
fn tiki_score_updates_after_revocation() {
	new_test_ext().execute_with(|| {
		let user = 2;
		let tiki1 = TikiEnum::Parlementer; // Puan: 100
		let tiki2 = TikiEnum::Mela;        // Puan: 50

		// Tiki'leri verelim
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), user, tiki1.clone()));
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), user, tiki2.clone()));

		// Puan 150 olmalı
		assert_eq!(TikiPallet::get_tiki_score(&user), 150);

		// Bir Tiki'yi geri alalım
		assert_ok!(TikiPallet::revoke_tiki(RuntimeOrigin::root(), user, tiki1.clone()));

		// Puan 50'ye düşmeli
		assert_eq!(TikiPallet::get_tiki_score(&user), 50);

		// İkinci Tiki'yi de geri alalım
		assert_ok!(TikiPallet::revoke_tiki(RuntimeOrigin::root(), user, tiki2.clone()));

		// Puan 0 olmalı
		assert_eq!(TikiPallet::get_tiki_score(&user), 0);
	});
}

#[test]
fn tiki_score_is_isolated_between_accounts() {
	new_test_ext().execute_with(|| {
		let user1 = 2;
		let user2 = 3;

		let tiki1 = TikiEnum::Axa;        // Puan: 250
		let tiki2 = TikiEnum::Xezinedar;  // Puan: 100

		// Tiki'leri farklı kullanıcılara verelim
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), user1, tiki1.clone()));
		assert_ok!(TikiPallet::grant_tiki(RuntimeOrigin::root(), user2, tiki2.clone()));

		// Her kullanıcının puanının doğru ve ayrı olduğunu kontrol edelim
		assert_eq!(TikiPallet::get_tiki_score(&user1), 250);
		assert_eq!(TikiPallet::get_tiki_score(&user2), 100);
	});
}