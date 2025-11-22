# Storage Migration Guide for PezkuwiChain

## Overview

This document describes the storage migration system implemented in PezkuwiChain pallets. Storage migrations are critical for maintaining data integrity during runtime upgrades when storage schemas change.

## Migration Architecture

### Storage Versioning

Each pallet maintains a storage version using Substrate's `StorageVersion` system:

```rust
use frame_support::traits::{StorageVersion, GetStorageVersion, OnRuntimeUpgrade};

pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
pub struct Pallet<T>(_);
```

### Migration Modules

Migration logic is separated into dedicated `migrations.rs` modules for each pallet:

- `pallets/tiki/src/migrations.rs` - Citizenship/Role NFT migrations
- `pallets/welati/src/migrations.rs` - Governance system migrations
- `pallets/pez-treasury/src/migrations.rs` - Treasury economics migrations

## Migration Workflow

### 1. Version Detection

Migrations run automatically during runtime upgrade via `OnRuntimeUpgrade` trait:

```rust
impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
    fn on_runtime_upgrade() -> Weight {
        let current = Pallet::<T>::on_chain_storage_version();

        if current == StorageVersion::new(0) {
            // Perform migration
            STORAGE_VERSION.put::<Pallet<T>>();
            // Return weight consumed
        } else {
            // Skip migration, already migrated
            T::DbWeight::get().reads(1)
        }
    }
}
```

### 2. Data Transformation

Each migration:
- Reads existing storage items
- Transforms data to new schema (if needed)
- Writes updated data
- Updates storage version
- Returns accurate weight

### 3. Validation (try-runtime)

When compiled with `try-runtime` feature, migrations include validation:

```rust
#[cfg(feature = "try-runtime")]
fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
    // Capture current state
    let count = StorageItem::<T>::iter().count() as u32;
    Ok(count.encode())
}

#[cfg(feature = "try-runtime")]
fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {
    // Verify migration succeeded
    let pre_count: u32 = Decode::decode(&mut &state[..])?;
    let post_count = StorageItem::<T>::iter().count() as u32;
    assert!(post_count >= pre_count, "Data lost during migration");
    Ok(())
}
```

## Pallet-Specific Migrations

### Pallet-Tiki (Role/Citizenship NFTs)

**Storage Items Tracked:**
- `CitizenNft<T>` - Maps accounts to citizen NFT IDs
- `UserTikis<T>` - Maps accounts to their role NFTs
- `TikiHolder<T>` - Maps NFT IDs to owners

**Migration v0→v1:**
- Sets initial storage version
- Counts existing NFT assignments
- Validates no data loss

**File:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/tiki/src/migrations.rs:15-130`

### Pallet-Welati (Governance)

**Storage Items Tracked (14 total):**
- `CurrentOfficials<T>` - Active government officials
- `CurrentMinisters<T>` - Active ministers
- `ParliamentMembers<T>` - Parliament membership list
- `DiwanMembers<T>` - Diwan council membership
- `AppointedOfficials<T>` - Officials by appointment
- `ActiveElections<T>` - Ongoing elections
- `ElectionCandidates<T>` - Candidate registrations
- `ElectionVotes<T>` - Cast votes
- `ElectionResults<T>` - Finalized results
- `ElectoralDistrictConfig<T>` - District configurations
- `PendingNominations<T>` - Nominations awaiting approval
- `AppointmentProcesses<T>` - Active appointment procedures
- `ActiveProposals<T>` - Governance proposals
- `CollectiveVotes<T>` - Collective decision votes

**Migration v0→v1:**
- Comprehensive tracking of all 14 governance storage items
- Validates democratic process continuity
- Ensures no election/vote data loss

**File:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/welati/src/migrations.rs:15-249`

### Pallet-Pez-Treasury (Token Economics)

**Storage Items Tracked:**
- `HalvingInfo<T>` - Current halving period data (ValueQuery)
- `MonthlyReleases<T>` - Historical distribution records (StorageMap)
- `NextReleaseMonth<T>` - Release counter (ValueQuery)
- `TreasuryStartBlock<T>` - Initialization block (OptionQuery)
- `GenesisDistributionDone<T>` - One-time genesis flag (ValueQuery)

**Migration v0→v1:**
- Tracks halving mechanism state
- Validates monthly release history
- Ensures treasury continuity
- Special handling for ValueQuery vs OptionQuery types

**File:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/pez-treasury/src/migrations.rs:15-160`

## Testing Migrations

### Unit Tests

Each pallet includes migration tests:

```bash
# Test individual pallet migrations
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/tiki
cargo test migrations::tests

