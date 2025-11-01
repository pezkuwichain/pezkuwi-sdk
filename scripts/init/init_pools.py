#!/usr/bin/env python3
"""
Automatic DEX Pool Initialization
Creates initial liquidity pools for HEZ/PEZ/wHEZ tokens
"""

import json
import time
import requests
from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException

# Colors for terminal output
class Colors:
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    RED = '\033[0;31m'
    CYAN = '\033[0;36m'
    NC = '\033[0m'  # No Color

def print_colored(message, color):
    print(f"{color}{message}{Colors.NC}")

# ⚠️ IMPORTANT: AssetConversion only supports Asset ↔ Asset swaps!
# Native HEZ ↔ wHEZ uses TokenWrapper.wrap/unwrap
# Only valid pool: wHEZ ↔ PEZ

# Asset IDs
WHEZ = 0       # Wrapped HEZ
PEZ = 1        # PEZ utility token

# Initial liquidity amounts (in Planck: 1 token = 10^12 Planck)
ONE_TOKEN = 10**12

# Only create wHEZ/PEZ pool
INITIAL_LIQUIDITY = {
    'WHEZ_PEZ': {
        'asset1': WHEZ,
        'asset2': PEZ,
        'amount1': 100_000 * ONE_TOKEN,   # 100K wHEZ
        'amount2': 500_000 * ONE_TOKEN,   # 500K PEZ (1:5 ratio)
    }
}

def wait_for_node(url="ws://127.0.0.1:9944", max_retries=30):
    """Wait for blockchain node to be ready"""
    print_colored("⏳ Waiting for node to start...", Colors.YELLOW)

    for i in range(max_retries):
        try:
            # Try HTTP RPC first
            response = requests.post(
                url.replace('ws://', 'http://').replace('9944', '9944'),
                json={"id": 1, "jsonrpc": "2.0", "method": "system_health", "params": []},
                timeout=2
            )
            if response.status_code == 200:
                print_colored("✅ Node is ready!", Colors.GREEN)
                return True
        except Exception:
            pass

        print_colored(f"Retry {i+1}/{max_retries}...", Colors.YELLOW)
        time.sleep(2)

    print_colored("❌ Node did not start in time!", Colors.RED)
    return False

def create_pool(substrate, keypair, asset1, asset2):
    """Create a liquidity pool"""
    print_colored(f"📦 Creating pool {asset1}/{asset2}...", Colors.CYAN)

    try:
        # Create the pool creation call - substrate-interface handles encoding
        pool_call = substrate.compose_call(
            call_module='AssetConversion',
            call_function='create_pool',
            call_params={
                'asset1': asset1,
                'asset2': asset2
            }
        )

        extrinsic = substrate.create_signed_extrinsic(call=pool_call, keypair=keypair)
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

        print_colored(f"✅ Pool {asset1}/{asset2} created! Block: {receipt.block_hash}", Colors.GREEN)
        return True

    except SubstrateRequestException as e:
        error_msg = str(e)
        if "PoolExists" in error_msg:
            print_colored(f"ℹ️  Pool {asset1}/{asset2} already exists", Colors.YELLOW)
            return True
        else:
            print_colored(f"❌ Failed to create pool: {error_msg}", Colors.RED)
            print_colored(f"   Trying alternative method...", Colors.YELLOW)
            return False
    except Exception as e:
        print_colored(f"❌ Unexpected error: {e}", Colors.RED)
        return False

def add_liquidity(substrate, keypair, asset1, asset2, amount1, amount2):
    """Add initial liquidity to pool"""
    print_colored(f"💧 Adding liquidity to {asset1}/{asset2}...", Colors.CYAN)
    print_colored(f"   Amount 1: {amount1 / ONE_TOKEN:,.0f} tokens", Colors.YELLOW)
    print_colored(f"   Amount 2: {amount2 / ONE_TOKEN:,.0f} tokens", Colors.YELLOW)

    try:
        # substrate-interface handles encoding automatically
        call = substrate.compose_call(
            call_module='AssetConversion',
            call_function='add_liquidity',
            call_params={
                'asset1': asset1,
                'asset2': asset2,
                'amount1_desired': amount1,
                'amount2_desired': amount2,
                'amount1_min': int(amount1 * 0.95),  # 5% slippage tolerance
                'amount2_min': int(amount2 * 0.95),
                'mint_to': keypair.ss58_address
            }
        )

        extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

        print_colored(f"✅ Liquidity added! Block: {receipt.block_hash}", Colors.GREEN)
        return True

    except Exception as e:
        print_colored(f"❌ Failed to add liquidity: {e}", Colors.RED)
        import traceback
        traceback.print_exc()
        return False

