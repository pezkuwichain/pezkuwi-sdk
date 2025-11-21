# Westend Parachain Deployment - Current Status

**Date**: 2025-11-21
**ParaId**: 2246 (Reserved)
**Balance**: 199.5503 WND (17.46 billion planck)

## Current Situation

### What We Accomplished ✅

1. **Reserved ParaId**: 2246 is reserved on Westend testnet
2. **Obtained WND tokens**: 199.5503 WND available for transactions
3. **Identified binaries**:
   - `pezkuwi` (141 MB) - Standalone relay chain (WRONG for parachain)
   - `pezkuwi-omni-node` (210 MB) - Correct parachain launcher
   - `pezkuwi-parachain` (232 MB) - Alternative parachain client
4. **Prepared files**:
   - `genesis-head.bin` (98 bytes) - Minimal genesis header
   - `parachain-runtime-final.wasm` (3.02 MB) - Pezkuwichain runtime
5. **Fixed encoding issue**: Updated registration script to use hex encoding
6. **Discovered Westend pallets**: Mapped available registration methods
7. **Setup local Polkadot.js Apps**: Running on localhost:3000

### What We Discovered ❌

1. **`registrar.register()` doesn't work for 3 MB WASM**:
   - Transaction too large for single extrinsic
   - Westend runtime panics during fee estimation
   - Error: `wasm trap: unreachable` in `TransactionPaymentApi_query_info`

2. **Available alternative methods**:
   - `registrar.scheduleCodeUpgrade(id, wasm)` - Upload WASM separately
   - `registrar.setCurrentHead(id, head)` - Set genesis head separately
   - `onDemandAssignmentProvider.placeOrderKeepAlive()` - Buy on-demand coretime
   - `coretime.assignCore()` - Assign coretime to ParaId
   - `slots.triggerOnboard()` - Trigger parachain onboarding

3. **Permission issue**: Most registrar methods require sudo/governance access on Westend

## Technical Analysis

### Why Registration Failed

```
Problem: registrar.register(2246, genesis_head, wasm_code) fails
Root Cause: 3 MB WASM file too large for single transaction
Error: Runtime panics when calculating fees for oversized transaction
Westend Limit: Practical limit much smaller than theoretical 10 MB
```

### Available Westend Pallets

**Parachain-related pallets discovered**:
- `registrar` - Parachain registration (requires permissions)
- `paras` - Parachain management (requires sudo)
- `parasShared`, `paraInclusion`, `paraInherent` - Internal parachains
- `parasDisputes`, `parasSlashing` - Consensus/security
- `parasSudoWrapper` - Sudo wrapper for paras
- `slots` - Parachain slot leasing (legacy auctions)
- `coretime` - Agile coretime system
- `onDemandAssignmentProvider` - On-demand coretime purchases
- `parameters` - System parameters

**Key finding**: Westend uses **Agile Coretime** model, not legacy auctions

## Files Created

### Documentation
- `CRITICAL_FINDINGS.md` - Initial problem discovery
- `REGISTRATION_ISSUE_ANALYSIS.md` - Technical root cause analysis
- `WESTEND_DEPLOYMENT_STATUS.md` - This file
- `WESTEND_REGISTRATION_GUIDE.md` - Manual registration instructions

### Scripts
- `register-parachain.js` - Single-step registration (FAILS with 3 MB WASM)
- `register-multistep.js` - Multi-step registration attempt (requires permissions)
- `extract-genesis-head.js` - Extract genesis from running node
- `check-westend-pallets.js` - Query available Westend pallets

### Data Files
- `genesis-head.bin` - 98 byte genesis header (from standalone chain)
- `genesis-head.hex` - Hex encoded genesis header
- `parachain-runtime-final.wasm` - 3.02 MB Pezkuwichain WASM runtime
- `pezkuwi-runtime.wasm` - Copy of runtime WASM
- `pezkuwi-westend-plain.json` - Westend chainspec (planned)
- `pezkuwi-westend-raw.json` - Raw Westend chainspec (planned)

## Recommended Next Steps

### Option 1: On-Demand Coretime (Easiest for Testing)

Westend uses Agile Coretime. To test parachain functionality:

```javascript
// 1. Purchase on-demand coretime
api.tx.onDemandAssignmentProvider.placeOrderKeepAlive(
  max_amount,  // Max WND to spend
  para_id      // 2246
).signAndSend(alice);

// 2. Start collator node (it will produce blocks when coretime assigned)
./target/release/pezkuwi-omni-node \
  --collator \
  --force-authoring \
  --chain=pezkuwi-westend-raw.json \
  --base-path=/tmp/parachain/alice \
  --port 40333 \
  --rpc-port 8844 \
  -- \
  --chain=westend \
  --port 30343 \
  --rpc-port 9977
```

**Pros**:
- No governance/sudo needed
- Can test XCM immediately
- Pay-per-block model

