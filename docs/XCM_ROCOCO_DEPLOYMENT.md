# XCM Rococo Testnet Deployment Guide

## Overview
This guide covers deploying Pezkuwi parachain to Rococo testnet and testing Asset Hub USDT → wUSDT XCM transfers.

## Prerequisites

### 1. Completed Implementation
- ✅ ForeignAssets pallet (Instance2) configured
- ✅ XCM config with ForeignFungiblesTransactor
- ✅ Asset Hub USDT location mapping
- ✅ wUSDT (Asset ID 1000) in genesis
- ✅ All local tests passing (10/10)

### 2. Required Tools
- Polkadot binary (for relay chain)
- Pezkuwi collator node
- Polkadot.js Apps UI
- ROC tokens (Rococo testnet)

## Deployment Steps

### Phase 1: Parachain Registration on Rococo

#### Step 1: Generate Parachain Files

```bash
cd /home/mamostehp/Pezkuwi-SDK

# Build release binary with XCM support
cargo build --release --bin pezkuwi

# Export WASM runtime
./target/release/pezkuwi export-genesis-wasm --chain rococo-local > pezkuwi-wasm

# Export genesis state
./target/release/pezkuwi export-genesis-state --chain rococo-local > pezkuwi-genesis
```

#### Step 2: Reserve Parachain ID
- Go to Rococo Polkadot.js Apps: https://polkadot.js.org/apps/?rpc=wss://rococo-rpc.polkadot.io#/parachains/parathreads
- Navigate to Network → Parachains → Parathreads
- Click "Reserve ParaId"
- Sign with account containing ROC tokens
- Note the assigned ParaId (e.g., 2XXX)

#### Step 3: Register Parachain
Using sudo or through governance:

```javascript
// Via Polkadot.js Apps → Developer → Extrinsics
// Select: paras.registerPara(id, genesisHead, validationCode)

parasSudoWrapper.sudoScheduleParaInitialize(
  id: 2XXX, // Your ParaId
  genesisHead: <paste pezkuwi-genesis hex>,
  validationCode: <paste pezkuwi-wasm hex>,
  paraKind: true // true for parachain
)
```

### Phase 2: Collator Setup

#### Start Collator Node

```bash
./target/release/pezkuwi \
  --collator \
  --name "Pezkuwi-Collator-1" \
  --base-path /tmp/pezkuwi-collator \
  --chain rococo-local \
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

#### Verify Collator Status
```bash
# Check if producing blocks
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944

# Expected: {"isSyncing": false, "peers": X, "shouldHavePeers": true}
```

### Phase 3: HRMP Channel Setup with Asset Hub

HRMP (Horizontally Relay-routed Message Passing) channels are required for XCM communication between parachains.

#### Channel Configuration
- **Asset Hub Para ID**: 1000
- **Pezkuwi Para ID**: 2XXX (your assigned ID)
- **Max Message Size**: 102400 bytes
- **Max Capacity**: 1000 messages

#### Open Channel Request

```javascript
// On Pezkuwi → Polkadot.js Apps → Developer → Extrinsics
// Using sudo or governance

xcmPallet.send(
  dest: { V3: { parents: 1, interior: Here } }, // To Relay Chain
  message: {
    V3: [
      {
        Transact: {
          originKind: 'Native',
          requireWeightAtMost: { refTime: 1000000000, proofSize: 0 },
          call: {
            encoded: hrmp.hrmpInitOpenChannel(
              recipient: 1000, // Asset Hub
              proposedMaxCapacity: 1000,
              proposedMaxMessageSize: 102400
            )
          }
        }
      }
    ]
  }
)
```

#### Accept Channel from Asset Hub

Asset Hub must accept the channel. This requires:
1. Governance proposal on Asset Hub OR
2. Direct acceptance if Asset Hub has sudo

```javascript
// On Asset Hub side
hrmp.hrmpAcceptOpenChannel(sender: 2XXX) // Your ParaId
```

#### Verify Channel Status

```bash
# On Relay Chain
# Check HRMP channels
polkadot.js query.hrmp.hrmpChannels(2XXX, 1000)
# Should return channel configuration if established
```

### Phase 4: XCM Reserve Transfer Test

#### Test 1: Verify ForeignAssets Pallet

```javascript
// Connect to Pezkuwi collator
const api = await ApiPromise.create({
  provider: new WsProvider('wss://your-collator-endpoint')
});

// Check ForeignAssets pallet
const hasForeignAssets = api.query.foreignAssets !== undefined;
console.log('ForeignAssets present:', hasForeignAssets);
```

#### Test 2: Reserve Transfer from Asset Hub

**On Asset Hub:**

```javascript
// Using Alice account with USDT balance
const dest = {
  V3: {
    parents: 1,
    interior: { X1: { Parachain: 2XXX } } // Your Pezkuwi ParaId
  }
};

