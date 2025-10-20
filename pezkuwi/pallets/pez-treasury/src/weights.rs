// pezkuwi/pallets/pez-treasury/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_pez_treasury`.
pub trait WeightInfo {
	fn initialize_treasury() -> Weight;
	fn force_genesis_distribution() -> Weight;
	fn release_monthly_funds() -> Weight;
}

/// Weights for `pallet_pez_treasury` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn initialize_treasury() -> Weight {
		Weight::from_parts(9_749_000, 0)
			.saturating_add(Weight::from_parts(0, 42))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}

	fn force_genesis_distribution() -> Weight {
		Weight::from_parts(62_700_000, 0)
			.saturating_add(Weight::from_parts(0, 615))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}

	fn release_monthly_funds() -> Weight {
		Weight::from_parts(94_980_000, 0)
			.saturating_add(Weight::from_parts(0, 772))
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	fn initialize_treasury() -> Weight {
		Weight::from_parts(9_749_000, 0)
			.saturating_add(Weight::from_parts(0, 42))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}

	fn force_genesis_distribution() -> Weight {
		Weight::from_parts(62_700_000, 0)
			.saturating_add(Weight::from_parts(0, 615))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}

	fn release_monthly_funds() -> Weight {
		Weight::from_parts(94_980_000, 0)
			.saturating_add(Weight::from_parts(0, 772))
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
}