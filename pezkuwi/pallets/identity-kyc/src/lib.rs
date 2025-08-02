#![cfg_attr(not(feature = "std"), no_std)]

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
			T::KycApprovalOrigin::ensure_origin(origin)?; // Sadece yetki kontrolü yap, sonucu değişkene atama
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
			Self::deposit_event(Event::KycApproved { who }); // 'reviewer' olmadan olayı yayınla
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::revoke_kyc())]
		pub fn revoke_kyc(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			T::KycApprovalOrigin::ensure_origin(origin)?; // Sadece yetki kontrolü yap, sonucu değişkene atama
			ensure!(
				KycStatuses::<T>::get(&who) == KycLevel::Approved,
				Error::<T>::CannotRevokeKycInCurrentState
			);
			KycStatuses::<T>::insert(&who, KycLevel::Revoked);
			Self::deposit_event(Event::KycRevoked { who }); // 'reviewer' olmadan olayı yayınla
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