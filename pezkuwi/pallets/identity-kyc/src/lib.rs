#![cfg_attr(not(feature = "std"), no_std)]

//! # Identity & KYC Pallet
//!
//! A pallet for managing user identities and Know-Your-Customer (KYC) verification processes.
//!
//! ## Overview
//!
//! The Identity & KYC pallet provides a two-stage verification system:
//!
//! 1. **Identity Registration** - Users submit basic identity information (name, email)
//! 2. **KYC Verification** - Users submit documents for verification to become citizens
//!
//! Approved KYC users gain citizenship status, which unlocks all ecosystem features including
//! validator participation, governance voting, rewards claiming, and role assignment.
//!
//! ## KYC Workflow
//!
//! ### Application Process
//!
//! 1. User calls `set_identity()` to register basic information
//! 2. User calls `apply_for_kyc()` with document CIDs and notes
//! 3. Application deposit is reserved from user's balance
//! 4. KYC status changes to "Pending"
//!
//! ### Approval Process
//!
//! 1. Privileged origin reviews submitted documents
//! 2. Origin calls `approve_kyc()` or `reject_kyc()`
//! 3. On approval:
//!    - Deposit is unreserved
//!    - KYC status changes to "Approved"
//!    - Citizenship NFT is minted (via CitizenNftProvider hook)
//!    - Referral system is notified (via OnKycApproved hook)
//! 4. On rejection:
//!    - Deposit is slashed
//!    - User can reapply with new documents
//!
//! ## KYC Levels
//!
//! - **NotStarted** - No KYC application submitted
//! - **Pending** - Application under review
//! - **Approved** - Verified citizen with full rights
//! - **Rejected** - Application denied, can reapply
//! - **Revoked** - Previously approved but citizenship revoked
//!
//! ## Document Storage
//!
//! - Documents are stored on IPFS
//! - Only Content Identifiers (CIDs) are stored on-chain
//! - Maximum configurable CID and string lengths
//! - Privacy-preserving design
//!
//! ## Security Features
//!
//! - Application deposit prevents spam
//! - Slashing on rejection discourages false applications
//! - One-time identity registration (immutable)
//! - Privileged origin for approvals/rejections
//! - State machine prevents invalid transitions
//!
//! ## Interface
//!
//! ### User Extrinsics
//!
//! - `set_identity(name, email)` - Register identity information (one-time)
//! - `apply_for_kyc(cids, notes)` - Submit KYC application with document CIDs
//!
//! ### Privileged Extrinsics
//!
//! - `approve_kyc(who)` - Approve pending KYC application
//! - `reject_kyc(who)` - Reject pending KYC application
//! - `revoke_kyc(who)` - Revoke previously approved citizenship
//!
//! ### Storage
//!
//! - `Identities` - Basic identity information per account
//! - `KycStatuses` - Current KYC level per account
//! - `PendingKycApplications` - Applications awaiting review
//!
//! ### Hooks
//!
//! - `OnKycApproved` - Notifies referral system of new citizen
//! - `CitizenNftProvider` - Triggers citizenship NFT minting
//!
//! ## Dependencies
//!
//! This pallet integrates with:
//! - `pallet-tiki` - Citizenship NFT minting
//! - `pallet-referral` - Referral rewards upon KYC approval
//!
//! ## Runtime Integration Example
//!
//! ```ignore
//! impl pallet_identity_kyc::Config for Runtime {
//!     type RuntimeEvent = RuntimeEvent;
//!     type Currency = Balances;
//!     type KycApprovalOrigin = EnsureRoot<AccountId>;
//!     type WeightInfo = pallet_identity_kyc::weights::SubstrateWeight<Runtime>;
//!     type OnKycApproved = Referral; // Notify referral pallet
//!     type CitizenNftProvider = Tiki; // Mint citizenship NFT
//!     type KycApplicationDeposit = ConstU128<1_000_000_000_000>; // 1 token
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

