#!/usr/bin/env node

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const fs = require('fs');
const path = require('path');

async function main() {
  console.log('🚀 Pezkuwi Parachain Registration (Multi-Step)');
  console.log('===============================================\n');

  // Configuration
  const WESTEND_RPC = 'wss://westend.api.onfinality.io/public-ws';
  const PARA_ID = 2246;
  const WASM_PATH = path.join(__dirname, 'parachain-runtime-final.wasm');
  const GENESIS_PATH = path.join(__dirname, 'genesis-head.bin');

  console.log('Configuration:');
  console.log(`  RPC: ${WESTEND_RPC}`);
  console.log(`  ParaId: ${PARA_ID}`);
  console.log(`  WASM: ${WASM_PATH}`);
  console.log(`  Genesis: ${GENESIS_PATH}\n`);

  // Read files and convert to hex
  console.log('📁 Reading files...');
  const wasmCodeBuffer = fs.readFileSync(WASM_PATH);
  const genesisHeadBuffer = fs.readFileSync(GENESIS_PATH);

  console.log(`  WASM size: ${(wasmCodeBuffer.length / 1024 / 1024).toFixed(2)} MB`);
  console.log(`  Genesis size: ${genesisHeadBuffer.length} bytes`);

  // Convert to hex format for Polkadot.js API
  console.log('🔄 Converting to hex format...');
  const wasmCode = '0x' + wasmCodeBuffer.toString('hex');
  const genesisHead = '0x' + genesisHeadBuffer.toString('hex');

  console.log(`  WASM hex length: ${wasmCode.length / 2 - 1} bytes`);
  console.log(`  Genesis hex length: ${genesisHead.length / 2 - 1} bytes\n`);

  // Connect to Westend
  console.log('🔌 Connecting to Westend...');
  const provider = new WsProvider(WESTEND_RPC);
  const api = await ApiPromise.create({ provider });

  const chain = await api.rpc.system.chain();
  const nodeVersion = await api.rpc.system.version();
  console.log(`  Connected to: ${chain} (${nodeVersion})\n`);

  // Create keyring and add Alice
  console.log('🔑 Setting up account...');
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  console.log(`  Account: ${alice.address}\n`);

  // Check balance
  const { data: { free: balance } } = await api.query.system.account(alice.address);
  console.log(`  Balance: ${balance.toHuman()}\n`);

  console.log('📝 Multi-Step Registration Process:\n');

  // Step 1: Set genesis head
  console.log('Step 1: Setting genesis head...');
  const tx1 = api.tx.registrar.setCurrentHead(PARA_ID, genesisHead);

  console.log('  Estimating fees...');
  try {
    const info1 = await tx1.paymentInfo(alice);
    console.log(`  Estimated fee: ${info1.partialFee.toHuman()}`);

    console.log('  Submitting transaction...');
    const result1 = await new Promise((resolve, reject) => {
      tx1.signAndSend(alice, ({ status, events, dispatchError }) => {
        console.log(`  Status: ${status.type}`);

        if (status.isInBlock) {
          console.log(`  ✅ In block: ${status.asInBlock.toHex()}`);

          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = api.registry.findMetaError(dispatchError.asModule);
              const { docs, name, section } = decoded;
              console.error(`  ❌ Error: ${section}.${name}: ${docs.join(' ')}`);
              reject(new Error(`${section}.${name}`));
            } else {
              console.error(`  ❌ Error: ${dispatchError.toString()}`);
              reject(new Error(dispatchError.toString()));
            }
          } else {
            console.log('  ✅ Genesis head set successfully!\n');
            resolve();
          }
        }
      });
    });
  } catch (error) {
    console.error(`  ❌ Failed: ${error.message}\n`);
    console.log('⚠️  Note: This may fail if you don\'t have sudo/governance permissions.');
    console.log('    On Westend testnet, you may need to use onDemand coretime instead.\n');
  }

  // Step 2: Schedule code upgrade
  console.log('Step 2: Scheduling code upgrade...');
  const tx2 = api.tx.registrar.scheduleCodeUpgrade(PARA_ID, wasmCode);

  console.log('  Estimating fees...');
  try {
    const info2 = await tx2.paymentInfo(alice);
    console.log(`  Estimated fee: ${info2.partialFee.toHuman()}`);

    console.log('  ⚠️  WARNING: This will submit a 3 MB transaction!');
    console.log('  This may timeout or fail due to size limits.\n');

    // Uncomment to actually submit:
    // console.log('  Submitting transaction...');
    // await new Promise((resolve, reject) => {
    //   tx2.signAndSend(alice, ({ status, dispatchError }) => {
    //     console.log(`  Status: ${status.type}`);
    //     if (status.isInBlock) {
    //       if (dispatchError) {
    //         reject(new Error(dispatchError.toString()));
    //       } else {
    //         console.log('  ✅ Code upgrade scheduled!\n');
    //         resolve();
    //       }
    //     }
    //   });
    // });

  } catch (error) {
    console.error(`  ❌ Failed: ${error.message}\n`);
  }

  console.log('📊 Alternative Approach: OnDemand Coretime\n');
  console.log('Since direct registration may not work, consider using:');
  console.log('  1. onDemandAssignmentProvider.placeOrderKeepAlive() to buy coretime');
  console.log('  2. Upload WASM via governance proposal');
  console.log('  3. Use local testnet (Chopsticks) for XCM testing\n');

  await api.disconnect();
  console.log('✅ Done!');
  console.log('\n⚠️  To actually deploy, you may need:');
  console.log('  - Sudo/governance access for scheduleCodeUpgrade');
  console.log('  - Or use Westend\'s onDemand coretime system');
  console.log('  - Or deploy to local testnet first\n');
}

main()
  .catch((error) => {
    console.error('❌ Error:', error.message);
    process.exit(1);
  });
