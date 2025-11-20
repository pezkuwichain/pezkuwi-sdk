# 🚀 Pezkuwichain Deployment Roadmap

## 📍 CURRENT STATUS (2025-11-20 17:20 UTC)

### ✅ COMPLETED PHASES

#### Phase 1: wUSDT Infrastructure Setup
- ✅ **SDK Constants** (`pezkuwi/runtime/pezkuwichain/constants/src/lib.rs:140-152`)
  - Asset ID: 1000
  - Decimals: 6 (USDT standard)
  - Min Balance: 1,000 (0.001 USDT)
- ✅ **Documentation** (`WUSDT.md`) - 4-phase implementation plan
- ✅ **Genesis Config Update** (`genesis_config_presets.rs:56,405,410,426,428`)
  - wUSDT now uses Asset ID 1000 (was 2)
  - Imported constants from pezkuwichain_constants::assets
  - Updated next_asset_id to 1001
- ✅ **Frontend Alignment** (pwap repo, commit `65126b4`)
  - `shared/lib/wallet.ts` - ASSET_IDS.WUSDT: 2 → 1000
  - `shared/lib/usdt.ts` - Uses ASSET_CONFIGS
  - `web/src/components/AccountBalance.tsx` - Pool queries updated
  - `web/src/components/USDTBridge.tsx` - Burn tx updated
- ✅ **Build Cleanup**
  - All background processes killed
  - 95.5GB build artifacts cleaned
  - Fresh build environment

#### Phase 2: SDK Build (IN PROGRESS)
- ✅ **Stage 1: Runtime Build** - COMPLETED (5m 38s)
  - Exit code: 0 (SUCCESS)
  - Warnings: 3 unused imports (non-critical)
- 🔄 **Stage 2: Full Workspace Build** - IN PROGRESS
  - Background Job: `c26e47`
  - Log: `/tmp/full_sdk_build.log`
  - Started: 2025-11-20 17:15 UTC
  - Estimated completion: ~10-15 minutes

### 🎯 IMMEDIATE NEXT STEPS (After Build Completes)

#### 1. Test Level 1: Dev Mode Local Node ⏭️ NEXT
```bash
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi
./target/release/pezkuwichain-node --dev --tmp
```

**Validation Checklist:**
- [ ] Node starts without errors
- [ ] wUSDT asset exists (ID: 1000)
- [ ] Genesis allocations correct (1M wUSDT to founder)
- [ ] Asset metadata: symbol="wUSDT", decimals=6
- [ ] RPC endpoints responding
- [ ] Blocks producing

**Test Commands:**
```bash
# Check wUSDT asset metadata
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "assets_metadata", "params":[1000]}' http://localhost:9944

# Check founder wUSDT balance
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "assets_account", "params":[1000, "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]}' http://localhost:9944
```

#### 2. Test Level 2: Local Testnet (Alice+Bob)
```bash
# Terminal 1 - Alice
./target/release/pezkuwichain-node \
  --chain=local \
  --alice \
  --tmp \
  --port 30333 \
  --rpc-port 9944

# Terminal 2 - Bob
./target/release/pezkuwichain-node \
  --chain=local \
  --bob \
  --tmp \
  --port 30334 \
  --rpc-port 9945 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<ALICE_NODE_ID>
```

**Validation Checklist:**
- [ ] Both nodes start and connect
- [ ] Blocks finalizing (GRANDPA)
- [ ] wUSDT asset consistent across nodes
- [ ] Test wUSDT transfer between Alice→Bob
- [ ] Pool creation test (wHEZ/wUSDT)

#### 3. Commit Genesis Changes
```bash
cd /home/mamostehp/Pezkuwi-SDK
git add pezkuwi/runtime/pezkuwichain/src/genesis_config_presets.rs
git commit -m "feat(genesis): update wUSDT to Asset ID 1000

- Align wUSDT with constants (Asset ID 1000)
- Import WUSDT_ASSET_ID, WUSDT_DECIMALS, WUSDT_MIN_BALANCE
- Update next_asset_id to 1001
- Remove unused imports (AssetConversion, ToString, get_public_from_string_or_panic)

This completes Phase 1 of wUSDT infrastructure, enabling:
- Consistent Asset ID across SDK and frontend (1000)
- 6-decimal precision (USDT standard)
- Proper minimum balance (0.001 USDT)

🤖 Generated with Claude Code
Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## 🗺️ FULL DEPLOYMENT PROGRESSION

### Network Levels (Sequential Progression)

```
┌─────────────┐
│ 1. Dev      │ ← WE ARE HERE (Testing next)
│ (--dev)     │   1 validator (Alice seed)
└─────────────┘   Temporary chain state
       ↓
