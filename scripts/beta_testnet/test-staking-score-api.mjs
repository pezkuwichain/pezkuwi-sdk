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

  console.log('🔍 Testing StakingScore API\n');
  console.log('Founder address:', founder.address);
  console.log('');

  // Check if stakingScore pallet exists
  console.log('📦 Available pallets:');
  if (api.query.stakingScore) {
    console.log('  ✓ stakingScore pallet found');
    console.log('  Storage queries:', Object.keys(api.query.stakingScore));
  } else {
    console.log('  ✗ stakingScore pallet NOT found');
  }

  console.log('');

  // Check runtime APIs
  console.log('🔌 Checking runtime APIs:');
  if (api.call) {
    console.log('  Available APIs:', Object.keys(api.call));

    // Check if stakingScore API exists
    if (api.call.stakingScoreApi || api.call.StakingScoreApi) {
      console.log('  ✓ StakingScore runtime API found');
      const apiObj = api.call.stakingScoreApi || api.call.StakingScoreApi;
      console.log('  Methods:', Object.keys(apiObj));
    } else {
      console.log('  ✗ No StakingScore runtime API found');
    }
  }

  console.log('');

  // Query stakingStartBlock for founder
  try {
    const startBlockResult = await api.query.stakingScore.stakingStartBlock(founder.address);

    if (startBlockResult.isSome) {
      const startBlock = startBlockResult.unwrap().toNumber();
      const currentBlock = (await api.query.system.number()).toNumber();
      const duration = currentBlock - startBlock;

      console.log('📊 Staking Score Data:');
      console.log('  Start Block:', startBlock);
      console.log('  Current Block:', currentBlock);
      console.log('  Duration (blocks):', duration);
      console.log('  Duration (approx days):', Math.floor(duration / (24 * 60 * 10)));
    } else {
      console.log('⚠️  No staking start block found for founder');
      console.log('   (User has not called startScoreTracking yet)');
    }
  } catch (error) {
    console.error('❌ Error querying staking start block:', error.message);
  }

  console.log('');

  // Check if we can query staking details
  try {
    const ledgerResult = await api.query.staking.ledger(founder.address);

    if (ledgerResult.isSome) {
      const ledger = ledgerResult.unwrap();
      const total = BigInt(ledger.total.toString());
      const totalHEZ = total / BigInt(10**12);

      console.log('💰 Staking Details:');
      console.log('  Total staked:', totalHEZ.toString(), 'HEZ');
    } else {
      console.log('  No staking ledger found');
    }
  } catch (error) {
    console.error('❌ Error querying staking ledger:', error.message);
  }

  await api.disconnect();
}

main().catch(console.error);
