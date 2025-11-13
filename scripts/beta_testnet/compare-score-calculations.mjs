import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';

// Frontend calculation (from staking.ts)
function calculateFrontendScore(stakedHEZ, durationInBlocks) {
  // Amount-based score (20-50 points)
  let amountScore = 20;

  if (stakedHEZ <= 100) {
    amountScore = 20;
  } else if (stakedHEZ <= 250) {
    amountScore = 30;
  } else if (stakedHEZ <= 750) {
    amountScore = 40;
  } else {
    amountScore = 50; // 751+ HEZ
  }

  // Duration multiplier
  const MONTH_IN_BLOCKS = 30 * 24 * 60 * 10; // 432,000 blocks
  let durationMultiplier = 1.0;

  if (durationInBlocks >= 12 * MONTH_IN_BLOCKS) {
    durationMultiplier = 2.0; // 12+ months
  } else if (durationInBlocks >= 6 * MONTH_IN_BLOCKS) {
    durationMultiplier = 1.7; // 6-11 months
  } else if (durationInBlocks >= 3 * MONTH_IN_BLOCKS) {
    durationMultiplier = 1.4; // 3-5 months
  } else if (durationInBlocks >= MONTH_IN_BLOCKS) {
    durationMultiplier = 1.2; // 1-2 months
  } else {
    durationMultiplier = 1.0; // < 1 month
  }

  // Final score (max 100)
  return Math.min(100, Math.floor(amountScore * durationMultiplier));
}

// Pallet calculation (exact replica from lib.rs)
function calculatePalletScore(stakedHEZ, durationInBlocks) {
  const MONTH_IN_BLOCKS = 30 * 24 * 60 * 10; // 432,000 blocks

  // Amount-based score
  let amountScore;
  if (stakedHEZ <= 100) {
    amountScore = 20;
  } else if (stakedHEZ <= 250) {
    amountScore = 30;
  } else if (stakedHEZ <= 750) {
    amountScore = 40;
  } else {
    amountScore = 50; // 751+ HEZ
  }

  // Duration-based multiplier (using integer math like Rust)
  let finalScore;
  if (durationInBlocks >= 12 * MONTH_IN_BLOCKS) {
    finalScore = amountScore * 2; // x2.0
  } else if (durationInBlocks >= 6 * MONTH_IN_BLOCKS) {
    finalScore = Math.floor(amountScore * 17 / 10); // x1.7
  } else if (durationInBlocks >= 3 * MONTH_IN_BLOCKS) {
    finalScore = Math.floor(amountScore * 14 / 10); // x1.4
  } else if (durationInBlocks >= MONTH_IN_BLOCKS) {
    finalScore = Math.floor(amountScore * 12 / 10); // x1.2
  } else {
    finalScore = amountScore; // x1.0
  }

  return Math.min(100, finalScore);
}

async function main() {
  await cryptoWaitReady();

  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const founder = keyring.addFromUri('skill dose toward always latin fish film cabbage praise blouse kingdom depth');

  console.log('🧮 Comparing Score Calculation: Frontend vs Pallet\n');

  // Get actual data from blockchain
  const ledgerResult = await api.query.staking.ledger(founder.address);
  const startBlockResult = await api.query.stakingScore.stakingStartBlock(founder.address);
  const currentBlock = (await api.query.system.number()).toNumber();

  if (ledgerResult.isNone) {
    console.log('❌ No staking ledger found');
    await api.disconnect();
    return;
  }

  if (startBlockResult.isNone) {
    console.log('❌ No staking start block found');
    await api.disconnect();
    return;
  }

  const ledger = ledgerResult.unwrap();
  const totalStaked = BigInt(ledger.total.toString());
  const stakedHEZ = Number(totalStaked / BigInt(10**12));

  const startBlock = startBlockResult.unwrap().toNumber();
  const durationInBlocks = currentBlock - startBlock;
  const durationInDays = Math.floor(durationInBlocks / (24 * 60 * 10));

  console.log('📊 Blockchain Data:');
  console.log('  Address:', founder.address);
  console.log('  Staked:', stakedHEZ, 'HEZ');
  console.log('  Start Block:', startBlock);
  console.log('  Current Block:', currentBlock);
  console.log('  Duration:', durationInBlocks, 'blocks (~' + durationInDays + ' days)');
  console.log('');

  // Test multiple scenarios
  const testCases = [
    { hez: stakedHEZ, blocks: durationInBlocks, desc: 'Actual current state' },
    { hez: 50, blocks: 100000, desc: 'Low stake, short time' },
    { hez: 100, blocks: 100000, desc: '100 HEZ, short time' },
    { hez: 200, blocks: 500000, desc: '200 HEZ, ~1 month' },
    { hez: 500, blocks: 1500000, desc: '500 HEZ, ~3 months' },
    { hez: 1000, blocks: 3000000, desc: '1000 HEZ, ~6 months' },
    { hez: 2000, blocks: 6000000, desc: '2000 HEZ, ~12 months' },
  ];

  console.log('🔬 Testing Score Calculations:\n');

  let allMatch = true;

  for (const test of testCases) {
    const frontendScore = calculateFrontendScore(test.hez, test.blocks);
    const palletScore = calculatePalletScore(test.hez, test.blocks);
    const match = frontendScore === palletScore;

    if (!match) allMatch = false;

    const status = match ? '✅' : '❌';
    console.log(`${status} ${test.desc}`);
    console.log(`   ${test.hez} HEZ, ${test.blocks} blocks`);
    console.log(`   Frontend: ${frontendScore} | Pallet: ${palletScore}`);
    if (!match) {
      console.log(`   ⚠️  MISMATCH! Difference: ${Math.abs(frontendScore - palletScore)}`);
    }
    console.log('');
  }

  if (allMatch) {
    console.log('✅ All calculations match! Frontend = Pallet logic');
  } else {
    console.log('❌ Some calculations differ! Frontend ≠ Pallet logic');
  }

  await api.disconnect();
}

main().catch(console.error);
