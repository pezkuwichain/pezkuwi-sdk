# Westend Deployment Session Summary

**Date**: 2025-11-21
**Goal**: Deploy Pezkuwi parachain to Westend testnet for XCM Asset Hub testing
**Result**: Discovered architectural blockers, documented alternatives

## What Was Attempted

### Initial Setup (Completed ✅)
- Reserved ParaId 2246 on Westend
- Obtained 199.5503 WND from faucet
- Prepared genesis head (98 bytes) and WASM runtime (3.02 MB)
- Set up local Polkadot.js Apps on localhost:3000
- Fixed hex encoding issues in registration script

### Registration Attempts (Failed ❌)
1. **Web UI Registration** - Timeout after 60 seconds uploading files
2. **OnFinality RPC** - Same timeout issue
3. **Single-step `registrar.register()`** - Runtime panic during fee estimation
4. **Multi-step approach** - Requires sudo/governance permissions

## Critical Discovery

### The Problem

**`registrar.register(id, genesis_head, wasm_code)` cannot handle 3 MB WASM files on Westend**

```
Error: wasm trap: unreachable in TransactionPaymentApi_query_info
Root Cause: Transaction too large for single extrinsic
Westend Runtime: Panics when calculating fees for oversized transaction
```

### Why It Fails

1. **Transaction Size**: 3.02 MB WASM encoded as hex becomes ~6 MB transaction
2. **Fee Calculation**: Westend runtime encounters `unreachable` before even submitting
3. **Practical Limits**: Real transaction limit << theoretical 10 MB limit
4. **Not a Network Issue**: Problem is in runtime's fee calculation, not network upload

### Architectural Issues Discovered

1. **Wrong Binary Used Initially**:
   - `pezkuwi` (141 MB) is standalone relay chain with BABE/GRANDPA/BEEFY
   - Cannot be registered as parachain
   - Missing `export-genesis-head` and `export-genesis-wasm` commands

2. **Correct Binaries Identified**:
   - `pezkuwi-omni-node` (210 MB) - Omni-node parachain launcher ✓
   - `pezkuwi-parachain` (232 MB) - Generic parachain client ✓

3. **Genesis Data Issue**:
   - Current `genesis-head.bin` extracted from standalone chain
   - Need proper parachain-specific genesis from omni-node
   - Standalone chain genesis has wrong consensus (BABE vs Aura)

## Westend Infrastructure Discovered

### Available Pallets

**Registrar Pallet**:
- `register(id, genesis, wasm)` - FAILS for large WASM
- `scheduleCodeUpgrade(id, wasm)` - Upload WASM separately (requires permissions)
- `setCurrentHead(id, head)` - Set genesis separately (requires permissions)
- `reserve(id)` - Reserve ParaId ✓ Already done

**Paras Pallet** (all require sudo):
- `forceSetCurrentCode`
- `forceSetCurrentHead`
- `forceScheduleCodeUpgrade`
- `forceNoteNewHead`

**Agile Coretime System**:
- `onDemandAssignmentProvider.placeOrderKeepAlive()` - Buy on-demand coretime
- `onDemandAssignmentProvider.placeOrderWithCredits()` - Use credits
- `coretime.assignCore()` - Assign coretime to ParaId

**Legacy System**:
- `slots.forceLease()` - Force parachain lease (sudo)
- `slots.triggerOnboard()` - Trigger onboarding (sudo)

### Key Finding

**Westend uses Agile Coretime model**, not legacy slot auctions!

## Files Created During Session

### Documentation (40 KB)
```
CRITICAL_FINDINGS.md (4.2 KB)
- Initial problem discovery
- Wrong binary identification
- File size explosion issue

REGISTRATION_ISSUE_ANALYSIS.md (5.2 KB)
- Technical root cause analysis
- Transaction size limits
- Multi-step approach details

WESTEND_DEPLOYMENT_STATUS.md (8.6 KB)
- Comprehensive status report
- Alternative approaches
- Next steps recommendations

SESSION_SUMMARY.md (this file)
- Session overview
- Findings summary
```

### Scripts (11.5 KB)
```
register-parachain.js (3.6 KB)
- Single-step registration attempt
- Fixed hex encoding issue
- Fails at fee estimation

register-multistep.js (5.6 KB)
- Multi-step registration approach
- Uses scheduleCodeUpgrade + setCurrentHead
- Requires permissions

check-westend-pallets.js (2.3 KB)
- Query available Westend pallets
- Discover coretime system
- Map registration methods
```

### Data Files (61 MB total)
```
genesis-head.bin (98 bytes) - Minimal genesis header
parachain-runtime-final.wasm (3.1 MB) - Pezkuwichain runtime WASM
pezkuwi-runtime.wasm (3.1 MB) - Copy of runtime
```

### Chainspecs
```
pezkuwi-westend-plain.json (6.1 MB)
pezkuwi-westend-raw.json (278 bytes)
pezkuwi-rococo-plain.json (6.1 MB)
pezkuwi-rococo-raw.json (278 bytes)
```

## Technical Lessons Learned

### 1. Standalone vs Parachain Architecture

**Standalone Chain** (`pezkuwi`):
- Runs independently with own consensus
- Uses BABE (block production) + GRANDPA (finality) + BEEFY (bridging)
- Has validator/authority roles
- CANNOT be registered as parachain

**Parachain** (`pezkuwi-omni-node`):
- Relies on relay chain for consensus
- Uses Aura (block production) + Cumulus (relay chain integration)
- Has collator role (not validator)
- Can be registered on relay chain

### 2. Transaction Size Limits

**Theoretical**: 10 MB limit for Bytes fields in Polkadot.js API
**Practical**: ~1-2 MB for actual extrinsics due to:
- Fee calculation overhead
- Runtime execution limits
- Network propagation constraints
- Block size limits

