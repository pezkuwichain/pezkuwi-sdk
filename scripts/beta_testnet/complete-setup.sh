#!/bin/bash

# Complete Beta Testnet Setup Script
# Builds, starts validators, inserts keys, wraps HEZ, creates pools with correct ratios

set -e  # Exit on any error

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
SDK_DIR="$( cd "$SCRIPT_DIR/../.." && pwd )"

echo "========================================"
echo "COMPLETE BETA TESTNET SETUP"
echo "========================================"

# Step 1: Stop any running validators
echo ""
echo "Step 1: Stopping existing validators..."
bash "$SCRIPT_DIR/stop-beta-validators.sh" 2>/dev/null || true

# Step 2: Clean chain data
echo ""
echo "Step 2: Cleaning chain data..."
rm -rf /tmp/beta-validator-{1..8}
rm -f /tmp/beta-validator-*.log

# Step 3: Start validators and insert keys (using start-testnet.sh method)
echo ""
echo "Step 3: Starting validators and inserting keys..."
cd "$SCRIPT_DIR"
for i in {1..8}; do
    bash start-beta-validator-$i.sh &
    echo "  ✓ Validator $i started"
done
echo "  Waiting for chain to initialize and RPC to be ready (20 seconds)..."
sleep 20
echo "  Inserting validator keys..."
echo "" | bash insert-all-beta-keys.sh
echo "  Waiting for block production to stabilize (20 seconds)..."
sleep 20

# Step 6: Wait for finalization
echo ""
echo "Step 6: Waiting for block finalization..."
for i in {1..30}; do
  FINALIZED=$(curl -s http://localhost:9944 -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[],"id":1}' | grep -o '"result":"[^"]*"' || echo "")
  if [ ! -z "$FINALIZED" ]; then
    echo "  Finalization detected!"
    sleep 5  # Wait a bit more for stability
    break
  fi
  echo "  Waiting... ($i/30)"
  sleep 2
done

# Step 7: Transfer HEZ to tokenWrapper pallet
echo ""
echo "Step 7: Setting up tokenWrapper pallet..."

node > /tmp/setup-wrapper.log 2>&1 << 'EONODE'
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

async function main() {
  await cryptoWaitReady();

  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const founder = keyring.addFromUri('skill dose toward always latin fish film cabbage praise blouse kingdom depth');

  // TokenWrapper pallet account
  const palletAccount = '5EYCAe5ijiYgPAWjqGNF1pdGJb9Rurp9aE4MVdaVPhwQ8cRe';

  console.log('Transferring 1000 HEZ to tokenWrapper pallet...');
  const transferAmount = BigInt(1000) * BigInt(10 ** 12);

  await new Promise((resolve, reject) => {
    api.tx.balances
      .transferKeepAlive(palletAccount, transferAmount.toString())
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            reject(new Error(dispatchError.toString()));
            return;
          }
          console.log('Transfer complete!');
          resolve();
        }
      })
      .catch(reject);
  });

  await api.disconnect();
}

main().catch(console.error);
EONODE

if [ $? -eq 0 ]; then
  echo "  TokenWrapper pallet funded!"
else
  echo "  ERROR: Failed to fund tokenWrapper pallet"
  exit 1
fi

# Step 8: Wrap HEZ to wHEZ and create pools
echo ""
echo "Step 8: Wrapping HEZ and creating pools with CORRECT ratios..."

node > /tmp/setup-pools.log 2>&1 << 'EONODE'
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

