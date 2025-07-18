//! Benchmarking setup for pallet-egitim

use super::{Pallet as Egitim, *};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_keyring::Sr25519Keyring;
use sp_std::vec;
use sp_std::vec::Vec;

#[benchmarks]
mod benchmarks {
	use super::*;

	// BoundedVec oluşturmak için bir yardımcı fonksiyon.
	fn create_bounded<L: Get<u32>>(s: &[u8]) -> BoundedVec<u8, L> {
		s.to_vec().try_into().unwrap()
	}

	#[benchmark]
	fn create_course() {
		// DÜZENLEME: create_course extrinsic'i için argümanları hazırlıyoruz.
		let name: BoundedVec<u8, T::MaxCourseNameLength> = create_bounded(b"Substrate Egitimi");
		let description: BoundedVec<u8, T::MaxCourseDescLength> =
			create_bounded(b"Bu egitim Substrate temellerini kapsar.");
		let content_link: BoundedVec<u8, T::MaxCourseLinkLength> =
			create_bounded(b"http://example.com");

		// EYLEM: `create_course` extrinsic'ini Root yetkisiyle çağırıyoruz.
		#[extrinsic_call]
		_(RawOrigin::Root, name, description, content_link);

		// DOĞRULAMA: Kursun başarıyla oluşturulduğunu kontrol ediyoruz.
		assert!(Courses::<T>::get(0).is_some());
	}

	#[benchmark]
	fn enroll() {
		let course_id = 0;
		// `Alice` hesabını kullanıyoruz çünkü dev zincirinde fonlanmış olduğunu biliyoruz.
		let student: T::AccountId = Sr25519Keyring::Alice.to_account_id();
		let student_origin = RawOrigin::Signed(student.clone());

		// Kurulum: Önce bir kurs oluşturmamız gerekiyor.
		Egitim::<T>::create_course(
			RawOrigin::Root.into(),
			create_bounded(b"Test Kursu"),
			create_bounded(b"Test aciklama"),
			create_bounded(b"http://test.com"),
		)
		.unwrap();

		// EYLEM: `enroll` extrinsic'ini çağırıyoruz.
		#[extrinsic_call]
		_(student_origin, course_id);

		// DOĞRULAMA: Öğrencinin kursa kaydedildiğini kontrol ediyoruz.
		assert!(Enrollments::<T>::get((student, course_id)).is_some());
	}

	#[benchmark]
	fn complete_course() {
		let course_id = 0;
		let student: T::AccountId = Sr25519Keyring::Alice.to_account_id();
		let student_origin = RawOrigin::Signed(student.clone());
		let points = 10;

		// Kurulum: Bir kurs oluşturup öğrenciyi kaydediyoruz.
		Egitim::<T>::create_course(RawOrigin::Root.into(), create_bounded(b"Test"), create_bounded(b"Test"), create_bounded(b"Test")).unwrap();
		Egitim::<T>::enroll(student_origin.clone().into(), course_id).unwrap();

		// EYLEM: `complete_course` extrinsic'ini çağırıyoruz.
		#[extrinsic_call]
		_(student_origin, course_id, points);

		// DOĞRULAMA: Kaydın "tamamlandı" olarak güncellendiğini kontrol ediyoruz.
		let enrollment = Enrollments::<T>::get((student, course_id)).unwrap();
		assert!(enrollment.completed_at.is_some());
		assert_eq!(enrollment.points_earned, points);
	}

	#[benchmark]
	fn archive_course() {
		let course_id = 0;
		// Kurulum: Bir kurs oluşturuyoruz.
		Egitim::<T>::create_course(RawOrigin::Root.into(), create_bounded(b"Test"), create_bounded(b"Test"), create_bounded(b"Test")).unwrap();

		// EYLEM: `archive_course` extrinsic'ini `Root` yetkisiyle çağırıyoruz.
		#[extrinsic_call]
		_(RawOrigin::Root, course_id);

		// DOĞRULAMA: Kursun durumunun 'Archived' olarak değiştiğini kontrol ediyoruz.
		let course = Courses::<T>::get(course_id).unwrap();
		assert_eq!(course.status, CourseStatus::Archived);
	}

	impl_benchmark_test_suite!(Egitim, crate::mock::new_test_ext(), crate::mock::Test);
}