# Runtime Upgrade Guide for PezkuwiChain

## Overview

This guide provides comprehensive procedures for performing runtime upgrades on PezkuwiChain. Runtime upgrades allow updating the blockchain logic without requiring a hard fork or network restart.

## Table of Contents

1. [Pre-Upgrade Preparation](#pre-upgrade-preparation)
2. [Building the Runtime](#building-the-runtime)
3. [Testing with try-runtime](#testing-with-try-runtime)
4. [Upgrade Methods](#upgrade-methods)
5. [Post-Upgrade Verification](#post-upgrade-verification)
6. [Rollback Procedures](#rollback-procedures)
7. [Emergency Response](#emergency-response)

## Pre-Upgrade Preparation

### 1. Code Review Checklist

Before initiating any runtime upgrade:

- [ ] All code changes peer-reviewed
- [ ] Storage migrations tested (see [STORAGE_MIGRATIONS.md](./STORAGE_MIGRATIONS.md))
- [ ] API documentation updated
- [ ] Changelog prepared
- [ ] Breaking changes documented
- [ ] Weight benchmarks regenerated
- [ ] Security audit completed (for major changes)
- [ ] Testnet deployment successful

### 2. Communication Plan

**Timeline: 2 weeks before upgrade**

- [ ] Announce upgrade to validators via official channels
- [ ] Publish detailed changelog and migration notes
- [ ] Schedule upgrade window (recommend low-traffic period)
- [ ] Prepare monitoring alerts
- [ ] Brief technical support team

**Timeline: 1 week before upgrade**

- [ ] Send reminder to validators
- [ ] Verify validator readiness (>67% acknowledgment)
- [ ] Test rollback procedures
- [ ] Prepare emergency contact list

**Timeline: 24 hours before upgrade**

- [ ] Final validator confirmation
- [ ] Take database snapshots
- [ ] Verify monitoring systems operational
- [ ] Place technical team on standby

### 3. Environment Setup

```bash
# Ensure you have latest codebase
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi
git pull origin main

# Check Rust toolchain
rustup update stable
rustup target add wasm32-unknown-unknown

# Install try-runtime CLI
cargo install --git https://github.com/paritytech/try-runtime-cli --locked

# Verify dependencies
cargo check --release
```

## Building the Runtime

### 1. Update Version Numbers

Edit `runtime/src/lib.rs`:

```rust
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("pezkuwichain"),
    impl_name: create_runtime_str!("pezkuwichain"),
    authoring_version: 1,
    spec_version: 101,  // INCREMENT THIS
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 2,  // INCREMENT IF TRANSACTION FORMAT CHANGED
    state_version: 1,
};
```

**Version Increment Rules:**
- `spec_version`: Increment for ANY runtime changes
- `transaction_version`: Increment if transaction format changes
- `impl_version`: Increment for optimizations without logic changes

### 2. Configure Storage Migrations

In `runtime/src/lib.rs`, add migrations:

```rust
pub type Migrations = (
    // Add new migrations here
    pallet_tiki::migrations::v1::MigrateToV1<Runtime>,
    pallet_welati::migrations::v1::MigrateToV1<Runtime>,
    pallet_pez_treasury::migrations::v1::MigrateToV1<Runtime>,
);

pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,  // Include migrations here
>;
```

### 3. Build Runtime WASM

```bash
# Clean build (recommended for production)
cargo clean

# Build optimized WASM
cargo build --release --features runtime-benchmarks

# Verify WASM exists
ls -lh target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.compact.compressed.wasm

# Note the file size (should be < 2MB for optimal performance)
```

### 4. Extract Runtime Blob

```bash
# Extract hex-encoded WASM
cat target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.compact.compressed.wasm | \
    hexdump -ve '1/1 "%.2x"' > runtime.hex

# Get hash for verification
sha256sum target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.compact.compressed.wasm
```

## Testing with try-runtime

### 1. Install try-runtime

```bash
# Build node with try-runtime feature
cargo build --release --features try-runtime

# Build runtime with try-runtime
cd runtime
cargo build --release --features try-runtime
cd ..
```

### 2. Fork Live Chain State

```bash
# Create a fork of production chain
./target/release/pezkuwi try-runtime \
    --runtime ./target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.wasm \
    on-runtime-upgrade \
    live \
    --uri wss://pezkuwi.network:443
```

### 3. Test Migrations

```bash
# Run migrations on forked state
./target/release/pezkuwi try-runtime \
    --runtime ./target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.wasm \
    on-runtime-upgrade \
    live \
    --uri wss://pezkuwi.network:443 \
    --checks all

# Expected output:
# ✅ pre_upgrade hooks passed
# ✅ on_runtime_upgrade executed
# ✅ post_upgrade hooks passed
# ✅ No invariants violated
```

### 4. Test on Testnet

```bash
# Deploy to testnet first
# 1. Build runtime
cargo build --release

# 2. Submit upgrade on testnet
polkadot-js-api \
    --ws wss://testnet.pezkuwi.network \
    --sudo \
    --seed "//Alice" \
    tx.sudo.sudoUncheckedWeight \
    tx.system.setCode \
    "0x$(cat runtime.hex)" \
    0

# 3. Monitor testnet for 24-48 hours
# 4. Verify all functionality works
```

## Upgrade Methods

### Method 1: Sudo Upgrade (Development/Testnet)

**Use Case:** Testing, development networks, or initial testnet deployments

```javascript
// Using Polkadot.js Apps
// 1. Go to Developer > Sudo
// 2. Select: system.setCode(code)
// 3. Upload: pezkuwichain_runtime.compact.compressed.wasm
// 4. Submit Sudo transaction

// Or via CLI:
polkadot-js-api \
    --ws wss://testnet.pezkuwi.network \
    --sudo \
    --seed "//Alice" \
    tx.sudo.sudoUncheckedWeight \
    tx.system.setCode \
    "0x$(cat runtime.hex)" \
    0
```

### Method 2: Democracy Proposal (Production)

**Use Case:** Production networks with governance

```javascript
// Step 1: Submit Preimage
api.tx.preimage.notePreimage(
    api.tx.system.setCode(runtimeWasm).toHex()
)

// Step 2: Submit Proposal
api.tx.democracy.propose(
    preimageHash,
    proposalBond
)

// Step 3: Wait for voting period
// Step 4: Execute if passed
```

### Method 3: Council Motion (Fast Track)

**Use Case:** Emergency upgrades or council-approved changes

```javascript
// Step 1: Council member proposes
api.tx.council.propose(
    threshold,
    api.tx.democracy.externalProposeMajority(
        preimageHash
    ),
    lengthBound
)

// Step 2: Council votes
api.tx.council.vote(proposalHash, index, approve)

// Step 3: Close and execute
api.tx.council.close(proposalHash, index, proposalWeight, lengthBound)
```

### Method 4: Scheduler (Delayed Upgrade)

**Use Case:** Scheduled maintenance windows

```javascript
// Schedule upgrade for specific block
api.tx.scheduler.schedule(
    targetBlock,
    null,  // no periodic
    priority,
    api.tx.system.setCode(runtimeWasm)
)
```

## Runtime Upgrade Workflow

### Standard Production Upgrade Process

```
Week -2: Code Freeze & Testing
├── Complete code review
├── Run try-runtime tests
├── Deploy to testnet
└── Announce upgrade to validators

Week -1: Validation & Preparation
├── Monitor testnet
├── Verify validator readiness
├── Prepare rollback plan
└── Schedule upgrade window

Day -1: Final Checks
├── Database snapshots
├── System health checks
├── Team briefing
└── Activate monitoring

Hour 0: Execute Upgrade
├── Submit runtime upgrade transaction
├── Monitor block production
├── Watch for migration logs
└── Verify new runtime active

Hour +1: Immediate Verification
├── Check all pallets functional
├── Verify storage migrations completed
├── Test critical paths
└── Monitor error rates

Hour +24: Extended Monitoring
├── Continuous health monitoring
├── User feedback collection
├── Performance analysis
└── Document any issues

Week +1: Post-Upgrade Review
├── Analyze metrics
├── Document lessons learned
├── Update procedures
└── Plan next upgrade
```

## Post-Upgrade Verification

### 1. Verify Runtime Version

```bash
# Via polkadot-js-api
polkadot-js-api --ws wss://pezkuwi.network query.system.lastRuntimeUpgrade

# Expected output:
# {
#   specVersion: 101,
#   specName: pezkuwichain
# }
```

### 2. Check Storage Migrations

```bash
# Check migration logs
tail -f /var/log/pezkuwi/node.log | grep -i migration

# Expected:
# 🔄 Running migration for pallet-tiki from v0 to v1
# ✅ Migrated 150 entries in pallet-tiki
# 🔄 Running migration for pallet-welati from v0 to v1
# ✅ Migrated 487 entries in pallet-welati
# 🔄 Running migration for pallet-pez-treasury from v0 to v1
# ✅ Migrated 245 entries in pallet-pez-treasury
```

### 3. Test Critical Functions

```javascript
// Test each pallet's main functionality
const tests = [
    // Tiki
    () => api.query.tiki.citizenNft(account),
    () => api.query.tiki.userTikis(account),

    // Welati
    () => api.query.welati.currentOfficials.entries(),
    () => api.query.welati.activeElections.entries(),

    // PezTreasury
    () => api.query.pezTreasury.halvingInfo(),
    () => api.query.pezTreasury.monthlyReleases.entries(),
];

for (const test of tests) {
    try {
        await test();
        console.log('✅ Test passed');
    } catch (e) {
        console.error('❌ Test failed:', e);
    }
}
```

### 4. Monitor Key Metrics

```bash
# Block production rate
echo "Checking block production..."
watch -n 5 'curl -s http://localhost:9933 -H "Content-Type: application/json" \
    -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\"}" | jq'

# Peer count
echo "Checking peer connections..."
curl -s http://localhost:9933 -H "Content-Type: application/json" \
    -d '{"id":1,"jsonrpc":"2.0","method":"system_health"}' | jq

# Chain finalization
echo "Checking finalization..."
curl -s http://localhost:9933 -H "Content-Type: application/json" \
    -d '{"id":1,"jsonrpc":"2.0","method":"chain_getFinalizedHead"}' | jq
```

## Rollback Procedures

### When to Rollback

Rollback if you observe:
- ❌ Block production stopped for > 5 minutes
- ❌ Migration failures in logs
- ❌ Critical storage data corruption
- ❌ Network split (> 33% validators on old version)
- ❌ Unrecoverable consensus failures

### Emergency Rollback Steps

#### Option 1: Database Snapshot Restore

```bash
# 1. Stop node immediately
systemctl stop pezkuwi-node

# 2. Restore database from backup
rm -rf /var/lib/pezkuwi/chains/pezkuwichain/db
cp -r /backup/pezkuwi-db-pre-upgrade/* /var/lib/pezkuwi/chains/pezkuwichain/db

# 3. Restore old runtime WASM
cp /backup/pezkuwi-runtime-previous.wasm \
   /var/lib/pezkuwi/chains/pezkuwichain/runtime.wasm

# 4. Restart node
systemctl start pezkuwi-node

# 5. Monitor recovery
tail -f /var/log/pezkuwi/node.log
```

#### Option 2: Reverse Migration

If database restore not feasible:

```rust
// Prepare reverse migration beforehand
pub mod rollback {
    pub struct RollbackMigration<T>(PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for RollbackMigration<T> {
        fn on_runtime_upgrade() -> Weight {
            // Undo changes from forward migration
            // This must be prepared in advance!
            StorageVersion::new(PREVIOUS_VERSION).put::<Pallet<T>>();
            Weight::zero()
        }
    }
}

// Submit rollback runtime
// Then submit original runtime with reverse migrations
```

### Coordination During Rollback

1. **Immediate Communication**
   ```
   URGENT: Runtime upgrade rollback in progress
   - Issue: [specific problem]
   - Action: Reverting to previous runtime
   - ETA: [estimate]
   - Status updates every 15 minutes
   ```

2. **Validator Coordination**
   - Notify all validators immediately
   - Coordinate simultaneous rollback if needed
   - Verify 67%+ validators on same version

3. **User Communication**
   - Post status page update
   - Pause dApp integrations temporarily
   - Provide estimated recovery time

## Emergency Response

### Emergency Contact Protocol

```
Tier 1: Core Team (15 min response)
- Lead Developer: [contact]
- DevOps Engineer: [contact]
- Runtime Engineer: [contact]

Tier 2: Extended Team (1 hour response)
- Security Auditor: [contact]
- Validator Liaisons: [contact]
- Community Manager: [contact]

Tier 3: External Support (4 hour response)
- Parity Tech Support: [contact]
- Substrate Builders Program: [contact]
```

### Emergency Decision Tree

```
Issue Detected
│
├─► Block production stopped?
│   ├─► YES → Emergency rollback (Option 1)
│   └─► NO → Continue monitoring
│
├─► Data corruption detected?
│   ├─► YES → Immediate rollback + investigation
│   └─► NO → Continue monitoring
│
├─► Migration failed?
│   ├─► Critical data? → Rollback
│   └─► Non-critical? → Deploy patch
│
└─► Performance degradation?
    ├─► > 50% slower? → Schedule rollback
    └─► < 50% slower? → Monitor + optimize
```

## Best Practices

### Do's ✅

1. **Always test on testnet first**
2. **Take database snapshots before upgrade**
3. **Use try-runtime for validation**
4. **Document all changes thoroughly**
5. **Coordinate with validators**
6. **Monitor continuously post-upgrade**
7. **Have rollback plan ready**
8. **Schedule during low-traffic periods**
9. **Increment version numbers correctly**
10. **Keep migration code in codebase**

### Don'ts ❌

1. **Don't skip testnet testing**
2. **Don't upgrade during peak hours**
3. **Don't rush the process**
4. **Don't ignore validator feedback**
5. **Don't deploy without snapshots**
6. **Don't forget to update documentation**
7. **Don't skip try-runtime tests**
8. **Don't leave team unavailable**
9. **Don't ignore warning signs**
10. **Don't delete old runtime files**

## Monitoring Dashboard

### Key Metrics to Track

```yaml
# Prometheus metrics
pezkuwi_block_height
pezkuwi_finalized_height
pezkuwi_peer_count
pezkuwi_transaction_pool_size
pezkuwi_runtime_version

# Alerts to configure
- Block production stopped > 5 minutes
- Peer count < 10
- Finalization lag > 50 blocks
- Transaction pool > 10000
- Runtime version mismatch across nodes
```

### Grafana Dashboard Panels

1. **Block Production Rate** (blocks/minute)
2. **Finalization Lag** (blocks behind)
3. **Peer Connections** (count)
4. **Transaction Throughput** (tx/second)
5. **Storage Migration Status** (progress %)
6. **Node Resource Usage** (CPU, RAM, Disk)
7. **Network Health** (validator status)

## Checklist Templates

### Pre-Upgrade Checklist

```markdown
## Technical Preparation
- [ ] Runtime WASM built and tested
- [ ] try-runtime validation passed
- [ ] Storage migrations tested
- [ ] Weight benchmarks updated
- [ ] Testnet deployment successful (48h+ stable)
- [ ] API documentation updated

## Operational Preparation
- [ ] Database snapshots taken
- [ ] Rollback plan documented
- [ ] Validators notified (100% acknowledged)
- [ ] Monitoring alerts configured
- [ ] Emergency contacts verified
- [ ] Status page prepared

## Communication
- [ ] Changelog published
- [ ] Upgrade announcement sent
- [ ] Technical support briefed
- [ ] Community informed
- [ ] Validator coordination complete
```

### Post-Upgrade Checklist

```markdown
## Immediate Verification (Hour 0-1)
- [ ] Runtime version updated
- [ ] Block production normal
- [ ] Storage migrations completed
- [ ] No error logs
- [ ] All pallets functional

## Extended Monitoring (Hour 1-24)
- [ ] Performance metrics normal
- [ ] User transactions processing
- [ ] Validator participation > 90%
- [ ] No consensus issues
- [ ] Finalization progressing

## Documentation (Week 1)
- [ ] Post-mortem completed
- [ ] Metrics analyzed
- [ ] Issues documented
- [ ] Procedures updated
- [ ] Team retrospective done
```

## References

- [Substrate Runtime Upgrades](https://docs.substrate.io/maintain/runtime-upgrades/)
- [try-runtime Documentation](https://paritytech.github.io/try-runtime-cli/)
- [Storage Migrations Guide](./STORAGE_MIGRATIONS.md)
- [Pallet API Documentation](./API_PALLET_TIKI.md)

## Support

For runtime upgrade assistance:
- Technical Issues: Check `/var/log/pezkuwi/node.log`
- Validator Coordination: [validator coordination channel]
- Emergency Contact: [emergency hotline]
- Documentation: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/`
