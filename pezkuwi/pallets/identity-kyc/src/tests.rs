use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_runtime::DispatchError;

// Kolay erişim için paletimize bir takma ad veriyoruz.
type IdentityKycPallet = crate::Pallet<Test>;

#[test]
fn set_identity_works() {
	new_test_ext().execute_with(|| {
		let user = 1;
		let name: BoundedVec<_, _> = b"Pezkuwi".to_vec().try_into().unwrap();
		let email: BoundedVec<_, _> = b"info@pezkuwi.com".to_vec().try_into().unwrap();

		assert_eq!(IdentityKycPallet::identity_of(user), None);

		assert_ok!(IdentityKycPallet::set_identity(
			RuntimeOrigin::signed(user),
			name.clone(),
			email.clone()
		));

		let stored_identity = IdentityKycPallet::identity_of(user).unwrap();
		assert_eq!(stored_identity.name, name);
		assert_eq!(stored_identity.email, email);

		System::assert_last_event(Event::IdentitySet { who: user }.into());
	});
}

#[test]
fn apply_for_kyc_works() {
	new_test_ext().execute_with(|| {
		let user = 1;
		let name: BoundedVec<_, _> = b"Pezkuwi".to_vec().try_into().unwrap();
		let email: BoundedVec<_, _> = b"info@pezkuwi.com".to_vec().try_into().unwrap();
		assert_ok!(IdentityKycPallet::set_identity(RuntimeOrigin::signed(user), name, email));

		let cids: BoundedVec<_, _> = vec![b"cid1".to_vec().try_into().unwrap()]
			.try_into()
			.unwrap();
		let notes: BoundedVec<_, _> = b"Application notes".to_vec().try_into().unwrap();

		assert_eq!(IdentityKycPallet::kyc_status_of(user), crate::KycLevel::NotStarted);
		assert_eq!(Balances::reserved_balance(user), 0);

		assert_ok!(IdentityKycPallet::apply_for_kyc(
			RuntimeOrigin::signed(user),
			cids.clone(),
			notes.clone()
		));

		assert_eq!(IdentityKycPallet::kyc_status_of(user), crate::KycLevel::Pending);
		let stored_app = IdentityKycPallet::pending_application_of(user).unwrap();
		assert_eq!(stored_app.cids, cids);
		assert_eq!(stored_app.notes, notes);
		assert_eq!(Balances::reserved_balance(user), KycApplicationDepositAmount::get());
		System::assert_last_event(Event::KycApplied { who: user }.into());
	});
}

#[test]
fn apply_for_kyc_fails_if_no_identity() {
	new_test_ext().execute_with(|| {
		let user = 1; // Bu kullanıcının kimliği hiç set edilmedi.
		let cids: BoundedVec<_, _> = vec![].try_into().unwrap();
		let notes: BoundedVec<_, _> = vec![].try_into().unwrap();

		assert_noop!(
			IdentityKycPallet::apply_for_kyc(RuntimeOrigin::signed(user), cids, notes),
			Error::<Test>::IdentityNotFound
		);
	});
}

#[test]
fn apply_for_kyc_fails_if_already_pending() {
	new_test_ext().execute_with(|| {
		let user = 1;
		// İlk başvuruyu yap
		assert_ok!(IdentityKycPallet::set_identity(RuntimeOrigin::signed(user), vec![].try_into().unwrap(), vec![].try_into().unwrap()));
		assert_ok!(IdentityKycPallet::apply_for_kyc(RuntimeOrigin::signed(user), vec![].try_into().unwrap(), vec![].try_into().unwrap()));

		// İkinci kez başvurmayı dene
		assert_noop!(
			IdentityKycPallet::apply_for_kyc(RuntimeOrigin::signed(user), vec![].try_into().unwrap(), vec![].try_into().unwrap()),
			Error::<Test>::KycApplicationAlreadyExists
		);
	});
}


#[test]
fn approve_kyc_works() {
	new_test_ext().execute_with(|| {
		let user = 1;
		// Başvuruyu yap
		assert_ok!(IdentityKycPallet::set_identity(RuntimeOrigin::signed(user), vec![].try_into().unwrap(), vec![].try_into().unwrap()));
		assert_ok!(IdentityKycPallet::apply_for_kyc(RuntimeOrigin::signed(user), vec![].try_into().unwrap(), vec![].try_into().unwrap()));
		assert_eq!(Balances::reserved_balance(user), KycApplicationDepositAmount::get());

		// Root olarak onayla
		assert_ok!(IdentityKycPallet::approve_kyc(RuntimeOrigin::root(), user));

		// Doğrulamalar
		assert_eq!(Balances::reserved_balance(user), 0);
		assert_eq!(IdentityKycPallet::pending_application_of(user), None);
		assert_eq!(IdentityKycPallet::kyc_status_of(user), crate::KycLevel::Approved);
		System::assert_last_event(Event::KycApproved { who: user }.into());
	});
}

#[test]
fn approve_kyc_fails_for_bad_origin() {
	new_test_ext().execute_with(|| {
		let user = 1;
		let non_root_user = 2;
		// Kurulum
		assert_ok!(IdentityKycPallet::set_identity(RuntimeOrigin::signed(user), vec![].try_into().unwrap(), vec![].try_into().unwrap()));
		assert_ok!(IdentityKycPallet::apply_for_kyc(RuntimeOrigin::signed(user), vec![].try_into().unwrap(), vec![].try_into().unwrap()));

		// Root olmayan kullanıcı onaylayamaz
		assert_noop!(
			IdentityKycPallet::approve_kyc(RuntimeOrigin::signed(non_root_user), user),
			DispatchError::BadOrigin
		);
	});
}

#[test]
fn revoke_kyc_works() {
	new_test_ext().execute_with(|| {
		let user = 1;
		// Kurulum: Başvur, onayla
		assert_ok!(IdentityKycPallet::set_identity(RuntimeOrigin::signed(user), vec![].try_into().unwrap(), vec![].try_into().unwrap()));
		assert_ok!(IdentityKycPallet::apply_for_kyc(RuntimeOrigin::signed(user), vec![].try_into().unwrap(), vec![].try_into().unwrap()));
		assert_ok!(IdentityKycPallet::approve_kyc(RuntimeOrigin::root(), user));
		assert_eq!(IdentityKycPallet::kyc_status_of(user), crate::KycLevel::Approved);

		// Eylem: Root olarak iptal et
		assert_ok!(IdentityKycPallet::revoke_kyc(RuntimeOrigin::root(), user));

		// Doğrulama
		assert_eq!(IdentityKycPallet::kyc_status_of(user), crate::KycLevel::Revoked);
		System::assert_last_event(Event::KycRevoked { who: user }.into());
	});
}