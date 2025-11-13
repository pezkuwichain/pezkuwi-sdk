import { ApiPromise, WsProvider } from '@polkadot/api';

async function main() {
  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  console.log('\n🔍 CHECKING STATE AT SPECIFIC BLOCKS:\n');
  console.log('='.repeat(70));

  const founderAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

  // After setIdentity
  const identityBlockHash = '0x0443f749f19336fff6f306e5376b36f99f5d878cf8eb29224893a6ad0c6a79f4';
  console.log(`\n1️⃣ STATE AFTER setIdentity BLOCK:`);
  console.log(`   Block: ${identityBlockHash.substring(0, 20)}...`);

  try {
    const apiAt1 = await api.at(identityBlockHash);
    const identity1 = await apiAt1.query.identityKyc.identities(founderAddress);
    const kycStatus1 = await apiAt1.query.identityKyc.kycStatuses(founderAddress);
    const pendingApp1 = await apiAt1.query.identityKyc.pendingKycApplications(founderAddress);

    console.log(`   Identity: ${identity1.isSome ? '✅ EXISTS' : '❌ NOT FOUND'}`);
    if (identity1.isSome) {
      const data = identity1.unwrap();
      console.log(`     - Name: ${data.name.toString()}`);
      console.log(`     - Email: ${data.email.toString()}`);
    }
    console.log(`   KYC Status: ${kycStatus1.toString()} (${kycStatus1.toHuman()})`);
    console.log(`   Pending App: ${pendingApp1.isSome ? '✅ EXISTS' : '❌ NOT FOUND'}`);
  } catch (e) {
    console.log(`   ⚠️  Error: ${e.message}`);
  }

  // Before applyForKyc (same as after setIdentity but let's confirm)
  const kycBlockHash = '0x7ddedf9a887cbe9caee6538e53704fdf20da3ea0b8dbd0829ae503024febdb97';
  console.log(`\n2️⃣ STATE AT applyForKyc BLOCK (BEFORE execution):`);
  console.log(`   Block: ${kycBlockHash.substring(0, 20)}...`);

  try {
    // Get parent block to check state BEFORE applyForKyc executed
    const kycBlock = await api.rpc.chain.getBlock(kycBlockHash);
    const parentBlockHash = kycBlock.block.header.parentHash;

    console.log(`   Parent block: ${parentBlockHash.toHex().substring(0, 20)}...`);

    const apiAtParent = await api.at(parentBlockHash);
    const identity2 = await apiAtParent.query.identityKyc.identities(founderAddress);
    const kycStatus2 = await apiAtParent.query.identityKyc.kycStatuses(founderAddress);
    const pendingApp2 = await apiAtParent.query.identityKyc.pendingKycApplications(founderAddress);

    console.log(`   Identity: ${identity2.isSome ? '✅ EXISTS' : '❌ NOT FOUND'}`);
    if (identity2.isSome) {
      const data = identity2.unwrap();
      console.log(`     - Name: ${data.name.toString()}`);
      console.log(`     - Email: ${data.email.toString()}`);
    }
    console.log(`   KYC Status: ${kycStatus2.toString()} (${kycStatus2.toHuman()})`);
    console.log(`   Pending App: ${pendingApp2.isSome ? '✅ EXISTS' : '❌ NOT FOUND'}`);
  } catch (e) {
    console.log(`   ⚠️  Error: ${e.message}`);
  }

  await api.disconnect();
}

main().catch(console.error);
