# PezkuwiChain Solochain → Parachain Migration Plan

**Date:** 2025-11-22
**Status:** Planning Phase
**Target:** Convert PezkuwiChain from standalone (solochain) to parachain for Westend/Rococo

---

## 🎯 Executive Summary

The frontend XCM Configuration Wizard has been implemented and is ready to use. However, to connect to real relay chains (Westend/Rococo) and test XCM functionality, PezkuwiChain must be converted from a **solochain** (standalone blockchain) to a **parachain** (parachain connected to a relay chain).

### Current State
- ✅ **Frontend:** XCM Configuration Wizard fully implemented
- ✅ **Backend:** xcm-wizard.ts functions ready
- ❌ **Blockchain:** Currently a SOLOCHAIN (cannot connect to relay chain)
- ❌ **Runtime:** Missing Cumulus pallet dependencies

### Target State
- ✅ Convert runtime to **PARACHAIN** architecture
- ✅ Use **Collators** instead of Validators
- ✅ Connect to **Westend/Rococo** relay chain
- ✅ Enable **XCM** cross-chain messaging
- ✅ Preserve all **custom pallets** and functionality

---

## 📊 Architecture Comparison

### Solochain (Current)
```
┌─────────────────────────────────────┐
│   PezkuwiChain (Standalone)         │
│                                     │
│  • Own consensus (BABE + GRANDPA)  │
│  • Own finalization                │
│  • Own security                    │
│  • Validators produce blocks       │
│  • No relay chain connection       │
│  • No XCM support                  │
└─────────────────────────────────────┘
```

### Parachain (Target)
```
┌─────────────────────────────────────┐
│      Westend/Rococo Relay Chain     │
│  (Shared Security & Finalization)   │
│                                     │
│  ┌───────────────────────────────┐  │
│  │  Parachain: PezkuwiChain     │  │
│  │                              │  │
│  │  • Collators produce blocks  │  │
│  │  • Relay chain finalizes     │  │
│  │  • XCM enabled               │  │
│  │  • HRMP channels             │  │
│  │  • Foreign assets            │  │
│  └───────────────────────────────┘  │
│                                     │
│  ┌───────────────────────────────┐  │
│  │  Parachain: Asset Hub        │  │
│  │  (USDT, DOT, etc.)           │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

---

## 🔍 Template Analysis Results

### Parachain Template Structure

#### 1. **Runtime Dependencies** (Cargo.toml)
**Key Addition:** All dependencies come from `pezkuwi-sdk` with cumulus features enabled.

```toml
[dependencies]
# Core parachain dependency
cumulus-pallet-parachain-system.workspace = true

# Features from pezkuwi-sdk
pezkuwi-sdk = {
  workspace = true,
  features = [
    "cumulus-pallet-aura-ext",           # Aura consensus for parachains
    "cumulus-pallet-session-benchmarking",
    "cumulus-pallet-weight-reclaim",      # Weight reclaim for efficiency
    "cumulus-pallet-xcm",                 # XCM support
    "cumulus-pallet-xcmp-queue",          # Cross-chain message queue
    "cumulus-primitives-aura",
    "cumulus-primitives-core",
    "cumulus-primitives-utility",
    "pallet-aura",                        # Aura consensus
    "pallet-authorship",
    "pallet-balances",
    "pallet-collator-selection",          # Collator selection pallet
    "pallet-message-queue",
    "pallet-session",                     # Session management
    "pallet-sudo",
    "pallet-timestamp",
    "pallet-transaction-payment",
    "pallet-transaction-payment-rpc-runtime-api",
    "pallet-xcm",                         # XCM pallet
    "parachains-common",
    "pezkuwi-parachain-primitives",
    "pezkuwi-runtime-common",
    "runtime",
    "staging-parachain-info",             # Parachain info
    "staging-xcm",
    "staging-xcm-builder",
    "staging-xcm-executor",
  ],
  default-features = false
}
```

#### 2. **Runtime Pallets** (lib.rs)
**Key Pallets for Parachain:**

```rust
#[runtime::pallet_index(0)]
pub type System = frame_system;

#[runtime::pallet_index(1)]
pub type ParachainSystem = cumulus_pallet_parachain_system;  // CRITICAL: Parachain system

#[runtime::pallet_index(2)]
pub type Timestamp = pallet_timestamp;

#[runtime::pallet_index(3)]
pub type ParachainInfo = parachain_info;  // CRITICAL: Parachain info

#[runtime::pallet_index(4)]
pub type WeightReclaim = cumulus_pallet_weight_reclaim;

// Collator support (replaces validator staking)
#[runtime::pallet_index(20)]
pub type Authorship = pallet_authorship;

