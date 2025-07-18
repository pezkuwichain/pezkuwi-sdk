//! Benchmarking setup for pallet-referral

use super::*;
use crate::Pallet as Referral;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_keyring::Sr25519Keyring;
use sp_runtime::traits::StaticLookup;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn initiate_referral() {
		// DÜZENLEME:
		// `referrer` (davet eden) olarak, geliştirme zincirinde her zaman fonlu
		// olduğunu bildiğimiz Alice'in hesabını kullanıyoruz.
		let referrer: T::AccountId = Sr25519Keyring::Alice.to_account_id();
		
		// `referred` (davet edilen) olarak, benchmark'lar için oluşturulmuş
		// standart bir test hesabı kullanıyoruz.
		let referred: T::AccountId = account("referred_user", 0, 0);
		
		// `referred` hesabının daha önce davet edilmediğinden emin olalım.
		PendingReferrals::<T>::remove(&referred);
		Referrals::<T>::remove(&referred);

		// EYLEM:
		// Bu bloğun içindeki extrinsic çağrısının ne kadar sürdüğünü ölçüyoruz.
		#[extrinsic_call]
		_(RawOrigin::Signed(referrer.clone()), T::Lookup::unlookup(referred.clone()));

		// DOĞRULAMA:
		// İşlem sonrasında depolamanın doğru duruma geldiğini teyit ediyoruz.
		assert_eq!(PendingReferrals::<T>::get(&referred), Some(referrer));
	}

	impl_benchmark_test_suite!(Referral, crate::mock::new_test_ext(), crate::mock::Test);
}