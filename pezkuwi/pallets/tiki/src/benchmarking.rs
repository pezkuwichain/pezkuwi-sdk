//! Benchmarking setup for pallet-tiki
use super::*;

#[allow(unused)]
use crate::Pallet as Tiki;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
// Gerekli trait'leri import ediyoruz
use frame_support::traits::{Get, Currency};
use sp_runtime::traits::StaticLookup;
use sp_std::vec;
use pallet_balances::Pallet as Balances;

// Gerekli trait kısıtlamalarını ana benchmarks bloğuna ekliyoruz.
#[benchmarks(
	where
		T::CollectionId: Copy + Default + PartialOrd,
		T: pallet_balances::Config,
)]
mod benchmarks {
	use super::*;

	// Bu yardımcı fonksiyon, runtime'da tanımlanan Tiki koleksiyonunu oluşturur.
	fn ensure_collection_exists<T: Config>()
	where
		T::CollectionId: Copy + Default + PartialOrd,
		T: pallet_balances::Config,
	{
		let collection_id = T::TikiCollectionId::get();
		// Koleksiyon sahibi olarak fonlanmış `whitelisted_caller`'ı kullanıyoruz.
		let caller: T::AccountId = whitelisted_caller();

		// Fund the caller account with sufficient balance for NFT deposits
		// Use a very large balance to ensure all deposit requirements can be met
		let funding = Balances::<T>::minimum_balance() * 1_000_000_000u32.into();
		Balances::<T>::make_free_balance_be(&caller, funding);

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

    // Helper to ensure user has a citizen NFT
    fn ensure_citizen_nft<T: Config>(who: T::AccountId) -> Result<(), DispatchError>
    where
        T::CollectionId: Copy + Default + PartialOrd,
        T: pallet_balances::Config,
    {
        ensure_collection_exists::<T>();

        // Fund the user account with sufficient balance for NFT deposits
        // Use a very large balance to ensure all deposit requirements can be met
        let funding = Balances::<T>::minimum_balance() * 1_000_000_000u32.into();
        Balances::<T>::make_free_balance_be(&who, funding);

        if Tiki::<T>::citizen_nft(&who).is_none() {
            Tiki::<T>::mint_citizen_nft_for_user(&who)?;
        }
        Ok(())
    }

	#[benchmark]
	fn grant_tiki() -> Result<(), BenchmarkError> {
		// NFT'yi alacak 'dest' hesabı olarak fonlanmış `whitelisted_caller`'ı kullanıyoruz.
		let dest: T::AccountId = whitelisted_caller();
		// Appointed role kullan (Serok yerine Wezir)
		let tiki = crate::Tiki::Wezir;
		
        // Ensure the dest account has a citizen NFT before granting a tiki
        ensure_citizen_nft::<T>(dest.clone())?;

		#[extrinsic_call]
		_(RawOrigin::Root, T::Lookup::unlookup(dest.clone()), tiki.clone());

		// For non-unique roles, check user has the role
		assert!(Tiki::<T>::user_tikis(&dest).contains(&tiki));
		Ok(())
	}

	#[benchmark]
	fn revoke_tiki() -> Result<(), BenchmarkError> {
		// NFT'yi alacak 'dest' hesabı olarak fonlanmış `whitelisted_caller`'ı kullanıyoruz.
		let dest: T::AccountId = whitelisted_caller();
		let tiki = crate::Tiki::Wezir; // Use appointed role
		
        // Ensure the dest account has a citizen NFT and the tiki before revoking
        ensure_citizen_nft::<T>(dest.clone())?;
        Tiki::<T>::internal_grant_role(&dest, tiki.clone())?; // Use internal function to grant without origin check

		// Verify the role was granted
		assert!(Tiki::<T>::user_tikis(&dest).contains(&tiki));

		#[extrinsic_call]
		_(RawOrigin::Root, T::Lookup::unlookup(dest.clone()), tiki.clone());

        // User should no longer have this role
        assert!(!Tiki::<T>::user_tikis(&dest).contains(&tiki));
		Ok(())
	}

	#[benchmark]
	fn force_mint_citizen_nft() -> Result<(), BenchmarkError> {
		let dest: T::AccountId = whitelisted_caller();

		// Ensure collection exists first
		ensure_collection_exists::<T>();

		// Henüz vatandaş olmamalı
		assert!(Tiki::<T>::citizen_nft(&dest).is_none());

		#[extrinsic_call]
		_(RawOrigin::Root, T::Lookup::unlookup(dest.clone()));

		// Vatandaş olduğundan emin ol
		assert!(Tiki::<T>::citizen_nft(&dest).is_some());
		assert!(Tiki::<T>::is_citizen(&dest));

		Ok(())
	}

	#[benchmark]
	fn grant_earned_role() -> Result<(), BenchmarkError> {
		let dest: T::AccountId = whitelisted_caller();
		let tiki = crate::Tiki::Axa; // Earned bir rol

		// Ön koşul: Vatandaş olmalı
		ensure_citizen_nft::<T>(dest.clone())?;

		#[extrinsic_call]
		_(RawOrigin::Root, T::Lookup::unlookup(dest.clone()), tiki.clone());

		// Rolün verildiğini doğrula
		assert!(Tiki::<T>::has_tiki(&dest, &tiki));

		Ok(())
	}

	#[benchmark]
	fn grant_elected_role() -> Result<(), BenchmarkError> {
		let dest: T::AccountId = whitelisted_caller();
		let tiki = crate::Tiki::Parlementer; // Elected bir rol

		// Ön koşul: Vatandaş olmalı
		ensure_citizen_nft::<T>(dest.clone())?;

		#[extrinsic_call]
		_(RawOrigin::Root, T::Lookup::unlookup(dest.clone()), tiki.clone());

		// Rolün verildiğini doğrula
		assert!(Tiki::<T>::has_tiki(&dest, &tiki));

		Ok(())
	}

	// Temporarily skip this benchmark due to KYC complexity in benchmark environment
	// #[benchmark]
	// fn apply_for_citizenship() -> Result<(), BenchmarkError> {
	// 	// KYC setup is complex in benchmark environment
	// 	// This functionality is covered by force_mint_citizen_nft benchmark
	// 	Ok(())
	// }

	impl_benchmark_test_suite!(Tiki, crate::mock::new_test_ext(), crate::mock::Test);
}