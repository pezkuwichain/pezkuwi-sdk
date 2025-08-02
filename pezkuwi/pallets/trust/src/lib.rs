#![cfg_attr(not(feature = "std"), no_std)]

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
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		type Score: Member + Parameter + MaxEncodedLen + Copy + Default + PartialOrd + Saturating + Zero + From<StakingRawScore> + Into<u128> + TryFrom<u128>;

		#[pallet::constant]
		type ScoreMultiplierBase: Get<u128>;

		/// Trust score güncellemelerinin yapılacağı block aralığı (örn. günlük)
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
		/// Bir kullanıcının Trust Puanı başarıyla güncellendi.
		TrustScoreUpdated { who: T::AccountId, old_score: T::Score, new_score: T::Score },
		/// Zincirdeki toplam aktif Trust Puanı güncellendi.
		TotalTrustScoreUpdated { new_total: T::Score },
		/// Bir batch Trust Puanı güncellemesi tamamlandı.
		BulkTrustScoreUpdate { count: u32 },
		/// Tüm Trust Puanları güncellemesi tamamlandı.
		AllTrustScoresUpdated { total_updated: u32 },
		/// Periyodik Trust Puanı güncellemesi sonraki defa için schedule edildi.
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
				// İlk periyodik güncellemeyi 1 gün sonraya schedule et
				let _first_update_block = frame_system::Pallet::<T>::block_number() + T::UpdateInterval::get();
				
				// Note: Genesis build sırasında scheduler kullanılamayabilir
				// Bu durumda manual başlatma gerekir veya runtime'da schedule edilir
				// Şimdilik sadece flag'i işaretliyoruz
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Belirli bir kullanıcının Trust Puanını manuel olarak yeniden hesaplamak için.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::force_recalculate_trust_score())]
		pub fn force_recalculate_trust_score(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			Self::update_score_for_account(&who)?;
			Ok(())
		}

		/// Tüm vatandaşların Trust Puanlarını toplu olarak günceller
		/// Büyük kullanıcı tabanı için batch'ler halinde çalışır
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::update_all_trust_scores())]
		pub fn update_all_trust_scores(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			
			let batch_size = Self::calculate_optimal_batch_size();
			let mut updated_count = 0u32;
			let mut all_processed = true;
			
			// Son işlenen hesabı al (yoksa baştan başla)
			let start_key = LastProcessedAccount::<T>::get();
			let mut found_start = start_key.is_none();
			
			// Tüm hesapları tara (gerçek implementasyonda KYC paletinden gelecek)
			// Şimdilik mock veriler kullanıyoruz - Bu kısmı gerçek implementasyonda değiştireceğiz
			let mock_accounts: sp_std::vec::Vec<T::AccountId> = sp_std::vec![];
			
			for account in mock_accounts.iter() {
				// Eğer başlangıç noktasını arıyorsak
				if !found_start {
					if Some(account) == start_key.as_ref() {
						found_start = true;
					}
					continue;
				}
				
				// Batch limiti doldu mu?
				if updated_count >= batch_size {
					// Son işlenen hesabı kaydet
					LastProcessedAccount::<T>::put(account.clone());
					all_processed = false;
					break;
				}
				
				// Vatandaş mı kontrol et ve güncelle
				if T::CitizenshipSource::is_citizen(account) {
					let _ = Self::update_score_for_account(account);
					updated_count += 1;
				}
			}
			
			// Eğer tüm hesaplar işlendiyse, başa dön
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
		#[pallet::weight(T::WeightInfo::periodic_trust_score_update())]
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
			// Mock implementation - gerçekte KYC paletinden alınacak
			let total_users = 100u32; // Placeholder
			
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