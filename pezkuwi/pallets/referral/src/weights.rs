#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_referral.
pub trait WeightInfo {
	fn initiate_referral() -> Weight;
	fn hook_on_kyc_approved_found() -> Weight;
	fn hook_on_kyc_approved_not_found() -> Weight;
}

/// Weights for pallet_referral using the Substrate node and recommended hardware.
/// **BU SADECE YER TUTUCUDUR! GERÇEK BENCHMARK SONUÇLARI KULLANILMALI!**
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Weight for the `initiate_referral` extrinsic.
	fn initiate_referral() -> Weight {
		// Örnek: 2 StorageMap Read + 1 StorageMap Write
		Weight::from_parts(20_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}

	/// Weight for the `on_kyc_approved` hook when a referral is found.
	fn hook_on_kyc_approved_found() -> Weight {
		// Örnek: 1 StorageMap Take + 1 StorageMap Mutate + 1 StorageMap Insert
		Weight::from_parts(50_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	/// Weight for the `on_kyc_approved` hook when no referral is found.
	fn hook_on_kyc_approved_not_found() -> Weight {
		// Örnek: 1 StorageMap Get
		Weight::from_parts(10_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	 fn initiate_referral() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	fn hook_on_kyc_approved_found() -> Weight {
		Weight::from_parts(50_000_000, 0)
			 .saturating_add(RocksDbWeight::get().reads(1))
			 .saturating_add(RocksDbWeight::get().writes(2))
	}
	fn hook_on_kyc_approved_not_found() -> Weight {
		Weight::from_parts(10_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
	}
}