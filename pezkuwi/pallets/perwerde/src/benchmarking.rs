//! Benchmarking setup for pallet-perwerde
#![cfg(feature = "runtime-benchmarks")]
use super::{Pallet as Perwerde, *};

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use pallet_collective::Instance1;
use sp_std::vec::Vec; // Vec için bu satırı ekledik
use sp_std::vec;

const SEED: u32 = 0;

benchmarks! {
	where_clause {
		where
			T: pallet_collective::Config<Instance1>,
	}

	create_course {
		let name: Vec<u8> = b"Substrate perwerdei".to_vec();
		let description: Vec<u8> = b"Bu perwerde Substrate temellerini kapsar.".to_vec();
		let content_link: Vec<u8> = b"http://example.com".to_vec();
		let name_len = name.len() as u32;
		let desc_len = description.len() as u32;
		let link_len = content_link.len() as u32;

		// Admin hesabını `account` fonksiyonu ile deterministik olarak oluştur.
		let admin: T::AccountId = whitelisted_caller();
		pallet_collective::Pallet::<T, Instance1>::set_members(RawOrigin::Root.into(), vec![admin.clone()], Some(admin.clone()), 0u32.into())?;

	}: _(RawOrigin::Signed(admin), name, description, content_link)
	verify {
		assert!(Courses::<T>::get(0).is_some());
	}

	enroll {
		let student: T::AccountId = whitelisted_caller();
		let course_id = 0;

		let admin: T::AccountId = account("admin", 0, SEED); // Admin'i whitelisted_caller yapmıyoruz, çünkü enroll benchmark'ında admin origin'i kullanmıyoruz.
		pallet_collective::Pallet::<T, Instance1>::set_members(RawOrigin::Root.into(), vec![admin.clone()], Some(admin.clone()), 0u32.into())?;
		
		Perwerde::<T>::create_course(
			RawOrigin::Signed(admin).into(),
			b"Test Kursu".to_vec(),
			b"Test aciklama".to_vec(),
			b"http://test.com".to_vec(),
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

		let admin: T::AccountId = account("admin", 0, SEED);
		pallet_collective::Pallet::<T, Instance1>::set_members(RawOrigin::Root.into(), vec![admin.clone()], Some(admin.clone()), 0u32.into())?;
		Perwerde::<T>::create_course(RawOrigin::Signed(admin).into(), b"Test".to_vec(), b"Test".to_vec(), b"Test".to_vec())?;
		Perwerde::<T>::enroll(student_origin.clone().into(), course_id)?;

	}: _(student_origin, course_id, points)
	verify {
		let enrollment = Enrollments::<T>::get((student, course_id)).unwrap();
		assert!(enrollment.completed_at.is_some());
		assert_eq!(enrollment.points_earned, points);
	}

	archive_course {
		let course_id = 0;
		
		let admin: T::AccountId = whitelisted_caller();
		pallet_collective::Pallet::<T, Instance1>::set_members(RawOrigin::Root.into(), vec![admin.clone()], Some(admin.clone()), 0u32.into())?;
		Perwerde::<T>::create_course(RawOrigin::Signed(admin.clone()).into(), b"Test".to_vec(), b"Test".to_vec(), b"Test".to_vec())?;

	}: _(RawOrigin::Signed(admin), course_id)
	verify {
		let course = Courses::<T>::get(course_id).unwrap();
		assert_eq!(course.status, CourseStatus::Archived);
	}
}

#[cfg(feature = "std")] // Burada boşluk kaldırıldı.
use frame_benchmarking::impl_benchmark_test_suite;
#[cfg(feature = "std")]
impl_benchmark_test_suite!(Perwerde, crate::mock::new_test_ext(), crate::mock::Test);