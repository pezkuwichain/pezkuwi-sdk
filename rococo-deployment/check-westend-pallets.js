#!/usr/bin/env node

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  console.log('🔍 Checking Westend Available Pallets and Methods');
  console.log('=================================================\n');

  const WESTEND_RPC = 'wss://westend.api.onfinality.io/public-ws';

  console.log('🔌 Connecting to Westend...');
  const provider = new WsProvider(WESTEND_RPC);
  const api = await ApiPromise.create({ provider });

  const chain = await api.rpc.system.chain();
  const version = await api.rpc.system.version();
  console.log(`  Connected to: ${chain} (${version})\n`);

  // Check for registrar pallet
  console.log('📋 Registrar Pallet Methods:');
  if (api.tx.registrar) {
    const methods = Object.keys(api.tx.registrar);
    methods.forEach(method => {
      console.log(`  - registrar.${method}`);
    });
  } else {
    console.log('  ❌ No registrar pallet found');
  }

  // Check for paras pallet
  console.log('\n📋 Paras Pallet Methods:');
  if (api.tx.paras) {
    const methods = Object.keys(api.tx.paras);
    methods.forEach(method => {
      console.log(`  - paras.${method}`);
    });
  } else {
    console.log('  ❌ No paras pallet found');
  }

  // Check for broker pallet (Agile Coretime)
  console.log('\n📋 Broker Pallet Methods (Agile Coretime):');
  if (api.tx.broker) {
    const methods = Object.keys(api.tx.broker);
    methods.forEach(method => {
      console.log(`  - broker.${method}`);
    });
  } else {
    console.log('  ❌ No broker pallet found');
  }

  // Check for slots pallet (legacy)
  console.log('\n📋 Slots Pallet Methods (Legacy Auctions):');
  if (api.tx.slots) {
    const methods = Object.keys(api.tx.slots);
    methods.forEach(method => {
      console.log(`  - slots.${method}`);
    });
  } else {
    console.log('  ❌ No slots pallet found');
  }

  // Check for coretime pallet
  console.log('\n📋 Coretime Pallet Methods:');
  if (api.tx.coretime) {
    const methods = Object.keys(api.tx.coretime);
    methods.forEach(method => {
      console.log(`  - coretime.${method}`);
    });
  } else {
    console.log('  ❌ No coretime pallet found');
  }

  // Check onDemand pallet
  console.log('\n📋 OnDemand Pallet Methods:');
  if (api.tx.onDemandAssignmentProvider || api.tx.onDemand) {
    const pallet = api.tx.onDemandAssignmentProvider || api.tx.onDemand;
    const methods = Object.keys(pallet);
    const palletName = api.tx.onDemandAssignmentProvider ? 'onDemandAssignmentProvider' : 'onDemand';
    methods.forEach(method => {
      console.log(`  - ${palletName}.${method}`);
    });
  } else {
    console.log('  ❌ No onDemand pallet found');
  }

  // Check all available pallets
  console.log('\n📋 All Available Pallets:');
  const pallets = Object.keys(api.tx);
  const paraRelated = pallets.filter(p =>
    p.includes('para') ||
    p.includes('registrar') ||
    p.includes('slot') ||
    p.includes('broker') ||
    p.includes('coretime') ||
    p.includes('onDemand')
  );

  if (paraRelated.length > 0) {
    console.log('\n  Parachain-related pallets:');
    paraRelated.forEach(p => console.log(`  - ${p}`));
  }

  await api.disconnect();
  console.log('\n✅ Done!');
}

main()
  .catch((error) => {
    console.error('❌ Error:', error.message);
    process.exit(1);
  });
