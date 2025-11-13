import { ApiPromise, WsProvider } from '@polkadot/api';

async function main() {
  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  console.log('\n🔍 CHECKING SPECIFIC TRANSACTION BLOCKS:\n');
  console.log('='.repeat(70));

  // Identity block
  const identityBlockHash = '0x0443f749f19336fff6f306e5376b36f99f5d878cf8eb29224893a6ad0c6a79f4';
  console.log(`\n1️⃣ IDENTITY TRANSACTION (setIdentity):`);
  console.log(`   Block: ${identityBlockHash}`);

  try {
    const apiAt1 = await api.at(identityBlockHash);
    const events1 = await apiAt1.query.system.events();

    events1.forEach((record) => {
      const { event, phase } = record;
      if (event.section === 'identityKyc' || event.section === 'system') {
        console.log(`   - ${event.section}.${event.method}:`, event.data.toString());
      }
    });
  } catch (e) {
    console.log(`   ⚠️  Error: ${e.message}`);
  }

  // KYC block
  const kycBlockHash = '0x7ddedf9a887cbe9caee6538e53704fdf20da3ea0b8dbd0829ae503024febdb97';
  console.log(`\n2️⃣ KYC APPLICATION TRANSACTION (applyForKyc):`);
  console.log(`   Block: ${kycBlockHash}`);

  try {
    const apiAt2 = await api.at(kycBlockHash);
    const events2 = await apiAt2.query.system.events();

    let foundError = false;
    events2.forEach((record) => {
      const { event, phase } = record;
      if (event.section === 'identityKyc' || event.section === 'system') {
        console.log(`   - ${event.section}.${event.method}:`, event.data.toString());
        if (event.method === 'ExtrinsicFailed') {
          foundError = true;
        }
      }
    });

    if (foundError) {
      console.log(`\n   ❌ TRANSACTION FAILED!`);
    } else {
      console.log(`\n   ✅ Transaction succeeded`);
    }
  } catch (e) {
    console.log(`   ⚠️  Error: ${e.message}`);
  }

  // Check current state
  console.log(`\n3️⃣ CURRENT BLOCKCHAIN STATE:`);
  const founderAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

  const identity = await api.query.identityKyc.identities(founderAddress);
  const kycStatus = await api.query.identityKyc.kycStatuses(founderAddress);
  const pendingApp = await api.query.identityKyc.pendingKycApplications(founderAddress);

  console.log(`   Identity: ${identity.isSome ? '✅ EXISTS' : '❌ NOT FOUND'}`);
  console.log(`   KYC Status: ${kycStatus.toString()}`);
  console.log(`   Pending App: ${pendingApp.isSome ? '✅ EXISTS' : '❌ NOT FOUND'}`);

  await api.disconnect();
}

main().catch(console.error);
