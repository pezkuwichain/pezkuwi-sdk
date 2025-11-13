// Calculate minimum liquidity required for PEZ-USDT pool
// MintMinLiquidity = 100 (raw units)

const MINT_MIN_LIQUIDITY = 100;
const PEZ_DECIMALS = 12;
const USDT_DECIMALS = 6;

console.log('\n🧮 Calculating minimum deposit for PEZ-USDT pool\n');
console.log(`MintMinLiquidity: ${MINT_MIN_LIQUIDITY} (raw units)\n`);

// We need: sqrt(amount0_raw * amount1_raw) >= MINT_MIN_LIQUIDITY
// So: amount0_raw * amount1_raw >= MINT_MIN_LIQUIDITY^2

const minProduct = MINT_MIN_LIQUIDITY * MINT_MIN_LIQUIDITY;
console.log(`Minimum product (raw): ${minProduct}\n`);

// Test different combinations
const testCases = [
  { pez: 0.01, usdt: 0.01 },
  { pez: 0.1, usdt: 0.1 },
  { pez: 1, usdt: 1 },
  { pez: 10, usdt: 10 },
  { pez: 100, usdt: 100 },
  { pez: 200, usdt: 10 },
  { pez: 1000, usdt: 100 },
];

console.log('Testing different deposit amounts:\n');
testCases.forEach(({ pez, usdt }) => {
  const pezRaw = BigInt(Math.floor(pez * Math.pow(10, PEZ_DECIMALS)));
  const usdtRaw = BigInt(Math.floor(usdt * Math.pow(10, USDT_DECIMALS)));

  const product = pezRaw * usdtRaw;
  const liquidityMinted = Math.floor(Math.sqrt(Number(product)));

  const status = liquidityMinted >= MINT_MIN_LIQUIDITY ? '✅ PASS' : '❌ FAIL';

  console.log(`${pez} PEZ + ${usdt} USDT:`);
  console.log(`  Liquidity minted: ${liquidityMinted}`);
  console.log(`  Status: ${status}\n`);
});

// Calculate exact minimum
console.log('\n📊 Exact minimum requirements:\n');
console.log('For equal ratio (1:1):');
const minEach = Math.ceil(Math.sqrt(minProduct));
console.log(`  Each asset needs: ${minEach} raw units`);
console.log(`  PEZ: ${minEach / Math.pow(10, PEZ_DECIMALS)} PEZ`);
console.log(`  USDT: ${minEach / Math.pow(10, USDT_DECIMALS)} USDT\n`);

// Practical minimum with safety margin
const safetyFactor = 1.1; // 10% safety margin
const safeMinEach = Math.ceil(minEach * safetyFactor);
console.log('Recommended minimum (with 10% safety margin):');
console.log(`  PEZ: ${safeMinEach / Math.pow(10, PEZ_DECIMALS)} PEZ`);
console.log(`  USDT: ${safeMinEach / Math.pow(10, USDT_DECIMALS)} USDT`);
