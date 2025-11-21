# Westend Registration Issue - Technical Analysis

**Date**: 2025-11-21
**ParaId**: 2246 (Reserved)
**Balance**: 199.5503 WND (~17.46 billion planck)

## Problem Summary

Attempted to register Pezkuwi parachain on Westend using `registrar.register()` extrinsic but encountered transaction size limitations.

## Technical Details

### Files Prepared
- **Genesis Head**: `genesis-head.bin` (98 bytes) ✓
- **WASM Runtime**: `parachain-runtime-final.wasm` (3.02 MB, 3,166,366 bytes) ✓
- Both files are within theoretical 10 MB limit

### Error Encountered

```
4003: Client error: Execution failed: Execution aborted due to trap: wasm trap: unreachable
WASM backtrace:
  TransactionPaymentApi_query_info
```

**Root Cause**: The fee estimation itself fails because the transaction is too large. Westend's runtime panics when trying to calculate fees for a 3 MB WASM upload in a single extrinsic.

## Why registrar.register() Fails

1. **Transaction Size**: Even though the WASM is 3.02 MB (within 10 MB limit), it's too large for a single extrinsic
2. **Fee Calculation Panic**: Westend runtime encounters `unreachable` when estimating fees for this size
3. **Network Limits**: Practical transaction size limit is much smaller than theoretical 10 MB

## Modern Westend Registration Process

According to current Polkadot/Westend practices:

### Option 1: Agile Coretime (Recommended for Westend)
Westend uses Agile Coretime model as of Polkadot 1.0:

1. **Reserve ParaId**: Already done ✓ (ParaId 2246)
2. **Purchase Coretime**: Use broker pallet to buy coretime
3. **Assign Coretime**: Assign purchased coretime to ParaId 2246
4. **Upload Code**: Use `paras.setCodeByHash()` or similar (not `registrar.register()`)

### Option 2: Manual Multi-Step Process
If registrar still accessible:

1. **Reserve ParaId**: ✓ Done
2. **Submit Genesis Head**: `paras.addGenesisHead(id, genesis_head)`
3. **Submit WASM Code**: `paras.setValidationCode(id, wasm_code)`
4. **Initiate Onboarding**: `paras.onboard(id)`

### Option 3: Use Chopsticks for Local Testing
Test parachain functionality locally first:
- Use Chopsticks to fork Westend
- Test XCM integration with Asset Hub
- Deploy to actual Westend later

## Files Ready

```
rococo-deployment/
├── genesis-head.bin (98 bytes) - Minimal parachain genesis head
├── parachain-runtime-final.wasm (3.02 MB) - Pezkuwichain runtime
├── register-parachain.js - Automated registration script (hex encoding fixed)
└── extract-genesis-head.js - Genesis extraction from local node
```

## Available Binaries

1. **pezkuwi** (141 MB) - Standalone relay chain (WRONG for parachain)
2. **pezkuwi-omni-node** (210 MB) - Omni-node parachain launcher (CORRECT)
3. **pezkuwi-parachain** (232 MB) - Generic parachain client (alternative)

## Next Steps

### Immediate Actions Needed

1. **Verify Current Westend Registration Method**
   - Check if Westend uses Coretime or legacy registrar
   - Consult Polkadot SDK examples
   - Review Westend governance forum

2. **Alternative Registration Approaches**
   - Try manual multi-step: addGenesisHead → setValidationCode → onboard
   - Use scheduler to upload WASM in chunks (if supported)
   - Submit governance proposal for larger transaction limits

3. **Local Testing First**
   - Use Chopsticks to fork Westend locally
   - Test full parachain functionality
   - Validate XCM Asset Hub integration
   - Only deploy to real Westend once tested

## Architecture Notes

### Why We Can't Use Standalone Chain Data

- `pezkuwi` binary is a standalone relay chain with BABE/GRANDPA/BEEFY
- Standalone chains cannot be registered as parachains
- Even though we extracted "genesis head", it's from wrong chain type
- Need proper parachain-specific genesis from `pezkuwi-omni-node`

### Correct Parachain Setup

```bash
# 1. Create parachain chainspec
./target/release/pezkuwi-omni-node build-spec \
  --chain=dev \
  --disable-default-bootnode \
  > /tmp/pezkuwi-para-plain.json

# 2. Modify for parachain (add para_id, remove relay chain consensus)
# Edit /tmp/pezkuwi-para-plain.json

# 3. Convert to raw
./target/release/pezkuwi-omni-node build-spec \
  --chain=/tmp/pezkuwi-para-plain.json \
  --raw \
  > pezkuwi-para-raw.json

# 4. Export genesis head
./target/release/pezkuwi-omni-node export-genesis-head \
  --chain=pezkuwi-para-raw.json \
  > parachain-genesis-head.bin

# 5. Export WASM
./target/release/pezkuwi-omni-node export-genesis-wasm \
  --chain=pezkuwi-para-raw.json \
  > parachain-runtime.wasm
```

## Resources

- Polkadot Wiki: https://wiki.polkadot.network/docs/learn-guides-coretime
- Westend Faucet: https://faucet.polkadot.io/westend
- Westend Apps: https://polkadot.js.org/apps/?rpc=wss://westend-rpc.polkadot.io
- Local Apps: http://localhost:3000

## Conclusion

**The `registrar.register()` approach is not viable for 3 MB WASM files on Westend.**

We need to either:
1. Use Westend's Agile Coretime system (if available)
2. Use multi-step manual upload process
3. Test locally with Chopsticks first
4. Wait for/request transaction size limit increase

**Recommended Path**: Set up proper parachain chainspec with `pezkuwi-omni-node`, test locally with Chopsticks, then investigate Westend's current coretime/registration process.
