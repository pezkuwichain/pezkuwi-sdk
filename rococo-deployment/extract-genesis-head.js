#!/usr/bin/env node

const { ApiPromise, WsProvider } = require('@polkadot/api');
const fs = require('fs');
const path = require('path');

async function main() {
  console.log('🔍 Extracting Genesis Head from Local Pezkuwi Node');
  console.log('=================================================\n');

  // Connect to local Pezkuwi node
  console.log('🔌 Connecting to local node...');
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });

  const chain = await api.rpc.system.chain();
  console.log(`  Connected to: ${chain}\n`);

  // Get genesis block
  console.log('📦 Fetching genesis block...');
  const genesisHash = await api.rpc.chain.getBlockHash(0);
  console.log(`  Genesis hash: ${genesisHash.toHex()}\n`);

  // Get genesis header
  console.log('🔍 Extracting genesis header...');
  const genesisHeader = await api.rpc.chain.getHeader(genesisHash);
  console.log(`  Block number: ${genesisHeader.number.toNumber()}`);
  console.log(`  Parent hash: ${genesisHeader.parentHash.toHex()}`);
  console.log(`  State root: ${genesisHeader.stateRoot.toHex()}`);
  console.log(`  Extrinsics root: ${genesisHeader.extrinsicsRoot.toHex()}\n`);

  // Encode header to bytes
  const genesisHeadHex = genesisHeader.toHex();
  console.log(`  Genesis head (hex): ${genesisHeadHex.substring(0, 66)}...`);
  console.log(`  Length: ${genesisHeadHex.length / 2 - 1} bytes\n`);

  // Save to file
  const outputPath = path.join(__dirname, 'genesis-head.bin');
  const outputHexPath = path.join(__dirname, 'genesis-head.hex');

  // Write binary (remove 0x prefix and convert hex to buffer)
  const hexData = genesisHeadHex.substring(2);
  const buffer = Buffer.from(hexData, 'hex');
  fs.writeFileSync(outputPath, buffer);
  console.log(`✅ Binary genesis head saved to: ${outputPath}`);
  console.log(`   Size: ${buffer.length} bytes\n`);

  // Write hex (without 0x prefix for easy copy-paste)
  fs.writeFileSync(outputHexPath, hexData);
  console.log(`✅ Hex genesis head saved to: ${outputHexPath}`);
  console.log(`   Size: ${hexData.length} characters\n`);

  // Verify file sizes
  const stats = fs.statSync(outputPath);
  const hexStats = fs.statSync(outputHexPath);

  console.log('📊 File verification:');
  console.log(`  Binary: ${stats.size} bytes`);
  console.log(`  Hex: ${hexStats.size} bytes (${hexStats.size / 2} bytes when decoded)`);

  if (stats.size < 10 * 1024 * 1024) {
    console.log('\n✅ Genesis head is under 10 MB - suitable for registration!');
  } else {
    console.log('\n⚠️  Warning: Genesis head exceeds 10 MB');
  }

  await api.disconnect();
  console.log('\n✅ Done!');
}

main()
  .catch((error) => {
    console.error('❌ Error:', error.message);
    process.exit(1);
  });
