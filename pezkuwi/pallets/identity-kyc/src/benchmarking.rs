//! Benchmarking setup for pallet-identity-kyc

use super::*;
#[allow(unused_imports)]
use crate::Pallet as IdentityKyc;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::prelude::*;
use crate::types::*;

benchmarks! {
	set_identity {
		let caller: T::AccountId = whitelisted_caller();
		let name: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		let email: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
	}: _(RawOrigin::Signed(caller.clone()), name, email)
	verify {
		assert!(Identities::<T>::contains_key(&caller));
	}

	apply_for_kyc {
		let caller: T::AccountId = whitelisted_caller();
		// `apply_for_kyc` çağrılmadan önce kullanıcının bir kimliği olmalı.
		let name: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		let email: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::set_identity(RawOrigin::Signed(caller.clone()).into(), name, email).unwrap();

		let cids: BoundedVec<BoundedVec<u8, T::MaxCidLength>, T::MaxCidLength> = vec![vec![0u8; T::MaxCidLength::get() as usize].try_into().unwrap()].try_into().unwrap();
		let notes: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();

	}: _(RawOrigin::Signed(caller.clone()), cids, notes)
	verify {
		assert_eq!(KycStatuses::<T>::get(&caller), KycLevel::Pending);
	}

	approve_kyc {
		let user: T::AccountId = whitelisted_caller();
		// `approve_kyc` çağrılmadan önce kullanıcının bekleyen bir başvurusu olmalı.
		// 1. Kimlik oluştur
		let name: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		let email: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::set_identity(RawOrigin::Signed(user.clone()).into(), name, email).unwrap();
		// 2. Başvuru yap
		let cids: BoundedVec<BoundedVec<u8, T::MaxCidLength>, T::MaxCidLength> = vec![vec![0u8; T::MaxCidLength::get() as usize].try_into().unwrap()].try_into().unwrap();
		let notes: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::apply_for_kyc(RawOrigin::Signed(user.clone()).into(), cids, notes).unwrap();

	}: _(RawOrigin::Root, user.clone())
	verify {
		assert_eq!(KycStatuses::<T>::get(&user), KycLevel::Approved);
	}

	revoke_kyc {
		let user: T::AccountId = whitelisted_caller();
		// `revoke_kyc` çağrılmadan önce kullanıcının KYC'si onaylanmış olmalı.
		// 1. Kimlik oluştur
		let name: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		let email: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::set_identity(RawOrigin::Signed(user.clone()).into(), name, email).unwrap();
		// 2. Başvuru yap
		let cids: BoundedVec<BoundedVec<u8, T::MaxCidLength>, T::MaxCidLength> = vec![vec![0u8; T::MaxCidLength::get() as usize].try_into().unwrap()].try_into().unwrap();
		let notes: BoundedVec<u8, T::MaxStringLength> = vec![0u8; T::MaxStringLength::get() as usize].try_into().unwrap();
		IdentityKyc::<T>::apply_for_kyc(RawOrigin::Signed(user.clone()).into(), cids, notes).unwrap();
		// 3. Onayla
		IdentityKyc::<T>::approve_kyc(RawOrigin::Root.into(), user.clone()).unwrap();

	}: _(RawOrigin::Root, user.clone())
	verify {
		assert_eq!(KycStatuses::<T>::get(&user), KycLevel::Revoked);
	}

	impl_benchmark_test_suite!(IdentityKyc, crate::mock::new_test_ext(), crate::mock::Test);
}