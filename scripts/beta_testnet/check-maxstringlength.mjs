import { ApiPromise, WsProvider } from '@polkadot/api';

async function main() {
  const api = await ApiPromise.create({
    provider: new WsProvider('ws://127.0.0.1:9944')
  });

  console.log('\n🔍 CHECKING IDENTITY-KYC PALLET CONSTANTS:\n');

  // Get pallet constants
  const maxStringLength = api.consts.identityKyc.maxStringLength;
  const maxCidLength = api.consts.identityKyc.maxCidLength;

  console.log(`MaxStringLength: ${maxStringLength.toString()} bytes`);
  console.log(`MaxCidLength: ${maxCidLength.toString()} bytes`);

  // Test string
  const testHash = 'a'.repeat(64); // 64-char hash
  const testNotes = `Citizenship application - Hash: ${testHash}`;

  console.log(`\n📏 STRING LENGTH TEST:`);
  console.log(`Test string: "${testNotes}"`);
  console.log(`Test string length: ${testNotes.length} bytes`);
  console.log(`Max allowed: ${maxStringLength.toString()} bytes`);
  console.log(`Will it fit? ${testNotes.length <= maxStringLength.toNumber() ? '✅ YES' : '❌ NO - TOO LONG!'}`);

  // Show what length works
  const maxAllowed = maxStringLength.toNumber();
  console.log(`\n💡 SOLUTIONS:`);
  console.log(`1. Shorten to: "${testNotes.substring(0, maxAllowed)}" (${maxAllowed} chars)`);
  console.log(`2. Or increase MaxStringLength in runtime to: ${testNotes.length + 50} (recommended: 256)`);

  await api.disconnect();
}

main().catch(console.error);
