import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';

async function main() {
  await cryptoWaitReady();

  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const sudo = keyring.addFromUri('//Alice'); // Sudo account

  const targetWallet = '5GgTgG9sRmPQAYU1RsTejZYnZRjwzKZKWD3awtuqjHioki45';

  console.log('\n🗑️  REJECTING CORRUPTED KYC APPLICATION:\n');
  console.log('='.repeat(70));
  console.log(`Target wallet: ${targetWallet}`);
  console.log('='.repeat(70));

  // Check current state
  const kycStatus = await api.query.identityKyc.kycStatuses(targetWallet);
  const pendingApp = await api.query.identityKyc.pendingKycApplications(targetWallet);
  const identity = await api.query.identityKyc.identities(targetWallet);

  console.log(`\nCurrent state:`);
  console.log(`  KYC Status: ${kycStatus.toString()}`);
  console.log(`  Pending App: ${pendingApp.isSome ? 'EXISTS' : 'NOT FOUND'}`);
  console.log(`  Identity: ${identity.isSome ? 'EXISTS' : 'NOT FOUND'}`);

  if (identity.isSome) {
    const data = identity.unwrap();
    console.log(`    - Name: ${data.name.toString()}`);
    console.log(`    - Email: ${data.email.toString()}`);
  }

  if (kycStatus.toString() !== 'Pending' && kycStatus.toString() !== '1') {
    console.log('\n❌ Cannot reject - status is not Pending!');
    await api.disconnect();
    return;
  }

  console.log(`\n📝 Submitting reject_kyc transaction...`);

  // Create reject_kyc call
  const rejectCall = api.tx.identityKyc.rejectKyc(targetWallet);

  // Wrap in sudo
  const sudoCall = api.tx.sudo.sudo(rejectCall);

  await new Promise((resolve, reject) => {
    sudoCall.signAndSend(sudo, ({ status, dispatchError, events }) => {
      if (status.isInBlock) {
        console.log(`\n✅ Transaction in block: ${status.asInBlock.toHex()}`);

        if (dispatchError) {
          if (dispatchError.isModule) {
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            console.log(`\n❌ Error: ${decoded.section}.${decoded.name}`);
            console.log(`   ${decoded.docs.join(' ')}`);
            reject(new Error(`${decoded.section}.${decoded.name}`));
          } else {
            console.log(`\n❌ Error: ${dispatchError.toString()}`);
            reject(new Error(dispatchError.toString()));
          }
          return;
        }

        // Log events
        console.log(`\n📋 Events:`);
        events.forEach(({ event }) => {
          if (event.section === 'identityKyc' || event.section === 'sudo') {
            console.log(`   - ${event.section}.${event.method}:`, event.data.toString());
          }
        });

        resolve();
      }
    }).catch(reject);
  });

  // Check final state
  console.log(`\n📊 FINAL STATE:`);
  const finalKycStatus = await api.query.identityKyc.kycStatuses(targetWallet);
  const finalPendingApp = await api.query.identityKyc.pendingKycApplications(targetWallet);
  const finalIdentity = await api.query.identityKyc.identities(targetWallet);

  console.log(`  KYC Status: ${finalKycStatus.toString()} (${finalKycStatus.toHuman()})`);
  console.log(`  Pending App: ${finalPendingApp.isSome ? 'STILL EXISTS' : '✅ REMOVED'}}`);
  console.log(`  Identity: ${finalIdentity.isSome ? 'EXISTS' : 'REMOVED'}`);

  console.log('\n💡 Next step: Test citizenship application with clean wallet state');

  await api.disconnect();
}

main().catch(console.error);