#[runtime::pallet_index(21)]
pub type CollatorSelection = pallet_collator_selection;  // CRITICAL: Collator selection

#[runtime::pallet_index(22)]
pub type Session = pallet_session;

#[runtime::pallet_index(23)]
pub type Aura = pallet_aura;  // CRITICAL: Aura consensus (not BABE/GRANDPA)

#[runtime::pallet_index(24)]
pub type AuraExt = cumulus_pallet_aura_ext;

// XCM support
#[runtime::pallet_index(30)]
pub type XcmpQueue = cumulus_pallet_xcmp_queue;  // CRITICAL: XCM message queue

#[runtime::pallet_index(31)]
pub type PezkuwiXcm = pallet_xcm;  // CRITICAL: XCM pallet

#[runtime::pallet_index(32)]
pub type CumulusXcm = cumulus_pallet_xcm;

#[runtime::pallet_index(33)]
pub type MessageQueue = pallet_message_queue;
```

#### 3. **Consensus Hook**
```rust
type ConsensusHook = cumulus_pallet_aura_ext::FixedVelocityConsensusHook<
    Runtime,
    RELAY_CHAIN_SLOT_DURATION_MILLIS,  // 6000ms
    BLOCK_PROCESSING_VELOCITY,          // 1 block per slot
    UNINCLUDED_SEGMENT_CAPACITY,        // 3 blocks buffered
>;
```

#### 4. **Validate Block Registration**
**CRITICAL:** This macro registers the parachain's block validation with the relay chain.

```rust
cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
}
```

### Collator Node Binary

#### Node Dependencies (Cargo.toml)
```toml
[dependencies]
clap = { features = ["derive"], workspace = true }
color-print = { workspace = true }
futures = { workspace = true }
jsonrpsee = { features = ["server"], workspace = true }
log = { workspace = true, default-features = true }
parachain-template-runtime.workspace = true

pezkuwi-sdk = {
  workspace = true,
  features = ["node"]  # Includes cumulus node dependencies
}

prometheus-endpoint.workspace = true
serde = { features = ["derive"], workspace = true }
```

#### Key Files
1. **`main.rs`** - Entry point, spawns collator
2. **`cli.rs`** - Command-line interface with relay chain args
3. **`command.rs`** - Command execution logic
4. **`chain_spec.rs`** - Chain specification builder
5. **`service.rs`** - Collator service setup
6. **`rpc.rs`** - RPC configuration

---

## 🚀 Migration Steps

### Phase 1: Prepare Directory Structure
1. Create new parachain runtime directory
2. Copy template files as base
3. Adapt for PezkuwiChain branding

### Phase 2: Migrate Custom Pallets
**Custom Pallets to Preserve:**
- ✅ `pallet-tiki` (Governance roles)
- ✅ `pallet-identity-kyc` (Zero-knowledge KYC)
- ✅ `pallet-perwerde` (Education platform)
- ✅ `pallet-validator-pool` (Validator pool) → **Rename to pallet-collator-pool**
- ✅ `pallet-staking-score` (Staking scoring)
- ✅ `pallet-trust` (Trust system)
- ✅ `pallet-pez-treasury` (PEZ treasury)
- ✅ `pallet-pez-rewards` (PEZ rewards)
- ✅ `pallet-welati` (P2P fiat trading)
- ✅ `pallet-token-wrapper` (Token wrapping)
- ✅ `pallet-presale` (Token presale)
- ✅ `pallet-referral` (Referral system)

**Pallets to Replace:**
- ❌ `pallet-babe` → Replace with `pallet-aura`
- ❌ `pallet-grandpa` → Removed (relay chain handles finalization)
- ❌ `pallet-staking` → Replace with `pallet-collator-selection`

### Phase 3: Update Runtime Cargo.toml
1. Add `cumulus-pallet-parachain-system` dependency
2. Add cumulus features to `pezkuwi-sdk`
3. Update feature flags for `std`, `runtime-benchmarks`, `try-runtime`

### Phase 4: Update Runtime lib.rs
1. Replace consensus imports (BABE/GRANDPA → Aura)
2. Add parachain system pallets
3. Add XCM configuration
4. Configure collator selection
5. Register validate block macro

### Phase 5: Create Collator Node
1. Create `pezkuwi-collator` directory under `pezkuwi/node/`
2. Copy parachain node template
3. Adapt CLI for PezkuwiChain
4. Configure RPC endpoints
5. Set up collator service

### Phase 6: Build & Test
1. Build parachain runtime: `cargo build --release -p pezkuwichain-parachain`
2. Build collator binary: `cargo build --release -p pezkuwi-collator`
3. Generate genesis state and WASM
4. Test local collator startup

### Phase 7: Generate Artifacts
```bash
# Generate genesis state
./target/release/pezkuwi-collator export-genesis-state \
  --chain=chain-specs/pezkuwi-parachain.json \
  > genesis-head.hex

