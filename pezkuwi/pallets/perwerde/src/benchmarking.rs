//! Benchmarking setup for pallet-perwerde

use super::{Pallet as perwerde, *};
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
		let name: BoundedVec<u8, T::MaxCourseNameLength> = create_bounded(b"Substrate perwerdei");
		let description: BoundedVec<u8, T::MaxCourseDescLength> =
			create_bounded(b"Bu perwerde Substrate temellerini kapsar.");
		let content_link: BoundedVec<u8, T::MaxCourseLinkLength> =
			create_bounded(b"http://example.com");

		let admin: T::AccountId = account("admin", 0, 0);
		// EYLEM: `create_course` extrinsic'ini admin yetkisiyle çağırıyoruz.
		#[extrinsic_call]
		_(RawOrigin::Signed(admin), name, description, content_link);

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
		perwerde::<T>::create_course(
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
		perwerde::<T>::create_course(RawOrigin::Root.into(), create_bounded(b"Test"), create_bounded(b"Test"), create_bounded(b"Test")).unwrap();
		perwerde::<T>::enroll(student_origin.clone().into(), course_id).unwrap();

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
		perwerde::<T>::create_course(RawOrigin::Root.into(), create_bounded(b"Test"), create_bounded(b"Test"), create_bounded(b"Test")).unwrap();

		// EYLEM: `archive_course` extrinsic'ini admin yetkisiyle çağırıyoruz.
		#[extrinsic_call]
		_(RawOrigin::Signed(course.owner), course_id);

		// DOĞRULAMA: Kursun durumunun 'Archived' olarak değiştiğini kontrol ediyoruz.
		let course = Courses::<T>::get(course_id).unwrap();
		assert_eq!(course.status, CourseStatus::Archived);
	}

	#[benchmark]
	fn benchmark_get_perwerde_score(c: Linear<0, { T::MaxStudentsPerCourse::get() }>) {
		// En kötü senaryoyu hazırlıyoruz: Bir öğrenci `c` kadar kursu tamamlıyor.
		let student: T::AccountId = account("student", 0, 0);
		let student_origin = RawOrigin::Signed(student.clone());
		let admin: T::AccountId = account("admin", 0, 1);

		for i in 0..c {
			let course_id = i as u64;
			// Kurs oluştur
			perwerde::<T>::create_course(
				RawOrigin::Signed(admin.clone()).into(),
				create_bounded(b"Benchmark Course"),
				create_bounded(b"Benchmark Description"),
				create_bounded(b"http://benchmark.com"),
			).unwrap();

			// Kursa kaydol
			perwerde::<T>::enroll(student_origin.clone().into(), course_id).unwrap();

			// Kursu tamamla
			perwerde::<T>::complete_course(student_origin.clone().into(), course_id, 10).unwrap();
		}

		// EYLEM: `get_perwerde_score` fonksiyonunu çağıran extrinsic'i çalıştırıyoruz.
		#[extrinsic_call]
		benchmark_get_perwerde_score(RawOrigin::Signed(student.clone()), student);

		// DOĞRULAMA: Bir durum değişikliği olmadığı için doğrulama gerekmez.
	}

	impl_benchmark_test_suite!(perwerde, crate::mock::new_test_ext(), crate::mock::Test);

}