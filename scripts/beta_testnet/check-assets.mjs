import { ApiPromise, WsProvider } from '@polkadot/api';
import { cryptoWaitReady } from '@polkadot/util-crypto';

await cryptoWaitReady();
const api = await ApiPromise.create({
  provider: new WsProvider('ws://127.0.0.1:9944')
});

console.log('\nAll registered assets:');
const assets = await api.query.assets.metadata.entries();
for (const [key, value] of assets) {
  if (value.name.length > 0) {
    const assetId = key.args[0].toString();
    console.log(`Asset ${assetId}:`, value.toHuman());
  }
}

await api.disconnect();
