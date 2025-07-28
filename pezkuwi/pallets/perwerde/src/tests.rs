use crate::{
	mock::{new_test_ext, RuntimeOrigin, System, Test, Perwerde as PerwerdePallet},
	Event,
};
use frame_support::{assert_noop, assert_ok, pallet_prelude::Get, BoundedVec};
use sp_runtime::DispatchError;

fn create_bounded_vec<L: Get<u32>>(s: &[u8]) -> BoundedVec<u8, L> {
	s.to_vec().try_into().unwrap()
}

#[test]
fn create_course_works() {
	new_test_ext().execute_with(|| {
		// Admin olarak mock.rs'te TestAdminProvider içinde tanımladığımız hesabı kullanıyoruz.
		let admin_account_id = 0;

		// Eylem: Yetkili admin ile kurs oluştur.
		assert_ok!(PerwerdePallet::create_course(
			RuntimeOrigin::signed(admin_account_id),
			create_bounded_vec(b"Blockchain 101"),
			create_bounded_vec(b"Giris seviyesi"),
			create_bounded_vec(b"http://example.com")
		));

		// Doğrulama
		assert!(crate::Courses::<Test>::contains_key(0));
		let course = crate::Courses::<Test>::get(0).unwrap();
		assert_eq!(course.owner, admin_account_id);
		System::assert_last_event(Event::CourseCreated { course_id: 0, owner: admin_account_id }.into());
	});
}

#[test]
fn create_course_fails_for_non_admin() {
	new_test_ext().execute_with(|| {
		// Admin (0) dışındaki bir hesap (2) kurs oluşturamaz.
		let non_admin = 2;
		assert_noop!(
			PerwerdePallet::create_course(
				RuntimeOrigin::signed(non_admin),
				create_bounded_vec(b"Hacking 101"),
				create_bounded_vec(b"Yetkisiz kurs"),
				create_bounded_vec(b"http://example.com")
			),
			DispatchError::BadOrigin
		);
	});
}