# Generate runtime WASM
./target/release/pezkuwi-collator export-genesis-wasm \
  --chain=chain-specs/pezkuwi-parachain.json \
  > runtime.wasm
```

### Phase 8: Westend/Rococo Registration
**Use the XCM Configuration Wizard frontend!**

1. **Step 1:** Reserve ParaId on Westend/Rococo
2. **Step 2:** Upload genesis-head.hex and runtime.wasm
3. **Step 3:** Register parachain on relay chain
4. **Step 4:** Open HRMP channels to Asset Hub (ParaId 1000)
5. **Step 5:** Register foreign assets (USDT, DOT, etc.)
6. **Step 6:** Test XCM transfer from Asset Hub

---

## 📋 File Changes Required

### New Files to Create

#### Runtime
```
pezkuwi/runtime/parachain/
├── Cargo.toml           (NEW - parachain runtime config)
├── build.rs             (NEW - build script)
└── src/
    ├── lib.rs           (NEW - main runtime logic)
    ├── apis.rs          (NEW - runtime APIs)
    ├── benchmarks.rs    (NEW - benchmarking)
    ├── genesis_config_presets.rs  (NEW - genesis presets)
    ├── weights/         (NEW - weight calculations)
    └── configs/
        ├── mod.rs       (NEW - config module)
        └── xcm_config.rs (NEW - XCM configuration)
```

#### Node
```
pezkuwi/node/collator/
├── Cargo.toml           (NEW - collator node config)
├── build.rs             (NEW - build script)
└── src/
    ├── main.rs          (NEW - entry point)
    ├── cli.rs           (NEW - CLI args)
    ├── command.rs       (NEW - command execution)
    ├── chain_spec.rs    (NEW - chain spec builder)
    ├── service.rs       (NEW - collator service)
    └── rpc.rs           (NEW - RPC configuration)
```

#### Chain Specs
```
chain-specs/parachain/
├── pezkuwi-westend-plain.json    (NEW - Westend config)
├── pezkuwi-westend-raw.json      (NEW - Westend raw)
├── pezkuwi-rococo-plain.json     (NEW - Rococo config)
├── pezkuwi-rococo-raw.json       (NEW - Rococo raw)
├── genesis-head-westend.hex      (GENERATED)
├── runtime-westend.wasm          (GENERATED)
├── genesis-head-rococo.hex       (GENERATED)
└── runtime-rococo.wasm           (GENERATED)
```

### Files to Modify

#### Workspace Cargo.toml
```toml
# Add new members
[workspace]
members = [
    # ... existing members
    "pezkuwi/runtime/parachain",     # NEW
    "pezkuwi/node/collator",         # NEW
]

