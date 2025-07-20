#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_perwerde`.
pub trait WeightInfo {
	fn create_course(name_len: u32, desc_len: u32, link_len: u32) -> Weight;
	fn enroll() -> Weight;
	fn complete_course() -> Weight;
	fn archive_course() -> Weight;
	fn benchmark_get_perwerde_score(c: u32, ) -> Weight;
}

/// Weights for `pallet_perwerde` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Perwerde::NextCourseId` (r:1 w:1)
	/// Proof: `Perwerde::NextCourseId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Perwerde::Courses` (r:0 w:1)
	/// Proof: `Perwerde::Courses` (`max_values`: None, `max_size`: Some(1724), added: 4199, mode: `MaxEncodedLen`)
	/// The range of component `name_len` is `[1, 100]`.
	/// The range of component `desc_len` is `[1, 500]`.
	/// The range of component `link_len` is `[1, 200]`.
	fn create_course(name_len: u32, desc_len: u32, link_len: u32, ) -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_parts(30_763_000, 0)
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(2_000, 0).saturating_mul(name_len.into()))
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(1_000, 0).saturating_mul(desc_len.into()))
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(2_000, 0).saturating_mul(link_len.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Perwerde::Courses` (r:1 w:1)
	/// Proof: `Perwerde::Courses` (`max_values`: None, `max_size`: Some(1724), added: 4199, mode: `MaxEncodedLen`)
	/// Storage: `Perwerde::Enrollments` (r:1 w:1)
	/// Proof: `Perwerde::Enrollments` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	fn enroll() -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_parts(36_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Perwerde::Enrollments` (r:1 w:1)
	/// Proof: `Perwerde::Enrollments` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Perwerde::CompletedCourses` (r:1 w:1)
	/// Proof: `Perwerde::CompletedCourses` (`max_values`: None, `max_size`: Some(40042), added: 42517, mode: `MaxEncodedLen`)
	fn complete_course() -> Weight {
		// Minimum execution time: 32_000 nanoseconds.
		Weight::from_parts(34_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Perwerde::Courses` (r:1 w:1)
	/// Proof: `Perwerde::Courses` (`max_values`: None, `max_size`: Some(1724), added: 4199, mode: `MaxEncodedLen`)
	fn archive_course() -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_parts(25_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Perwerde::CompletedCourses` (r:1 w:0)
	/// Proof: `Perwerde::CompletedCourses` (`max_values`: None, `max_size`: Some(40042), added: 42517, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn benchmark_get_perwerde_score(c: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_parts(9_064_500, 0)
			// Standard Error: 3_000
			.saturating_add(Weight::from_parts(1_000, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}

// For backwards compatibility, we have a hardware specific implementation file.
// You can ignore this block, benchmark will overwrite this automatically.
impl WeightInfo for () {
	/// Storage: `Perwerde::NextCourseId` (r:1 w:1)
	/// Proof: `Perwerde::NextCourseId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Perwerde::Courses` (r:0 w:1)
	/// Proof: `Perwerde::Courses` (`max_values`: None, `max_size`: Some(1724), added: 4199, mode: `MaxEncodedLen`)
	/// The range of component `name_len` is `[1, 100]`.
	/// The range of component `desc_len` is `[1, 500]`.
	/// The range of component `link_len` is `[1, 200]`.
	fn create_course(name_len: u32, desc_len: u32, link_len: u32, ) -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_parts(30_763_000, 0)
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(2_000, 0).saturating_mul(name_len.into()))
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(1_000, 0).saturating_mul(desc_len.into()))
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(2_000, 0).saturating_mul(link_len.into()))
			.saturating_add(Weight::from_parts(0, 0).saturating_mul(2_u64))
			.saturating_add(Weight::from_parts(0, 0).saturating_mul(1_u64))
	}
	/// Storage: `Perwerde::Courses` (r:1 w:1)
	/// Proof: `Perwerde::Courses` (`max_values`: None, `max_size`: Some(1724), added: 4199, mode: `MaxEncodedLen`)
	/// Storage: `Perwerde::Enrollments` (r:1 w:1)
	/// Proof: `Perwerde::Enrollments` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	fn enroll() -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_parts(36_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0).saturating_mul(2_u64))
			.saturating_add(Weight::from_parts(0, 0).saturating_mul(2_u64))
	}
	/// Storage: `Perwerde::Enrollments` (r:1 w:1)
	/// Proof: `Perwerde::Enrollments` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Perwerde::CompletedCourses` (r:1 w:1)
	/// Proof: `Perwerde::CompletedCourses` (`max_values`: None, `max_size`: Some(40042), added: 42517, mode: `MaxEncodedLen`)
	fn complete_course() -> Weight {
		// Minimum execution time: 32_000 nanoseconds.
		Weight::from_parts(34_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0).saturating_mul(2_u64))
			.saturating_add(Weight::from_parts(0, 0).saturating_mul(2_u64))
	}
	/// Storage: `Perwerde::Courses` (r:1 w:1)
	/// Proof: `Perwerde::Courses` (`max_values`: None, `max_size`: Some(1724), added: 4199, mode: `MaxEncodedLen`)
	fn archive_course() -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_parts(25_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0).saturating_mul(1_u64))
			.saturating_add(Weight::from_parts(0, 0).saturating_mul(1_u64))
	}
	/// Storage: `Perwerde::CompletedCourses` (r:1 w:0)
	/// Proof: `Perwerde::CompletedCourses` (`max_values`: None, `max_size`: Some(40042), added: 42517, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn benchmark_get_perwerde_score(c: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_parts(9_064_500, 0)
			// Standard Error: 3_000
			.saturating_add(Weight::from_parts(1_000, 0).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 0).saturating_mul(1_u64))
	}
}