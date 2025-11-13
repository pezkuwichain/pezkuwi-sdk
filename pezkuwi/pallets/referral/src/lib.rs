#![cfg_attr(not(feature = "std"), no_std)]

//! # Referral Pallet
//!
//! A pallet for managing user referrals and tracking network growth through invitation mechanics.
//!
//! ## Overview
//!
//! The Referral pallet implements a referral system that incentivizes user growth by tracking
//! and rewarding users who successfully invite others to complete KYC verification. Referral
//! counts contribute to trust scores and validator eligibility.
//!
//! ## Referral Workflow
//!
//! ### Initiation Phase
//!
//! 1. User A calls `initiate_referral(user_b_account)` to invite User B
//! 2. System creates a pending referral record linking B to A
//! 3. User B must not have been referred by anyone else
//! 4. Self-referral is prevented
//!
//! ### Confirmation Phase
//!
//! 1. User B completes identity registration and KYC application
//! 2. KYC authority approves User B's application
//! 3. `OnKycApproved` hook automatically fires
//! 4. System:
//!    - Converts pending referral to confirmed referral
//!    - Increments User A's referral count
//!    - Records block number of confirmation
//!    - Emits `ReferralConfirmed` event
//!
//! ## Referral Score System
//!
//! The referral count contributes to the trust score calculation in `pallet-trust`:
//! - Each successful referral increases the referrer's reputation
//! - Referral count is used by `ReferralScoreProvider` trait
//! - Higher referral counts improve validator pool eligibility
//! - Community validators require active referral participation
//!
//! ## Security Features
//!
//! - **One Referrer Per User**: Each user can only be referred once
//! - **No Self-Referral**: Users cannot refer themselves
//! - **KYC Verification Required**: Referrals only count after KYC approval
//! - **Immutable History**: Confirmed referrals cannot be changed
//! - **Block Number Recording**: Transparent audit trail
//!
//! ## Interface
//!
//! ### User Extrinsics
//!
//! - `initiate_referral(referred)` - Invite a new user to the ecosystem
//!
//! ### Storage
//!
//! - `PendingReferrals` - Invited users awaiting KYC approval (referred → referrer)
//! - `ReferralCount` - Number of successful referrals per user (referrer → count)
//! - `Referrals` - Confirmed referral records with metadata (referred → ReferralInfo)
//!
//! ### Trait Implementations
//!
//! - `OnKycApproved` - Hook called by `pallet-identity-kyc` upon KYC approval
//! - `ReferralScoreProvider` - Query interface for trust score calculation
//! - `InviterProvider` - Query who referred a specific user
//!
//! ## Integration Points
//!
//! ### With pallet-identity-kyc
//! - Listens for KYC approval events via `OnKycApproved` hook
//! - Automatically confirms pending referrals upon approval
//!
//! ### With pallet-trust
//! - Provides referral scores for composite trust calculation
//! - Contributes to overall reputation metrics
//!
//! ### With pallet-validator-pool
//! - Community validator category requires referral participation
//! - Referral count affects pool eligibility
//!
//! ## Runtime Integration Example
//!
//! ```ignore
//! impl pallet_referral::Config for Runtime {
//!     type RuntimeEvent = RuntimeEvent;
//!     type WeightInfo = pallet_referral::weights::SubstrateWeight<Runtime>;
//! }
//!
//! // Configure pallet-identity-kyc to notify referral pallet
//! impl pallet_identity_kyc::Config for Runtime {
//!     // ...
//!     type OnKycApproved = Referral; // Hook referral confirmation
//! }
//! ```

pub use pallet::*;
pub mod weights;
pub mod types; // Adding our new types module
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
	use pallet_identity_kyc::types::{KycStatus, OnKycApproved};
	use crate::types::{
		InviterProvider, ReferralScoreProvider, RawScore
	};
	use sp_std::prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_identity_kyc::Config + TypeInfo {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: weights::WeightInfo;
	}

	// --- Storage Items ---

	/// Holds users awaiting to join system via referral.
	/// (Referred AccountId -> Referrer AccountId)
	#[pallet::storage]
	#[pallet::getter(fn pending_referrals)]
	pub type PendingReferrals<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId, OptionQuery>;

	/// Holds successfully completed referral count per user.
	/// (Referrer AccountId -> Count)
	#[pallet::storage]
	#[pallet::getter(fn referral_count)]
	pub type ReferralCount<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	/// Holds who a user invited and transaction details.
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
		/// When a user invites another user.
		ReferralInitiated { referrer: T::AccountId, referred: T::AccountId },
		/// When invited user successfully completes KYC process.
		ReferralConfirmed { referrer: T::AccountId, referred: T::AccountId, new_referrer_count: u32 },
	}

	// --- Hatalar (Errors) ---
	#[pallet::error]
	pub enum Error<T> {
		/// A user cannot invite themselves.
		SelfReferral,
		/// This user has already been invited by someone else.
		AlreadyReferred,
	}

	// --- Extrinsics (Callables) ---
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
