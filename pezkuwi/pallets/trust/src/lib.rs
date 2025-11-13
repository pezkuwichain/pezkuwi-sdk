#![cfg_attr(not(feature = "std"), no_std)]

//! # Trust Score Pallet
//!
//! A pallet for calculating and managing composite trust scores based on multiple ecosystem metrics.
//!
//! ## Overview
//!
//! The Trust Score pallet aggregates multiple reputation and activity metrics to produce
//! a unified trust score for each citizen. This score is used throughout the ecosystem for:
//!
//! - Validator pool eligibility (trust-based validators)
//! - Reward distribution weighting (pez-rewards)
//! - Governance participation rights
//! - Social reputation tracking
//!
//! ## Trust Score Components
//!
//! The trust score is calculated from four primary sources:
//!
//! 1. **Staking Score**: Economic security through token staking
//! 2. **Referral Score**: Network growth contribution via referrals
//! 3. **Perwerde Score**: Educational achievement and verification
//! 4. **Tiki Score**: Social engagement and platform activity
//!
//! ## Score Calculation
//!
//! ```text
//! trust_score = (staking_score + referral_score + perwerde_score + tiki_score) * multiplier
//! ```
//!
//! Where:
//! - Each component score is normalized and weighted
//! - The multiplier is configurable via `ScoreMultiplierBase`
//! - Citizenship status is required (KYC approved)
//!
//! ## Update Mechanisms
//!
//! ### Automatic Updates
//! - Periodic batch updates scheduled at `UpdateInterval` (e.g., daily)
//! - Processes all citizens in batches to manage computational load
//! - Maintains update progress across blocks for large user bases
//!
//! ### Manual Updates
//! - Individual score recalculation via privileged call
//! - Full batch update trigger (root only)
//! - Component change hooks from other pallets
//!
//! ## Storage
//!
//! - `TrustScores` - Per-account trust score mapping
//! - `TotalActiveTrustScore` - Aggregate trust score across all citizens
//! - `BatchUpdateInProgress` - Flag for ongoing batch update process
//! - `LastProcessedAccount` - Checkpoint for resumable batch updates
//!
//! ## Interface
//!
//! ### Extrinsics
//!
//! - `force_recalculate_trust_score(who)` - Manually recalculate specific user's score (root)
//! - `update_all_trust_scores()` - Trigger batch update of all citizens (root)
//!
//! ### Trait Implementations
//!
//! - `TrustScoreProvider` - Query trust scores from other pallets
//! - `TrustScoreUpdater` - Receive notifications of component changes
//!
//! ## Dependencies
//!
//! This pallet requires integration with:
//! - `pallet-identity-kyc` - Citizenship status verification
//! - `pallet-staking-score` - Staking metrics provider
//! - `pallet-referral` - Referral score provider
//! - `pallet-perwerde` - Education score provider
//! - `pallet-tiki` - Social engagement provider
//!
//! ## Runtime Integration Example
//!
//! ```ignore
//! impl pallet_trust::Config for Runtime {
//!     type RuntimeEvent = RuntimeEvent;
//!     type WeightInfo = pallet_trust::weights::SubstrateWeight<Runtime>;
//!     type Score = u128;
//!     type ScoreMultiplierBase = ConstU128<100>;
//!     type UpdateInterval = ConstU32<14400>; // ~1 day in blocks
//!     type StakingScoreSource = StakingScore;
//!     type ReferralScoreSource = Referral;
//!     type PerwerdeScoreSource = Perwerde;
//!     type TikiScoreSource = Tiki;
//!     type CitizenshipSource = IdentityKyc;
//! }
//! ```

pub use pallet::*;

pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet_staking_score::{StakingScoreProvider, RawScore as StakingRawScore};
/* use pezkuwi_primitives::traits::{
    CitizenshipStatusProvider, PerwerdeScoreProvider, ReferralScoreProvider, RawScore,
    StakingDetails, StakingScoreProvider, TikiScoreProvider, TrustScoreUpdater, TrustScoreProvider
}; */

use frame_system::pallet_prelude::BlockNumberFor;
use core::convert::TryFrom;

use frame_support::pallet_prelude::{Get, MaxEncodedLen, Member, IsType, Parameter, ValueQuery, OptionQuery};

pub trait ReferralScoreProvider<AccountId> {
	fn get_referral_score(who: &AccountId) -> u32;
}

pub trait CitizenshipStatusProvider<AccountId> {
    fn is_citizen(who: &AccountId) -> bool;
}

