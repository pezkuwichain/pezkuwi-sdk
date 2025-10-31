const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const chalk = require('chalk');

async function runDevTests() {
    console.log(chalk.blue('🧪 Starting Dev Network Tests...\n'));
    
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });
    
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    
    const results = {
        network: 'dev',
        timestamp: new Date().toISOString(),
        passed: 0,
        failed: 0,
        tests: []
    };
    
    // Test 1: HEZ Transfer
    try {
        console.log(chalk.yellow('📝 Test 1: HEZ Transfer (Alice → Bob 100 HEZ)'));
        
        const balanceBefore = (await api.query.system.account(bob.address)).data.free;
        console.log(`   Bob balance before: ${balanceBefore.toHuman()}`);
        
        const transfer = api.tx.balances.transfer(bob.address, 100n * 1000000000000n);
        
        await new Promise((resolve, reject) => {
            transfer.signAndSend(alice, ({ status, events }) => {
                if (status.isInBlock) {
                    console.log(chalk.gray(`   In block: ${status.asInBlock.toHex()}`));
                    resolve();
                }
            });
        });
        
        const balanceAfter = (await api.query.system.account(bob.address)).data.free;
        console.log(`   Bob balance after: ${balanceAfter.toHuman()}`);
        
        if (balanceAfter.gt(balanceBefore)) {
            console.log(chalk.green('   ✅ PASSED\n'));
            results.passed++;
            results.tests.push({ 
                name: 'HEZ Transfer', 
                status: 'PASSED',
                details: {
                    from: 'Alice',
                    to: 'Bob',
                    amount: '100 HEZ',
                    balanceBefore: balanceBefore.toHuman(),
                    balanceAfter: balanceAfter.toHuman()
                }
            });
        } else {
            throw new Error('Balance did not increase');
        }
    } catch (error) {
        console.log(chalk.red(`   ❌ FAILED: ${error.message}\n`));
        results.failed++;
        results.tests.push({ 
            name: 'HEZ Transfer', 
            status: 'FAILED', 
            error: error.message 
        });
    }
    
    // Test 2: Check Validator
    try {
        console.log(chalk.yellow('📝 Test 2: Validator Count'));
        
        const validators = await api.query.session.validators();
        console.log(`   Active validators: ${validators.length}`);
        
        if (validators.length >= 1) {
            console.log(chalk.green('   ✅ PASSED\n'));
            results.passed++;
            results.tests.push({ 
                name: 'Validator Count', 
                status: 'PASSED',
                details: { count: validators.length }
            });
        } else {
            throw new Error(`Expected at least 1 validator, got ${validators.length}`);
        }
    } catch (error) {
        console.log(chalk.red(`   ❌ FAILED: ${error.message}\n`));
        results.failed++;
        results.tests.push({ 
            name: 'Validator Count', 
            status: 'FAILED', 
            error: error.message 
        });
    }
    
    // Test 3: PEZ Asset Exists
    try {
        console.log(chalk.yellow('📝 Test 3: PEZ Asset Check'));
        
        const assetId = 1;
        const asset = await api.query.assets.asset(assetId);
        
        if (asset.isSome) {
            const assetData = asset.unwrap();
            console.log(`   PEZ Asset found`);
            console.log(`   Owner: ${assetData.owner.toHuman()}`);
            console.log(chalk.green('   ✅ PASSED\n'));
            results.passed++;
            results.tests.push({ 
                name: 'PEZ Asset Check', 
                status: 'PASSED',
                details: { assetId: 1, owner: assetData.owner.toHuman() }
            });
        } else {
            throw new Error('PEZ asset not found');
        }
    } catch (error) {
        console.log(chalk.red(`   ❌ FAILED: ${error.message}\n`));
        results.failed++;
        results.tests.push({ 
            name: 'PEZ Asset Check', 
            status: 'FAILED', 
            error: error.message 
        });
    }
    
    // Summary
    console.log(chalk.blue('═══════════════════════════════════════'));
    console.log(chalk.blue('📊 Test Summary - Dev Network'));
    console.log(chalk.blue('═══════════════════════════════════════'));
    console.log(chalk.green(`✅ Passed: ${results.passed}`));
    console.log(chalk.red(`❌ Failed: ${results.failed}`));
    console.log(chalk.blue(`📅 Timestamp: ${results.timestamp}`));
    console.log(chalk.blue('═══════════════════════════════════════\n'));
    
    // Save report
    const fs = require('fs');
    fs.writeFileSync(
        'test-reports/dev-test-report.json', 
        JSON.stringify(results, null, 2)
    );
    console.log(chalk.gray('💾 Report saved to: test-reports/dev-test-report.json\n'));
    
    await api.disconnect();
    
    process.exit(results.failed > 0 ? 1 : 0);
}

runDevTests().catch(error => {
    console.error(chalk.red('Fatal error:'), error);
    process.exit(1);
});
