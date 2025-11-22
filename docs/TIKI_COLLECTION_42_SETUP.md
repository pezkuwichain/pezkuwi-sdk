# Tiki NFT Collection 42 Setup Guide

## Overview

The Tiki pallet uses NFT Collection #42 for citizenship and role management. This collection **must be created manually** after blockchain initialization before the citizenship system can function.

## Why Collection 42?

- Collection 42 is hardcoded in the Tiki pallet as `TikiCollectionId = 42`
- This collection stores all citizenship and role NFTs
- Must exist before any `selfConfirmCitizenship` or role assignment operations

## Setup Process

### Step 1: Start the Blockchain

```bash
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi
cargo build --release
./target/release/pezkuwi --dev --tmp
```

### Step 2: Create Collection 42

**Option A: Using the Automated Script (Recommended)**

A script is provided at `/home/mamostehp/pwap/scripts/create_collection_42.js` that:
- Checks current NextCollectionId
- Creates all necessary collections (1 through 42) if needed
- Handles edge cases (already created, already past 42, etc.)
- Provides clear progress output

```bash
# From any directory with @polkadot/api installed
node /home/mamostehp/pwap/scripts/create_collection_42.js [ws://127.0.0.1:9944]
```

**Option B: Manual Creation via Polkadot.js Apps**

1. Navigate to: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944
2. Go to Developer → Extrinsics
3. Select: `sudo` → `sudo(call)`
4. Inner call: `nfts` → `forceCreate(owner, config)`
5. Parameters:
   - owner: Select Alice (or admin account)
   - config: Use default settings
6. **Important**: You must create collections sequentially (1, 2, 3... up to 42) since `forceCreate` no longer accepts a collection_id parameter

### Step 3: Verify Collection Creation

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function verifyCollection42() {
  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  const nextId = await api.query.nfts.nextCollectionId();
  const currentId = nextId.isNone ? 0 : nextId.unwrap().toNumber();

  console.log(`Current NextCollectionId: ${currentId}`);

  if (currentId >= 43) {
    console.log('✅ Collection 42 exists!');
  } else {
    console.log('❌ Collection 42 not created yet.');
    console.log(`   Need to create ${43 - currentId} more collections.`);
  }

  await api.disconnect();
}

verifyCollection42();
```

## Script Details

The `/home/mamostehp/pwap/scripts/create_collection_42.js` script:

### Features
- ✅ Automatic detection of current NextCollectionId
- ✅ Creates only necessary collections to reach 42
- ✅ Handles all edge cases:
  - NextCollectionId already at 42 → Creates only Collection 42
  - NextCollectionId past 42 → Error (need fresh chain)
  - NextCollectionId below 42 → Creates all missing collections
- ✅ Clear progress output with emojis
- ✅ Proper error handling and transaction validation

### Usage Examples

```bash
# Using default endpoint (ws://127.0.0.1:9944)
node scripts/create_collection_42.js

# Using custom endpoint
node scripts/create_collection_42.js ws://remote-node:9944
```

### Expected Output

```
🔗 Connected to ws://127.0.0.1:9944

🎯 Target: Create NFT Collection #42 for Tiki citizenship system

📊 Current NextCollectionId: 1
📝 Need to create 42 collections (IDs 1 through 42)

   Creating Collection #1...
   ✓ Collection #1 created (placeholder)
   Creating Collection #2...
   ✓ Collection #2 created (placeholder)
   ...
   Creating Collection #42...
   ✅ Collection #42 created! 🎯 THIS IS THE TIKI COLLECTION!

🎉 Success! Collection 42 has been created and is ready for Tiki citizenship NFTs.
   You can now use the self-confirmation citizenship system.
```

## Post-Setup

Once Collection 42 is created:

1. ✅ Users can call `selfConfirmCitizenship()`
2. ✅ Citizen NFTs will be minted automatically
3. ✅ Dashboard will display "Citizen" role
4. ✅ Tiki Score system will function
5. ✅ Role-based features will activate

## Troubleshooting

### Error: "NextCollectionId is already past 42"

This means collections were created manually and skipped past 42. Solutions:
1. Start a fresh blockchain with `--dev --tmp`
2. Run the script immediately after chain initialization
3. OR: Adjust the `TikiCollectionId` constant in the Tiki pallet to match an available collection ID

### Error: "Cannot decode value"

This indicates the forceCreate API signature has changed. The current signature is:
```rust
forceCreate(owner: AccountIdLookupOf<T>, config: CollectionConfigFor<T>)
```

The script uses the correct format:
```javascript
api.tx.nfts.forceCreate(
  { Id: alice.address },
  config
)
```

### Frontend Error: "GET .../Dashboard.tsx 500 Error"

This was caused by missing `/pwap/shared/lib/kyc.ts` file. Fixed by creating:

```typescript
// /pwap/shared/lib/kyc.ts
export { getKycStatus } from './citizenship-workflow';
```

## Production Deployment

For production networks:

1. **Alpha/Beta/Staging**: Run `create_collection_42.js` once after genesis
2. **Main Net**: Include in deployment checklist:
   - [ ] Chain started
   - [ ] Collection 42 created
   - [ ] Verified via `nextCollectionId` query
   - [ ] Test citizenship confirmation
   - [ ] Monitor events for `tiki.CitizenshipConfirmed`

## Related Files

- Pallet source: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/tiki/src/lib.rs`
- Collection script: `/home/mamostehp/pwap/scripts/create_collection_42.js`
- Frontend lib: `/home/mamostehp/pwap/shared/lib/kyc.ts`
- Dashboard: `/home/mamostehp/pwap/web/src/pages/Dashboard.tsx`

## Support

If you encounter issues:
1. Check that the blockchain is running
2. Verify `@polkadot/api` is installed: `npm list @polkadot/api`
3. Check node WebSocket endpoint is accessible
4. Review script output for specific error messages
5. Verify Alice account has sudo permissions

---

Last Updated: November 18, 2025
Version: 1.0.0
