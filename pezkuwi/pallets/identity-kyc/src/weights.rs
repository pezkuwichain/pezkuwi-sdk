#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// pallet_identity_kyc için gereken ağırlık fonksiyonları.
pub trait WeightInfo {
	fn set_identity() -> Weight;
	fn apply_for_kyc() -> Weight;
	fn approve_kyc() -> Weight;
	fn revoke_kyc() -> Weight;
}

/// Substrate düğümü ve önerilen donanım için ağırlıklar.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn set_identity() -> Weight {
		Weight::from_parts(10_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn apply_for_kyc() -> Weight {
		Weight::from_parts(20_000, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn approve_kyc() -> Weight {
		Weight::from_parts(20_000, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn revoke_kyc() -> Weight {
		Weight::from_parts(15_000, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// Geriye dönük uyumluluk ve testler için
impl WeightInfo for () {
	fn set_identity() -> Weight {
		Weight::from_parts(10_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn apply_for_kyc() -> Weight {
		Weight::from_parts(20_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn approve_kyc() -> Weight {
		Weight::from_parts(20_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn revoke_kyc() -> Weight {
		Weight::from_parts(15_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}