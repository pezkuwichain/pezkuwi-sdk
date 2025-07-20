#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_staking_score.
pub trait WeightInfo {
	fn start_score_tracking() -> Weight;
}

/// Weights for pallet_staking_score using the Substrate node and recommended hardware.
/// BU SADECE YER TUTUCUDUR! GERÇEK BENCHMARK SONUÇLARI KULLANILMALI!
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Weight for the `start_score_tracking` extrinsic.
	fn start_score_tracking() -> Weight {
		// Örnek: 1 Storage Read + 1 StorageMap Get + 1 StorageMap Insert
		Weight::from_parts(30_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	 fn start_score_tracking() -> Weight {
		Weight::from_parts(30_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}