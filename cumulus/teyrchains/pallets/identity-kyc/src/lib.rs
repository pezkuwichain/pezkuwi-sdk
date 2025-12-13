#![cfg_attr(not(feature = "std"), no_std)]

//! # Identity & KYC Pallet - TRUSTLESS MODEL
//!
//! A privacy-preserving, decentralized citizenship verification system.
//!
//! ## Overview
//!
//! This pallet implements a **TRUSTLESS** citizenship verification where:
//! - NO personal data is stored on-chain (only hash)
//! - NO central authority/bot approves applications
//! - Existing citizens vouch for new applicants (referral-based)
//! - Direct responsibility: Referrers are accountable for their referrals
//!
//! ## Security Design (Kurdish People Safety)
//!
//! This system is designed to protect vulnerable populations (like Kurdish people)
//! from hostile regimes that might try to identify applicants:
//! - Only H256 hash of identity stored on-chain
//! - Actual documents stored off-chain (IPFS/encrypted)
//! - No admin can see or leak personal data
//! - Referral chain creates accountability without central authority
//!
//! ## Citizenship Workflow
//!
//! ### 1. Application Phase
//! - User creates identity hash off-chain: `H256(name + email + documents)`
//! - User calls `apply_for_citizenship(identity_hash, referrer_account)`
//! - Referrer MUST be an existing citizen (KycLevel::Approved)
//! - Status changes to `PendingReferral`
//!
//! ### 2. Referrer Approval Phase
//! - Referrer reviews applicant (off-chain verification)
//! - Referrer calls `approve_referral(applicant)` to vouch for them
//! - Status changes to `ReferrerApproved`
//! - Referrer takes personal responsibility for this referral
//!
//! ### 3. Self-Confirmation Phase (Welati NFT Only)
//! - Applicant calls `confirm_citizenship()` to complete the process
//! - Status changes to `Approved`
//! - Citizen NFT (Welati) is minted via self-confirmation
//! - Referral hooks are triggered
//!
//! ## KYC Levels
//!
//! - **NotStarted** - No application submitted
//! - **PendingReferral** - Waiting for referrer approval
//! - **ReferrerApproved** - Referrer approved, ready for self-confirmation
//! - **Approved** - Full citizen with all rights
//! - **Revoked** - Citizenship revoked (governance decision)
//!
//! ## Privacy Features
//!
//! - **Hash-only storage**: No personal data on-chain
//! - **Off-chain documents**: IPFS or encrypted storage
//! - **No admin access**: Decentralized verification
//! - **Referral accountability**: Social trust, not central authority
//!
//! ## Direct Responsibility Model
//!
//! When a citizen is found to be malicious:
//! - ONLY their direct referrer is penalized
//! - Penalty: Trust score reduction + potential citizenship review
//! - Chain reactions are limited to direct relationships
//! - Good referrals from bad actors are NOT penalized
//!
//! ## Interface
//!
//! ### User Extrinsics
//!
//! - `apply_for_citizenship(identity_hash, referrer)` - Submit citizenship application
//! - `confirm_citizenship()` - Self-confirm after referrer approval (Welati only)
//! - `renounce_citizenship()` - Voluntarily give up citizenship
//!
//! ### Referrer Extrinsics
//!
//! - `approve_referral(applicant)` - Vouch for an applicant
//!
//! ### Governance Extrinsics (Root only)
//!
//! - `revoke_citizenship(who)` - Revoke citizenship (governance decision)
//!
//! ## Runtime Integration Example
//!
//! ```ignore
//! impl pallet_identity_kyc::Config for Runtime {
//!     type RuntimeEvent = RuntimeEvent;
//!     type Currency = Balances;
//!     type WeightInfo = pallet_identity_kyc::weights::SubstrateWeight<Runtime>;
//!     type OnKycApproved = Referral;
//!     type CitizenNftProvider = Tiki;
//!     type KycApplicationDeposit = ConstU128<1_000_000_000_000>; // Spam prevention
//!     type MaxStringLength = ConstU32<128>;
//!     type MaxCidLength = ConstU32<64>;
//! }
//! ```

pub use pallet::*;
pub mod types;
use types::*;
pub mod weights;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