**Cons**:
- Costs WND per block
- Temporary (need to keep buying coretime)
- Still need proper parachain genesis

### Option 2: Local Testing with Chopsticks (Recommended First)

Test everything locally before Westend deployment:

```bash
# 1. Install Chopsticks
npm install -g @acala-network/chopsticks

# 2. Fork Westend locally
chopsticks \
  --endpoint wss://westend-rpc.polkadot.io \
  --port 8000

# 3. Test parachain registration on fork
# 4. Test XCM with Asset Hub on fork
# 5. Once working, deploy to real Westend
```

**Pros**:
- Free testing
- Full control (can sudo anything)
- Can test XCM with Asset Hub
- No risk of wasting WND

**Cons**:
- Local only
- Need to understand Chopsticks setup

### Option 3: Governance Proposal (Production Approach)

Submit governance proposal to Westend for full registration:

1. Create forum post explaining Pezkuwi parachain
2. Submit `registrar.scheduleCodeUpgrade` via governance
3. Community votes on proposal
4. If approved, parachain registers

**Pros**:
- Proper permanent registration
- Community involvement

**Cons**:
- Takes time (days/weeks)
- Requires community approval
- May not be accepted for test project

### Option 4: Request Sudo/Governance Access

Contact Parity/Web3 Foundation:

1. Explain Pezkuwi project and XCM testing goals
2. Request temporary sudo access or sponsored registration
3. If granted, use `parasSudoWrapper` to force registration

**Pros**:
- Direct registration possible
- Could get official support

**Cons**:
- Unlikely to be granted for test project
- May take long time to respond

## Immediate Recommendations

**For XCM Testing** (Primary Goal):

1. ✅ **Use Chopsticks first**:
   - Fork Westend locally
   - Register Pezkuwi parachain on fork
   - Test XCM Asset Hub integration thoroughly
   - Document any issues

2. **If local test succeeds**:
   - Try on-demand coretime on real Westend
   - Purchase small amount of coretime
   - Test actual XCM transactions

3. **If everything works**:
   - Consider governance proposal for permanent slot
   - Or continue using on-demand model

**For Production Deployment**:

1. Wait for proper parachain chainspec from `pezkuwi-omni-node`
2. Test extensively on local testnet
3. Deploy to Rococo first (easier testnet)
4. Only then attempt Westend/Kusama/Polkadot

## What's Missing

### Critical Missing Piece: Proper Parachain Chainspec

Current genesis-head.bin is from **standalone relay chain**, not parachain!

**Need to create**:

```bash
# 1. Build dev chainspec
./target/release/pezkuwi-omni-node build-spec \
  --chain=dev \
  --disable-default-bootnode \
  > /tmp/pezkuwi-para-dev.json

# 2. Modify for parachain:
# - Add "para_id": 2246
# - Remove BABE/GRANDPA consensus
# - Add cumulus consensus
# - Configure XCM

# 3. Convert to raw
./target/release/pezkuwi-omni-node build-spec \
  --chain=/tmp/pezkuwi-para-dev-modified.json \
  --raw \
  > pezkuwi-westend-raw.json

# 4. Export proper parachain genesis
./target/release/pezkuwi-omni-node export-genesis-head \
  --chain=pezkuwi-westend-raw.json \
  > parachain-genesis-head-proper.bin

# 5. Export WASM (should match existing)
./target/release/pezkuwi-omni-node export-genesis-wasm \
  --chain=pezkuwi-westend-raw.json \
  > parachain-runtime-proper.wasm
```

**Why this matters**:
- Current genesis is from standalone chain (has BABE/GRANDPA)
- Parachains use different consensus (Cumulus/Aura)
- Wrong genesis will cause validation failures
- Collator won't be able to produce blocks

## Resources

- **Westend Apps**: https://polkadot.js.org/apps/?rpc=wss://westend-rpc.polkadot.io
- **Local Apps**: http://localhost:3000
- **Westend Faucet**: https://faucet.polkadot.io/westend
- **Polkadot Wiki Coretime**: https://wiki.polkadot.network/docs/learn-guides-coretime
- **Cumulus Tutorial**: https://docs.substrate.io/tutorials/build-a-parachain/
- **Chopsticks**: https://github.com/AcalaNetwork/chopsticks

## Conclusion

**We CANNOT directly register on Westend using `registrar.register()` due to transaction size limits.**

**Best path forward**:

1. Create proper parachain chainspec with `pezkuwi-omni-node`
2. Test locally with Chopsticks (fork Westend + Asset Hub)
3. Validate XCM integration works perfectly
4. Then choose deployment method:
   - On-demand coretime (easiest for testing)
   - Governance proposal (for permanent slot)
   - Or stay on local testnet

**Next session should focus on**: Creating proper parachain chainspec and setting up Chopsticks for local XCM testing.
