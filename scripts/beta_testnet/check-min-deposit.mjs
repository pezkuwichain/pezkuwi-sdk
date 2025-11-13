import { ApiPromise, WsProvider } from '@polkadot/api';

async function checkMinDeposit() {
  const wsProvider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  console.log('\n🔍 Checking minimum deposit requirements...\n');

  // Check PEZ asset (ID: 1)
  const pezAsset = await api.query.assets.asset(1);
  if (pezAsset.isSome) {
    const pezDetails = pezAsset.unwrap().toJSON();
    console.log('📦 PEZ Asset Details:', {
      minBalance: pezDetails.minBalance,
      minBalanceDecimal: Number(pezDetails.minBalance) / 1e12
    });
  }

  // Check USDT asset (ID: 2)
  const usdtAsset = await api.query.assets.asset(2);
  if (usdtAsset.isSome) {
    const usdtDetails = usdtAsset.unwrap().toJSON();
    console.log('📦 USDT Asset Details:', {
      minBalance: usdtDetails.minBalance,
      minBalanceDecimal: Number(usdtDetails.minBalance) / 1e6
    });
  }

  // Check assetConversion pallet constants
  console.log('\n🔧 AssetConversion Pallet Constants:');
  if (api.consts.assetConversion) {
    const constants = api.consts.assetConversion;
    Object.keys(constants).forEach(key => {
      console.log(`  - ${key}:`, constants[key].toHuman());
    });
  }

  // Check if there's MintMinLiquidity
  if (api.consts.assetConversion.mintMinLiquidity) {
    const mintMin = api.consts.assetConversion.mintMinLiquidity.toString();
    console.log('\n💎 MintMinLiquidity constant:', mintMin);
    console.log('   As decimal (12 decimals):', Number(mintMin) / 1e12);
  }

  await api.disconnect();
}

checkMinDeposit().catch(console.error);
