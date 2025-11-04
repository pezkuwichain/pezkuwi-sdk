#!/usr/bin/env node
/**
 * Initial Pool Setup for Beta Testnet
 *
 * This script sets up the 3 DEX pools with correct ratios:
 * - wHEZ/PEZ: 100k:500k (1:5 ratio)
 * - wHEZ/wUSDT: 40k:10k (4:1 ratio)
 * - PEZ/wUSDT: 200k:10k (20:1 ratio)
 *
 * Target exchange rates:
 * - 1 wUSDT = 4 wHEZ = 20 PEZ
 * - 1 wHEZ = 5 PEZ = 0.25 wUSDT
 * - 1 PEZ = 0.2 wHEZ = 0.05 wUSDT
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';

const FOUNDER_SEED = 'skill dose toward always latin fish film cabbage praise blouse kingdom depth';
const WS_ENDPOINT = 'ws://127.0.0.1:9944';

async function wait(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function main() {
  console.log('\n🚀 Beta Testnet Initial Pool Setup\n');
  console.log('════════════════════════════════════════════════════════\n');

  await cryptoWaitReady();

  const wsProvider = new WsProvider(WS_ENDPOINT);
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: 'sr25519' });
  const founder = keyring.addFromUri(FOUNDER_SEED);

  console.log('✓ Connected to Beta Testnet');
  console.log(`✓ Founder address: ${founder.address}\n`);

  // Step 1: Create wUSDT asset
  console.log('📝 Step 1: Creating wUSDT asset (ID: 2, 6 decimals)...');
  await new Promise((resolve, reject) => {
    api.tx.assets
      .create(2, founder.address, 1000000) // min_balance = 1 wUSDT
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.log(`   Warning: ${dispatchError.toString()}`);
          }
          console.log('   ✅ wUSDT asset created (or already exists)\n');
          resolve();
        }
      })
      .catch(reject);
  });

  // Step 2: Set wUSDT metadata
  console.log('📝 Step 2: Setting wUSDT metadata...');
  await new Promise((resolve, reject) => {
    api.tx.assets
      .setMetadata(2, 'Wrapped USDT', 'wUSDT', 6)
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.log(`   Warning: ${dispatchError.toString()}`);
          }
          console.log('   ✅ wUSDT metadata set\n');
          resolve();
        }
      })
      .catch(reject);
  });

  // Step 3: Mint wUSDT
  console.log('📝 Step 3: Minting 1,000,000 wUSDT to founder...');
  const wusdtMintAmount = BigInt(1_000_000) * BigInt(10 ** 6);
  await new Promise((resolve, reject) => {
    api.tx.assets
      .mint(2, founder.address, wusdtMintAmount.toString())
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.log(`   ❌ Error: ${dispatchError.toString()}`);
            reject(new Error(dispatchError.toString()));
            return;
          }
          console.log('   ✅ wUSDT minted\n');
          resolve();
        }
      })
      .catch(reject);
  });

  await wait(3000);

  // Step 4: Create wHEZ/PEZ pool
  console.log('📝 Step 4: Creating wHEZ/PEZ pool (100k:500k)...');
  const whezAmount1 = BigInt(100_000) * BigInt(10 ** 12);
  const pezAmount1 = BigInt(500_000) * BigInt(10 ** 12);

  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .addLiquidity(
        0, 1, // wHEZ, PEZ
        whezAmount1.toString(),
        pezAmount1.toString(),
        '0', '0',
        founder.address
      )
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.log(`   ❌ Error: ${dispatchError.toString()}`);
            reject(new Error(dispatchError.toString()));
            return;
          }
          console.log('   ✅ wHEZ/PEZ pool created (1 wHEZ = 5 PEZ)\n');
          resolve();
        }
      })
      .catch(reject);
  });

  await wait(3000);

  // Step 5: Create wHEZ/wUSDT pool
  console.log('📝 Step 5: Creating wHEZ/wUSDT pool (40k:10k)...');
  const whezAmount2 = BigInt(40_000) * BigInt(10 ** 12);
  const wusdtAmount1 = BigInt(10_000) * BigInt(10 ** 6);

  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .addLiquidity(
        0, 2, // wHEZ, wUSDT
        whezAmount2.toString(),
        wusdtAmount1.toString(),
        '0', '0',
        founder.address
      )
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.log(`   ❌ Error: ${dispatchError.toString()}`);
            reject(new Error(dispatchError.toString()));
            return;
          }
          console.log('   ✅ wHEZ/wUSDT pool created (1 wUSDT = 4 wHEZ)\n');
          resolve();
        }
      })
      .catch(reject);
  });

  await wait(3000);

  // Step 6: Create PEZ/wUSDT pool
  console.log('📝 Step 6: Creating PEZ/wUSDT pool (200k:10k)...');
  const pezAmount2 = BigInt(200_000) * BigInt(10 ** 12);
  const wusdtAmount2 = BigInt(10_000) * BigInt(10 ** 6);

  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .addLiquidity(
        1, 2, // PEZ, wUSDT
        pezAmount2.toString(),
        wusdtAmount2.toString(),
        '0', '0',
        founder.address
      )
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.log(`   ❌ Error: ${dispatchError.toString()}`);
            reject(new Error(dispatchError.toString()));
            return;
          }
          console.log('   ✅ PEZ/wUSDT pool created (1 wUSDT = 20 PEZ)\n');
          resolve();
        }
      })
      .catch(reject);
  });

  console.log('════════════════════════════════════════════════════════\n');
  console.log('🎉 SUCCESS! All pools initialized with correct ratios\n');
  console.log('Exchange Rates:');
  console.log('   • 1 wUSDT = 4 wHEZ = 20 PEZ');
  console.log('   • 1 wHEZ = 5 PEZ = 0.25 wUSDT');
  console.log('   • 1 PEZ = 0.2 wHEZ = 0.05 wUSDT\n');
  console.log('════════════════════════════════════════════════════════\n');

  await api.disconnect();
  process.exit(0);
}

main().catch((error) => {
  console.error('\n❌ Setup failed:', error.message);
  process.exit(1);
});
