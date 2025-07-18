// === pallets/hemwelati-odul/src/weights.rs ===

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_hemwelati_odul.
pub trait WeightInfo {
    fn claim_reward() -> Weight;
    fn start_new_period() -> Weight;
    fn complete_period() -> Weight;
}

/// Weights for pallet_hemwelati_odul using the Substrate node and recommended hardware.
/// **BU SADECE YER TUTUCUDUR! BENCHMARK ÇALIŞTIRILMALI!**
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn claim_reward() -> Weight {
        // Örnek ağırlık, gerçek benchmark sonuçları ile güncellenmeli
        Weight::from_parts(45_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(3)) // ClaimedRewards, RewardPeriods, (Pallet Account read for transfer)
            .saturating_add(T::DbWeight::get().writes(2)) // ClaimedRewards, (Beneficiary Account write for transfer)
    }
    fn start_new_period() -> Weight {
        Weight::from_parts(25_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(1)) // NextPeriodId
            .saturating_add(T::DbWeight::get().writes(2)) // RewardPeriods, NextPeriodId
    }
    fn complete_period() -> Weight {
        Weight::from_parts(20_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))  // RewardPeriods
            .saturating_add(T::DbWeight::get().writes(1)) // RewardPeriods
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn claim_reward() -> Weight {
        Weight::from_parts(45_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
    fn start_new_period() -> Weight {
        Weight::from_parts(25_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
    fn complete_period() -> Weight {
        Weight::from_parts(20_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
}