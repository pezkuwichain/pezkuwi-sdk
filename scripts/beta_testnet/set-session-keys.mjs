import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';
import fs from 'fs';

async function main() {
  await cryptoWaitReady();

  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  // Read validator data
  const validatorData = JSON.parse(
    fs.readFileSync('/home/mamostehp/Pezkuwi-SDK/pezkuwi/runtime/validators/beta_testnet_validators.json', 'utf8')
  );

  const keyring = new Keyring({ type: 'sr25519' });

  for (let i = 0; i < 8; i++) {
    const validator = validatorData.beta[i];
    console.log(`\n🔑 Setting keys for ${validator.name}...`);

    // Create stash account from seed
    const stashAccount = keyring.addFromUri(validator.stash_seed);
    console.log(`   Stash: ${stashAccount.address}`);

    // Construct session keys object
    const sessionKeys = {
      babe: validator.babe,
      grandpa: validator.grandpa,
      paraValidator: validator.para_validator,
      paraAssignment: validator.para_assignment,
      authorityDiscovery: validator.authority_discovery,
      beefy: validator.beefy
    };

    console.log('   Session keys:', sessionKeys);

    // Set the session keys on-chain
    try {
      const result = await new Promise((resolve, reject) => {
        api.tx.session
          .setKeys(sessionKeys, '0x')
          .signAndSend(stashAccount, ({ status, dispatchError }) => {
            if (status.isInBlock) {
              if (dispatchError) {
                console.error(`   ❌ Failed: ${dispatchError.toString()}`);
                reject(new Error(dispatchError.toString()));
              } else {
                console.log(`   ✅ Keys set in block: ${status.asInBlock}`);
                resolve(status.asInBlock);
              }
            }
          });
      });
    } catch (error) {
      console.error(`   ❌ Error: ${error.message}`);
    }

    // Wait a bit between validators
    await new Promise(resolve => setTimeout(resolve, 2000));
  }

  console.log('\n✅ All validator keys set!');
  console.log('Waiting for next session to apply keys...\n');

  await api.disconnect();
}

main().catch(console.error);