extern crate alloc;
use frame_support::{pallet_prelude::*, traits::ReservableCurrency};
use frame_system::pallet_prelude::*;
use sp_core::H256;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Origin that can revoke citizenship (governance/root)
		type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		type WeightInfo: WeightInfo;

		/// Hook called when citizenship is approved - used by referral pallet
		type OnKycApproved: crate::types::OnKycApproved<Self::AccountId>;

		/// Hook called when citizenship is revoked - used by referral pallet for penalty
		type OnCitizenshipRevoked: crate::types::OnCitizenshipRevoked<Self::AccountId>;

		/// Provider for minting citizen NFTs - used by tiki pallet
		type CitizenNftProvider: crate::types::CitizenNftProvider<Self::AccountId>;

		/// Deposit required to apply (spam prevention, returned on approval)
		#[pallet::constant]
		type KycApplicationDeposit: Get<BalanceOf<Self>>;

		/// Max string length for legacy storage
		#[pallet::constant]
		type MaxStringLength: Get<u32>;

		/// Max CID length for legacy storage
		#[pallet::constant]
		type MaxCidLength: Get<u32>;
	}

	pub type BalanceOf<T> = <<T as Config>::Currency as frame_support::traits::Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	// ============= STORAGE =============

	/// Citizenship applications (applicant -> application)
	/// PRIVACY: Only hash stored, no personal data
	#[pallet::storage]
	#[pallet::getter(fn applications)]
	pub type Applications<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, CitizenshipApplication<T::AccountId>>;

	/// Current citizenship status per account
	#[pallet::storage]
	#[pallet::getter(fn kyc_status_of)]
	pub type KycStatuses<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, KycLevel, ValueQuery>;

	/// Identity hashes of approved citizens (for verification)
	/// Can be used to prove citizenship without revealing identity
	#[pallet::storage]
	#[pallet::getter(fn identity_hash_of)]
	pub type IdentityHashes<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, H256>;

	/// Referrer of approved citizens (for direct responsibility tracking)
	/// Kept permanently for penalty system even after application is removed
	#[pallet::storage]
	#[pallet::getter(fn citizen_referrer)]
	pub type CitizenReferrers<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId>;

	// ============= LEGACY STORAGE (for migration) =============

	/// Legacy: Identity info storage (deprecated, kept for migration)
	#[pallet::storage]
	pub type Identities<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, IdentityInfo<T::MaxStringLength>>;

	/// Legacy: Pending KYC applications (deprecated, kept for migration)
	#[pallet::storage]
	pub type PendingKycApplications<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		KycApplication<T::MaxStringLength, T::MaxCidLength>,
	>;

	// ============= GENESIS CONFIG =============

	/// Genesis configuration for bootstrapping initial citizens
	/// BOOTSTRAP: Solves chicken-egg problem - first citizens need to exist for others to join
	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		/// List of founding citizens (AccountId, IdentityHash)
		/// These accounts start with Approved status and can accept referrals immediately
		pub founding_citizens: alloc::vec::Vec<(T::AccountId, H256)>,
		#[serde(skip)]
		pub _phantom: core::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// Initialize founding citizens with Approved status
			for (account, identity_hash) in &self.founding_citizens {
				// Set status to Approved (citizen)
				KycStatuses::<T>::insert(account, KycLevel::Approved);
				// Store identity hash
				IdentityHashes::<T>::insert(account, *identity_hash);
			}
		}
	}

	// ============= EVENTS =============

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New citizenship application submitted
		CitizenshipApplied { applicant: T::AccountId, referrer: T::AccountId, identity_hash: H256 },
		/// Referrer approved the application
		ReferralApproved { referrer: T::AccountId, applicant: T::AccountId },
		/// Applicant self-confirmed their citizenship (Welati NFT minted)
		CitizenshipConfirmed { who: T::AccountId },
		/// Citizenship was revoked (by governance)
		CitizenshipRevoked { who: T::AccountId },
		/// User renounced their citizenship
		CitizenshipRenounced { who: T::AccountId },
		/// Application was cancelled by the applicant
		ApplicationCancelled { who: T::AccountId },
	}

	// ============= ERRORS =============

	#[pallet::error]
	pub enum Error<T> {
		/// Application already exists for this account
		ApplicationAlreadyExists,
		/// No application found for this account
		ApplicationNotFound,
		/// Referrer is not a citizen (must have Approved status)
		ReferrerNotCitizen,
		/// Cannot refer yourself
		SelfReferral,
		/// Cannot approve referral in current state (must be PendingReferral)
		CannotApproveInCurrentState,
		/// Cannot confirm in current state (must be ReferrerApproved)
		CannotConfirmInCurrentState,
		/// Cannot revoke in current state (must be Approved)
		CannotRevokeInCurrentState,
		/// User is not a citizen (cannot renounce)
		NotACitizen,
		/// Only the referrer can approve this application
		NotTheReferrer,
		/// Cannot cancel application in current state (must be PendingReferral)
		CannotCancelInCurrentState,
	}

	// ============= EXTRINSICS =============

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Apply for citizenship with identity hash and referrer
		///
		/// TRUSTLESS: No admin involved, referrer vouches for applicant
		/// PRIVACY: Only hash stored, actual identity is off-chain
		///
		/// # Arguments
		/// - `identity_hash`: H256 hash of identity documents (calculated off-chain)
		/// - `referrer`: Account of existing citizen who will vouch for you
		///
		/// # Workflow
		/// 1. Applicant submits hash + referrer
		/// 2. Deposit is reserved (spam prevention)
		/// 3. Status becomes PendingReferral
		/// 4. Referrer must call approve_referral
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::apply_for_citizenship())]
		pub fn apply_for_citizenship(
			origin: OriginFor<T>,
			identity_hash: H256,
			referrer: T::AccountId,
		) -> DispatchResult {
			let applicant = ensure_signed(origin)?;

			// Cannot refer yourself
			ensure!(applicant != referrer, Error::<T>::SelfReferral);

			// Must not have existing application
			ensure!(
				KycStatuses::<T>::get(&applicant) == KycLevel::NotStarted,
				Error::<T>::ApplicationAlreadyExists
			);

			// Referrer must be an approved citizen
			ensure!(
				KycStatuses::<T>::get(&referrer) == KycLevel::Approved,
				Error::<T>::ReferrerNotCitizen
			);

			// Reserve deposit (spam prevention, returned on approval)
			let deposit = T::KycApplicationDeposit::get();
			T::Currency::reserve(&applicant, deposit)?;

			// Store application (only hash, no personal data)
			let application = CitizenshipApplication { identity_hash, referrer: referrer.clone() };
			Applications::<T>::insert(&applicant, application);

			// Update status
			KycStatuses::<T>::insert(&applicant, KycLevel::PendingReferral);

			Self::deposit_event(Event::CitizenshipApplied { applicant, referrer, identity_hash });
			Ok(())
		}

		/// Referrer approves an applicant's citizenship application
		///
		/// TRUSTLESS: Referrer takes personal responsibility for this referral
		/// ACCOUNTABILITY: If applicant turns out malicious, referrer is penalized
		///
		/// # Arguments
		/// - `applicant`: Account of the person you're vouching for
		///
		/// # Requirements
		/// - Caller must be the referrer specified in the application
		/// - Application must be in PendingReferral state
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::approve_referral())]
		pub fn approve_referral(origin: OriginFor<T>, applicant: T::AccountId) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			// Must be in PendingReferral state
			ensure!(
				KycStatuses::<T>::get(&applicant) == KycLevel::PendingReferral,
				Error::<T>::CannotApproveInCurrentState
			);

			// Get application
			let application =
				Applications::<T>::get(&applicant).ok_or(Error::<T>::ApplicationNotFound)?;

			// Only the referrer can approve
			ensure!(application.referrer == caller, Error::<T>::NotTheReferrer);

			// Update status to ReferrerApproved
			KycStatuses::<T>::insert(&applicant, KycLevel::ReferrerApproved);

			Self::deposit_event(Event::ReferralApproved { referrer: caller, applicant });
			Ok(())
		}

		/// Self-confirm citizenship after referrer approval
		///
		/// TRUSTLESS: Applicant confirms themselves, no admin needed
		/// WELATI ONLY: This mints the Citizen NFT via self-confirmation
		///
		/// # Workflow
		/// 1. Deposit is returned
		/// 2. Identity hash is stored permanently
		/// 3. Status becomes Approved
		/// 4. Citizen NFT (Welati) is minted
		/// 5. Referral hooks are triggered
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::confirm_citizenship())]
		pub fn confirm_citizenship(origin: OriginFor<T>) -> DispatchResult {
			let applicant = ensure_signed(origin)?;

			// Must be in ReferrerApproved state
			ensure!(
				KycStatuses::<T>::get(&applicant) == KycLevel::ReferrerApproved,
				Error::<T>::CannotConfirmInCurrentState
			);

			// Get application
			let application =
				Applications::<T>::take(&applicant).ok_or(Error::<T>::ApplicationNotFound)?;

			// Return deposit
			let deposit = T::KycApplicationDeposit::get();
			T::Currency::unreserve(&applicant, deposit);

			// Store identity hash permanently (for proof of citizenship)
			IdentityHashes::<T>::insert(&applicant, application.identity_hash);

			// Store referrer permanently (for direct responsibility tracking)
			// This is needed even after Applications is removed for penalty system
			CitizenReferrers::<T>::insert(&applicant, application.referrer.clone());

			// Update status to Approved
			KycStatuses::<T>::insert(&applicant, KycLevel::Approved);

			// Mint citizen NFT with self-confirmation (Welati tiki)
			if let Err(e) = T::CitizenNftProvider::mint_citizen_nft_confirmed(&applicant) {
				log::warn!("Failed to mint citizen NFT for {:?}: {:?}", applicant, e);
				// Don't fail - user is still a citizen
			}

			// Trigger referral hooks (for referral pallet)
			// Pass referrer parameter to avoid data loss between pallets
			T::OnKycApproved::on_kyc_approved(&applicant, &application.referrer);

			Self::deposit_event(Event::CitizenshipConfirmed { who: applicant });
			Ok(())
		}

		/// Revoke citizenship (governance only)
		///
		/// Used for malicious actors identified by governance
		/// DIRECT RESPONSIBILITY: Triggers penalty for the referrer via referral pallet
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::revoke_citizenship())]
		pub fn revoke_citizenship(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			T::GovernanceOrigin::ensure_origin(origin)?;

			ensure!(
				KycStatuses::<T>::get(&who) == KycLevel::Approved,
				Error::<T>::CannotRevokeInCurrentState
			);

			// Update status
			KycStatuses::<T>::insert(&who, KycLevel::Revoked);

			// Burn citizen NFT
			if let Err(e) = T::CitizenNftProvider::burn_citizen_nft(&who) {
				log::warn!("Failed to burn citizen NFT for {:?}: {:?}", who, e);
			}

			// Trigger direct responsibility penalty for the referrer
			// This hook notifies the referral pallet to penalize the referrer
			T::OnCitizenshipRevoked::on_citizenship_revoked(&who);

			Self::deposit_event(Event::CitizenshipRevoked { who });
			Ok(())
		}

		/// Renounce citizenship (voluntary exit)
		///
		/// Users can freely leave the system
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::renounce_citizenship())]
		pub fn renounce_citizenship(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(KycStatuses::<T>::get(&who) == KycLevel::Approved, Error::<T>::NotACitizen);

			// Burn citizen NFT
			T::CitizenNftProvider::burn_citizen_nft(&who)?;

			// Reset status
			KycStatuses::<T>::insert(&who, KycLevel::NotStarted);

			// Remove identity hash
			IdentityHashes::<T>::remove(&who);

			Self::deposit_event(Event::CitizenshipRenounced { who });
			Ok(())
		}

		/// Cancel pending application and retrieve deposit
		///
		/// Useful if referrer is unresponsive or user made a mistake.
		/// SAFETY: Only works in PendingReferral state (not yet approved)
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::cancel_application())]
		pub fn cancel_application(origin: OriginFor<T>) -> DispatchResult {
			let applicant = ensure_signed(origin)?;

			// Must be in PendingReferral state (not yet approved by referrer)
			ensure!(
				KycStatuses::<T>::get(&applicant) == KycLevel::PendingReferral,
				Error::<T>::CannotCancelInCurrentState
			);

			// Remove application
			Applications::<T>::remove(&applicant);

			// Reset status
			KycStatuses::<T>::insert(&applicant, KycLevel::NotStarted);

			// Unreserve deposit
			let deposit = T::KycApplicationDeposit::get();
			T::Currency::unreserve(&applicant, deposit);

			Self::deposit_event(Event::ApplicationCancelled { who: applicant });
			Ok(())
		}
	}
}

