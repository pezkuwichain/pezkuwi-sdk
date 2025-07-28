#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn force_recalculate_trust_score() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn force_recalculate_trust_score() -> Weight {
		Weight::from_parts(10_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

impl WeightInfo for () {
	fn force_recalculate_trust_score() -> Weight {
		Weight::from_parts(10_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}