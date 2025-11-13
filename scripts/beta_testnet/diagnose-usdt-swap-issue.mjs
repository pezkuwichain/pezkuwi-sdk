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

  console.log('🔍 Diagnosing USDT Swap Issue...\n');
  console.log('Founder address:', founder.address);

  const WHEZ_ASSET = 0;
  const PEZ_ASSET = 1;
  const WUSDT_ASSET = 2;

  // Check founder balances
  console.log('\n💰 Founder Balances:');
  const { data: hezBalance } = await api.query.system.account(founder.address);
  console.log('  HEZ:', (BigInt(hezBalance.free.toString()) / BigInt(10**12)).toString());

  const whezBalance = await api.query.assets.account(WHEZ_ASSET, founder.address);
  console.log('  wHEZ:', whezBalance.isSome ? (BigInt(whezBalance.unwrap().balance.toString()) / BigInt(10**12)).toString() : '0');

  const pezBalance = await api.query.assets.account(PEZ_ASSET, founder.address);
  console.log('  PEZ:', pezBalance.isSome ? (BigInt(pezBalance.unwrap().balance.toString()) / BigInt(10**12)).toString() : '0');

  const wusdtBalance = await api.query.assets.account(WUSDT_ASSET, founder.address);
  console.log('  wUSDT:', wusdtBalance.isSome ? (BigInt(wusdtBalance.unwrap().balance.toString()) / BigInt(10**6)).toString() : '0');

  // Check pool existence
  console.log('\n💧 Pool Status:');

  // HEZ-USDT pool
  const hezUsdtPool = await api.query.assetConversion.pools([WHEZ_ASSET, WUSDT_ASSET]);
  if (hezUsdtPool.isSome) {
    console.log('  ✅ wHEZ-wUSDT pool exists');
    try {
      const reserves = await api.call.assetConversionApi.getReserves(WHEZ_ASSET, WUSDT_ASSET);
      if (reserves.isSome) {
        const [r0, r1] = reserves.unwrap();
        console.log('    wHEZ reserve:', (BigInt(r0.toString()) / BigInt(10**12)).toString());
        console.log('    wUSDT reserve:', (BigInt(r1.toString()) / BigInt(10**6)).toString());
      }
    } catch (e) {
      console.log('    ⚠️  Reserve query skipped (decoding issue)');
    }
  } else {
    console.log('  ❌ wHEZ-wUSDT pool not found');
  }

  // PEZ-USDT pool
  const pezUsdtPool = await api.query.assetConversion.pools([PEZ_ASSET, WUSDT_ASSET]);
  if (pezUsdtPool.isSome) {
    console.log('  ✅ PEZ-wUSDT pool exists');
    try {
      const reserves = await api.call.assetConversionApi.getReserves(PEZ_ASSET, WUSDT_ASSET);
      if (reserves.isSome) {
        const [r0, r1] = reserves.unwrap();
        console.log('    PEZ reserve:', (BigInt(r0.toString()) / BigInt(10**12)).toString());
        console.log('    wUSDT reserve:', (BigInt(r1.toString()) / BigInt(10**6)).toString());
      }
    } catch (e) {
      console.log('    ⚠️  Reserve query skipped (decoding issue)');
    }
  } else {
    console.log('  ❌ PEZ-wUSDT pool not found');
  }

  // Check minBalance requirements
  console.log('\n📏 MinBalance Requirements:');
  const whezMeta = await api.query.assets.asset(WHEZ_ASSET);
  const pezMeta = await api.query.assets.asset(PEZ_ASSET);
  const wusdtMeta = await api.query.assets.asset(WUSDT_ASSET);

  if (whezMeta.isSome) {
    console.log('  wHEZ minBalance:', (BigInt(whezMeta.unwrap().minBalance.toString()) / BigInt(10**12)).toString(), 'wHEZ');
  }
  if (pezMeta.isSome) {
    console.log('  PEZ minBalance:', (BigInt(pezMeta.unwrap().minBalance.toString()) / BigInt(10**12)).toString(), 'PEZ');
  }
  if (wusdtMeta.isSome) {
    console.log('  wUSDT minBalance:', (BigInt(wusdtMeta.unwrap().minBalance.toString()) / BigInt(10**6)).toString(), 'wUSDT');
  }

  // Test swap calculation: USDT → PEZ
  console.log('\n🧮 Testing Swap Calculation: 1 wUSDT → PEZ');
  try {
    const swapAmount = BigInt(1) * BigInt(10**6); // 1 USDT
    const quote = await api.call.assetConversionApi.quotePriceExactTokensForTokens(
      WUSDT_ASSET,
      PEZ_ASSET,
      swapAmount.toString(),
      false // includesFee
    );

    if (quote.isSome) {
      const output = quote.unwrap();
      console.log('  Expected output:', (BigInt(output.toString()) / BigInt(10**12)).toString(), 'PEZ');
    } else {
      console.log('  ❌ Quote calculation failed');
    }
  } catch (error) {
    console.error('  ❌ Quote error:', error.message);
  }

  // Test swap calculation: USDT → HEZ
  console.log('\n🧮 Testing Swap Calculation: 1 wUSDT → wHEZ');
  try {
    const swapAmount = BigInt(1) * BigInt(10**6); // 1 USDT
    const quote = await api.call.assetConversionApi.quotePriceExactTokensForTokens(
      WUSDT_ASSET,
      WHEZ_ASSET,
      swapAmount.toString(),
      false
    );

    if (quote.isSome) {
      const output = quote.unwrap();
      console.log('  Expected output:', (BigInt(output.toString()) / BigInt(10**12)).toString(), 'wHEZ');
    } else {
      console.log('  ❌ Quote calculation failed');
    }
  } catch (error) {
    console.error('  ❌ Quote error:', error.message);
  }

  console.log('\n✅ Diagnosis complete!');
  await api.disconnect();
}

main().catch(console.error);
