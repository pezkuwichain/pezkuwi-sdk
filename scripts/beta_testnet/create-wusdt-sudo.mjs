import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';

async function main() {
  await cryptoWaitReady();
  
  const wsProvider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider: wsProvider });
  
  const keyring = new Keyring({ type: 'sr25519' });
  const founder = keyring.addFromUri('skill dose toward always latin fish film cabbage praise blouse kingdom depth');
  
  console.log('Creating wUSDT asset via SUDO...');
  
  // Use sudo to force create the asset with min_balance = 1
  await new Promise((resolve, reject) => {
    api.tx.sudo
      .sudo(
        api.tx.assets.create(2, founder.address, 1)
      )
      .signAndSend(founder, ({ status, dispatchError, events }) => {
        console.log('Status:', status.type);
        if (status.isInBlock) {
          if (dispatchError) {
            console.log('Error:', dispatchError.toString());
            reject(new Error(dispatchError.toString()));
            return;
          }
          events.forEach(({ event }) => {
            console.log(event.section + '.' + event.method);
          });
          console.log('✅ wUSDT asset created!');
          resolve();
        }
      })
      .catch(reject);
  });
  
  console.log('\nSetting wUSDT metadata...');
  await new Promise((resolve, reject) => {
    api.tx.assets
      .setMetadata(2, 'Wrapped USDT', 'wUSDT', 6)
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.log('Metadata error:', dispatchError.toString());
          }
          console.log('✅ Metadata set!');
          resolve();
        }
      })
      .catch(reject);
  });
  
  await api.disconnect();
  console.log('\n✅ wUSDT asset ready!');
}

main().catch(console.error);
