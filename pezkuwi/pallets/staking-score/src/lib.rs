#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::pallet_prelude::*;
use pezkuwi_primitives::{
    traits::{StakingInfoProvider, StakingScoreProvider},
    types::RawScore,
};
use sp_runtime::traits::Saturating;

// Zaman aralıklarını blok numarası cinsinden tanımlıyoruz (Rococo için 6 saniyede 1 blok varsayımıyla)
pub const MINUTE_IN_BLOCKS: u32 = 10;
pub const HOUR_IN_BLOCKS: u32 = 60 * MINUTE_IN_BLOCKS;
pub const DAY_IN_BLOCKS: u32 = 24 * HOUR_IN_BLOCKS;
pub const MONTH_IN_BLOCKS: u32 = 30 * DAY_IN_BLOCKS;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Staking verilerini okumak için kullanılacak arayüz.
        type StakingInfo: StakingInfoProvider<Self::AccountId, BalanceOf<Self>>;
    }
    
    // Bu palet bir hesaplama motoru olduğu için event, error, storage veya call'a ihtiyaç duymaz.
}

// Balance tipini Config'den almak için bir takma ad.
type BalanceOf<T> = <<T as Config>::StakingInfo as StakingInfoProvider<
    <T as frame_system::Config>::AccountId,
>>::Balance;

// Staking Puanı hesaplama mantığını burada implemente ediyoruz.
impl<T: Config> StakingScoreProvider<T::AccountId> for Pallet<T> {
    fn get_staking_score(who: &T::AccountId) -> RawScore {
        let (staked_amount, start_block) = T::StakingInfo::get_staking_details(who);
        
        // --- Miktar Puanı Hesaplama ---
        const UNITS: u128 = 1_000_000_000_000; // 1 HEZ = 10^12
        let staked_hez = staked_amount / UNITS;

        let amount_score = match staked_hez {
            0 => return 0, // Sıfır stake, sıfır puan kuralı
            1..=100 => 20,
            101..=250 => 30,
            251..=750 => 40,
            _ => 50, // 751+ HEZ
        };

        // --- Süre Çarpanı Hesaplama ---
        let current_block = frame_system::Pallet::<T>::block_number();
        let duration_in_blocks = current_block.saturating_sub(start_block.into());

        let duration_multiplier = if duration_in_blocks > 12 * MONTH_IN_BLOCKS {
            Perbill::from_rational(20u32, 10u32) // x2.0
        } else if duration_in_blocks > 6 * MONTH_IN_BLOCKS {
            Perbill::from_rational(17u32, 10u32) // x1.7
        } else if duration_in_blocks > 3 * MONTH_IN_BLOCKS {
            Perbill::from_rational(14u32, 10u32) // x1.4
        } else if duration_in_blocks > MONTH_IN_BLOCKS {
            Perbill::from_rational(12u32, 10u32) // x1.2
        } else {
            Perbill::from_rational(10u32, 10u32) // x1.0
        };

        // --- Nihai Puan ---
        let final_score = duration_multiplier * amount_score;
        final_score.min(100)
    }
}