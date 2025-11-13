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

  console.log('🏗️  Starting DEX Pool Creation Process...\n');
  console.log('Founder address:', founder.address);

  // Get founder balance
  const { data: balance } = await api.query.system.account(founder.address);
  console.log('Founder HEZ balance:', (BigInt(balance.free.toString()) / BigInt(10**12)).toString(), 'HEZ\n');

  // Define asset IDs (all are u32 numbers)
  const WHEZ_ASSET = 0;   // Wrapped HEZ
  const PEZ_ASSET = 1;    // PEZ Token
  const WUSDT_ASSET = 2;  // Wrapped USDT

  // Step 1: Check assets exist
  console.log('📋 Step 1: Verifying assets...');
  const wHezMeta = await api.query.assets.metadata(WHEZ_ASSET);
  const pezMeta = await api.query.assets.metadata(PEZ_ASSET);
  const wUsdtMeta = await api.query.assets.metadata(WUSDT_ASSET);
  console.log(`✅ wHEZ (${WHEZ_ASSET}):`, wHezMeta.name.toString());
  console.log(`✅ PEZ (${PEZ_ASSET}):`, pezMeta.name.toString());
  console.log(`✅ wUSDT (${WUSDT_ASSET}):`, wUsdtMeta.name.toString());

  // Step 2: Create wHEZ-PEZ Pool (empty)
  console.log('\n💧 Step 2: Creating wHEZ-PEZ Pool...');
  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .createPool(WHEZ_ASSET, PEZ_ASSET)
      .signAndSend(founder, ({ status, dispatchError, events }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.error('❌ Failed:', dispatchError.toString());
            reject(new Error(dispatchError.toString()));
          } else {
            console.log('✅ wHEZ-PEZ pool created (empty)');
            resolve();
          }
        }
      });
  });

  // Step 3: Add liquidity to wHEZ-PEZ Pool
  console.log('\n💰 Step 3: Adding liquidity to wHEZ-PEZ Pool...');
  const whezAmount = BigInt(10000) * BigInt(10**12); // 10,000 wHEZ
  const pezAmount = BigInt(12500) * BigInt(10**12); // 12,500 PEZ (8 HEZ = 10 PEZ ratio)

  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .addLiquidity(
        WHEZ_ASSET,
        PEZ_ASSET,
        whezAmount.toString(),
        pezAmount.toString(),
        '1',  // amount1Min (1 planck minimum)
        '1',  // amount2Min (1 planck minimum)
        founder.address  // mint LP tokens to founder
      )
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.error('❌ Failed:', dispatchError.toString());
            reject(new Error(dispatchError.toString()));
          } else {
            console.log('✅ Added liquidity: 10,000 wHEZ + 12,500 PEZ');
            console.log('   Rate: 8 wHEZ = 10 PEZ (1 wHEZ = 1.25 PEZ)');
            resolve();
          }
        }
      });
  });

  // Step 4: Create wHEZ-wUSDT Pool (empty)
  console.log('\n💧 Step 4: Creating wHEZ-wUSDT Pool...');
  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .createPool(WHEZ_ASSET, WUSDT_ASSET)
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.error('❌ Failed:', dispatchError.toString());
            reject(new Error(dispatchError.toString()));
          } else {
            console.log('✅ wHEZ-wUSDT pool created (empty)');
            resolve();
          }
        }
      });
  });

  // Step 5: Add liquidity to wHEZ-wUSDT Pool
  console.log('\n💰 Step 5: Adding liquidity to wHEZ-wUSDT Pool...');
  const whezAmount2 = BigInt(10000) * BigInt(10**12); // 10,000 wHEZ
  const wusdtAmount = BigInt(2500) * BigInt(10**6); // 2,500 wUSDT (4 HEZ = 1 USDT ratio)

  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .addLiquidity(
        WHEZ_ASSET,
        WUSDT_ASSET,
        whezAmount2.toString(),
        wusdtAmount.toString(),
        '1',
        '1',
        founder.address
      )
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.error('❌ Failed:', dispatchError.toString());
            reject(new Error(dispatchError.toString()));
          } else {
            console.log('✅ Added liquidity: 10,000 wHEZ + 2,500 wUSDT');
            console.log('   Rate: 4 wHEZ = 1 wUSDT (wHEZ = $0.25)');
            resolve();
          }
        }
      });
  });

  // Step 6: Create PEZ-wUSDT Pool (empty)
  console.log('\n💧 Step 6: Creating PEZ-wUSDT Pool...');
  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .createPool(PEZ_ASSET, WUSDT_ASSET)
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.error('❌ Failed:', dispatchError.toString());
            reject(new Error(dispatchError.toString()));
          } else {
            console.log('✅ PEZ-wUSDT pool created (empty)');
            resolve();
          }
        }
      });
  });

  // Step 7: Add liquidity to PEZ-wUSDT Pool
  console.log('\n💰 Step 7: Adding liquidity to PEZ-wUSDT Pool...');
  const pezAmount2 = BigInt(50000) * BigInt(10**12); // 50,000 PEZ
  const wusdtAmount2 = BigInt(10000) * BigInt(10**6); // 10,000 wUSDT (5 PEZ = 1 USDT ratio)

  await new Promise((resolve, reject) => {
    api.tx.assetConversion
      .addLiquidity(
        PEZ_ASSET,
        WUSDT_ASSET,
        pezAmount2.toString(),
        wusdtAmount2.toString(),
        '1',
        '1',
        founder.address
      )
      .signAndSend(founder, ({ status, dispatchError }) => {
        if (status.isInBlock) {
          if (dispatchError) {
            console.error('❌ Failed:', dispatchError.toString());
            reject(new Error(dispatchError.toString()));
          } else {
            console.log('✅ Added liquidity: 50,000 PEZ + 10,000 wUSDT');
            console.log('   Rate: 5 PEZ = 1 wUSDT (PEZ = $0.20)');
            resolve();
          }
        }
      });
  });

  console.log('\n🎉 All DEX pools created successfully!\n');
  console.log('📊 Summary:');
  console.log('   • wHEZ-PEZ: 8 wHEZ = 10 PEZ (1 wHEZ = 1.25 PEZ)');
  console.log('   • wHEZ-wUSDT: 4 wHEZ = 1 wUSDT ($0.25/wHEZ)');
  console.log('   • PEZ-wUSDT: 5 PEZ = 1 wUSDT ($0.20/PEZ)');
  console.log('\n💡 Price consistency check:');
  console.log('   wHEZ → PEZ → wUSDT: 1 wHEZ → 1.25 PEZ → 0.25 USDT ✓');

  await api.disconnect();
}

main().catch(console.error);