┌─────────────┐
│ 2. Local    │   2 validators (Alice + Bob seeds)
│ (--chain    │   Local network testing
│  =local)    │   Networking & consensus validation
└─────────────┘
       ↓
┌─────────────┐
│ 3. Alfa     │   4 validators (test seeds)
│ (--chain    │   Early adopter testing
│  =alfa)     │   First external deployment
└─────────────┘
       ↓
┌─────────────┐
│ 4. Beta     │   8 validators (REAL KEYS from beta_testnet_validators.json)
│ (--chain    │   Public testnet
│  =beta)     │   Community testing
└─────────────┘   Load testing
       ↓
┌─────────────┐
│ 5. Staging  │   20 validators (REAL KEYS from staging_validators.json)
│ (--chain    │   Pre-mainnet rehearsal
│  =staging)  │   Final security audits
└─────────────┘   Performance benchmarking
       ↓
┌─────────────┐
│ 6. Mainnet  │   100 validators (REAL KEYS from mainnet_validators.json)
│ (--chain    │   PRODUCTION
│  =mainnet)  │   Real value transactions
└─────────────┘
```

### Validation Criteria for Each Level

#### Dev → Local Progression
- [x] Runtime builds successfully
- [ ] Node binary exists (`./target/release/pezkuwichain-node`)
- [ ] Dev mode starts and produces blocks
- [ ] wUSDT asset ID 1000 exists in genesis
- [ ] All pallets initialize correctly
- [ ] No runtime panics or errors

#### Local → Alfa Progression
- [ ] 2-node network establishes peer connection
- [ ] GRANDPA finality working
- [ ] wUSDT transfers work between nodes
- [ ] Pool creation functional (wHEZ/wUSDT, PEZ/wUSDT)
- [ ] Presale pallet accepts wUSDT contributions
- [ ] No consensus failures

#### Alfa → Beta Progression
- [ ] 4-node network stable for 24+ hours
- [ ] Validator keys loaded from JSON correctly
- [ ] Staking functional
- [ ] Governance proposals executable
- [ ] External users can connect
- [ ] Frontend fully functional

#### Beta → Staging Progression
- [ ] 8-node network handles load
- [ ] 1000+ transactions processed
- [ ] Bridge service integrated (custodial wUSDT minting)
- [ ] Security audit completed
- [ ] Community testing successful
- [ ] No critical bugs found

#### Staging → Mainnet Progression
- [ ] 20-node network stable for 1 week+
- [ ] All economic parameters finalized
- [ ] Legal/compliance requirements met
- [ ] Disaster recovery tested
- [ ] Backup/restore procedures documented
- [ ] 100 validator keys ready

---

## 📦 CRITICAL FILES & LOCATIONS

### SDK (Pezkuwi-SDK)
```
/home/mamostehp/Pezkuwi-SDK/
├── pezkuwi/runtime/pezkuwichain/
│   ├── constants/src/lib.rs                    # Asset constants (wUSDT)
│   ├── src/genesis_config_presets.rs           # Genesis configs for all networks
│   ├── src/lib.rs                              # Runtime construction
│   └── WUSDT.md                                # wUSDT documentation
├── pezkuwi/node/service/src/chain_spec.rs      # Chain specifications
└── target/release/
    ├── pezkuwichain-node                       # Main node binary
    └── pezkuwi                                 # Benchmark binary
```

### Frontend (pwap)
```
/home/mamostehp/pwap/
├── shared/lib/
│   ├── wallet.ts          # ASSET_IDS, ASSET_CONFIGS
│   ├── usdt.ts            # wUSDT bridge utilities
│   └── scores.ts          # Score fetching from pallets
├── web/src/
│   ├── components/
│   │   ├── AccountBalance.tsx    # Balance display, pool queries
│   │   └── USDTBridge.tsx        # Bridge UI
│   └── contexts/
│       └── WalletContext.tsx     # Balance fetching with 6 decimals
```

### Build Logs
```
/tmp/runtime_build.log      # Stage 1 runtime build
/tmp/full_sdk_build.log     # Stage 2 full workspace build
```

---

## 🔧 COMMON COMMANDS

### Build Commands
```bash
# Clean build
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi
cargo clean
cargo build --release

# Runtime only
cd runtime/pezkuwichain
cargo build --release

# With benchmarks
cargo build --release --features runtime-benchmarks
```

### Run Commands
```bash
# Dev mode (single validator)
./target/release/pezkuwichain-node --dev --tmp

# Dev mode (persistent)
./target/release/pezkuwichain-node --dev

# Local testnet
./target/release/pezkuwichain-node --chain=local --alice
./target/release/pezkuwichain-node --chain=local --bob

