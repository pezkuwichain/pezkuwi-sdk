//! Benchmarking setup for pallet-perwerde
#![cfg(feature = "runtime-benchmarks")]
use super::{Pallet as Perwerde, *};

// Gerekli modülleri içe aktarıyoruz.
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{pallet_prelude::Get, BoundedVec};
use frame_system::RawOrigin;
use pallet_collective::Instance1; // Council için Instance1'i kullanıyoruz.
use sp_std::vec;

const SEED: u32 = 0;

// Benchmark'larda BoundedVec oluşturmak için bir yardımcı fonksiyon.
fn create_bounded_vec<L: Get<u32>>(s: &[u8]) -> BoundedVec<u8, L> {
	s.to_vec().try_into().unwrap()
}

benchmarks! {
	// Bu 'where' koşulu, benchmark ortamının T'nin aynı zamanda
	// `pallet_collective::Config<Instance1>`'i de uyguladığını bilmesini sağlar. Bu kritik öneme sahiptir.
	where_clause {
		where T: pallet_collective::Config<Instance1>
	}

	create_course {
		let name: BoundedVec<u8, T::MaxCourseNameLength> = create_bounded_vec(b"Substrate egitimi");
		let description: BoundedVec<u8, T::MaxCourseDescLength> = create_bounded_vec(b"Bu egitim Substrate temellerini kapsar.");
		let content_link: BoundedVec<u8, T::MaxCourseLinkLength> = create_bounded_vec(b"http://example.com");

		let name_len = name.len() as u32;
		let desc_len = description.len() as u32;
		let link_len = content_link.len() as u32;


		// Deterministik bir admin hesabı oluştur.
		let admin: T::AccountId = account("admin", 0, SEED);
		// Bu admin'i `Council` (Instance1) üyesi yap.
		// Bu `set_members` çağrısının kendisi Root yetkisi gerektirir.
		pallet_collective::Pallet::<T, Instance1>::set_members(
			RawOrigin::Root.into(),
			vec![admin.clone()],
			Some(admin.clone()), // Prime üye olarak da ayarla
			0u32.into(),
		)?;

	}: _(RawOrigin::Signed(admin), name.clone(), description.clone(), content_link.clone())
	verify {
		assert!(Courses::<T>::get(0).is_some());
	}

	enroll {
		let student: T::AccountId = whitelisted_caller();
		let course_id = 0;

		// Kurulum: Önce bir adminin kurs oluşturması gerekir.
		let admin: T::AccountId = account("admin", 0, SEED);
		pallet_collective::Pallet::<T, Instance1>::set_members(
			RawOrigin::Root.into(),
			vec![admin.clone()],
			Some(admin.clone()),
			0u32.into(),
		)?;
		Perwerde::<T>::create_course(
			RawOrigin::Signed(admin).into(),
			create_bounded_vec(b"Benchmark Course"),
			create_bounded_vec(b"Description"),
			create_bounded_vec(b"Link")
		)?;

	}: _(RawOrigin::Signed(student.clone()), course_id)
	verify {
		assert!(Enrollments::<T>::get((student, course_id)).is_some());
	}

	complete_course {
		let student: T::AccountId = whitelisted_caller();
		let student_origin = RawOrigin::Signed(student.clone());
		let course_id = 0;
		let points = 10;

		// Kurulum: Admin kurs oluşturur, öğrenci kaydolur.
		let admin: T::AccountId = account("admin", 0, SEED);
		pallet_collective::Pallet::<T, Instance1>::set_members(
			RawOrigin::Root.into(),
			vec![admin.clone()],
			Some(admin.clone()),
			0u32.into(),
		)?;
		Perwerde::<T>::create_course(
			RawOrigin::Signed(admin).into(),
			create_bounded_vec(b"Benchmark Course"),
			create_bounded_vec(b"Description"),
			create_bounded_vec(b"Link")
		)?;
		Perwerde::<T>::enroll(student_origin.clone().into(), course_id)?;

	}: _(student_origin, course_id, points)
	verify {
		let enrollment = Enrollments::<T>::get((student, course_id)).unwrap();
		assert!(enrollment.completed_at.is_some());
		assert_eq!(enrollment.points_earned, points);
	}

	archive_course {
		let course_id = 0;

		// Kurulum: Admin kurs oluşturur.
		let admin: T::AccountId = account("admin", 0, SEED);
		pallet_collective::Pallet::<T, Instance1>::set_members(
			RawOrigin::Root.into(),
			vec![admin.clone()],
			Some(admin.clone()),
			0u32.into(),
		)?;
		Perwerde::<T>::create_course(
			RawOrigin::Signed(admin.clone()).into(),
			create_bounded_vec(b"Benchmark Course"),
			create_bounded_vec(b"Description"),
			create_bounded_vec(b"Link")
		)?;

	}: _(RawOrigin::Signed(admin), course_id)
	verify {
		let course = Courses::<T>::get(course_id).unwrap();
		assert_eq!(course.status, CourseStatus::Archived);
	}
}

#[cfg(feature = "std")]
use frame_benchmarking::impl_benchmark_test_suite;
#[cfg(feature = "std")]
impl_benchmark_test_suite!(Perwerde, crate::mock::new_test_ext(), crate::mock::Test);