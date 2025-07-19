#![cfg_attr(not(feature = "std"), no_std)]

// Bu paletin benchmark'larını içerecek olan modül
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// Bu paletin birim testlerini içerecek olan modül
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// Bu paletin ağırlıklarını içeren modül
pub mod weights;

// Gerekli import'lar
use frame_support::pallet_prelude::{BoundedBTreeSet, *};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Zero;
use sp_std::prelude::*;
use pezkuwi_primitives::traits::PerwerdeScoreProvider;

// Pallet'in kendisini ve ağırlık bilgisini dışa aktarma
pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    // Tiplerimizi tanımlıyoruz
    pub type CourseId = u64;
    pub type Points = u64;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum CourseStatus {
        Active,
        Archived,
    }

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct CourseDetails<T: Config> {
        pub owner: T::AccountId,
        pub name: BoundedVec<u8, T::MaxCourseNameLength>,
        pub description: BoundedVec<u8, T::MaxCourseDescLength>,
        pub content_link: BoundedVec<u8, T::MaxCourseLinkLength>,
        pub status: CourseStatus,
        pub students: BoundedBTreeSet<T::AccountId, T::MaxStudentsPerCourse>,
    }

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Enrollment<T: Config> {
        pub student: T::AccountId,
        pub enrolled_at: BlockNumberFor<T>,
        pub completed_at: Option<BlockNumberFor<T>>,
        pub points_earned: Points,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
   pub trait Config: frame_system::Config + TypeInfo {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>; // Admin yetkisi ve AccountId'si
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

    #[pallet::storage]
    #[pallet::getter(fn next_course_id)]
    pub type NextCourseId<T: Config> = StorageValue<_, CourseId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn courses)]
    pub type Courses<T: Config> =
        StorageMap<_, Blake2_128Concat, CourseId, CourseDetails<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn enrollments)]
    pub type Enrollments<T: Config> =
        StorageMap<_, Blake2_128Concat, (T::AccountId, CourseId), Enrollment<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn completed_courses)]
    pub type CompletedCourses<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedBTreeSet<CourseId, T::MaxStudentsPerCourse>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CourseCreated { course_id: CourseId, owner: T::AccountId },
        CourseEnrolled { student: T::AccountId, course_id: CourseId },
        CourseCompleted { student: T::AccountId, course_id: CourseId, points_earned: Points },
        CourseArchived { course_id: CourseId },
    }

    #[pallet::error]
    pub enum Error<T> {
        CourseNotFound,
        AlreadyEnrolled,
        NotEnrolled,
        AlreadyCompleted,
        NotCourseOwner,
        CourseArchived,
        TooManyStudents,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_course(name.len() as u32, description.len() as u32, content_link.len() as u32))]
        pub fn create_course(
            origin: OriginFor<T>,
            name: BoundedVec<u8, T::MaxCourseNameLength>,
            description: BoundedVec<u8, T::MaxCourseDescLength>,
            content_link: BoundedVec<u8, T::MaxCourseLinkLength>,
        ) -> DispatchResult {
            let owner = T::AdminOrigin::ensure_origin(origin)?;

            let course_id = NextCourseId::<T>::get();
            let new_course_id = course_id.checked_add(1).ok_or(Error::<T>::CourseNotFound)?; // Basit bir overflow kontrolü

            let details = CourseDetails::<T> {
                owner: owner.clone(),
                name,
                description,
                content_link,
                status: CourseStatus::Active,
                students: BoundedBTreeSet::new(),
            };

            Courses::<T>::insert(course_id, details);
            NextCourseId::<T>::put(new_course_id);

            Self::deposit_event(Event::CourseCreated { course_id, owner });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::enroll())]
        pub fn enroll(origin: OriginFor<T>, course_id: CourseId) -> DispatchResult {
            let student = ensure_signed(origin)?;
            let mut course = Courses::<T>::get(course_id).ok_or(Error::<T>::CourseNotFound)?;
            ensure!(course.status == CourseStatus::Active, Error::<T>::CourseArchived);
            ensure!(!Enrollments::<T>::contains_key((&student, course_id)), Error::<T>::AlreadyEnrolled);

            course.students.try_insert(student.clone()).map_err(|_| Error::<T>::TooManyStudents)?;
            Courses::<T>::insert(course_id, course);

            let enrollment = Enrollment {
                student: student.clone(),
                enrolled_at: <frame_system::Pallet<T>>::block_number(),
                completed_at: None,
                points_earned: Zero::zero(),
            };
            Enrollments::<T>::insert((student.clone(), course_id), enrollment);

            Self::deposit_event(Event::CourseEnrolled { student, course_id });
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::complete_course())]
        pub fn complete_course(origin: OriginFor<T>, course_id: CourseId, points_earned: Points) -> DispatchResult {
            let student = ensure_signed(origin)?;
            let mut enrollment = Enrollments::<T>::get((&student, course_id)).ok_or(Error::<T>::NotEnrolled)?;
            ensure!(enrollment.completed_at.is_none(), Error::<T>::AlreadyCompleted);

            enrollment.completed_at = Some(<frame_system::Pallet<T>>::block_number());
            enrollment.points_earned = points_earned;
            Enrollments::<T>::insert((student.clone(), course_id), enrollment);
            
            CompletedCourses::<T>::try_mutate(&student, |completed| {
                completed.try_insert(course_id).map_err(|_| Error::<T>::TooManyStudents)
            })?;

            Self::deposit_event(Event::CourseCompleted { student, course_id, points_earned });
            Ok(())
        }
        
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::archive_course())]
        pub fn archive_course(origin: OriginFor<T>, course_id: CourseId) -> DispatchResult {
            let sender = T::AdminOrigin::ensure_origin(origin)?;
            let mut course = Courses::<T>::get(course_id).ok_or(Error::<T>::CourseNotFound)?;
            ensure!(course.owner == sender, Error::<T>::NotCourseOwner);
            
            course.status = CourseStatus::Archived;
            Courses::<T>::insert(course_id, course);

            Self::deposit_event(Event::CourseArchived { course_id });
            Ok(())
        }
        // Bu extrinsic sadece `get_perwerde_score` fonksiyonunun ağırlığını ölçmek için kullanılır.
		#[cfg(feature = "runtime-benchmarks")]
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::get_perwerde_score(T::MaxStudentsPerCourse::get()))]
		pub fn benchmark_get_perwerde_score(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_signed(origin)?;
			let _score = Self::get_perwerde_score(&who);
			Ok(())
		}
    }
}

// PerwerdeScoreProvider trait'inin implementasyonu
impl<T: Config> PerwerdeScoreProvider<T::AccountId> for Pallet<T> {
    type Score = u32;

    fn get_perwerde_score(who: &T::AccountId) -> u32 {
        let completed = CompletedCourses::<T>::get(who);
        let completed_count = completed.len() as u32;

        let course_score = match completed_count {
            0 => 0,
            1..=5 => completed_count * 10,
            _ => 50,
        };

        (course_score).min(100)
    }
}