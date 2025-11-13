import { ApiPromise, WsProvider } from '@polkadot/api';

async function main() {
  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  console.log('\n🔍 VERIFYING FOUNDER WALLET STATE:\n');
  console.log('='.repeat(70));

  // Founder wallet address
  const founderAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
  console.log(`Wallet: ${founderAddress}`);
  console.log('='.repeat(70));

  // 1. Check KYC Status
  console.log('\n1️⃣ CHECKING KYC STATUS:');
  const kycStatus = await api.query.identityKyc.kycStatuses(founderAddress);
  console.log(`   KYC Status: ${kycStatus.toString()}`);
  console.log(`   Status Details: ${JSON.stringify(kycStatus.toHuman())}`);

  // 2. Check Identity
  console.log('\n2️⃣ CHECKING IDENTITY:');
  const identity = await api.query.identityKyc.identities(founderAddress);
  if (identity.isSome) {
    const identityData = identity.unwrap();
    console.log(`   Identity Found:`);
    console.log(`   - Name: ${identityData.name.toString()}`);
    console.log(`   - Email: ${identityData.email.toString()}`);
  } else {
    console.log(`   ❌ No identity found`);
  }

  // 3. Check Pending Application
  console.log('\n3️⃣ CHECKING PENDING APPLICATION:');
  const pendingApp = await api.query.identityKyc.pendingKycApplications(founderAddress);
  if (pendingApp.isSome) {
    const appData = pendingApp.unwrap();
    console.log(`   ✅ Pending application found:`);
    console.log(`   - CIDs: ${appData.cids.toString()}`);
    console.log(`   - Notes: ${appData.notes.toString()}`);
  } else {
    console.log(`   ❌ No pending application`);
  }

  // 4. Check Citizen NFT
  console.log('\n4️⃣ CHECKING CITIZEN NFT (WELATI TIKI):');
  try {
    const citizenNft = await api.query.tiki.citizenNft(founderAddress);
    if (citizenNft.isSome) {
      console.log(`   ✅ NFT EXISTS!`);
      console.log(`   NFT Data: ${JSON.stringify(citizenNft.toHuman())}`);
    } else {
      console.log(`   ❌ No NFT minted`);
    }
  } catch (e) {
    console.log(`   ❌ Error checking NFT: ${e.message}`);
  }

  // 5. Check last events related to this account
  console.log('\n5️⃣ CHECKING RECENT BLOCKCHAIN EVENTS:');
  const blockHash = '0x4f5043370fd986234ec8ef3dd07dd25a91e1e41b225694ad808f108bd5c691c1';
  try {
    const signedBlock = await api.rpc.chain.getBlock(blockHash);
    const apiAt = await api.at(signedBlock.block.header.hash);
    const allRecords = await apiAt.query.system.events();

    console.log(`\n   Events in block ${blockHash.substring(0, 16)}...:`);
    allRecords.forEach((record, index) => {
      const { event } = record;
      if (event.section === 'identityKyc' || event.section === 'tiki') {
        console.log(`   - ${event.section}.${event.method}:`, event.data.toString());
      }
    });
  } catch (e) {
    console.log(`   ⚠️  Could not fetch events: ${e.message}`);
  }

  console.log('\n' + '='.repeat(70));
  console.log('📊 SUMMARY:');
  console.log('='.repeat(70));

  const statusStr = kycStatus.toString();
  if (statusStr === 'Approved' || statusStr === '2') {
    console.log('✅ Wallet is APPROVED');
    console.log('🎫 NFT should exist (check above)');
  } else if (statusStr === 'Pending' || statusStr === '1') {
    console.log('⏳ Wallet has PENDING application');
    console.log('⚠️  Not yet approved by admin');
  } else if (statusStr === 'NotStarted' || statusStr === '0') {
    console.log('❌ No KYC application started');
  } else {
    console.log(`❓ Unknown status: ${statusStr}`);
  }

  await api.disconnect();
}

main().catch(console.error);
