import { ApiPromise, WsProvider } from '@polkadot/api';

async function main() {
  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  console.log('\n🔍 CHECKING THE WALLET FROM BLOCKCHAIN EVENT:\n');
  console.log('='.repeat(70));

  // The wallet from the IdentitySet event
  const eventWallet = '5GgTgG9sRmPQAYU1RsTejZYnZRjwzKZKWD3awtuqjHioki45';
  console.log(`Wallet from event: ${eventWallet}`);
  console.log('='.repeat(70));

  const identity = await api.query.identityKyc.identities(eventWallet);
  const kycStatus = await api.query.identityKyc.kycStatuses(eventWallet);
  const pendingApp = await api.query.identityKyc.pendingKycApplications(eventWallet);

  console.log(`\n📊 STATE OF THIS WALLET:`);
  console.log(`   Identity: ${identity.isSome ? '✅ EXISTS' : '❌ NOT FOUND'}`);
  if (identity.isSome) {
    const data = identity.unwrap();
    console.log(`     - Name: ${data.name.toString()}`);
    console.log(`     - Email: ${data.email.toString()}`);
  }
  console.log(`   KYC Status: ${kycStatus.toString()} (${kycStatus.toHuman()})`);
  console.log(`   Pending App: ${pendingApp.isSome ? '✅ EXISTS' : '❌ NOT FOUND'}`);
  if (pendingApp.isSome) {
    const app = pendingApp.unwrap();
    console.log(`     - Notes: ${app.notes.toString()}`);
  }

  console.log('\n' + '='.repeat(70));
  console.log('💡 EXPLANATION:');
  console.log('='.repeat(70));

  if (kycStatus.toString() === 'Pending' || kycStatus.toString() === '1') {
    console.log('✅ This wallet has PENDING KYC application');
    console.log('❌ Cannot apply again - KycApplicationAlreadyExists error');
  } else if (kycStatus.toString() === 'Approved' || kycStatus.toString() === '2') {
    console.log('✅ This wallet is APPROVED');
    console.log('❌ Cannot apply again - already approved');
  } else if (identity.isSome && kycStatus.toString() === 'NotStarted') {
    console.log('✅ This wallet has identity but no KYC application');
    console.log('✅ Can apply for KYC');
  } else {
    console.log('❌ This wallet has no identity');
  }

  await api.disconnect();
}

main().catch(console.error);