# Add workspace dependencies
[workspace.dependencies]
pezkuwichain-parachain-runtime = { path = "pezkuwi/runtime/parachain", default-features = false }
pezkuwi-collator = { path = "pezkuwi/node/collator" }
```

### Files to Keep (No Changes)
- ✅ All custom pallets in `pezkuwi/pallets/`
- ✅ All frontend code in `/home/mamostehp/pwap/`
- ✅ All shared libraries
- ✅ Documentation

---

## ⚠️ Critical Considerations

### 1. **Pallet Compatibility**
**All custom pallets MUST be compatible with parachain runtime.**

**Action Items:**
- Review each pallet's `Config` trait
- Ensure no dependencies on BABE/GRANDPA
- Update any validator-specific logic to collator logic
- Test all pallets in parachain context

### 2. **Consensus Change**
**BABE + GRANDPA → Aura (parachain consensus)**

**Key Differences:**
- Aura uses 6-second slots (configurable)
- No finality gadget (relay chain handles finalization)
- Lighter weight consensus
- Collators vs Validators

### 3. **Staking Changes**
**Validator staking → Collator selection**

**Migration Strategy:**
- Replace `pallet-staking` with `pallet-collator-selection`
- Collators bond tokens to produce blocks
- No slashing (handled by relay chain)
- Simpler economic model

### 4. **XCM Integration**
**Must configure XCM for cross-chain communication.**

**Required Configuration:**
- `XcmConfig` implementation
- Barrier configuration (security)
- Asset transactor setup
- Location conversions
- Fee payment handling

### 5. **Genesis State**
**Fresh genesis state required.**

**No State Migration:**
- ❌ Cannot migrate existing balances
- ❌ Cannot migrate existing accounts
- ❌ Cannot migrate existing governance state
- ✅ Users must re-register on parachain
- ✅ Airdrops to previous users (optional)

### 6. **Network IDs**
**ParaId assignment on Westend/Rococo.**

**Process:**
1. Reserve ParaId on relay chain (costs ~40 WND/ROC)
2. Upload genesis state and runtime WASM
3. Wait for governance approval (Westend) or sudo (Rococo)
4. Start collator with assigned ParaId

---

## 🎯 Success Criteria

### Phase 1: Local Testing
- ✅ Parachain runtime compiles without errors
- ✅ Collator binary runs locally
- ✅ Can produce parachain blocks
- ✅ All custom pallets functional
- ✅ RPC endpoints accessible

### Phase 2: Relay Chain Connection
- ✅ ParaId reserved on Westend/Rococo
- ✅ Genesis artifacts uploaded successfully
- ✅ Parachain registered on relay chain
- ✅ First parachain block included in relay chain
- ✅ Collator syncing with relay chain

### Phase 3: XCM Functionality
- ✅ HRMP channels opened to Asset Hub
- ✅ Foreign assets registered (USDT, DOT, etc.)
- ✅ XCM transfer from Asset Hub → PezkuwiChain successful
- ✅ XCM transfer from PezkuwiChain → Asset Hub successful
- ✅ Frontend XCM wizard fully functional

---

## 📅 Estimated Timeline

| Phase | Duration | Tasks |
|-------|----------|-------|
| **Phase 1:** Directory Setup | 1 hour | Create directories, copy template |
| **Phase 2:** Runtime Migration | 4-6 hours | Migrate pallets, update Cargo.toml |
| **Phase 3:** Node Binary | 2-3 hours | Create collator node |
| **Phase 4:** Build & Debug | 2-4 hours | Fix compilation errors |
| **Phase 5:** Local Testing | 1-2 hours | Test parachain locally |
| **Phase 6:** Generate Artifacts | 30 min | Export genesis and WASM |
| **Phase 7:** Westend Registration | 1-2 days | Reserve ParaId, wait for governance |
| **Phase 8:** XCM Testing | 2-3 hours | Test cross-chain transfers |
| **TOTAL** | **3-5 days** | (Including relay chain governance wait) |

---

## 🔧 Tools & Resources

### Build Tools
- **Cargo:** Rust build system
- **substrate-wasm-builder:** WASM runtime builder
- **pezkuwi-sdk:** Unified SDK with all dependencies

### Testing Tools
- **Zombienet:** Local parachain testing (for initial validation)
- **Polkadot.js Apps:** Blockchain explorer and interaction
- **XCM Configuration Wizard:** Frontend tool (already implemented!)

### Relay Chains
- **Westend:** Public testnet (requires governance approval)
- **Rococo:** Public testnet (sudo-based, faster registration)

### Documentation
- **Cumulus Tutorial:** https://docs.substrate.io/tutorials/build-a-parachain/
- **XCM Format:** https://github.com/paritytech/xcm-format
- **Polkadot Wiki:** https://wiki.polkadot.network/docs/learn-parachains

---

## 🚦 Next Steps

### Immediate Actions
1. ✅ **Analysis complete** - Template structure understood
2. ⏳ **Create parachain runtime directory**
3. ⏳ **Copy and adapt template files**
4. ⏳ **Migrate custom pallets**
5. ⏳ **Build and test locally**

### Decision Points
- **Which relay chain first?** Rococo (faster) vs Westend (production-like)
- **ParaId strategy:** Request specific ID or use assigned ID?
- **Genesis state:** Fresh start or planned airdrop to previous users?
- **Collator requirements:** Minimum bond amount, collator count

### User Communication
- **Announce migration** to community
- **Explain benefits** of parachain architecture
- **Provide migration guide** for users
- **Set expectations** for timeline

---

## 📝 Notes

### Why Parachain?
1. **Shared Security:** No need to bootstrap own validator set
2. **XCM Support:** Native cross-chain communication
3. **Interoperability:** Connect with Polkadot ecosystem
4. **Lower Cost:** No validator staking required
5. **Production Ready:** Battle-tested infrastructure

### Risks
- **Genesis Reset:** All existing state lost (users must re-register)
- **Complexity:** More moving parts than solochain
- **Relay Chain Dependency:** Requires relay chain to be operational
- **ParaId Cost:** ~40 WND/ROC to reserve ParaId

### Alternatives Considered
- ❌ **Keep Solochain:** Cannot use XCM wizard, isolated ecosystem
- ❌ **State Migration:** Too complex, high risk of data corruption
- ✅ **Fresh Parachain Start:** Clean slate, proven approach

---

**Document Version:** 1.0
**Last Updated:** 2025-11-22
**Author:** Claude Code AI Assistant
**Status:** ✅ Ready for Implementation
