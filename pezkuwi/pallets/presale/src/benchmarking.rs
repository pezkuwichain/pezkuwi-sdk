//! Benchmarking setup for pallet-presale

use super::*;

#[allow(unused)]
use crate::Pallet as Presale;
use frame_benchmarking::v2::*;
use frame_support::traits::{fungibles::Mutate, Get};
use frame_system::RawOrigin;
use sp_runtime::traits::Zero;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn start_presale() {
        #[extrinsic_call]
        start_presale(RawOrigin::Root);

        assert!(PresaleActive::<T>::get());
    }

    #[benchmark]
    fn contribute() {
        // Setup: Create assets and start presale
        let caller: T::AccountId = account("caller", 0, 0);
        let amount: u128 = 100_000_000; // 100 wUSDT

        // Fund caller with wUSDT
        let asset_id = T::WUsdtAssetId::get();
        <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
            asset_id.clone(),
            &caller,
            1_000_000_000u128.try_into().ok().unwrap(),
        )
        .ok();

        // Start presale
        let _ = Presale::<T>::start_presale(RawOrigin::Root.into());

        #[extrinsic_call]
        contribute(RawOrigin::Signed(caller.clone()), amount);

        assert_eq!(Contributions::<T>::get(&caller), amount);
    }

    #[benchmark]
    fn finalize_presale(n: Linear<1, 100>) {
        // Setup: Create n contributors
        let asset_id_wusdt = T::WUsdtAssetId::get();
        let asset_id_pez = T::PezAssetId::get();
        let treasury = Pallet::<T>::account_id();

        // Fund treasury with PEZ
        <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
            asset_id_pez.clone(),
            &treasury,
            100_000_000_000_000_000_000u128.try_into().ok().unwrap(),
        )
        .ok();

        // Start presale
        let _ = Presale::<T>::start_presale(RawOrigin::Root.into());

        // Create n contributors
        for i in 0..n {
            let contributor: T::AccountId = account("contributor", i, 0);

            // Fund contributor
            <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
                asset_id_wusdt.clone(),
                &contributor,
                1_000_000_000u128.try_into().ok().unwrap(),
            )
            .ok();

            // Contribute
            let _ = Presale::<T>::contribute(RawOrigin::Signed(contributor).into(), 100_000_000);
        }

        // Move to end of presale
        let current_block = frame_system::Pallet::<T>::block_number();
        let end_block = current_block + T::PresaleDuration::get();
        frame_system::Pallet::<T>::set_block_number(end_block);

        #[extrinsic_call]
        finalize_presale(RawOrigin::Root);

        assert!(!PresaleActive::<T>::get());
    }

    #[benchmark]
    fn emergency_pause() {
        let _ = Presale::<T>::start_presale(RawOrigin::Root.into());

        #[extrinsic_call]
        emergency_pause(RawOrigin::Root);

        assert!(Paused::<T>::get());
    }

    #[benchmark]
    fn emergency_unpause() {
        let _ = Presale::<T>::start_presale(RawOrigin::Root.into());
        let _ = Presale::<T>::emergency_pause(RawOrigin::Root.into());

        #[extrinsic_call]
        emergency_unpause(RawOrigin::Root);

        assert!(!Paused::<T>::get());
    }

    impl_benchmark_test_suite!(Presale, crate::mock::new_test_ext(), crate::mock::Test);
}
