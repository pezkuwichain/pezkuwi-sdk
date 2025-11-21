#!/usr/bin/env node

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const fs = require('fs');

const WASM_PATH = './rococo-deployment/parachain-runtime-final.wasm';
const RPC_ENDPOINT = 'ws://127.0.0.1:9944'; // Alice's RPC

async function upgradeRuntime() {
  console.log('🚀 Pezkuwichain Runtime Upgrade - Alfa Testnet\n');

  // Read WASM file
  console.log('📁 Reading runtime WASM...');
  const wasmCodeBuffer = fs.readFileSync(WASM_PATH);
  console.log(`  WASM size: ${(wasmCodeBuffer.length / 1024 / 1024).toFixed(2)} MB`);

  // Convert to hex format
  const wasmCode = '0x' + wasmCodeBuffer.toString('hex');
  console.log(`  Hex length: ${wasmCode.length / 2 - 1} bytes\n`);

  // Connect to node
  console.log(`🔌 Connecting to ${RPC_ENDPOINT}...`);
  const provider = new WsProvider(RPC_ENDPOINT);
  const api = await ApiPromise.create({ provider });

  console.log(`  ✅ Connected to chain: ${await api.rpc.system.chain()}`);

  // Get current runtime version
  const runtimeVersion = await api.rpc.state.getRuntimeVersion();
  console.log(`  Current runtime version: ${runtimeVersion.specVersion.toNumber()}\n`);

  // Setup Alice account
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  console.log(`👤 Using sudo account: ${alice.address}\n`);

  // Check if Alice has sudo
  const sudoKey = await api.query.sudo.key();
  if (sudoKey.toString() !== alice.address) {
    console.error('❌ Error: Alice is not the sudo key!');
    console.error(`  Sudo key: ${sudoKey.toString()}`);
    console.error(`  Alice:    ${alice.address}`);
    process.exit(1);
  }

  console.log('✅ Alice has sudo permissions\n');

  // Create runtime upgrade proposal
  console.log('📦 Creating runtime upgrade transaction...');
  const setCodeCall = api.tx.system.setCode(wasmCode);

  console.log('🔐 Preparing sudo call...');

  // Use sudoUncheckedWeight to bypass weight checks
  // Weight reference: { refTime: 0, proofSize: 0 } means "don't check weight"
  const sudoTx = api.tx.sudo.sudoUncheckedWeight(
    setCodeCall,
    { refTime: 0, proofSize: 0 }
  );

  console.log('📤 Submitting runtime upgrade...\n');

  return new Promise((resolve, reject) => {
    sudoTx.signAndSend(alice, ({ status, events, dispatchError }) => {
      console.log(`  Transaction status: ${status.type}`);

      if (status.isInBlock) {
        console.log(`  ✅ Included in block: ${status.asInBlock.toHex()}`);
      }

      if (status.isFinalized) {
        console.log(`  🎉 Finalized in block: ${status.asFinalized.toHex()}\n`);

        // Check for errors
        if (dispatchError) {
          if (dispatchError.isModule) {
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;
            console.error(`❌ Error: ${section}.${name}: ${docs.join(' ')}`);
            reject(new Error(`${section}.${name}`));
          } else {
            console.error(`❌ Error: ${dispatchError.toString()}`);
            reject(new Error(dispatchError.toString()));
          }
          return;
        }

        // Check events
        events.forEach(({ event: { data, method, section } }) => {
          console.log(`  Event: ${section}.${method}`, data.toString());

          if (section === 'system' && method === 'CodeUpdated') {
            console.log('\n✅ Runtime code updated successfully!');
          }

          if (section === 'sudo' && method === 'Sudid') {
            console.log('  ✅ Sudo call executed');
          }
        });

        // Wait a bit and check new runtime version
        setTimeout(async () => {
          const newRuntimeVersion = await api.rpc.state.getRuntimeVersion();
          console.log(`\n📊 New runtime version: ${newRuntimeVersion.specVersion.toNumber()}`);

          if (newRuntimeVersion.specVersion.toNumber() !== runtimeVersion.specVersion.toNumber()) {
            console.log('🎉 Runtime upgrade successful!');
          } else {
            console.log('⚠️  Warning: Runtime version unchanged');
          }

          await api.disconnect();
          resolve();
        }, 3000);
      }
    }).catch((error) => {
      console.error('❌ Transaction failed:', error.message);
      reject(error);
    });
  });
}

upgradeRuntime()
  .then(() => {
    console.log('\n✅ Runtime upgrade process completed');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Runtime upgrade failed:', error.message);
    process.exit(1);
  });
