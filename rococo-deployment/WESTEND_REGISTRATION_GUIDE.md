# Westend Parachain Registration Guide
## ParaId 2246 - Pezkuwi Parachain

**Date**: 2025-11-21
**Network**: Westend Testnet
**ParaId**: 2246
**Account**: //ALICE (5Genwa...)
**WND Balance**: 199.5503 WND
**Reserved Deposit**: 177.2064 WND

---

## Problem: File Upload Timeout

The Polkadot.js Apps web interface times out when uploading large binary files (3.1 MB WASM + 6.1 MB genesis state).

## Solution: Use Polkadot.js Developer > Extrinsics with Hex Data

Instead of using the "register parathread" form, we'll submit the registration transaction manually using the Developer > Extrinsics interface, which accepts hex-encoded data directly.

---

## Step-by-Step Registration Process

### Step 1: Access Developer > Extrinsics

1. Go to: `https://polkadot.js.org/apps/?rpc=wss://westend-rpc.polkadot.io#/extrinsics`
2. Or navigate: Developer → Extrinsics

### Step 2: Select Registrar Extrinsic

In the extrinsic submission form:

1. **Account**: Select `//ALICE` (5Genwa...)
2. **Submit the following**: Select `registrar`
3. **Extrinsic**: Select `register(id, genesisHead, validationCode)`

### Step 3: Fill in Parameters

#### Parameter 1: `id` (ParaId)
```
2246
```

#### Parameter 2: `genesisHead` (Genesis State Hex)

**Option A - Using File Path (Best Method)**:
Copy the hex data from the file:

```bash
cat /home/mamostehp/Pezkuwi-SDK/rococo-deployment/pezkuwi-genesis-state.hex
```

Then:
1. Click the "file upload" toggle next to genesisHead field
2. Or manually paste: `0x` + [entire hex content from file]

**Important**: The hex file is **12,713,782 characters** long. You'll need to:
- Copy the entire content
- Ensure it starts with `0x`
- Paste into the genesisHead field

#### Parameter 3: `validationCode` (WASM Hex)

**Option A - Using File Path (Best Method)**:
Copy the hex data from the file:

```bash
cat /home/mamostehp/Pezkuwi-SDK/rococo-deployment/pezkuwi-runtime.wasm.hex
```

Then:
1. Click the "file upload" toggle next to validationCode field
2. Or manually paste: `0x` + [entire hex content from file]

**Important**: The hex file is **6,332,732 characters** long.

### Step 4: Submit Transaction

1. Click "Submit Transaction" button
2. **Sign and Submit** the transaction with //ALICE account
3. **Wait for confirmation** - this may take 1-2 minutes due to large data size

---

## Alternative Method: Using Sudo (If You Have Sudo Access)

If the registrar doesn't work or you have sudo access on Westend:

### Navigate to Developer > Sudo

1. Go to: `https://polkadot.js.org/apps/?rpc=wss://westend-rpc.polkadot.io#/sudo`
2. Select: `parasSudoWrapper` → `sudoScheduleParaInitialize`

### Fill Parameters:

```
id: 2246
genesisHead: 0x[paste hex from pezkuwi-genesis-state.hex]
validationCode: 0x[paste hex from pezkuwi-runtime.wasm.hex]
paraKind: true (parachain)
```

---

## How to Get Hex Data Easily

### Method 1: Direct File Read
Open terminal and run:

```bash
# For Genesis State (12.7 MB hex)
cat /home/mamostehp/Pezkuwi-SDK/rococo-deployment/pezkuwi-genesis-state.hex

# For WASM Runtime (6.3 MB hex)
cat /home/mamostehp/Pezkuwi-SDK/rococo-deployment/pezkuwi-runtime.wasm.hex
```

### Method 2: Copy to Clipboard (if xclip installed)
```bash
# Genesis State to clipboard
cat /home/mamostehp/Pezkuwi-SDK/rococo-deployment/pezkuwi-genesis-state.hex | xclip -selection clipboard

# WASM Runtime to clipboard
cat /home/mamostehp/Pezkuwi-SDK/rococo-deployment/pezkuwi-runtime.wasm.hex | xclip -selection clipboard
```

