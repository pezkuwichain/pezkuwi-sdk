#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod weights;
pub mod types; // Yeni types modülümüzü ekliyoruz
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
use crate::weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pallet_identity_kyc::types::KycStatus;
	use crate::types::{
		InviterProvider, OnKycApproved, ReferralScoreProvider, RawScore
	};
	use sp_std::prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_identity_kyc::Config + TypeInfo {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: weights::WeightInfo;
	}

	// --- Depolama Alanları (Storage) ---

	/// Bir kullanıcının referansıyla sisteme dahil olmayı bekleyen kişileri tutar.
	/// (Referred AccountId -> Referrer AccountId)
	#[pallet::storage]
	#[pallet::getter(fn pending_referrals)]
	pub type PendingReferrals<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId, OptionQuery>;

	/// Bir kullanıcının başarıyla tamamlanmış referans sayısını tutar.
	/// (Referrer AccountId -> Count)
	#[pallet::storage]
	#[pallet::getter(fn referral_count)]
	pub type ReferralCount<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	/// Bir kullanıcının kimi davet ettiğini ve işlemin detaylarını tutar.
	/// (Referred AccountId -> ReferralInfo)
	#[pallet::storage]
	#[pallet::getter(fn referrals)]
	pub type Referrals<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, ReferralInfo<T>, OptionQuery>;

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct ReferralInfo<T: Config> {
		pub referrer: T::AccountId,
		pub created_at: BlockNumberFor<T>,
	}

	// --- Olaylar (Events) ---
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Bir kullanıcı, başka bir kullanıcıyı davet ettiğinde.
		ReferralInitiated { referrer: T::AccountId, referred: T::AccountId },
		/// Davet edilen kullanıcının KYC süreci başarıyla tamamlandığında.
		ReferralConfirmed { referrer: T::AccountId, referred: T::AccountId, new_referrer_count: u32 },
	}

	// --- Hatalar (Errors) ---
	#[pallet::error]
	pub enum Error<T> {
		/// Bir kullanıcı kendini davet edemez.
		SelfReferral,
		/// Bu kullanıcı zaten başka birisi tarafından davet edilmiş.
		AlreadyReferred,
	}

	// --- Çağrılar (Callables) ---
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Başka bir kullanıcıyı sisteme davet etmek için bir referans kaydı başlatır.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::initiate_referral())]
		pub fn initiate_referral(
			origin: OriginFor<T>,
			referred: T::AccountId,
		) -> DispatchResult {
			let referrer = ensure_signed(origin)?;
			ensure!(referrer != referred, Error::<T>::SelfReferral);
			ensure!(!Referrals::<T>::contains_key(&referred), Error::<T>::AlreadyReferred);
			ensure!(!PendingReferrals::<T>::contains_key(&referred), Error::<T>::AlreadyReferred);

			PendingReferrals::<T>::insert(&referred, &referrer);
			Self::deposit_event(Event::ReferralInitiated { referrer, referred });
			Ok(())
		}
	}

	// --- Trait Implementasyonları ---

	impl<T: Config> OnKycApproved<T::AccountId> for Pallet<T> {
		fn on_kyc_approved(who: &T::AccountId) {
			// Güvenlik kontrolü: Referansı onaylamadan önce kullanıcının KYC durumunun
			// gerçekten "Approved" olduğunu zincir üzerinde teyit et.
			// Artık pallet_identity_kyc'nin depolama alanına doğrudan erişiyoruz.
			if pallet_identity_kyc::Pallet::<T>::get_kyc_status(who) == pallet_identity_kyc::types::KycLevel::Approved {
				if let Some(referrer) = PendingReferrals::<T>::take(who) {
					let new_count = ReferralCount::<T>::get(&referrer).saturating_add(1);
                    ReferralCount::<T>::insert(&referrer, new_count);

					let referral_info = ReferralInfo {
						referrer: referrer.clone(),
						created_at: frame_system::Pallet::<T>::block_number(),
					};
					Referrals::<T>::insert(who.clone(), referral_info);

					Self::deposit_event(Event::ReferralConfirmed {
						referrer,
						referred: who.clone(),
						new_referrer_count: new_count,
					});
				}
			}
		}
	}

	impl<T: Config> ReferralScoreProvider<T::AccountId> for Pallet<T> {
		type Score = RawScore;

		fn get_referral_score(who: &T::AccountId) -> RawScore {
			let referral_count = ReferralCount::<T>::get(who);

			let score = match referral_count {
				0 => 0,
				1..=5 => referral_count * 4,
				6..=20 => 20 + ((referral_count - 5) * 2),
				_ => 50, // Örnek olarak basitleştirildi, detaylı mantık eklenebilir.
			};

			score.into()
		}
	}

	impl<T: Config> InviterProvider<T::AccountId> for Pallet<T> {
		fn get_inviter(who: &T::AccountId) -> Option<T::AccountId> {
			Referrals::<T>::get(who).map(|info| info.referrer)
		}
	}
}