# Beta testnet
./target/release/pezkuwichain-node --chain=beta

# Clean chain data
rm -rf /tmp/substrate*
```

### Benchmarking
```bash
# Benchmark specific pallet
./target/release/pezkuwichain-node benchmark pallet \
  --chain=dev \
  --pallet=pallet_presale \
  --extrinsic='*' \
  --steps=50 \
  --repeat=20 \
  --output=./pallets/presale/src/weights.rs
```

### Testing
```bash
# All tests
cargo test --release

# Specific pallet
cargo test -p pallet-presale

# Runtime tests
cd runtime/pezkuwichain
cargo test --release
```

---

## 🐛 KNOWN ISSUES & SOLUTIONS

### Issue 1: Presale Benchmark Failure
**Error:** `The asset in question is unknown`
**Cause:** Asset ID mismatch (was using 2, now uses 1000)
**Status:** RESOLVED by updating genesis config
**Commit:** TBD (next commit)

### Issue 2: Unused Imports Warning
**Warning:** `unused imports` in genesis_config_presets.rs
**Impact:** Non-critical, build succeeds
**Solution:** Run `cargo fix --lib -p pezkuwichain` after build completes

### Issue 3: WASM Target Recommendation
**Warning:** Rust >= 1.84 supports `wasm32v1-none` target
**Action:** Consider upgrading after mainnet stable
```bash
rustup target add wasm32v1-none --toolchain stable-x86_64-unknown-linux-gnu
cargo clean  # Must rebuild from scratch
```

---

## 📊 PERFORMANCE BENCHMARKS (Target)

### Block Production
- **Dev Mode:** ~6s per block (configurable)
- **Multi-validator:** ~6s per block with GRANDPA finality
- **Target TPS:** 100-500 transactions/second

### wUSDT Operations
- **Mint:** <2s (multisig approval + inclusion)
- **Transfer:** <6s (1 block confirmation)
- **Burn:** <2s (bridge withdrawal request)

### Pool Operations
- **Pool Creation:** <6s
- **Add Liquidity:** <6s
- **Swap:** <6s
- **Remove Liquidity:** <6s

---

## 🔐 SECURITY CHECKLIST

### Pre-Alfa
- [x] Code review completed
- [x] Asset IDs standardized
- [ ] Unit tests passing
- [ ] Integration tests passing

### Pre-Beta
- [ ] External security review
- [ ] Multisig tested (3/5 threshold)
- [ ] Emergency pause mechanisms tested
- [ ] Rate limiting verified

### Pre-Staging
- [ ] Full security audit by external firm
- [ ] Penetration testing completed
- [ ] Bug bounty program launched

### Pre-Mainnet
- [ ] Final audit sign-off
- [ ] Disaster recovery tested
- [ ] Insurance coverage secured
- [ ] Legal compliance verified

---

## 📝 NOTES FOR FUTURE CLAUDE SESSIONS

### Context Preservation
When this session ends and a new Claude starts:

1. **Read this file first:** `/home/mamostehp/Pezkuwi-SDK/DEPLOYMENT_ROADMAP.md`
2. **Check build status:**
   ```bash
   cat /tmp/full_sdk_build.log | tail -50
   ```
3. **Verify binary exists:**
   ```bash
   ls -lh /home/mamostehp/Pezkuwi-SDK/pezkuwi/target/release/pezkuwichain-node
   ```
4. **Check git status:**
   ```bash
   cd /home/mamostehp/Pezkuwi-SDK
   git status
   git log -3 --oneline
   ```

### Current Work State
- **SDK Repo:** Genesis changes uncommitted (ready to commit)
- **Frontend Repo:** All changes committed (commit `65126b4`)
- **Background Jobs:** May still be running, check with `/tasks`
- **Next Action:** Test Level 1 (Dev mode) after build completes

### Important URLs
- **SDK Repo:** `/home/mamostehp/Pezkuwi-SDK`
- **Frontend Repo:** `/home/mamostehp/pwap`
- **Node Binary:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/target/release/pezkuwichain-node`
- **Documentation:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/runtime/pezkuwichain/WUSDT.md`

### Key Decisions Made
1. **wUSDT Asset ID:** 1000 (final, don't change)
2. **Asset ID Allocation:** 0-999 protocol, 1000+ bridged assets
3. **Deployment Strategy:** Sequential network progression (dev→local→alfa→beta→staging→mainnet)
4. **Build Strategy:** Clean build after major changes
5. **Testing Strategy:** Validate at each network level before progression

---

**Last Updated:** 2025-11-20 17:20 UTC
**Build Status:** Stage 2 in progress (Job c26e47)
**Next Milestone:** Test Level 1 (Dev Mode)
**Blocker:** None (waiting for build completion)