const beneficiary = {
  V3: {
    parents: 0,
    interior: {
      X1: {
        AccountId32: {
          network: null,
          id: '0x...' // Your account on Pezkuwi
        }
      }
    }
  }
};

const assets = {
  V3: [
    {
      id: {
        Concrete: {
          parents: 0,
          interior: {
            X2: [
              { PalletInstance: 50 }, // pallet_assets instance
              { GeneralIndex: 1984 }   // USDT asset ID
            ]
          }
        }
      },
      fun: { Fungible: 10000000 } // 10 USDT (6 decimals)
    }
  ]
};

const feeAssetItem = 0;

await api.tx.polkadotXcm.limitedReserveTransferAssets(
  dest,
  beneficiary,
  assets,
  feeAssetItem,
  { Unlimited: null }
).signAndSend(alice);
```

#### Test 3: Verify wUSDT Balance on Pezkuwi

```javascript
// On Pezkuwi parachain
const account = '0x...'; // Beneficiary account

// Check ForeignAssets balance
const balance = await api.query.foreignAssets.account(1000, account);

if (balance.isSome) {
  const { balance: amount } = balance.unwrap();
  console.log('wUSDT Balance:', amount.toHuman());
  // Expected: 10 USDT = 10,000,000 (6 decimals)
} else {
  console.log('No wUSDT balance yet');
}
```

### Phase 5: Monitoring and Debugging

#### Monitor XCM Messages

**On Relay Chain:**
```bash
# Watch for incoming XCM messages
polkadot.js query.xcmPallet.queries()
```

**On Pezkuwi:**
```bash
# Check XCM execution events
# Filter for: xcmPallet.Attempted, xcmPallet.Sent, xcmPallet.AssetsTrapped

curl -X POST http://localhost:9944 -H "Content-Type: application/json" -d '{
  "id":1,
  "jsonrpc":"2.0",
  "method":"state_getStorage",
  "params":["0x..."]
}'
```

#### Common Issues and Solutions

**Issue 1: HRMP Channel Not Established**
```
Error: No HRMP channel between parachains
```
Solution:
- Verify channel was accepted by Asset Hub
- Check relay chain for pending channel requests
- Ensure sufficient deposit for channel opening

**Issue 2: XCM Message Failed**
```
xcmPallet.Attempted { outcome: Incomplete(weight, XcmError::...) }
```
Solutions:
- Check `AssetTransactors` configuration in `xcm_config.rs`
- Verify location mapping for Asset Hub USDT
- Ensure ForeignAssets pallet has correct configuration

**Issue 3: Asset Not Received**
```
Balance remains 0 after transfer
```
Debugging:
```bash
# Check if asset was trapped
api.query.xcmPallet.assetTraps()

# Check XCM execution outcome
api.query.system.events() // Look for xcmPallet events
```

## Testing Checklist

Before live deployment, verify:

- [ ] Runtime compiled with ForeignAssets support
- [ ] XCM configuration includes ForeignFungiblesTransactor
- [ ] Asset Hub USDT location correctly mapped
- [ ] wUSDT genesis configuration present
- [ ] ParaId reserved on Rococo
- [ ] WASM and genesis files generated
- [ ] Collator node synced with relay chain
- [ ] HRMP channel established with Asset Hub (1000)
- [ ] Test account has ROC tokens for fees
- [ ] Asset Hub test account has USDT tokens

## Success Criteria

✅ Parachain registered on Rococo with ParaId 2XXX
✅ Collator producing blocks
✅ HRMP channel active: Pezkuwi ↔ Asset Hub
✅ Reserve transfer from Asset Hub successful
✅ wUSDT balance increased on Pezkuwi
✅ XCM events showing successful execution

## Next Steps After Successful Deployment

1. **Asset Conversion Pool**: Create HEZ/wUSDT liquidity pool
2. **Frontend Integration**: Update dApp to support wUSDT
3. **Presale Integration**: Enable wUSDT payments in presale pallet
4. **Monitoring**: Set up alerting for XCM failures
5. **Mainnet Preparation**: Document deployment for Polkadot mainnet

## Resources

- Rococo Faucet: https://paritytech.github.io/polkadot-testnet-faucet/
- Polkadot.js Apps (Rococo): https://polkadot.js.org/apps/?rpc=wss://rococo-rpc.polkadot.io
- Asset Hub Info: https://wiki.polkadot.network/docs/learn-system-chains#asset-hub
- XCM Documentation: https://wiki.polkadot.network/docs/learn-xcm
- HRMP Guide: https://wiki.polkadot.network/docs/learn-xcm-pallet#hrmp-channels

## Contact

For support with Rococo deployment:
- GitHub Issues: https://github.com/pezkuwichain/pezkuwi-sdk/issues
- Discord: [Your Discord Link]
- Telegram: [Your Telegram Link]

---

**Document Version**: 1.0
**Last Updated**: 2025-11-21
**Status**: Ready for Rococo Testnet Deployment
