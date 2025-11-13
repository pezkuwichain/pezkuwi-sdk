//! Storage migrations for pallet-welati (Governance)

use super::*;
use frame_support::{
    traits::{Get, GetStorageVersion, OnRuntimeUpgrade, StorageVersion},
    weights::Weight,
};
use sp_std::marker::PhantomData;

/// Current storage version
pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

/// Migration from v0 to v1
/// This migration handles the initial version setup for pallet-welati
pub mod v1 {
    use super::*;

    pub struct MigrateToV1<T>(PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            let current = Pallet::<T>::on_chain_storage_version();

            log::info!(
                "🔄 Running migration for pallet-welati from {:?} to {:?}",
                current,
                STORAGE_VERSION
            );

            if current == StorageVersion::new(0) {
                let migrated;
                let mut weight = Weight::zero();

                // Example migration logic for governance storage
                // If storage format changes in the future, implement transformation here

                // Count existing storage items for logging
                let officials_count = CurrentOfficials::<T>::iter().count() as u64;
                let ministers_count = CurrentMinisters::<T>::iter().count() as u64;
                let elections_count = ActiveElections::<T>::iter().count() as u64;
                let proposals_count = ActiveProposals::<T>::iter().count() as u64;

                migrated = officials_count + ministers_count + elections_count + proposals_count;

                // Update storage version
                STORAGE_VERSION.put::<Pallet<T>>();

                log::info!("✅ Migrated {} entries in pallet-welati", migrated);
                log::info!("   Officials: {}, Ministers: {}, Elections: {}, Proposals: {}",
                    officials_count, ministers_count, elections_count, proposals_count);

                // Return weight used
                // Reads: all storage items + version read
                // Writes: version write
                weight = weight.saturating_add(T::DbWeight::get().reads_writes(
                    migrated + 1,
                    1
                ));

                weight
            } else {
                log::info!("👌 pallet-welati migration not needed, current version is {:?}", current);
                T::DbWeight::get().reads(1)
            }
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<sp_std::vec::Vec<u8>, sp_runtime::TryRuntimeError> {
            let current = Pallet::<T>::on_chain_storage_version();

            log::info!("🔍 Pre-upgrade check for pallet-welati");
            log::info!("   Current version: {:?}", current);

            // Encode current storage counts for verification
            let officials_count = CurrentOfficials::<T>::iter().count() as u32;
            let ministers_count = CurrentMinisters::<T>::iter().count() as u32;
            let parliament_count = ParliamentMembers::<T>::get().len() as u32;
            let diwan_count = DiwanMembers::<T>::get().len() as u32;
            let appointed_count = AppointedOfficials::<T>::iter().count() as u32;
            let elections_count = ActiveElections::<T>::iter().count() as u32;
            let candidates_count = ElectionCandidates::<T>::iter().count() as u32;
            let votes_count = ElectionVotes::<T>::iter().count() as u32;
            let results_count = ElectionResults::<T>::iter().count() as u32;
            let districts_count = ElectoralDistrictConfig::<T>::iter().count() as u32;
            let nominations_count = PendingNominations::<T>::iter().count() as u32;
            let appointments_count = AppointmentProcesses::<T>::iter().count() as u32;
            let proposals_count = ActiveProposals::<T>::iter().count() as u32;
            let collective_votes_count = CollectiveVotes::<T>::iter().count() as u32;

            log::info!("   CurrentOfficials entries: {}", officials_count);
            log::info!("   CurrentMinisters entries: {}", ministers_count);
            log::info!("   ParliamentMembers entries: {}", parliament_count);
            log::info!("   DiwanMembers entries: {}", diwan_count);
            log::info!("   AppointedOfficials entries: {}", appointed_count);
            log::info!("   ActiveElections entries: {}", elections_count);
            log::info!("   ElectionCandidates entries: {}", candidates_count);
            log::info!("   ElectionVotes entries: {}", votes_count);
            log::info!("   ElectionResults entries: {}", results_count);
            log::info!("   ElectoralDistrictConfig entries: {}", districts_count);
            log::info!("   PendingNominations entries: {}", nominations_count);
            log::info!("   AppointmentProcesses entries: {}", appointments_count);
            log::info!("   ActiveProposals entries: {}", proposals_count);
            log::info!("   CollectiveVotes entries: {}", collective_votes_count);

            Ok((
                officials_count,
                ministers_count,
                parliament_count,
                diwan_count,
                appointed_count,
                elections_count,
                candidates_count,
                votes_count,
                results_count,
                districts_count,
                nominations_count,
                appointments_count,
                proposals_count,
                collective_votes_count,
            ).encode())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(state: sp_std::vec::Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
            use codec::Decode;

            let (
                pre_officials_count,
                pre_ministers_count,
                pre_parliament_count,
                pre_diwan_count,
                pre_appointed_count,
                pre_elections_count,
                pre_candidates_count,
                pre_votes_count,
                pre_results_count,
                pre_districts_count,
                pre_nominations_count,
                pre_appointments_count,
                pre_proposals_count,
                pre_collective_votes_count,
            ): (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) =
                Decode::decode(&mut &state[..])
                    .map_err(|_| "Failed to decode pre-upgrade state")?;

            log::info!("🔍 Post-upgrade check for pallet-welati");

            // Verify storage version was updated
            let current_version = Pallet::<T>::on_chain_storage_version();
            assert_eq!(
                current_version, STORAGE_VERSION,
                "Storage version not updated correctly"
            );
            log::info!("✅ Storage version updated to {:?}", current_version);

            // Verify storage counts (should be same or more, never less)
            let post_officials_count = CurrentOfficials::<T>::iter().count() as u32;
            let post_ministers_count = CurrentMinisters::<T>::iter().count() as u32;
            let post_parliament_count = ParliamentMembers::<T>::get().len() as u32;
            let post_diwan_count = DiwanMembers::<T>::get().len() as u32;
            let post_appointed_count = AppointedOfficials::<T>::iter().count() as u32;
            let post_elections_count = ActiveElections::<T>::iter().count() as u32;
            let post_candidates_count = ElectionCandidates::<T>::iter().count() as u32;
            let post_votes_count = ElectionVotes::<T>::iter().count() as u32;
            let post_results_count = ElectionResults::<T>::iter().count() as u32;
            let post_districts_count = ElectoralDistrictConfig::<T>::iter().count() as u32;
            let post_nominations_count = PendingNominations::<T>::iter().count() as u32;
            let post_appointments_count = AppointmentProcesses::<T>::iter().count() as u32;
            let post_proposals_count = ActiveProposals::<T>::iter().count() as u32;
            let post_collective_votes_count = CollectiveVotes::<T>::iter().count() as u32;

            log::info!("   CurrentOfficials entries: {} -> {}", pre_officials_count, post_officials_count);
            log::info!("   CurrentMinisters entries: {} -> {}", pre_ministers_count, post_ministers_count);
            log::info!("   ParliamentMembers entries: {} -> {}", pre_parliament_count, post_parliament_count);
            log::info!("   DiwanMembers entries: {} -> {}", pre_diwan_count, post_diwan_count);
            log::info!("   AppointedOfficials entries: {} -> {}", pre_appointed_count, post_appointed_count);
            log::info!("   ActiveElections entries: {} -> {}", pre_elections_count, post_elections_count);
            log::info!("   ElectionCandidates entries: {} -> {}", pre_candidates_count, post_candidates_count);
            log::info!("   ElectionVotes entries: {} -> {}", pre_votes_count, post_votes_count);
            log::info!("   ElectionResults entries: {} -> {}", pre_results_count, post_results_count);
            log::info!("   ElectoralDistrictConfig entries: {} -> {}", pre_districts_count, post_districts_count);
            log::info!("   PendingNominations entries: {} -> {}", pre_nominations_count, post_nominations_count);
            log::info!("   AppointmentProcesses entries: {} -> {}", pre_appointments_count, post_appointments_count);
            log::info!("   ActiveProposals entries: {} -> {}", pre_proposals_count, post_proposals_count);
            log::info!("   CollectiveVotes entries: {} -> {}", pre_collective_votes_count, post_collective_votes_count);

            // Verify no data was lost
            assert!(
                post_officials_count >= pre_officials_count,
                "CurrentOfficials entries decreased during migration"
            );
            assert!(
                post_ministers_count >= pre_ministers_count,
                "CurrentMinisters entries decreased during migration"
            );
            assert!(
                post_parliament_count >= pre_parliament_count,
                "ParliamentMembers entries decreased during migration"
            );
            assert!(
                post_diwan_count >= pre_diwan_count,
                "DiwanMembers entries decreased during migration"
            );
            assert!(
                post_appointed_count >= pre_appointed_count,
                "AppointedOfficials entries decreased during migration"
            );
            assert!(
                post_elections_count >= pre_elections_count,
                "ActiveElections entries decreased during migration"
            );
            assert!(
                post_candidates_count >= pre_candidates_count,
                "ElectionCandidates entries decreased during migration"
            );
            assert!(
                post_votes_count >= pre_votes_count,
                "ElectionVotes entries decreased during migration"
            );
            assert!(
                post_results_count >= pre_results_count,
                "ElectionResults entries decreased during migration"
            );
            assert!(
                post_districts_count >= pre_districts_count,
                "ElectoralDistrictConfig entries decreased during migration"
            );
            assert!(
                post_nominations_count >= pre_nominations_count,
                "PendingNominations entries decreased during migration"
            );
            assert!(
                post_appointments_count >= pre_appointments_count,
                "AppointmentProcesses entries decreased during migration"
            );
            assert!(
                post_proposals_count >= pre_proposals_count,
                "ActiveProposals entries decreased during migration"
            );
            assert!(
                post_collective_votes_count >= pre_collective_votes_count,
                "CollectiveVotes entries decreased during migration"
            );

            log::info!("✅ Post-upgrade checks passed for pallet-welati");
            Ok(())
        }
    }
}

/// Example migration for future version changes
/// This demonstrates how to handle storage format changes in governance data
pub mod v2 {
    use super::*;

