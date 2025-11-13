import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';

async function main() {
  await cryptoWaitReady();

  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const founder = keyring.addFromUri('skill dose toward always latin fish film cabbage praise blouse kingdom depth');

  const WHEZ_ASSET = 0;
  const PEZ_ASSET = 1;
  const WUSDT_ASSET = 2;

  console.log('🧮 Testing actual swap: 1 USDT → PEZ\n');

  const swapAmount = BigInt(1) * BigInt(10**6); // 1 USDT

  try {
    const tx = api.tx.assetConversion.swapExactTokensForTokens(
      [WUSDT_ASSET, PEZ_ASSET],  // path
      swapAmount.toString(),      // amountIn
      '1',                        // amountOutMin
      founder.address,            // sendTo
      false                       // keepAlive
    );

    console.log('Transaction created successfully');
    console.log('Trying to send transaction...\n');

    await new Promise((resolve, reject) => {
      tx.signAndSend(founder, ({ status, dispatchError, events }) => {
        if (status.isInBlock) {
          console.log('Transaction included in block');

          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = api.registry.findMetaError(dispatchError.asModule);
              const { docs, name, section } = decoded;
              console.log('❌ Error:', `${section}.${name}: ${docs.join(' ')}`);
              reject(new Error(`${section}.${name}`));
            } else {
              console.log('❌ Error:', dispatchError.toString());
              reject(new Error(dispatchError.toString()));
            }
          } else {
            console.log('✅ Swap successful!');

            // Print events
            events.forEach(({ event }) => {
              if (event.section === 'assetConversion') {
                console.log(`Event: ${event.section}.${event.method}`);
                console.log(`Data: ${event.data.toString()}`);
              }
            });

            resolve();
          }
        }
      });
    });

  } catch (error) {
    console.error('❌ Transaction error:', error.message);
  }

  await api.disconnect();
}

main().catch(console.error);
