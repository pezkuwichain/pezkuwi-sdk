#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

// --- ARAYÜZLER (TRAITS) ---

/// Ham puan değerlerini temsil eden genel tip.
pub type RawScore = u32;

/// Staking puanı verilerini sağlayacak arayüz.
pub trait StakingScoreProvider<AccountId, BlockNumber> {
	fn get_staking_score(who: &AccountId) -> (RawScore, BlockNumber);
}

/// Referans puanı verilerini sağlayacak arayüz.
pub trait ReferralScoreProvider<AccountId> {
	fn get_referral_score(who: &AccountId) -> RawScore;
}

/// Eğitim (Perwerde) puanı verilerini sağlayacak arayüz.
pub trait PerwerdeScoreProvider<AccountId> {
	fn get_perwerde_score(who: &AccountId) -> RawScore;
}

/// `pallet-tiki`'den gelen birleşik (temel + bonus) puanı sağlayacak arayüz.
pub trait TikiScoreProvider<AccountId> {
	fn get_tiki_score(who: &AccountId) -> RawScore;
}

/// Kullanıcının vatandaşlık (Hemwelatî) durumunu sağlayacak arayüz.
pub trait CitizenshipStatusProvider<AccountId> {
	fn is_citizen(who: &AccountId) -> bool;
}

/// Puanı etkileyen bir değişiklik olduğunda `pallet-trust`'ı bilgilendirmek için arayüz.
pub trait TrustScoreUpdater<AccountId> {
	fn on_score_component_changed(who: &AccountId);
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

		/// Puanları temsil eden tip. `u128` seçimi, ara hesaplamalarda taşmayı (overflow) önler.
		type Score: Member + Parameter + MaxEncodedLen + Copy + Default + PartialOrd + Saturating + Zero + From<RawScore> + Into<u128>;

		/// Ondalık sayı matematiği için kullanılacak taban çarpan. Runtime'da 1000 olarak ayarlanmalıdır.
		#[pallet::constant]
		type ScoreMultiplierBase: Get<u128>;

		// --- Veri Sağlayıcı Paletlerin Bağlantıları ---
		type StakingScoreSource: StakingScoreProvider<Self::AccountId, BlockNumberFor<Self>>;
		type ReferralScoreSource: ReferralScoreProvider<Self::AccountId>;
		type PerwerdeScoreSource: PerwerdeScoreProvider<Self::AccountId>;
		type TikiScoreSource: TikiScoreProvider<Self::AccountId>;
		type CitizenshipSource: CitizenshipStatusProvider<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn trust_score_of)]
	/// Her bir hesabın güncel Trust Puanını saklar.
	pub type TrustScores<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Score, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_active_trust_score)]
	/// Zincirdeki tüm aktif (vatandaş) kullanıcıların toplam Trust Puanını tutar.
	pub type TotalActiveTrustScore<T: Config> = StorageValue<_, T::Score, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Bir kullanıcının Trust Puanı başarıyla güncellendi.
		TrustScoreUpdated { who: T::AccountId, old_score: T::Score, new_score: T::Score },
		/// Zincirdeki toplam aktif Trust Puanı güncellendi.
		TotalTrustScoreUpdated { new_total: T::Score },
	}

	#[pallet::error]
	pub enum Error<T> {
		CalculationOverflow,
		NotACitizen,
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
	}

	impl<T: Config> Pallet<T> {
		/// Yeni formülü uygulayarak bir kullanıcının Trust Puanını hesaplar.
		pub fn calculate_trust_score(who: &T::AccountId) -> Result<T::Score, Error<T>> {
			ensure!(T::CitizenshipSource::is_citizen(who), Error::<T>::NotACitizen);

			let (staking_score, _) = T::StakingScoreSource::get_staking_score(who);
			if staking_score.is_zero() {
				return Ok(T::Score::zero());
			}

			let staking_u128: u128 = T::Score::from(staking_score).into();
			let referral_u128: u128 = T::Score::from(T::ReferralScoreSource::get_referral_score(who)).into();
			let perwerde_u128: u128 = T::Score::from(T::PerwerdeScoreSource::get_perwerde_score(who)).into();
			let tiki_u128: u128 = T::Score::from(T::TikiScoreSource::get_tiki_score(who)).into();
			
			// --- Yeni Formülün Güvenli Matematik ile Uygulanması ---
			// Formül: Staking * ( (Staking*0.1) + (Referral*0.3) + (Perwerde*0.3) + (Tiki*0.3) )
			
			let base = T::ScoreMultiplierBase::get(); // 1000

			// Ağırlıklı iç toplamı yeni katsayılarla hesapla (100, 300, 300, 300)
			let weighted_sum = staking_u128.saturating_mul(100)
				.saturating_add(referral_u128.saturating_mul(300))
				.saturating_add(perwerde_u128.saturating_mul(300))
				.saturating_add(tiki_u128.saturating_mul(300));

			// Önce ana çarpanla çarp, sonra tabana böl.
			let final_score_u128 = staking_u128
				.saturating_mul(weighted_sum)
				.checked_div(base)
				.ok_or(Error::<T>::CalculationOverflow)?;

			// Sonucu paletin `Score` tipine güvenle dönüştür.
			let final_score = T::Score::try_from(final_score_u128).map_err(|_| Error::<T>::CalculationOverflow)?;
			
			Ok(final_score)
		}

		/// Bir kullanıcının puanını hesaplar, depolar ve toplam puanı günceller.
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
	}

	impl<T: Config> TrustScoreUpdater<T::AccountId> for Pallet<T> {
		fn on_score_component_changed(who: &T::AccountId) {
			if let Err(e) = Self::update_score_for_account(who) {
				log::error!("Failed to update trust score for {:?}: {:?}", who, e);
			}
		}
	}
}