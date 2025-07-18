//! Benchmarking setup for pallet-tiki
use super::*;

#[allow(unused)]
use crate::Pallet as Tiki;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
// Gerekli trait'leri import ediyoruz
use frame_support::traits::{Currency, Get};
use sp_runtime::traits::StaticLookup;

// Gerekli trait kısıtlamalarını ana benchmarks bloğuna ekliyoruz.
#[benchmarks(
	where
		T::CollectionId: Copy + Default + PartialOrd,
)]
mod benchmarks {
	use super::*;

	// Bu yardımcı fonksiyon, runtime'da tanımlanan Tiki koleksiyonunu oluşturur.
	fn ensure_collection_exists<T: Config>()
	where
		T::CollectionId: Copy + Default + PartialOrd,
	{
		let collection_id = T::TikiCollectionId::get();
		// Koleksiyon sahibi olarak fonlanmış `whitelisted_caller`'ı kullanıyoruz.
		let caller: T::AccountId = whitelisted_caller();

		// `while` döngüsü, 'Step' trait'ine olan ihtiyacı ortadan kaldırır.
		while pallet_nfts::NextCollectionId::<T>::get().unwrap_or_default() <= collection_id {
			let _ = pallet_nfts::Pallet::<T>::force_create(
				RawOrigin::Root.into(),
				T::Lookup::unlookup(caller.clone()),
				pallet_nfts::CollectionConfig {
					settings: Default::default(),
					max_supply: None,
					mint_settings: Default::default(),
				},
			);
		}
	}

	#[benchmark]
	fn grant_tiki() {
		// NFT'yi alacak 'dest' hesabı olarak fonlanmış `whitelisted_caller`'ı kullanıyoruz.
		let dest: T::AccountId = whitelisted_caller();
		let tiki = crate::Tiki::Serok;
		ensure_collection_exists::<T>();

		#[extrinsic_call]
		_(RawOrigin::Root, T::Lookup::unlookup(dest.clone()), tiki.clone());

		assert_eq!(TikiHolder::<T>::get(&tiki), Some(dest));
	}

	#[benchmark]
	fn revoke_tiki() {
		// NFT'yi alacak 'dest' hesabı olarak fonlanmış `whitelisted_caller`'ı kullanıyoruz.
		let dest: T::AccountId = whitelisted_caller();
		let tiki = crate::Tiki::Wezir;
		ensure_collection_exists::<T>();

		// Revoke edebilmek için önce rolü verelim
		Tiki::<T>::grant_tiki(
			RawOrigin::Root.into(),
			T::Lookup::unlookup(dest.clone()),
			tiki.clone(),
		)
		.unwrap();
		assert_eq!(TikiHolder::<T>::get(&tiki), Some(dest.clone()));

		#[extrinsic_call]
		_(RawOrigin::Root, T::Lookup::unlookup(dest.clone()), tiki.clone());

		assert_eq!(TikiHolder::<T>::get(&tiki), None);
	}

	impl_benchmark_test_suite!(Tiki, crate::mock::new_test_ext(), crate::mock::Test);
}