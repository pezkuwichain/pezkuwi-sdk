#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

// Bu modüller sadece `std` ortamında derlenmelidir.
#[cfg(all(feature = "std", any(test, feature = "runtime-benchmarks")))]
pub mod mock;

#[cfg(all(feature = "std", test))]
mod tests;

pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{EnsureOrigin, Get},
	};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MaxCourseNameLength: Get<u32>;
		#[pallet::constant]
		type MaxCourseDescLength: Get<u32>;
		#[pallet::constant]
		type MaxCourseLinkLength: Get<u32>;
		#[pallet::constant]
		type MaxStudentsPerCourse: Get<u32>;
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum CourseStatus {
		Active,
		Archived,
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Course<T: Config> {
		pub id: u32,
		pub owner: T::AccountId,
		pub name: BoundedVec<u8, T::MaxCourseNameLength>,
		pub description: BoundedVec<u8, T::MaxCourseDescLength>,
		pub content_link: BoundedVec<u8, T::MaxCourseLinkLength>,
		pub status: CourseStatus,
		pub created_at: BlockNumberFor<T>,
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Enrollment<T: Config> {
		pub student: T::AccountId,
		pub course_id: u32,
		pub enrolled_at: BlockNumberFor<T>,
		pub completed_at: Option<BlockNumberFor<T>>,
		pub points_earned: u32,
	}

	#[pallet::storage]
	#[pallet::getter(fn courses)]
	pub type Courses<T: Config> = StorageMap<_, Blake2_128Concat, u32, Course<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_course_id)]
	pub type NextCourseId<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn enrollments)]
	pub type Enrollments<T: Config> =
		StorageMap<_, Blake2_128Concat, (T::AccountId, u32), Enrollment<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn student_courses)]
	pub type StudentCourses<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<u32, T::MaxStudentsPerCourse>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CourseCreated { course_id: u32, owner: T::AccountId },
		StudentEnrolled { student: T::AccountId, course_id: u32 },
		CourseCompleted { student: T::AccountId, course_id: u32, points: u32 },
		CourseArchived { course_id: u32 },
	}

	#[pallet::error]
	pub enum Error<T> {
		CourseNotFound,
		AlreadyEnrolled,
		NotEnrolled,
		CourseNotActive,
		CourseAlreadyCompleted,
		NotCourseOwner,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_course(name.len() as u32, description.len() as u32, content_link.len() as u32))]
		pub fn create_course(
			origin: OriginFor<T>,
			name: Vec<u8>,
			description: Vec<u8>,
			content_link: Vec<u8>,
		) -> DispatchResult {
			let owner = T::AdminOrigin::ensure_origin(origin)?;
			let course_id = NextCourseId::<T>::get();

			let bounded_name: BoundedVec<u8, T::MaxCourseNameLength> =
				name.try_into().expect("Name too long");
			let bounded_desc: BoundedVec<u8, T::MaxCourseDescLength> =
				description.try_into().expect("Description too long");
			let bounded_link: BoundedVec<u8, T::MaxCourseLinkLength> =
				content_link.try_into().expect("Link too long");

			let course = Course {
				id: course_id,
				owner: owner.clone(),
				name: bounded_name,
				description: bounded_desc,
				content_link: bounded_link,
				status: CourseStatus::Active,
				created_at: frame_system::Pallet::<T>::block_number(),
			};

			Courses::<T>::insert(course_id, course);
			NextCourseId::<T>::mutate(|id| *id += 1);

			Self::deposit_event(Event::CourseCreated { course_id, owner });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::enroll())]
		pub fn enroll(origin: OriginFor<T>, course_id: u32) -> DispatchResult {
			let student = ensure_signed(origin)?;
			let course = Courses::<T>::get(course_id).ok_or(Error::<T>::CourseNotFound)?;
			ensure!(course.status == CourseStatus::Active, Error::<T>::CourseNotActive);
			ensure!(!Enrollments::<T>::contains_key((&student, course_id)), Error::<T>::AlreadyEnrolled);

			let enrollment = Enrollment {
				student: student.clone(),
				course_id,
				enrolled_at: frame_system::Pallet::<T>::block_number(),
				completed_at: None,
				points_earned: 0,
			};

			Enrollments::<T>::insert((&student, course_id), enrollment);
			StudentCourses::<T>::mutate(&student, |courses| {
				courses.try_push(course_id).expect("Student has too many courses");
			});

			Self::deposit_event(Event::StudentEnrolled { student, course_id });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::complete_course())]
		pub fn complete_course(origin: OriginFor<T>, course_id: u32, points: u32) -> DispatchResult {
			let student = ensure_signed(origin)?;
			let mut enrollment = Enrollments::<T>::get((&student, course_id)).ok_or(Error::<T>::NotEnrolled)?;
			ensure!(enrollment.completed_at.is_none(), Error::<T>::CourseAlreadyCompleted);

			enrollment.completed_at = Some(frame_system::Pallet::<T>::block_number());
			enrollment.points_earned = points;

			Enrollments::<T>::insert((&student, course_id), enrollment);

			Self::deposit_event(Event::CourseCompleted { student, course_id, points });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::archive_course())]
		pub fn archive_course(origin: OriginFor<T>, course_id: u32) -> DispatchResult {
			let caller = T::AdminOrigin::ensure_origin(origin)?;
			let mut course = Courses::<T>::get(course_id).ok_or(Error::<T>::CourseNotFound)?;
			ensure!(course.owner == caller, Error::<T>::NotCourseOwner);

			course.status = CourseStatus::Archived;
			Courses::<T>::insert(course_id, course);

			Self::deposit_event(Event::CourseArchived { course_id });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn get_perwerde_score(who: &T::AccountId) -> u32 {
			StudentCourses::<T>::get(who)
				.iter()
				.filter_map(|course_id| Enrollments::<T>::get((who, *course_id)))
				.filter(|enrollment| enrollment.completed_at.is_some())
				.map(|enrollment| enrollment.points_earned)
				.sum()
		}
	}
}