cd /home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/welati
cargo test migrations::tests

cd /home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/pez-treasury
cargo test migrations::tests
```

### try-runtime Testing

For production readiness, test migrations with real chain state:

```bash
# Build with try-runtime
cargo build --release --features try-runtime

# Test migration on live chain state
./target/release/pezkuwi try-runtime \
    --runtime ./target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.wasm \
    on-runtime-upgrade \
    live --uri wss://pezkuwi.network:443
```

## Adding New Migrations

### Step 1: Create Migration Module

```rust
// In pallets/your-pallet/src/migrations.rs
pub mod v2 {
    use super::*;

    pub struct MigrateToV2<T>(PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            let current = Pallet::<T>::on_chain_storage_version();

            if current < StorageVersion::new(2) {
                // Migration logic here
                StorageVersion::new(2).put::<Pallet<T>>();
                // Return weight
            } else {
                T::DbWeight::get().reads(1)
            }
        }
    }
}
```

### Step 2: Update Storage Version

```rust
// Update the constant
pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);
```

### Step 3: Add to Runtime

```rust
// In runtime/src/lib.rs
pub type Migrations = (
    pallet_tiki::migrations::v2::MigrateToV2<Runtime>,
    pallet_welati::migrations::v2::MigrateToV2<Runtime>,
    // ... other migrations
);
```

### Step 4: Test Thoroughly

1. Write unit tests
2. Add try-runtime pre/post upgrade checks
3. Test on testnet
4. Verify with real data snapshots

## Migration Best Practices

### Do's

1. **Always increment version numbers** - Never reuse version numbers
2. **Make migrations idempotent** - Safe to run multiple times
3. **Calculate accurate weights** - Account for all reads/writes
4. **Add comprehensive logging** - Use `log::info!` for debugging
5. **Validate data integrity** - Check counts before/after
6. **Test with try-runtime** - Validate on real chain state
7. **Document breaking changes** - Explain what changed and why

### Don'ts

1. **Don't skip versions** - Migrate incrementally
2. **Don't delete old migration code** - Keep for reference
3. **Don't assume empty storage** - Handle existing data gracefully
4. **Don't forget weight accounting** - Migrations consume resources
5. **Don't migrate in genesis** - Use genesis config instead
6. **Don't hard-code values** - Use configuration when possible

## Weight Calculation

Migrations must return accurate weights for fee calculation:

```rust
let mut weight = Weight::zero();

// Count operations
let reads = storage_items_read as u64;
let writes = storage_items_written as u64;

// Add to weight
weight = weight.saturating_add(
    T::DbWeight::get().reads_writes(reads, writes)
);

weight
```

## Rollback Strategy

Substrate migrations are one-way by design. For rollback:

1. **Prepare reverse migration** - Write code to undo changes
2. **Take snapshots** - Backup state before upgrade
3. **Test rollback** - Verify reverse migration works
4. **Document procedure** - Clear rollback instructions

**Example reverse migration:**

```rust
pub mod rollback_v2 {
    pub struct RollbackToV1<T>(PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for RollbackToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            let current = Pallet::<T>::on_chain_storage_version();

            if current == StorageVersion::new(2) {
                // Reverse the v2 changes
                // ...
                StorageVersion::new(1).put::<Pallet<T>>();
            }
            // Return weight
        }
    }
}
```

## Production Checklist

Before deploying migrations to production:

- [ ] Unit tests pass for all migrations
- [ ] try-runtime validation successful
- [ ] Testnet deployment successful
- [ ] Weight benchmarks updated
- [ ] Migration documented
- [ ] Rollback procedure prepared
- [ ] Team review completed
- [ ] Validator coordination planned
- [ ] Monitoring alerts configured
- [ ] Backup created

## Monitoring

After migration deployment:

1. **Check version** - Verify storage version updated
2. **Verify data** - Spot check critical storage items
3. **Monitor logs** - Watch for migration errors
4. **Track performance** - Ensure weight estimates accurate
5. **User impact** - Check for any service disruptions

## References

- [Substrate Storage Migrations](https://docs.substrate.io/maintain/runtime-upgrades/)
- [try-runtime Documentation](https://paritytech.github.io/try-runtime-cli/)
- [Storage Versioning](https://docs.substrate.io/build/upgrade-the-runtime/)

## Support

For migration issues or questions:
- Review pallet-specific migration code in `/pallets/*/src/migrations.rs`
- Check test cases in `/pallets/*/src/migrations.rs` (test modules)
- Consult [RUNTIME_UPGRADES.md](./RUNTIME_UPGRADES.md) for deployment procedures