### Method 3: Create Smaller Preview Files
```bash
# Create first 100 chars preview
head -c 100 /home/mamostehp/Pezkuwi-SDK/rococo-deployment/pezkuwi-genesis-state.hex
```

---

## Expected Results After Submission

### Success Indicators:
1. Transaction appears in recent events
2. `registrar.Registered` event emitted with `para_id: 2246`
3. ParaId 2246 appears in Network → Parachains → Parathreads list
4. Status changes from "Onboarding" → "Parathread" → "Parachain"

### Timeline:
- **Transaction submission**: Immediate
- **Inclusion in block**: ~12 seconds (2 Westend blocks)
- **Parachain activation**: 2-4 minutes (depends on relay chain session)

---

## Troubleshooting

### If Hex Paste Doesn't Work:

**Problem**: Browser freezes or field doesn't accept large hex string
**Solution**: Try alternative RPC endpoints:

1. OnFinality RPC:
   ```
   wss://westend.api.onfinality.io/public-ws
   ```

2. IBP Network RPC:
   ```
   wss://rpc.ibp.network/westend
   ```

3. Run Local Polkadot.js Apps:
   ```bash
   git clone https://github.com/polkadot-js/apps
   cd apps
   yarn && yarn start
   # Opens at http://localhost:3000
   ```

### If Registration Fails:

1. **Check ParaId is still reserved**:
   - Go to Network → Parachains → Parathreads
   - Verify ParaId 2246 is listed under your account

2. **Check WND balance**:
   - Minimum ~200 WND needed for registration
   - You have 199.5503 WND - this should be sufficient

3. **Verify hex files are valid**:
   ```bash
   # Should output file sizes
   ls -lh /home/mamostehp/Pezkuwi-SDK/rococo-deployment/*.hex

   # Should start with valid hex characters
   head -c 20 /home/mamostehp/Pezkuwi-SDK/rococo-deployment/pezkuwi-genesis-state.hex
   ```

---

## Next Steps After Registration

Once registration succeeds, you'll need to:

### 1. Start Collator Node

```bash
cd /home/mamostehp/Pezkuwi-SDK

./target/release/pezkuwi \
  --collator \
  --name "Pezkuwi-Collator-1" \
  --base-path /tmp/pezkuwi-westend \
  --chain rococo-deployment/pezkuwi-westend-raw.json \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-external \
  --ws-external \
  --prometheus-external \
  --prometheus-port 9615 \
  -- \
  --chain westend \
  --port 30334 \
  --rpc-port 9945 \
  --execution wasm
```

### 2. Verify Node is Running

Check that:
- Relay chain is syncing
- Parachain blocks are being produced
- Connection to Westend relay chain is established

### 3. Open HRMP Channel to Asset Hub

After collator is running and producing blocks, establish HRMP channel with Asset Hub (Para 1000) to enable XCM transfers.

See `docs/XCM_ROCOCO_DEPLOYMENT.md` for complete HRMP setup instructions (same process for Westend).

---

## File Locations Reference

```
/home/mamostehp/Pezkuwi-SDK/rococo-deployment/
├── pezkuwi-genesis-state          # 6.1 MB - Binary genesis state
├── pezkuwi-genesis-state.hex      # 12.7 MB - Hex encoded genesis state ✅
├── pezkuwi-runtime.wasm           # 3.1 MB - Binary WASM runtime
├── pezkuwi-runtime.wasm.hex       # 6.3 MB - Hex encoded WASM runtime ✅
├── pezkuwi-westend-plain.json     # Human-readable chainspec
└── WESTEND_REGISTRATION_GUIDE.md  # This file
```

---

## Support Resources

- **Westend Polkadot.js Apps**: https://polkadot.js.org/apps/?rpc=wss://westend-rpc.polkadot.io
- **Parachain Registration Docs**: https://wiki.polkadot.network/docs/learn-parathreads
- **Asset Hub (Para 1000)**: https://wiki.polkadot.network/docs/learn-system-chains#asset-hub
- **XCM Documentation**: https://wiki.polkadot.network/docs/learn-xcm

---

**Summary**: Use Developer > Extrinsics > registrar.register() with hex data from files instead of the web form file upload to avoid timeout issues.
