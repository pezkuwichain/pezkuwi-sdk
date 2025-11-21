#!/usr/bin/env node

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const fs = require('fs');
const path = require('path');

async function main() {
  console.log('🚀 Pezkuwi Parachain Registration Script');
  console.log('=========================================\n');

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
  console.log(`  Genesis size: ${(genesisHeadBuffer.length / 1024 / 1024).toFixed(2)} MB`);

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

  // Register parachain
  console.log('📝 Creating registration transaction...');
  const tx = api.tx.registrar.register(
    PARA_ID,
    genesisHead,
    wasmCode
  );

  console.log('💰 Estimating fees...');
  const info = await tx.paymentInfo(alice);
  console.log(`  Estimated fee: ${info.partialFee.toHuman()}\n`);

  // Sign and send
  console.log('✍️  Signing and sending transaction...');
  console.log('  (This may take a while due to large file sizes)\n');

  const unsub = await tx.signAndSend(alice, ({ status, events }) => {
    console.log(`  Transaction status: ${status.type}`);

    if (status.isInBlock) {
      console.log(`  ✅ Included in block: ${status.asInBlock.toHex()}`);

      events.forEach(({ event: { data, method, section } }) => {
        console.log(`    ${section}.${method}: ${data.toString()}`);
      });
    } else if (status.isFinalized) {
      console.log(`  🎉 Finalized in block: ${status.asFinalized.toHex()}\n`);
      console.log('✅ Parachain registration successful!');
      console.log(`   ParaId ${PARA_ID} is now registered on Westend\n`);

      unsub();
      process.exit(0);
    } else if (status.isError) {
      console.error('  ❌ Transaction failed!');
      unsub();
      process.exit(1);
    }
  });
}

main()
  .catch((error) => {
    console.error('❌ Error:', error.message);
    process.exit(1);
  });