// ============= TRAIT IMPLEMENTATIONS =============

pub use types::KycStatus;

impl<T: Config> types::KycStatus<T::AccountId> for Pallet<T> {
	fn get_kyc_status(who: &T::AccountId) -> KycLevel {
		KycStatuses::<T>::get(who)
	}
}

impl<T: Config> IdentityInfoProvider<T::AccountId, T::MaxStringLength> for Pallet<T> {
	fn get_identity_info(who: &T::AccountId) -> Option<IdentityInfo<T::MaxStringLength>> {
		// Legacy: Return from old storage if exists
		Identities::<T>::get(who)
	}
}

/// Helper methods for checking citizenship
impl<T: Config> Pallet<T> {
	/// Check if account is a citizen
	pub fn is_citizen(who: &T::AccountId) -> bool {
		KycStatuses::<T>::get(who) == KycLevel::Approved
	}

	/// Count total number of citizens
	pub fn citizen_count() -> u32 {
		KycStatuses::<T>::iter()
			.filter(|(_, status)| *status == KycLevel::Approved)
			.count() as u32
	}

	/// Get the referrer of a citizen or applicant
	/// Checks both pending applications and approved citizen records
	pub fn get_referrer(who: &T::AccountId) -> Option<T::AccountId> {
		// First check permanent storage (for approved citizens)
		CitizenReferrers::<T>::get(who)
			// Then check pending applications
			.or_else(|| Applications::<T>::get(who).map(|app| app.referrer))
	}

	/// Get identity hash of a citizen
	pub fn get_identity_hash(who: &T::AccountId) -> Option<H256> {
		IdentityHashes::<T>::get(who)
	}
}

/// Trait for trust pallet integration
pub trait CitizenshipStatusProvider<AccountId> {
	fn is_citizen(who: &AccountId) -> bool;
}

impl<T: Config> CitizenshipStatusProvider<T::AccountId> for Pallet<T> {
	fn is_citizen(who: &T::AccountId) -> bool {
		KycStatuses::<T>::get(who) == KycLevel::Approved
	}
}
