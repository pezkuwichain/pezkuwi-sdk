//! Storage migrations for pallet-tiki

use super::*;
use frame_support::{
    traits::{Get, GetStorageVersion, OnRuntimeUpgrade, StorageVersion},
    weights::Weight,
};
use sp_std::marker::PhantomData;

/// Current storage version
pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

/// Migration from v0 to v1
/// This is a template migration that can be customized based on actual storage changes
pub mod v1 {
    use super::*;

    pub struct MigrateToV1<T>(PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            let current = Pallet::<T>::on_chain_storage_version();

            log::info!(
                "🔄 Running migration for pallet-tiki from {:?} to {:?}",
                current,
                STORAGE_VERSION
            );

            if current == StorageVersion::new(0) {
                // Perform migration logic here
                // Example: Iterate over storage and transform data

                let migrated = 0u64;
                let mut weight = Weight::zero();

                // Example: Migrate CitizenNft storage if format changed
                // for (account, nft_id) in CitizenNft::<T>::iter() {
                //     // Transform data if needed
                //     migrated += 1;
                // }

                // Update storage version
                STORAGE_VERSION.put::<Pallet<T>>();

                log::info!("✅ Migrated {} entries in pallet-tiki", migrated);

                // Return weight used
                // Reads: migrated items + version read
                // Writes: migrated items + version write
                weight = weight.saturating_add(T::DbWeight::get().reads_writes(
                    migrated + 1,
                    migrated + 1
                ));

                weight
            } else {
                log::info!("👌 pallet-tiki migration not needed, current version is {:?}", current);
                T::DbWeight::get().reads(1)
            }
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<sp_std::vec::Vec<u8>, sp_runtime::TryRuntimeError> {
            let current = Pallet::<T>::on_chain_storage_version();

            log::info!("🔍 Pre-upgrade check for pallet-tiki");
            log::info!("   Current version: {:?}", current);

            // Encode current storage counts for verification
            let citizen_count = CitizenNft::<T>::iter().count() as u32;
            let user_tikis_count = UserTikis::<T>::iter().count() as u32;
            let tiki_holder_count = TikiHolder::<T>::iter().count() as u32;

            log::info!("   CitizenNft entries: {}", citizen_count);
            log::info!("   UserTikis entries: {}", user_tikis_count);
            log::info!("   TikiHolder entries: {}", tiki_holder_count);

            Ok((citizen_count, user_tikis_count, tiki_holder_count).encode())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(state: sp_std::vec::Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
            use codec::Decode;

            let (
                pre_citizen_count,
                pre_user_tikis_count,
                pre_tiki_holder_count
            ): (u32, u32, u32) = Decode::decode(&mut &state[..])
                .map_err(|_| "Failed to decode pre-upgrade state")?;

            log::info!("🔍 Post-upgrade check for pallet-tiki");

            // Verify storage version was updated
            let current_version = Pallet::<T>::on_chain_storage_version();
            assert_eq!(
                current_version, STORAGE_VERSION,
                "Storage version not updated correctly"
            );
            log::info!("✅ Storage version updated to {:?}", current_version);

            // Verify storage counts (should be same or more, never less)
            let post_citizen_count = CitizenNft::<T>::iter().count() as u32;
            let post_user_tikis_count = UserTikis::<T>::iter().count() as u32;
            let post_tiki_holder_count = TikiHolder::<T>::iter().count() as u32;

            log::info!("   CitizenNft entries: {} -> {}", pre_citizen_count, post_citizen_count);
            log::info!("   UserTikis entries: {} -> {}", pre_user_tikis_count, post_user_tikis_count);
            log::info!("   TikiHolder entries: {} -> {}", pre_tiki_holder_count, post_tiki_holder_count);

            assert!(
                post_citizen_count >= pre_citizen_count,
                "CitizenNft entries decreased during migration"
            );
            assert!(
                post_user_tikis_count >= pre_user_tikis_count,
                "UserTikis entries decreased during migration"
            );
            assert!(
                post_tiki_holder_count >= pre_tiki_holder_count,
                "TikiHolder entries decreased during migration"
            );

            log::info!("✅ Post-upgrade checks passed for pallet-tiki");
            Ok(())
        }
    }
}

/// Example migration for future version changes
/// This demonstrates how to handle storage item renames or format changes
pub mod v2 {
    use super::*;

    /// Example: Migration when storage format changes
    pub struct MigrateToV2<T>(PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            let current = Pallet::<T>::on_chain_storage_version();

            if current < StorageVersion::new(2) {
                log::info!("🔄 Running migration for pallet-tiki to v2");

                // Example migration logic
                // 1. Create new storage with modified format
                // 2. Migrate data from old storage to new
                // 3. Remove old storage
                // 4. Update version

                // For now, this is just a template
                STORAGE_VERSION.put::<Pallet<T>>();

                log::info!("✅ Completed migration to pallet-tiki v2");

                T::DbWeight::get().reads_writes(1, 1)
            } else {
                log::info!("👌 pallet-tiki v2 migration not needed");
                T::DbWeight::get().reads(1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{Test, new_test_ext};
    use frame_support::traits::OnRuntimeUpgrade;

    #[test]
    fn test_migration_v1() {
        new_test_ext().execute_with(|| {
            // Set initial storage version to 0
            StorageVersion::new(0).put::<Pallet<Test>>();

            // Run migration
            let weight = v1::MigrateToV1::<Test>::on_runtime_upgrade();

            // Verify version was updated
            assert_eq!(Pallet::<Test>::on_chain_storage_version(), STORAGE_VERSION);

            // Verify weight is non-zero
            assert!(weight != Weight::zero());
        });
    }

    #[test]
    fn test_migration_idempotent() {
        new_test_ext().execute_with(|| {
            // Set current version
            STORAGE_VERSION.put::<Pallet<Test>>();

            // Run migration again
            let weight = v1::MigrateToV1::<Test>::on_runtime_upgrade();

            // Should be a no-op
            assert_eq!(weight, frame_support::weights::constants::RocksDbWeight::get().reads(1));
        });
    }
}
