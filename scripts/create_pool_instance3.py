#!/usr/bin/env python3
"""
Create wHEZ/PEZ pool with the new Instance3 PoolAssets configuration
"""

from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException
import time

# Connect to node
print("🔗 Connecting to ws://127.0.0.1:9944...")
substrate = SubstrateInterface(url="ws://127.0.0.1:9944")

# Alice's keypair
alice = Keypair.create_from_uri("//Alice")
print(f"✅ Using Alice: {alice.ss58_address}")

# Asset IDs
WHEZ_ID = 0
PEZ_ID = 1

print("\n📊 Checking if pool already exists...")
try:
    pool_info = substrate.query(
        module='AssetConversion',
        storage_function='Pools',
        params=[[WHEZ_ID, PEZ_ID]]  # Sorted order
    )

    if pool_info.value is not None:
        print(f"✅ Pool already exists: {pool_info.value}")
        print("\n💧 Now you can add liquidity to the pool!")
        exit(0)
    else:
        print("ℹ️  Pool does not exist yet, creating...")
except Exception as e:
    print(f"⚠️  Error checking pool: {e}")
    print("Proceeding to create pool...")

print(f"\n🏊 Creating wHEZ({WHEZ_ID})/PEZ({PEZ_ID}) pool...")

# Create pool extrinsic
call = substrate.compose_call(
    call_module='AssetConversion',
    call_function='create_pool',
    call_params={
        'asset1': WHEZ_ID,
        'asset2': PEZ_ID,
    }
)

print(f"📝 Submitting transaction...")
try:
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=alice)
    receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

    print(f"✅ Transaction included in block: {receipt.block_hash}")
    print(f"   Extrinsic hash: {receipt.extrinsic_hash}")

    # Check for errors
    if receipt.is_success:
        print("✅ Pool created successfully!")

        # Query pool info
        time.sleep(2)
        pool_info = substrate.query(
            module='AssetConversion',
            storage_function='Pools',
            params=[[WHEZ_ID, PEZ_ID]]
        )
        print(f"\n📊 Pool Info: {pool_info.value}")
    else:
        print(f"❌ Transaction failed!")
        print(f"   Error: {receipt.error_message}")
        for event in receipt.triggered_events:
            print(f"   Event: {event.value}")

except SubstrateRequestException as e:
    print(f"❌ Error: {e}")
    exit(1)

print("\n🎉 Pool creation complete!")
print("\n💡 Next steps:")
print("   1. Add liquidity to the pool")
print("   2. Test swaps in the UI")
