import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';

await cryptoWaitReady();
const api = await ApiPromise.create({
  provider: new WsProvider('ws://127.0.0.1:9944')
});

const keyring = new Keyring({ type: 'sr25519' });
const founder = keyring.addFromUri('skill dose toward always latin fish film cabbage praise blouse kingdom depth');

console.log('Founder address:', founder.address);
console.log('\nChecking asset balances:');

// Check wHEZ balance (asset 0)
const whezBalance = await api.query.assets.account(0, founder.address);
console.log('wHEZ (0):', whezBalance.toHuman());

// Check PEZ balance (asset 1)
const pezBalance = await api.query.assets.account(1, founder.address);
console.log('PEZ (1):', pezBalance.toHuman());

// Check wUSDT balance (asset 2)
const wusdtBalance = await api.query.assets.account(2, founder.address);
console.log('wUSDT (2):', wusdtBalance.toHuman());

await api.disconnect();
