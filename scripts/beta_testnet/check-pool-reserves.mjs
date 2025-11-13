import { ApiPromise, WsProvider } from '@polkadot/api';
import { stringToU8a } from '@polkadot/util';
import { blake2AsU8a } from '@polkadot/util-crypto';

async function checkPoolReserves() {
  const wsProvider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  const PEZ_ID = 1;
  const USDT_ID = 2;

  console.log('\n🔍 Checking PEZ-USDT pool reserves...\n');

  // Derive pool account
  const PALLET_ID = stringToU8a('py/ascon');
  const poolIdType = api.createType('(u32, u32)', [PEZ_ID, USDT_ID]);
  const palletIdType = api.createType('[u8; 8]', PALLET_ID);
  const fullTuple = api.createType('([u8; 8], (u32, u32))', [palletIdType, poolIdType]);
  const accountHash = blake2AsU8a(fullTuple.toU8a(), 256);
  const poolAccountId = api.createType('AccountId32', accountHash);

  console.log(`📍 Pool Account: ${poolAccountId.toString()}\n`);

  // Check pool exists
  const poolInfo = await api.query.assetConversion.pools([PEZ_ID, USDT_ID]);
  console.log(`Pool exists: ${poolInfo.isSome}`);
  if (poolInfo.isSome) {
    console.log(`Pool info:`, poolInfo.unwrap().toHuman());
  }
  console.log();

  // Check PEZ reserve
  const pezBalance = await api.query.assets.account(PEZ_ID, poolAccountId);
  if (pezBalance.isSome) {
    const data = pezBalance.unwrap().toJSON();
    console.log(`📦 PEZ reserve:`, {
      raw: data.balance,
      decimal: Number(data.balance) / 1e12,
      status: data.status
    });
  } else {
    console.log(`📦 PEZ reserve: NONE`);
  }

  // Check USDT reserve
  const usdtBalance = await api.query.assets.account(USDT_ID, poolAccountId);
  if (usdtBalance.isSome) {
    const data = usdtBalance.unwrap().toJSON();
    console.log(`📦 USDT reserve:`, {
      raw: data.balance,
      decimal: Number(data.balance) / 1e6,
      status: data.status
    });
  } else {
    console.log(`📦 USDT reserve: NONE`);
  }

  // Check LP token supply
  const lpTokenId = await api.query.assetConversion.nextPoolAssetId();
  console.log(`\n🪙 Next LP Token ID: ${lpTokenId.toString()}`);

  // Try to find the actual LP token for this pool
  // LP tokens start from ID 0 in poolAssets
  for (let i = 0; i < 10; i++) {
    const assetInfo = await api.query.poolAssets.asset(i);
    if (assetInfo.isSome) {
      const info = assetInfo.unwrap().toJSON();
      console.log(`\n💎 LP Token ${i}:`, {
        supply: info.supply,
        supplyDecimal: Number(info.supply) / 1e12,
        accounts: info.accounts,
        sufficients: info.sufficients
      });
    }
  }

  await api.disconnect();
}

checkPoolReserves().catch(console.error);
