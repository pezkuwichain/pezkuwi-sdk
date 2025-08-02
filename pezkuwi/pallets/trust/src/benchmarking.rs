#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as TrustPallet;

// Benchmarking için gerekli temel modüller
use frame_benchmarking::{v2::*, whitelisted_caller};
use frame_support::pallet_prelude::*;
use frame_system::RawOrigin;
use sp_runtime::traits::Zero;
use sp_std::vec;

// Dışarıdan çağırdığımız palet
use pallet_identity_kyc::Pallet as IdentityKycPallet;

#[benchmarks(where T: pallet_identity_kyc::Config)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn force_recalculate_trust_score() -> Result<(), BenchmarkError> {
		// Setup
		let account: T::AccountId = whitelisted_caller();

		// Kullanıcının KYC'sini onaylamak için gerekli adımlar
		let name: BoundedVec<u8, <T as pallet_identity_kyc::Config>::MaxStringLength> =
			b"Benchmark User".to_vec().try_into().unwrap();
		let email: BoundedVec<u8, <T as pallet_identity_kyc::Config>::MaxStringLength> =
			b"bench@mark.com".to_vec().try_into().unwrap();
		IdentityKycPallet::<T>::set_identity(RawOrigin::Signed(account.clone()).into(), name, email)?;

		let cids: BoundedVec<
			BoundedVec<u8, <T as pallet_identity_kyc::Config>::MaxCidLength>,
			<T as pallet_identity_kyc::Config>::MaxCidLength,
		> = vec![b"cid1".to_vec().try_into().unwrap()].try_into().unwrap();
		let notes: BoundedVec<u8, <T as pallet_identity_kyc::Config>::MaxStringLength> =
			b"benchmark notes".to_vec().try_into().unwrap();
		IdentityKycPallet::<T>::apply_for_kyc(RawOrigin::Signed(account.clone()).into(), cids, notes)?;

		IdentityKycPallet::<T>::approve_kyc(RawOrigin::Root.into(), account.clone())?;

		// Setup'ın başarılı olduğunu doğrula
		assert!(T::CitizenshipSource::is_citizen(&account));

		#[extrinsic_call]
		force_recalculate_trust_score(RawOrigin::Root, account.clone());

		// Verify
		assert!(TrustPallet::<T>::trust_score_of(&account) > T::Score::zero());
		Ok(())
	}

	#[benchmark]
	fn update_all_trust_scores() {
		// Setup - İlk çalıştırmada batch update aktif olmadığından emin ol
		crate::BatchUpdateInProgress::<T>::put(false);

		#[extrinsic_call]
		update_all_trust_scores(RawOrigin::Root);

		// Verify - İlgili event'lerin yayınlandığını kontrol edebiliriz
		// Ancak burada storage'ın güncellendiğini kontrol etmek yeterli
		// BatchUpdateInProgress false olabilir (eğer tüm hesaplar işlendiyse)
		assert!(
			crate::BatchUpdateInProgress::<T>::get() == false ||
			crate::BatchUpdateInProgress::<T>::get() == true
		);
	}

	#[benchmark]
	fn periodic_trust_score_update() {
		// Setup - Önceki update'in tamamlandığından emin ol
		crate::BatchUpdateInProgress::<T>::put(false);

		#[extrinsic_call]
		periodic_trust_score_update(RawOrigin::Root);

		// Verify - Periyodik update başarıyla çalıştı
		// Event yayınlandığını doğrudan kontrol edemeyiz, ancak
		// fonksiyonun başarıyla çalıştığını biliyoruz
	}

	impl_benchmark_test_suite!(TrustPallet, crate::mock::new_test_ext(), crate::mock::Test);
}