def wrap_hez_to_whez(substrate, keypair, amount):
    """Wrap native HEZ to wHEZ"""
    print_colored(f"🔄 Wrapping {amount / ONE_TOKEN:,.0f} HEZ to wHEZ...", Colors.CYAN)

    try:
        call = substrate.compose_call(
            call_module='TokenWrapper',
            call_function='wrap',
            call_params={
                'amount': amount
            }
        )

        extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

        print_colored(f"✅ Wrapped successfully! Block: {receipt.block_hash}", Colors.GREEN)
        return True
    except Exception as e:
        print_colored(f"❌ Failed to wrap HEZ: {e}", Colors.RED)
        return False

def main():
    print_colored("=" * 60, Colors.CYAN)
    print_colored("   DEX Pool Automatic Initialization (Python)", Colors.CYAN)
    print_colored("=" * 60, Colors.CYAN)
    print()

    # Wait for node
    if not wait_for_node():
        return 1

    print()

    # Connect to node
    print_colored("🔗 Connecting to blockchain...", Colors.CYAN)
    try:
        substrate = SubstrateInterface(
            url="ws://127.0.0.1:9944",
            ss58_format=42,  # Generic Substrate format
            type_registry_preset='substrate-node-template'
        )
        print_colored("✅ Connected successfully!", Colors.GREEN)
    except Exception as e:
        print_colored(f"❌ Connection failed: {e}", Colors.RED)
        return 1

    print()

    # Use Alice account for dev/local networks
    print_colored("🔑 Using Alice account (//Alice)", Colors.CYAN)
    keypair = Keypair.create_from_uri('//Alice')
    print_colored(f"   Address: {keypair.ss58_address}", Colors.YELLOW)

    print()
    print_colored("🏊 Creating wHEZ/PEZ liquidity pool...", Colors.CYAN)
    print_colored("ℹ️  Note: Native HEZ ↔ wHEZ uses TokenWrapper.wrap/unwrap", Colors.YELLOW)
    print()

    # Step 1: Wrap HEZ to wHEZ (need 100K wHEZ for liquidity)
    print_colored("📍 Step 1: Wrapping HEZ to wHEZ for liquidity...", Colors.CYAN)
    wrap_amount = 150_000 * ONE_TOKEN  # Wrap 150K HEZ to be safe
    if not wrap_hez_to_whez(substrate, keypair, wrap_amount):
        print_colored("⚠️  Failed to wrap HEZ, continuing anyway...", Colors.YELLOW)
    time.sleep(2)
    print()

    # Step 2: Create pools
    print_colored("📍 Step 2: Creating wHEZ/PEZ liquidity pool...", Colors.CYAN)
    pools_created = 0
    for pool_name, config in INITIAL_LIQUIDITY.items():
        print_colored(f"▶ Pool: {pool_name}", Colors.YELLOW)

        if create_pool(substrate, keypair, config['asset1'], config['asset2']):
            pools_created += 1
            time.sleep(2)  # Wait for block finalization

        print()

    print_colored(f"✅ {pools_created}/1 pool created!", Colors.GREEN)
    print()

    # Step 3: Add liquidity
    print_colored("📍 Step 3: Adding initial liquidity (100K wHEZ + 500K PEZ)...", Colors.CYAN)
    print()

    liquidity_added = 0
    for pool_name, config in INITIAL_LIQUIDITY.items():
        print_colored(f"▶ Pool: {pool_name}", Colors.YELLOW)

        if add_liquidity(
            substrate, keypair,
            config['asset1'], config['asset2'],
            config['amount1'], config['amount2']
        ):
            liquidity_added += 1
            time.sleep(2)

        print()

    print_colored(f"✅ {liquidity_added}/1 pool funded!", Colors.GREEN)
    print()
    print_colored("=" * 60, Colors.GREEN)
    print_colored("   🎉 DEX initialization complete!", Colors.GREEN)
    print_colored("=" * 60, Colors.GREEN)

    return 0

if __name__ == "__main__":
    try:
        exit(main())
    except KeyboardInterrupt:
        print()
        print_colored("⚠️  Interrupted by user", Colors.YELLOW)
        exit(1)
    except Exception as e:
        print_colored(f"❌ Unexpected error: {e}", Colors.RED)
        exit(1)
