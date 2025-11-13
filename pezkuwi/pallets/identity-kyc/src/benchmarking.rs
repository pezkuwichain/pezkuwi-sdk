//! Benchmarking setup for pallet-identity-kyc

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as IdentityKyc;
use crate::types::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_std::prelude::*;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_identity() {
		let caller: T::AccountId = whitelisted_caller();
		let name: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		let email: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();

		#[extrinsic_call]
		set_identity(RawOrigin::Signed(caller.clone()), name, email);

		assert!(Identities::<T>::contains_key(&caller));
	}

	#[benchmark]
	fn apply_for_kyc() {
		let caller: T::AccountId = whitelisted_caller();
		// Before calling `apply_for_kyc`, user must have an identity
		let name: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		let email: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::set_identity(RawOrigin::Signed(caller.clone()).into(), name, email).unwrap();

		let cids: BoundedVec<BoundedVec<u8, T::MaxCidLength>, T::MaxCidLength> = vec![vec![0u8; T::MaxCidLength::get() as usize].try_into().unwrap()].try_into().unwrap();
		let notes: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();

		#[extrinsic_call]
		apply_for_kyc(RawOrigin::Signed(caller.clone()), cids, notes);

		assert_eq!(KycStatuses::<T>::get(&caller), KycLevel::Pending);
	}

	#[benchmark]
	fn approve_kyc() {
		let user: T::AccountId = whitelisted_caller();
		// Before calling `approve_kyc`, user must have a pending application
		// 1. Create identity
		let name: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		let email: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::set_identity(RawOrigin::Signed(user.clone()).into(), name, email).unwrap();
		// 2. Apply for KYC
		let cids: BoundedVec<BoundedVec<u8, T::MaxCidLength>, T::MaxCidLength> = vec![vec![0u8; T::MaxCidLength::get() as usize].try_into().unwrap()].try_into().unwrap();
		let notes: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::apply_for_kyc(RawOrigin::Signed(user.clone()).into(), cids, notes).unwrap();

		#[extrinsic_call]
		approve_kyc(RawOrigin::Root, user.clone());

		assert_eq!(KycStatuses::<T>::get(&user), KycLevel::Approved);
	}

	#[benchmark]
	fn revoke_kyc() {
		let user: T::AccountId = whitelisted_caller();
		// Before calling `revoke_kyc`, user's KYC must be approved
		// 1. Create identity
		let name: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		let email: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::set_identity(RawOrigin::Signed(user.clone()).into(), name, email).unwrap();
		// 2. Apply for KYC
		let cids: BoundedVec<BoundedVec<u8, T::MaxCidLength>, T::MaxCidLength> = vec![vec![0u8; T::MaxCidLength::get() as usize].try_into().unwrap()].try_into().unwrap();
		let notes: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::apply_for_kyc(RawOrigin::Signed(user.clone()).into(), cids, notes).unwrap();
		// 3. Approve
		IdentityKyc::<T>::approve_kyc(RawOrigin::Root.into(), user.clone()).unwrap();

		#[extrinsic_call]
		revoke_kyc(RawOrigin::Root, user.clone());

		assert_eq!(KycStatuses::<T>::get(&user), KycLevel::Revoked);
	}

	impl_benchmark_test_suite!(IdentityKyc, crate::mock::new_test_ext(), crate::mock::Test);
}
