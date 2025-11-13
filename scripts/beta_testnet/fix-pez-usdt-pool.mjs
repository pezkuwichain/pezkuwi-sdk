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

  const PEZ_ASSET = 1;
  const WUSDT_ASSET = 2;

  console.log('🔧 Fixing PEZ-USDT Pool by Adding Liquidity\n');
  console.log('Founder address:', founder.address);

  // Check current balances
  const pezBalance = await api.query.assets.account(PEZ_ASSET, founder.address);
  const usdtBalance = await api.query.assets.account(WUSDT_ASSET, founder.address);

  console.log('\n💰 Current Balances:');
  console.log('  PEZ:', pezBalance.isSome ? (BigInt(pezBalance.unwrap().balance.toString()) / BigInt(10**12)).toString() : '0');
  console.log('  USDT:', usdtBalance.isSome ? (BigInt(usdtBalance.unwrap().balance.toString()) / BigInt(10**6)).toString() : '0');

  // Add MASSIVE liquidity to fix the pool
  // Using 5 PEZ = 1 USDT ratio
  const pezAmount = BigInt(100000) * BigInt(10**12); // 100,000 PEZ
  const usdtAmount = BigInt(20000) * BigInt(10**6);  // 20,000 USDT

  console.log('\n➕ Adding liquidity:');
  console.log('  PEZ:', (pezAmount / BigInt(10**12)).toString());
  console.log('  USDT:', (usdtAmount / BigInt(10**6)).toString());
  console.log('  Ratio: 5 PEZ = 1 USDT\n');

  try {
    await new Promise((resolve, reject) => {
      api.tx.assetConversion.addLiquidity(
        PEZ_ASSET,
        WUSDT_ASSET,
        pezAmount.toString(),
        usdtAmount.toString(),
        '1',  // amount1Min - very low to ensure it succeeds
        '1',  // amount2Min - very low to ensure it succeeds
        founder.address  // mint LP tokens to founder
      )
      .signAndSend(founder, ({ status, dispatchError, events }) => {
        if (status.isInBlock) {
          console.log('✅ Transaction included in block');

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
            console.log('✅ Liquidity added successfully!');

            // Print events
            events.forEach(({ event }) => {
              if (event.section === 'assetConversion') {
                console.log(`\nEvent: ${event.section}.${event.method}`);
                console.log(`Data: ${event.data.toString()}`);
              }
            });

            resolve();
          }
        }
      });
    });

    // Check new reserves
    console.log('\n🔍 Checking new pool reserves...');
    const poolData = await api.query.assetConversion.pools([PEZ_ASSET, WUSDT_ASSET]);
    if (poolData.isSome) {
      console.log('Pool still exists ✓');

      // Get pool account
      const poolAccountId = '5F3jG29CLmGP8Cn2mFSznZZFKGGpLmpMVfUzA6jbiowPMJF4'; // From earlier check

      const pezReserve = await api.query.assets.account(PEZ_ASSET, poolAccountId);
      const usdtReserve = await api.query.assets.account(WUSDT_ASSET, poolAccountId);

      console.log('\n📊 New Reserves:');
      if (pezReserve.isSome) {
        console.log('  PEZ:', (BigInt(pezReserve.unwrap().balance.toString()) / BigInt(10**12)).toString());
      }
      if (usdtReserve.isSome) {
        console.log('  USDT:', (BigInt(usdtReserve.unwrap().balance.toString()) / BigInt(10**6)).toString());
      }
    }

    console.log('\n✅ Pool fixed! Now test USDT → PEZ swap');

  } catch (error) {
    console.error('\n❌ Failed to add liquidity:', error.message);
  }

  await api.disconnect();
}

main().catch(console.error);