pub trait TrustScoreUpdater<AccountId> {
	fn on_score_component_changed(who: &AccountId);
}

pub trait PerwerdeScoreProvider<AccountId> {
	fn get_perwerde_score(who: &AccountId) -> u32;
}

pub trait TrustScoreProvider<AccountId> {
	fn trust_score_of(who: &AccountId) -> u128;
}

pub trait TikiScoreProvider<AccountId> {
	fn get_tiki_score(who: &AccountId) -> u32;
} 

#[frame_support::pallet]
pub mod pallet {
	use super::{*, weights::WeightInfo};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{Saturating, Zero};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_identity_kyc::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		type Score: Member + Parameter + MaxEncodedLen + Copy + Default + PartialOrd + Saturating + Zero + From<StakingRawScore> + Into<u128> + TryFrom<u128>;

		#[pallet::constant]
		type ScoreMultiplierBase: Get<u128>;

		/// Block interval for Trust score updates (e.g. daily)
		#[pallet::constant]
		type UpdateInterval: Get<BlockNumberFor<Self>>;

		type StakingScoreSource: StakingScoreProvider<Self::AccountId, BlockNumberFor<Self>>;
		type ReferralScoreSource: ReferralScoreProvider<Self::AccountId>;
		type PerwerdeScoreSource: PerwerdeScoreProvider<Self::AccountId>;
		type TikiScoreSource: TikiScoreProvider<Self::AccountId>;
		type CitizenshipSource: CitizenshipStatusProvider<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn trust_score_of)]
	pub type TrustScores<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Score, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_active_trust_score)]
	pub type TotalActiveTrustScore<T: Config> = StorageValue<_, T::Score, ValueQuery>;

	#[pallet::storage]
	pub type LastProcessedAccount<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	pub type BatchUpdateInProgress<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user's Trust Score was successfully updated.
		TrustScoreUpdated { who: T::AccountId, old_score: T::Score, new_score: T::Score },
		/// Total active Trust Score on chain updated.
		TotalTrustScoreUpdated { new_total: T::Score },
		/// A batch Trust Score update completed.
		BulkTrustScoreUpdate { count: u32 },
		/// All Trust Scores update completed.
		AllTrustScoresUpdated { total_updated: u32 },
		/// Periodic Trust Score update scheduled for next time.
		PeriodicUpdateScheduled { next_block: BlockNumberFor<T> },
	}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		CalculationOverflow,
		NotACitizen,
		UpdateInProgress,
	}

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub start_periodic_updates: bool,
		#[serde(skip)]
		pub _phantom: core::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			if self.start_periodic_updates {
				// Schedule first periodic update for 1 day later
				let _first_update_block = frame_system::Pallet::<T>::block_number() + T::UpdateInterval::get();
				
				// Note: Scheduler may not be available during Genesis build
				// In this case, manual start required or scheduled in runtime
				// For now, we are just marking the flag
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// To manually recalculate a specific user's Trust Score.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::force_recalculate_trust_score())]
		pub fn force_recalculate_trust_score(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			Self::update_score_for_account(&who)?;
			Ok(())
		}

		/// Updates Trust Scores of all citizens in bulk
		/// Works in batches for large user base
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::update_all_trust_scores())]
		pub fn update_all_trust_scores(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			
			let batch_size = Self::calculate_optimal_batch_size();
			let mut updated_count = 0u32;
			let mut all_processed = true;
			
			// Get last processed account (or start from beginning)
			let start_key = LastProcessedAccount::<T>::get();
			let mut found_start = start_key.is_none();

			// Iterate over all accounts with KYC status from identity-kyc pallet
			// Only process accounts with Approved KYC status (citizens)
			for (account, kyc_level) in pallet_identity_kyc::KycStatuses::<T>::iter() {
				// If we are looking for the starting point
				if !found_start {
					if Some(&account) == start_key.as_ref() {
						found_start = true;
					}
					continue;
				}

				// Is batch limit full?
				if updated_count >= batch_size {
					// Save last processed account
					LastProcessedAccount::<T>::put(account.clone());
					all_processed = false;
					break;
				}

				// Only process accounts with Approved KYC (citizens)
				// We already have kyc_level from iterator, no need for redundant lookup
				if kyc_level == pallet_identity_kyc::types::KycLevel::Approved {
					let _ = Self::update_score_for_account(&account);
					updated_count += 1;
				}
			}
			
			// If all accounts processed, return to start
			if all_processed {
				LastProcessedAccount::<T>::kill();
				BatchUpdateInProgress::<T>::put(false);
				Self::deposit_event(Event::AllTrustScoresUpdated { total_updated: updated_count });
			} else {
				BatchUpdateInProgress::<T>::put(true);
				Self::deposit_event(Event::BulkTrustScoreUpdate { count: updated_count });
			}
			
			Ok(())
		}

		/// Periyodik güncellemeyi başlatan function
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::periodic_trust_score_update())]
		pub fn periodic_trust_score_update(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			
			// Eğer önceki update devam ediyorsa bekle
			ensure!(!BatchUpdateInProgress::<T>::get(), Error::<T>::UpdateInProgress);
			
			// Yeni periyodik güncellemeyi başlat
			Self::update_all_trust_scores(OriginFor::<T>::root())?;
			
			// Bir sonraki periyodik güncellemeyi schedule et
			let current_block = frame_system::Pallet::<T>::block_number();
			let next_update_block = current_block + T::UpdateInterval::get();
			
			Self::deposit_event(Event::PeriodicUpdateScheduled { next_block: next_update_block });
			
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn calculate_trust_score(who: &T::AccountId) -> Result<T::Score, Error<T>> {
			ensure!(T::CitizenshipSource::is_citizen(who), Error::<T>::NotACitizen);

			let (staking_score_raw, _) = T::StakingScoreSource::get_staking_score(who);
			if staking_score_raw.is_zero() {
				return Ok(T::Score::zero());
			}

			let staking_u128: u128 = staking_score_raw.into();
			let referral_u128: u128 = T::ReferralScoreSource::get_referral_score(who).into();
			let perwerde_u128: u128 = T::PerwerdeScoreSource::get_perwerde_score(who).into();
			let tiki_u128: u128 = T::TikiScoreSource::get_tiki_score(who).into();
			
			let base = T::ScoreMultiplierBase::get();

			let weighted_sum = staking_u128.saturating_mul(100)
				.saturating_add(referral_u128.saturating_mul(300))
				.saturating_add(perwerde_u128.saturating_mul(300))
				.saturating_add(tiki_u128.saturating_mul(300));

			let final_score_u128 = staking_u128
				.saturating_mul(weighted_sum)
				.checked_div(base)
				.ok_or(Error::<T>::CalculationOverflow)?;

			let new_trust_score = T::Score::try_from(final_score_u128).map_err(|_| Error::<T>::CalculationOverflow)?;
			
			Ok(new_trust_score)
		}

		pub fn update_score_for_account(who: &T::AccountId) -> Result<T::Score, Error<T>> {
			let old_score = Self::trust_score_of(who);
			let new_score = Self::calculate_trust_score(who)?;

			if old_score != new_score {
				<TrustScores<T>>::insert(who, new_score);
				let old_total = Self::total_active_trust_score();
				let new_total = old_total.saturating_sub(old_score).saturating_add(new_score);
				<TotalActiveTrustScore<T>>::put(new_total);
				Self::deposit_event(Event::TrustScoreUpdated { who: who.clone(), old_score, new_score });
				Self::deposit_event(Event::TotalTrustScoreUpdated { new_total });
			}
			Ok(new_score)
		}

		/// Kullanıcı sayısına göre dinamik batch size hesaplar
		fn calculate_optimal_batch_size() -> u32 {
			// Count total users with KYC from identity-kyc pallet
			let total_users = pallet_identity_kyc::KycStatuses::<T>::iter()
				.filter(|(_, level)| *level == pallet_identity_kyc::types::KycLevel::Approved)
				.count() as u32;

			match total_users {
				0..=100 => total_users,           // Az kullanıcı varsa hepsini işle
				101..=1000 => 100,                // Orta: 100'lük batch'ler
				1001..=10000 => 200,              // Çok: 200'lük batch'ler
				_ => 500,                         // Çok fazla: 500'lük batch'ler
			}
		}
	}

	impl<T: Config> TrustScoreProvider<T::AccountId> for Pallet<T> {
		fn trust_score_of(who: &T::AccountId) -> u128 {
			Self::trust_score_of(who).into()
		}
	}

	impl<T: Config> TrustScoreUpdater<T::AccountId> for Pallet<T> {
		fn on_score_component_changed(who: &T::AccountId) {
			if let Err(e) = Self::update_score_for_account(who) {
				log::error!("Failed to update trust score for {:?}: {:?}", who, e);
			}
		}
	}
}