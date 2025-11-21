# Westend Parachain Deployment - Critical Findings

**Date**: 2025-11-21
**ParaId**: 2246 (Reserved on Westend)
**WND Balance**: 199.5503 WND

## PROBLEM DISCOVERED

During deployment attempt, we discovered we were using the **WRONG BINARY**:

### What We Used (INCORRECT):
- Binary: `/target/release/pezkuwi` (141 MB)
- Type: **Standalone Relay Chain Node**
- Features: BABE, GRANDPA, BEEFY consensus (relay chain features)
- Issue: **CANNOT be registered as a parachain** on Westend

### Root Cause of Errors:
1. **File Size Issue**: When reading the standalone chain's "genesis state" (which was actually a JSON chainspec), it became 223 MB when hex-encoded
2. **Wrong Architecture**: Trying to register a relay chain as a parachain is fundamentally impossible
3. **Missing Commands**: The standalone binary doesn't have `export-genesis-head` (parachain-specific command)

## CORRECT BINARIES FOUND

### Option 1: pezkuwi-omni-node (RECOMMENDED)
- Path: `/target/release/pezkuwi-omni-node` (210 MB)
- Type: Omni-node pattern parachain launcher
- Features:
  - Has `export-genesis-head` command ✓
  - Has `export-genesis-wasm` command ✓
  - Can run parachains with custom WASM runtime ✓
- Status: Built and ready

### Option 2: pezkuwi-parachain
- Path: `/target/release/pezkuwi-parachain` (232 MB)
- Type: Generic parachain client (like polkadot-parachain)
- Purpose: Runs Polkadot system chains (Asset Hub, Bridge Hub, etc.)
- Issue: Requires pre-configured chainspecs (all empty in our case)

## FILES WE HAVE

### Correct Runtime WASM:
```
/target/release/wbuild/pezkuwichain/pezkuwichain.compact.compressed.wasm
Size: 3.02 MB
SHA256: c7fecd355778f8f7d244d499e29b8cfed61091089d0fde8bfa0790ee08bd87a1
```

This is the **correct parachain runtime** that needs to be registered.

## NEXT STEPS REQUIRED

### Step 1: Build Parachain Chainspec
The omni-node needs a proper chainspec. We need to create it using the standalone chain's build-spec as a template:

```bash
# Generate plain chainspec from standalone
./target/release/pezkuwi build-spec --disable-default-bootnode --chain=dev > /tmp/pezkuwi-plain.json

# Modify it for parachain use (remove relay chain consensus, add parachain ID)
# Then convert to raw
./target/release/pezkuwi-omni-node build-spec --chain=/tmp/pezkuwi-plain-modified.json --raw > pezkuwi-parachain-raw.json
```

### Step 2: Export Genesis Head and WASM
```bash
./target/release/pezkuwi-omni-node export-genesis-head \
  --chain=pezkuwi-parachain-raw.json \
  --raw > parachain-genesis-head.bin

./target/release/pezkuwi-omni-node export-genesis-wasm \
  --chain=pezkuwi-parachain-raw.json > parachain-runtime.wasm
```

### Step 3: Register on Westend
Use the exported files with the registration script (register-parachain.js)

## ALTERNATIVE APPROACH

Since the runtime WASM already exists and is correct, we could:

1. Use the existing WASM: `pezkuwi-runtime.wasm` (already copied to rococo-deployment/)
2. Create a minimal genesis head using a script
3. Register directly

## FILES READY FOR USE

```
rococo-deployment/
├── pezkuwi-runtime.wasm (3.1 MB) - Correct WASM ✓
├── genesis-head.bin (98 bytes) - From standalone chain (might work)
├── register-parachain.js - Registration script ✓
└── WESTEND_REGISTRATION_GUIDE.md - Manual instructions ✓
```

## CRITICAL DECISION NEEDED

**Option A**: Take time to properly configure omni-node chainspec (correct but complex)

**Option B**: Try using the existing WASM + minimal genesis head (quick but might fail)

**Option C**: Use Asset Hub or Bridge Hub chainspec as template and modify for Pezkuwi

## TECHNICAL NOTES

- Westend registrar expects: `registrar.register(id, genesisHead, validationCode)`
- Genesis head size limit: ~10 MB
- WASM size limit: ~10 MB
- Our WASM: 3.02 MB ✓ (within limit)
- Our genesis head: 98 bytes ✓ (within limit)

## RESOURCES

- Polkadot.js Apps (local): http://localhost:3000
- Westend RPC: wss://westend.api.onfinality.io/public-ws
- ParaId: 2246
- Account: //Alice (5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)

---

**RECOMMENDATION**: In next session, create proper parachain chainspec using omni-node, then proceed with registration.