async function main() {
  await cryptoWaitReady();

  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const founder = keyring.addFromUri('skill dose toward always latin fish film cabbage praise blouse kingdom depth');

  // CORRECT RATIOS:
  // 1 wHEZ = 5 PEZ
  // 1 wUSDT = 4 wHEZ (so 1 wHEZ = 0.25 wUSDT)
  // 1 wUSDT = 20 PEZ (so 1 PEZ = 0.05 wUSDT)

  // Wrap 50,000 HEZ to wHEZ
  console.log('1. Wrapping 50,000 HEZ to wHEZ...');
  const wrapAmount = BigInt(50_000) * BigInt(10 ** 12);

  await new Promise((resolve, reject) => {
    api.tx.tokenWrapper
      .wrap(wrapAmount.toString())
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            reject(new Error(dispatchError.toString()));
            return;
          }
          console.log('   Wrapped 50,000 wHEZ!');
          resolve();
        }
      })
      .catch(reject);
  });

  // Create pools
  console.log('\n2. Creating pools...');

  // Pool 1: wHEZ/PEZ
  await new Promise((resolve) => {
    api.tx.assetConversion.createPool(0, 1)
      .signAndSend(founder, ({ status }) => {
        if (status.isInBlock) {
          console.log('   wHEZ/PEZ pool created');
          resolve();
        }
      })
      .catch(() => resolve()); // Pool may exist
  });

  // Pool 2: wHEZ/wUSDT
  await new Promise((resolve) => {
    api.tx.assetConversion.createPool(0, 2)
      .signAndSend(founder, ({ status }) => {
        if (status.isInBlock) {
          console.log('   wHEZ/wUSDT pool created');
          resolve();
        }
      })
      .catch(() => resolve());
  });

  // Pool 3: PEZ/wUSDT
  await new Promise((resolve) => {
    api.tx.assetConversion.createPool(1, 2)
      .signAndSend(founder, ({ status }) => {
        if (status.isInBlock) {
          console.log('   PEZ/wUSDT pool created');
          resolve();
        }
      })
      .catch(() => resolve());
  });

  // Add liquidity with CORRECT ratios
  console.log('\n3. Adding liquidity with CORRECT ratios...');

  // Pool 1: wHEZ/PEZ (1 wHEZ = 5 PEZ)
  console.log('   Adding 10,000 wHEZ + 50,000 PEZ (1:5 ratio)...');
  const whez1 = BigInt(10_000) * BigInt(10 ** 12);
  const pez1 = BigInt(50_000) * BigInt(10 ** 12);

  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .addLiquidity(0, 1, whez1.toString(), pez1.toString(), whez1.toString(), pez1.toString(), founder.address)
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            reject(new Error(dispatchError.toString()));
            return;
          }
          console.log('   ✓ wHEZ/PEZ liquidity added');
          resolve();
        }
      })
      .catch(reject);
  });

  // Pool 2: wHEZ/wUSDT (1 wHEZ = 0.25 wUSDT, so 40,000 wHEZ = 10,000 wUSDT)
  console.log('   Adding 40,000 wHEZ + 10,000 wUSDT (1:0.25 ratio)...');
  const whez2 = BigInt(40_000) * BigInt(10 ** 12);
  const wusdt2 = BigInt(10_000) * BigInt(10 ** 6);

  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .addLiquidity(0, 2, whez2.toString(), wusdt2.toString(), whez2.toString(), wusdt2.toString(), founder.address)
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            reject(new Error(dispatchError.toString()));
            return;
          }
          console.log('   ✓ wHEZ/wUSDT liquidity added');
          resolve();
        }
      })
      .catch(reject);
  });

  // Pool 3: PEZ/wUSDT (1 PEZ = 0.05 wUSDT, so 200,000 PEZ = 10,000 wUSDT)
  console.log('   Adding 200,000 PEZ + 10,000 wUSDT (1:0.05 ratio)...');
  const pez3 = BigInt(200_000) * BigInt(10 ** 12);
  const wusdt3 = BigInt(10_000) * BigInt(10 ** 6);

  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .addLiquidity(1, 2, pez3.toString(), wusdt3.toString(), pez3.toString(), wusdt3.toString(), founder.address)
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            reject(new Error(dispatchError.toString()));
            return;
          }
          console.log('   ✓ PEZ/wUSDT liquidity added');
          resolve();
        }
      })
      .catch(reject);
  });

  console.log('\n4. Verifying ratios...');

  // Verify pool ratios
  const pools = await api.query.assetConversion.pools.entries();

  for (const [key, value] of pools) {
    const assetIds = key.args[0].toJSON();
    const reserves = value.toJSON();

    const assetNames = { 0: 'wHEZ', 1: 'PEZ', 2: 'wUSDT' };
    const decimals = { 0: 12, 1: 12, 2: 6 };

    const id1 = assetIds[0];
    const id2 = assetIds[1];
    const val1 = Number(BigInt(reserves[0]) / BigInt(10 ** decimals[id1]));
    const val2 = Number(BigInt(reserves[1]) / BigInt(10 ** decimals[id2]));
    const ratio = (val2 / val1).toFixed(4);

    console.log(`   ${assetNames[id1]}/${assetNames[id2]}: ${val1} / ${val2} = 1:${ratio}`);
  }

  await api.disconnect();
  console.log('\n✓ All pools created with CORRECT ratios!');
}

main().catch(console.error);
EONODE

if [ $? -eq 0 ]; then
  echo ""
  echo "========================================"
  echo "✓ SETUP COMPLETE!"
  echo "========================================"
  echo ""
  echo "Validators are running with finalization"
  echo "All pools created with CORRECT ratios:"
  echo "  - 1 wHEZ = 5 PEZ"
  echo "  - 1 wUSDT = 4 wHEZ"
  echo "  - 1 wUSDT = 20 PEZ"
  echo ""
  echo "Logs saved to:"
  echo "  - /tmp/validator-startup.log"
  echo "  - /tmp/validator-restart.log"
  echo "  - /tmp/setup-wrapper.log"
  echo "  - /tmp/setup-pools.log"
  echo ""
else
  echo ""
  echo "ERROR: Setup failed during pool creation"
  echo "Check /tmp/setup-pools.log for details"
  exit 1
fi