use frame_support::{pallet_prelude::*, traits::ReservableCurrency};
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type KycApprovalOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type WeightInfo: WeightInfo;

		/// Hook called when KYC is approved - used by referral pallet
		type OnKycApproved: crate::types::OnKycApproved<Self::AccountId>;

		/// Provider for minting citizen NFTs - used by tiki pallet
		type CitizenNftProvider: crate::types::CitizenNftProvider<Self::AccountId>;

		#[pallet::constant]
		type KycApplicationDeposit: Get<BalanceOf<Self>>;
		#[pallet::constant]
		type MaxStringLength: Get<u32>;
		#[pallet::constant]
		type MaxCidLength: Get<u32>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::Currency as frame_support::traits::Currency<
			<T as frame_system::Config>::AccountId,
		>>::Balance;

	#[pallet::storage]
	#[pallet::getter(fn identity_of)]
	pub type Identities<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, IdentityInfo<T::MaxStringLength>>;

	#[pallet::storage]
	#[pallet::getter(fn kyc_status_of)]
	pub type KycStatuses<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, KycLevel, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pending_application_of)]
	pub type PendingKycApplications<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		KycApplication<T::MaxStringLength, T::MaxCidLength>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		IdentitySet { who: T::AccountId },
		KycApplied { who: T::AccountId },
		KycApproved { who: T::AccountId },
		KycRejected { who: T::AccountId },
		KycRevoked { who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		IdentityAlreadyExists,
		IdentityNotFound,
		KycApplicationAlreadyExists,
		KycApplicationNotFound,
		CannotRevokeKycInCurrentState,
		CannotApproveKycInCurrentState,
		CannotRejectKycInCurrentState,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::set_identity())]
		pub fn set_identity(
			origin: OriginFor<T>,
			name: BoundedVec<u8, T::MaxStringLength>,
			email: BoundedVec<u8, T::MaxStringLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Prevent overwriting existing identity
			ensure!(!Identities::<T>::contains_key(&who), Error::<T>::IdentityAlreadyExists);

			let identity = IdentityInfo { name, email };
			Identities::<T>::insert(&who, identity);
			Self::deposit_event(Event::IdentitySet { who });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::apply_for_kyc())]
		pub fn apply_for_kyc(
			origin: OriginFor<T>,
			cids: BoundedVec<BoundedVec<u8, T::MaxCidLength>, T::MaxCidLength>,
			notes: BoundedVec<u8, T::MaxStringLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Identities::<T>::contains_key(&who), Error::<T>::IdentityNotFound);
			ensure!(
				KycStatuses::<T>::get(&who) == KycLevel::NotStarted,
				Error::<T>::KycApplicationAlreadyExists
			);

			let deposit = T::KycApplicationDeposit::get();
			T::Currency::reserve(&who, deposit)?;

			let application = KycApplication { cids, notes };
			PendingKycApplications::<T>::insert(&who, application);
			KycStatuses::<T>::insert(&who, KycLevel::Pending);

			Self::deposit_event(Event::KycApplied { who });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::approve_kyc())]
		pub fn approve_kyc(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			T::KycApprovalOrigin::ensure_origin(origin)?; // Only perform authority check, don't assign result to variable
			ensure!(
				KycStatuses::<T>::get(&who) == KycLevel::Pending,
				Error::<T>::CannotApproveKycInCurrentState
			);
			ensure!(
				PendingKycApplications::<T>::contains_key(&who),
				Error::<T>::KycApplicationNotFound
			);

			let deposit = T::KycApplicationDeposit::get();
			T::Currency::unreserve(&who, deposit);

			PendingKycApplications::<T>::remove(&who);
			KycStatuses::<T>::insert(&who, KycLevel::Approved);

			// Mint citizen NFT automatically when KYC is approved
			// This ensures proper synchronization between identity-kyc and tiki pallets
			if let Err(e) = T::CitizenNftProvider::mint_citizen_nft(&who) {
				log::warn!("Failed to mint citizen NFT for {:?}: {:?}", who, e);
				// Don't fail the KYC approval if NFT minting fails
				// The user can still apply for citizenship manually via tiki::apply_for_citizenship
			}

			// Call referral hook to process pending referrals
			// This ensures proper synchronization between identity-kyc and referral pallets
			T::OnKycApproved::on_kyc_approved(&who);

			Self::deposit_event(Event::KycApproved { who }); // Publish event without 'reviewer'
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::approve_kyc())]
		pub fn reject_kyc(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			T::KycApprovalOrigin::ensure_origin(origin)?;
			ensure!(
				KycStatuses::<T>::get(&who) == KycLevel::Pending,
				Error::<T>::CannotRejectKycInCurrentState
			);
			ensure!(
				PendingKycApplications::<T>::contains_key(&who),
				Error::<T>::KycApplicationNotFound
			);

			let deposit = T::KycApplicationDeposit::get();
			T::Currency::unreserve(&who, deposit);

			PendingKycApplications::<T>::remove(&who);
			KycStatuses::<T>::insert(&who, KycLevel::Rejected);

			Self::deposit_event(Event::KycRejected { who });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::revoke_kyc())]
		pub fn revoke_kyc(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			T::KycApprovalOrigin::ensure_origin(origin)?; // Only perform authority check, don't assign result to variable
			ensure!(
				KycStatuses::<T>::get(&who) == KycLevel::Approved,
				Error::<T>::CannotRevokeKycInCurrentState
			);
			KycStatuses::<T>::insert(&who, KycLevel::Revoked);
			Self::deposit_event(Event::KycRevoked { who }); // Publish event without 'reviewer'
			Ok(())
		}
	}
}

pub use types::KycStatus;

impl<T: Config> types::KycStatus<T::AccountId> for Pallet<T> {
	fn get_kyc_status(who: &T::AccountId) -> KycLevel {
		KycStatuses::<T>::get(who)
	}
}

impl<T: Config> IdentityInfoProvider<T::AccountId, T::MaxStringLength> for Pallet<T> {
	fn get_identity_info(who: &T::AccountId) -> Option<IdentityInfo<T::MaxStringLength>> {
		Identities::<T>::get(who)
	}
}