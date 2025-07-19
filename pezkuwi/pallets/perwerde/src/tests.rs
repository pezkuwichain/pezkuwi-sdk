use crate::{mock::*, Error, Event, Pallet as EgitimPallet};
use frame_support::{assert_noop, assert_ok};

fn create_bounded_vec(s: &[u8]) -> Vec<u8> {
	s.to_vec()
}

#[test]
fn create_course_works() {
	new_test_ext().execute_with(|| {
		// Eylem: Admin yetkisine sahip 0 ID'li hesap ile bir kurs oluşturulur.
		assert_ok!(EgitimPallet::create_course(
			RuntimeOrigin::signed(0),
			create_bounded_vec(b"Blockchain 101").try_into().unwrap(),
			create_bounded_vec(b"Giris seviyesi").try_into().unwrap(),
			create_bounded_vec(b"http://example.com").try_into().unwrap()
		));

		// Doğrulama: Kursun doğru bir şekilde oluşturulduğunu ve olay'ın yayınlandığını kontrol et.
		assert!(crate::Courses::<Test>::contains_key(0));
		let course = crate::Courses::<Test>::get(0).unwrap();
		assert_eq!(course.name.to_vec(), b"Blockchain 101".to_vec());
		System::assert_last_event(Event::CourseCreated { course_id: 0, owner: 0 }.into());
	});
}

#[test]
fn create_course_fails_for_non_admin() {
	new_test_ext().execute_with(|| {
		// Eylem & Doğrulama: Normal bir kullanıcının kurs oluşturamaması gerekir.
		assert_noop!(
			EgitimPallet::create_course(
				RuntimeOrigin::signed(1),
				create_bounded_vec(b"Hacking 101").try_into().unwrap(),
				create_bounded_vec(b"Yetkisiz kurs").try_into().unwrap(),
				create_bounded_vec(b"http://example.com").try_into().unwrap()
			),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn enroll_works() {
	new_test_ext().execute_with(|| {
		// Kurulum: Önce bir kurs oluştur.
		let _ = EgitimPallet::create_course(RuntimeOrigin::root(), vec![].try_into().unwrap(), vec![].try_into().unwrap(), vec![].try_into().unwrap());
		
		// Eylem: 1 numaralı kullanıcı 0 ID'li kursa kaydolur.
		assert_ok!(EgitimPallet::enroll(RuntimeOrigin::signed(1), 0));

		// Doğrulama: Kaydın oluştuğunu ve olayın yayınlandığını kontrol et.
		assert!(crate::Enrollments::<Test>::contains_key((1, 0)));
		let course = crate::Courses::<Test>::get(0).unwrap();
		assert!(course.students.contains(&1));
		System::assert_last_event(Event::CourseEnrolled { student: 1, course_id: 0 }.into());
	});
}

#[test]
fn enroll_fails_if_already_enrolled() {
	new_test_ext().execute_with(|| {
		// Kurulum: Kurs oluştur ve kullanıcıyı kaydet.
		let _ = EgitimPallet::create_course(RuntimeOrigin::root(), vec![].try_into().unwrap(), vec![].try_into().unwrap(), vec![].try_into().unwrap());
		assert_ok!(EgitimPallet::enroll(RuntimeOrigin::signed(1), 0));

		// Eylem & Doğrulama: Aynı kursa tekrar kaydolmaya çalışınca hata vermeli.
		assert_noop!(
			EgitimPallet::enroll(RuntimeOrigin::signed(1), 0),
			Error::<Test>::AlreadyEnrolled
		);
	});
}

#[test]
fn complete_course_works() {
	new_test_ext().execute_with(|| {
		// Kurulum: Kurs oluştur ve kullanıcıyı kaydet.
		let _ = EgitimPallet::create_course(RuntimeOrigin::root(), vec![].try_into().unwrap(), vec![].try_into().unwrap(), vec![].try_into().unwrap());
		assert_ok!(EgitimPallet::enroll(RuntimeOrigin::signed(1), 0));

		// Eylem: Kullanıcı kursu 10 puanla tamamlar.
		assert_ok!(EgitimPallet::complete_course(RuntimeOrigin::signed(1), 0, 10));

		// Doğrulama: Kaydın tamamlandı olarak güncellendiğini ve puanın işlendiğini kontrol et.
		let enrollment = crate::Enrollments::<Test>::get((1, 0)).unwrap();
		assert!(enrollment.completed_at.is_some());
		assert_eq!(enrollment.points_earned, 10);
		assert!(crate::CompletedCourses::<Test>::get(1).contains(&0));
		System::assert_last_event(Event::CourseCompleted { student: 1, course_id: 0, points_earned: 10 }.into());
	});
}

#[test]
fn complete_course_fails_if_not_enrolled() {
	new_test_ext().execute_with(|| {
		// Kurulum: Kurs oluştur.
		let _ = EgitimPallet::create_course(RuntimeOrigin::root(), vec![].try_into().unwrap(), vec![].try_into().unwrap(), vec![].try_into().unwrap());

		// Eylem & Doğrulama: Kayıtlı olmadığı bir kursu tamamlamaya çalışınca hata vermeli.
		assert_noop!(
			EgitimPallet::complete_course(RuntimeOrigin::signed(1), 0, 10),
			Error::<Test>::NotEnrolled
		);
	});
}