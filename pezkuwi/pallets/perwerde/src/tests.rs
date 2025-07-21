use crate::{
	mock::{new_test_ext, RuntimeOrigin, System, Test, ADMIN_ACCOUNT},
	Error, Event, Pallet,
};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;
type PerwerdePallet = Pallet<Test>;

#[test]
fn create_course_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PerwerdePallet::create_course(
			RuntimeOrigin::signed(ADMIN_ACCOUNT),
			b"Blockchain 101".to_vec(),
			b"Giris seviyesi".to_vec(),
			b"http://example.com".to_vec()
		));

		assert!(crate::Courses::<Test>::contains_key(0));
		let course = crate::Courses::<Test>::get(0).unwrap();
		assert_eq!(course.owner, ADMIN_ACCOUNT);
		System::assert_last_event(Event::CourseCreated { course_id: 0, owner: ADMIN_ACCOUNT }.into());
	});
}

#[test]
fn create_course_fails_for_non_admin() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			PerwerdePallet::create_course(
				RuntimeOrigin::signed(2), // 2, admin değil
				b"Hacking 101".to_vec(),
				b"Yetkisiz kurs".to_vec(),
				b"http://example.com".to_vec()
			),
			DispatchError::BadOrigin
		);
	});
}

#[test]
fn enroll_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PerwerdePallet::create_course(
            RuntimeOrigin::signed(ADMIN_ACCOUNT),
            b"Test Course".to_vec(), b"Test Desc".to_vec(), b"http://test.com".to_vec()
        ));
		
		assert_ok!(PerwerdePallet::enroll(RuntimeOrigin::signed(2), 0));

		assert!(crate::Enrollments::<Test>::contains_key((2, 0)));
		System::assert_last_event(Event::StudentEnrolled { student: 2, course_id: 0 }.into());
	});
}

#[test]
fn enroll_fails_if_already_enrolled() {
	new_test_ext().execute_with(|| {
		assert_ok!(PerwerdePallet::create_course(
            RuntimeOrigin::signed(ADMIN_ACCOUNT),
            b"Test Course".to_vec(), b"Test Desc".to_vec(), b"http://test.com".to_vec()
        ));
		assert_ok!(PerwerdePallet::enroll(RuntimeOrigin::signed(2), 0));

		assert_noop!(
			PerwerdePallet::enroll(RuntimeOrigin::signed(2), 0),
			Error::<Test>::AlreadyEnrolled
		);
	});
}

#[test]
fn complete_course_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PerwerdePallet::create_course(
            RuntimeOrigin::signed(ADMIN_ACCOUNT),
            b"Test Course".to_vec(), b"Test Desc".to_vec(), b"http://test.com".to_vec()
        ));
		assert_ok!(PerwerdePallet::enroll(RuntimeOrigin::signed(2), 0));

		assert_ok!(PerwerdePallet::complete_course(RuntimeOrigin::signed(2), 0, 10));

		let enrollment = crate::Enrollments::<Test>::get((2, 0)).unwrap();
		assert!(enrollment.completed_at.is_some());
		assert_eq!(enrollment.points_earned, 10);
		System::assert_last_event(Event::CourseCompleted { student: 2, course_id: 0, points: 10 }.into());
	});
}

#[test]
fn complete_course_fails_if_not_enrolled() {
	new_test_ext().execute_with(|| {
		assert_ok!(PerwerdePallet::create_course(
            RuntimeOrigin::signed(ADMIN_ACCOUNT),
            b"Test Course".to_vec(), b"Test Desc".to_vec(), b"http://test.com".to_vec()
        ));

		assert_noop!(
			PerwerdePallet::complete_course(RuntimeOrigin::signed(2), 0, 10),
			Error::<Test>::NotEnrolled
		);
	});
}