    /// Example: Migration when election or proposal format changes
    pub struct MigrateToV2<T>(PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            let current = Pallet::<T>::on_chain_storage_version();

            if current < StorageVersion::new(2) {
                log::info!("🔄 Running migration for pallet-welati to v2");

                // Example migration logic
                // 1. Transform election data if format changed
                // 2. Migrate proposal structure if needed
                // 3. Update parliament/diwan member format
                // 4. Update version

                // For now, this is just a template
                StorageVersion::new(2).put::<Pallet<T>>();

                log::info!("✅ Completed migration to pallet-welati v2");

                T::DbWeight::get().reads_writes(1, 1)
            } else {
                log::info!("👌 pallet-welati v2 migration not needed");
                T::DbWeight::get().reads(1)
            }
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<sp_std::vec::Vec<u8>, sp_runtime::TryRuntimeError> {
            log::info!("🔍 Pre-upgrade check for pallet-welati v2");
            Ok(sp_std::vec::Vec::new())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(_state: sp_std::vec::Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
            log::info!("✅ Post-upgrade check passed for pallet-welati v2");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{Test, ExtBuilder};
    use frame_support::traits::OnRuntimeUpgrade;

    #[test]
    fn test_migration_v1() {
        ExtBuilder::default().build().execute_with(|| {
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
        ExtBuilder::default().build().execute_with(|| {
            // Set current version
            STORAGE_VERSION.put::<Pallet<Test>>();

            // Run migration again
            let weight = v1::MigrateToV1::<Test>::on_runtime_upgrade();

            // Should be a no-op
            assert_eq!(weight, frame_support::weights::constants::RocksDbWeight::get().reads(1));
        });
    }
}
