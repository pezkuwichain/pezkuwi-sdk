import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';

await cryptoWaitReady();
const api = await ApiPromise.create({
  provider: new WsProvider('ws://127.0.0.1:9944')
});

const keyring = new Keyring({ type: 'sr25519' });
const founder = keyring.addFromUri('skill dose toward always latin fish film cabbage praise blouse kingdom depth');

console.log('Wrapping 50,000 HEZ to wHEZ...');
const wrapAmount = BigInt(50000) * BigInt(10**12); // 50,000 HEZ

await new Promise((resolve, reject) => {
  api.tx.tokenWrapper
    .wrap(wrapAmount.toString())
    .signAndSend(founder, ({ status, dispatchError }) => {
      if (status.isInBlock) {
        if (dispatchError) {
          console.error('❌ Failed:', dispatchError.toString());
          reject(new Error(dispatchError.toString()));
        } else {
          console.log('✅ Successfully wrapped 50,000 HEZ to wHEZ');
          resolve();
        }
      }
    });
});

// Check balance
const whezBalance = await api.query.assets.account(0, founder.address);
console.log('\nNew wHEZ balance:', whezBalance.toHuman());

await api.disconnect();