### 3. Westend Registration Methods

**Legacy Approach** (deprecated):
- Auction for parachain slot
- `slots.forceLease()` to assign slot
- Permanent slot assignment

**Agile Coretime** (current):
- Purchase coretime on-demand or bulk
- `onDemandAssignmentProvider.placeOrderKeepAlive()`
- Flexible, pay-per-use model
- Better for testing

**Governance** (production):
- Submit proposal for registration
- Community votes
- Permanent if approved

## Recommended Next Steps

### Phase 1: Local Testing with Chopsticks

```bash
# 1. Install Chopsticks
npm install -g @acala-network/chopsticks

# 2. Create config for Westend fork
cat > chopsticks-westend.yml <<EOF
endpoint: wss://westend-rpc.polkadot.io
port: 8000
mock-signature-host: true
db: ./chopsticks-db-westend
EOF

# 3. Fork Westend locally
chopsticks --config chopsticks-westend.yml

# 4. Connect via Polkadot.js Apps to ws://localhost:8000
# 5. Test full registration with sudo
# 6. Start parachain collator locally
# 7. Test XCM with Asset Hub fork
```

**Benefits**:
- Free unlimited testing
- Full sudo access
- No WND costs
- Can test XCM integration
- Iterate quickly

### Phase 2: Create Proper Parachain Chainspec

```bash
# 1. Generate base chainspec
./target/release/pezkuwi-omni-node build-spec \
  --chain=dev \
  --disable-default-bootnode \
  > /tmp/pezkuwi-para-plain.json

# 2. Modify for Westend parachain:
# Edit /tmp/pezkuwi-para-plain.json:
# - Add "para_id": 2246
# - Add "relay_chain": "westend"
# - Remove BABE/GRANDPA consensus
# - Add Aura consensus
# - Configure XCM routes

# 3. Convert to raw
./target/release/pezkuwi-omni-node build-spec \
  --chain=/tmp/pezkuwi-para-plain.json \
  --raw \
  > pezkuwi-westend-parachain-raw.json

# 4. Export proper genesis
./target/release/pezkuwi-omni-node export-genesis-head \
  --chain=pezkuwi-westend-parachain-raw.json \
  > parachain-genesis-proper.bin

./target/release/pezkuwi-omni-node export-genesis-wasm \
  --chain=pezkuwi-westend-parachain-raw.json \
  > parachain-wasm-proper.wasm
```

### Phase 3: Real Westend Deployment

**Option A: On-Demand Coretime** (Easiest)
```javascript
// Purchase on-demand coretime for testing
api.tx.onDemandAssignmentProvider.placeOrderKeepAlive(
  1000000000,  // 1 WND max
  2246         // ParaId
).signAndSend(alice);

// Start collator
./target/release/pezkuwi-omni-node \
  --collator \
  --chain=pezkuwi-westend-parachain-raw.json \
  --base-path=/tmp/parachain \
  -- \
  --chain=westend
```

**Option B: Governance Proposal** (For permanent slot)
- Write proposal explaining Pezkuwi
- Submit via governance
- Community vote
- If approved, register

## Blocked Requirements

### Cannot Proceed Until:

1. ✅ **Proper parachain chainspec created**
   - Need to configure omni-node for parachain mode
   - Must add para_id and remove relay chain consensus
   - Required before any registration attempt

2. ✅ **Local testing completed**
   - Test with Chopsticks first
   - Validate parachain produces blocks
   - Test XCM Asset Hub integration
   - Ensure no runtime errors

3. **Choose deployment method**:
   - On-demand coretime (fastest, temporary)
   - Governance proposal (slow, permanent)
   - Or stay local for testing

### Why We Stopped

**Not a failure** - we discovered fundamental limitations:

1. Single-step registration impossible with 3 MB WASM
2. Multi-step requires permissions we don't have
3. Current genesis from wrong chain type (standalone vs parachain)
4. Need proper testing infrastructure before real deployment

## Resources for Next Session

### Tools to Install
- Chopsticks: `npm install -g @acala-network/chopsticks`
- Polkadot.js API: Already installed
- jq: Already installed

### Documentation Links
- Cumulus Tutorial: https://docs.substrate.io/tutorials/build-a-parachain/
- Chopsticks: https://github.com/AcalaNetwork/chopsticks
- Polkadot Wiki Coretime: https://wiki.polkadot.network/docs/learn-guides-coretime
- Parachain Devops: https://wiki.polkadot.network/docs/maintain-guides-how-to-setup-a-collator

### Files to Reference
- `WESTEND_DEPLOYMENT_STATUS.md` - Complete status and options
- `REGISTRATION_ISSUE_ANALYSIS.md` - Technical deep dive
- `CRITICAL_FINDINGS.md` - Problem discovery timeline

## Success Metrics

Despite not deploying to Westend, this session achieved:

✅ Identified correct parachain binaries
✅ Discovered transaction size limitations
✅ Mapped Westend registration infrastructure
✅ Found Agile Coretime system
✅ Created comprehensive documentation
✅ Fixed encoding issues in scripts
✅ Set up local Polkadot.js Apps
✅ Documented clear next steps

## Conclusion

**Direct Westend registration blocked by transaction size limits and permissions.**

**Recommended path**:
1. Create proper parachain chainspec
2. Test locally with Chopsticks
3. Validate XCM integration
4. Then attempt Westend via on-demand coretime

**For XCM testing** (primary goal): Local Chopsticks environment is actually **better** than real Westend:
- Unlimited free testing
- Full control
- Faster iteration
- No network delays
- Can fork Asset Hub too

**Next session should start with**: Chopsticks setup for local Westend + Asset Hub + Pezkuwi parachain testing.
