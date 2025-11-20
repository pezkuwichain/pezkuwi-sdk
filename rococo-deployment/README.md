# Pezkuwi Rococo Testnet Deployment Package

**Created**: 2025-11-21
**Status**: Ready for Rococo Deployment

## Contents

This directory contains all necessary files for deploying Pezkuwi parachain to Rococo testnet.

### Files

1. **pezkuwi-runtime.wasm** (3.1 MB)
   - Compressed WebAssembly runtime blob
   - SHA256: `c7fecd355778f8f7d244d499e29b8cfed61091089d0fde8bfa0790ee08bd87a1`
   - Used for parachain registration on Rococo relay chain

2. **pezkuwi-genesis-state** (6.1 MB)
   - Genesis state export for parachain initialization
   - SHA256: `da37c15b4a77624596e2f1814e3da5d0669dabfb2c2ee34996c0a2191cc880cb`
   - Contains initial blockchain state including:
     - ForeignAssets pallet configuration
     - XCM configuration for Asset Hub integration
     - wUSDT (Asset ID 1000) setup
     - Validator set
     - Balances and initial accounts

3. **pezkuwi-rococo-plain.json** (6.1 MB)
   - Human-readable chain specification
   - Network: Pezkuwi Development
   - Chain ID: `pezkuwichain_dev`
   - Protocol ID: `pezkuwi`
   - Token: HEZ (12 decimals)

4. **pezkuwi-rococo-raw.json** (278 bytes)
   - Raw chain specification for node startup
   - Contains encoded genesis configuration

## Deployment Steps

### 1. Get ROC Tokens

Visit the Rococo faucet to obtain testnet tokens:
- **Faucet**: https://paritytech.github.io/polkadot-testnet-faucet/
- **Required**: ~50 ROC for ParaId reservation and deposits

### 2. Reserve ParaId

1. Go to https://polkadot.js.org/apps/?rpc=wss://rococo-rpc.polkadot.io#/parachains/parathreads
2. Navigate to: Network → Parachains → Parathreads
3. Click "Reserve ParaId"
4. Sign transaction with account containing ROC tokens
5. Note your assigned ParaId (e.g., 2XXX)

### 3. Register Parachain

Using sudo or governance on Rococo:

```javascript
// Via Polkadot.js Apps → Developer → Extrinsics
parasSudoWrapper.sudoScheduleParaInitialize(
  id: 2XXX,  // Your reserved ParaId
  genesisHead: <paste hex from pezkuwi-genesis-state>,
  validationCode: <paste hex from pezkuwi-runtime.wasm>,
  paraKind: true  // true = parachain
)
```

### 4. Start Collator Node

```bash
cd /home/mamostehp/Pezkuwi-SDK

./target/release/pezkuwi \
  --collator \
  --name "Pezkuwi-Collator-1" \
  --base-path /tmp/pezkuwi-collator \
  --chain rococo-deployment/pezkuwi-rococo-raw.json \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-external \
  --ws-external \
  --prometheus-external \
  --prometheus-port 9615 \
  -- \
  --chain rococo \
  --port 30334 \
  --rpc-port 9945 \
  --execution wasm
```

### 5. Establish HRMP Channel with Asset Hub

Asset Hub Para ID: **1000**

#### Open Channel Request

```javascript
// On Pezkuwi → Polkadot.js Apps
xcmPallet.send(
  dest: { V3: { parents: 1, interior: Here } },
  message: {
    V3: [{
      Transact: {
        originKind: 'Native',
        requireWeightAtMost: { refTime: 1000000000, proofSize: 0 },
        call: {
          encoded: hrmp.hrmpInitOpenChannel(
            recipient: 1000,  // Asset Hub
            proposedMaxCapacity: 1000,
            proposedMaxMessageSize: 102400
          )
        }
      }
    }]
  }
)
```

#### Verify Channel Status

```bash
# Check on Relay Chain
polkadot.js query.hrmp.hrmpChannels(2XXX, 1000)
```

### 6. Test XCM Reserve Transfer

Once HRMP channel is active, test USDT → wUSDT transfer from Asset Hub.

See `docs/XCM_ROCOCO_DEPLOYMENT.md` for detailed testing instructions.

## Network Configuration

### Pezkuwi Parachain
- **Chain ID**: pezkuwichain_dev
- **Token**: HEZ
- **Decimals**: 12
- **SS58 Format**: 42
- **XCM Version**: 3

### XCM Asset Configuration
- **Foreign Asset**: wUSDT
- **Asset ID**: 1000
- **Decimals**: 6
- **Source**: Asset Hub USDT (Para 1000, Asset 1984)
- **Location**: `{ parents: 1, interior: X2[Parachain(1000), GeneralIndex(1984)] }`

### Required Pallets
- ✅ ForeignAssets (Instance2)
- ✅ XcmPallet
- ✅ Assets (Instance1)
- ✅ PoolAssets (Instance3)
- ✅ AssetConversion

## Pre-Deployment Checklist

- [x] Runtime compiled with XCM support
- [x] WASM runtime exported (3.1 MB)
- [x] Genesis state exported (6.1 MB)
- [x] Chain specifications generated
- [x] ForeignAssets pallet configured
- [x] XCM ForeignFungiblesTransactor configured
- [x] Asset Hub USDT location mapped
- [x] Local tests passed (10/10)
- [ ] ROC tokens obtained
- [ ] ParaId reserved on Rococo
- [ ] Parachain registered
- [ ] Collator node running
- [ ] HRMP channel with Asset Hub established
- [ ] XCM reserve transfer tested

## Resources

- **Rococo Polkadot.js**: https://polkadot.js.org/apps/?rpc=wss://rococo-rpc.polkadot.io
- **Asset Hub Info**: https://wiki.polkadot.network/docs/learn-system-chains#asset-hub
- **XCM Documentation**: https://wiki.polkadot.network/docs/learn-xcm
- **HRMP Guide**: https://wiki.polkadot.network/docs/learn-xcm-pallet#hrmp-channels
- **Deployment Guide**: `docs/XCM_ROCOCO_DEPLOYMENT.md`
- **Implementation Report**: `docs/XCM_IMPLEMENTATION_REPORT.md`

## Support

For questions or issues:
- GitHub: https://github.com/pezkuwichain/pezkuwi-sdk
- Check deployment documentation in `docs/` directory

---

**Next Steps**: Follow the deployment guide in `docs/XCM_ROCOCO_DEPLOYMENT.md` for complete instructions.
