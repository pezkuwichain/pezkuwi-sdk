# USDT Swap Issue - Diagnostic Report

**Date:** 2025-11-11
**Status:** ❌ ROOT CAUSE IDENTIFIED

## 🔍 Problem

USDT → PEZ and USDT → HEZ swap operations fail with "insufficient balance" error, even though the founder wallet has 955,065 wUSDT.

## 💡 Root Cause

The **`AssetConversionApi` runtime API is MISSING** from the Pezkuwichain runtime.

### Evidence

1. **Runtime APIs Available:**
```
Available runtime APIs:
- accountNonceApi
- authorityDiscoveryApi
- babeApi
- beefyMmrApi
- blockBuilder
- core
- genesisBuilder
- grandpaApi
- metadata
- mmrApi
- offchainWorkerApi
- sessionKeys
- taggedTransactionQueue
- transactionPaymentApi
```

❌ **AssetConversionApi is NOT in the list!**

2. **Error from diagnostic script:**
```
❌ Quote error: Cannot read properties of undefined (reading 'quotePriceExactTokensForTokens')
```

The polkadot.js API tries to call `api.call.assetConversionApi.quotePriceExactTokensForTokens()` but `assetConversionApi` is `undefined` because the runtime doesn't expose it.

## 📊 Diagnostic Results

✅ **Founder Balances:**
- HEZ: 68,799,667
- wHEZ: 99,988
- PEZ: 92,399,927
- wUSDT: 955,065

✅ **Pools Exist:**
- wHEZ-wUSDT pool: ✅ EXISTS
- PEZ-wUSDT pool: ✅ EXISTS

✅ **MinBalance Requirements:**
- wHEZ minBalance: 0
- PEZ minBalance: 0
- wUSDT minBalance: 0

❌ **Swap Quote Calculation:**
- Cannot calculate quotes because runtime API is missing

## 🔧 Solution

Add the `AssetConversionApi` implementation to the Pezkuwichain runtime.

### Location
File: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/runtime/pezkuwichain/src/lib.rs`

### Code to Add (Inside `sp_api::impl_runtime_apis!` block)

```rust
impl pallet_asset_conversion::AssetConversionApi<
    Block,
    Balance,
    u32,
> for Runtime
{
    fn quote_price_exact_tokens_for_tokens(
        asset1: u32,
        asset2: u32,
        amount: Balance,
        include_fee: bool,
    ) -> Option<Balance> {
        AssetConversion::quote_price_exact_tokens_for_tokens(asset1, asset2, amount, include_fee)
    }

    fn quote_price_tokens_for_exact_tokens(
        asset1: u32,
        asset2: u32,
        amount: Balance,
        include_fee: bool,
    ) -> Option<Balance> {
        AssetConversion::quote_price_tokens_for_exact_tokens(asset1, asset2, amount, include_fee)
    }

    fn get_reserves(asset1: u32, asset2: u32) -> Option<(Balance, Balance)> {
        AssetConversion::get_reserves(asset1, asset2).ok()
    }
}
```

### Why `u32` instead of `NativeOrWithId<u32>`?

The Pezkuwichain runtime config shows:
```rust
impl pallet_asset_conversion::Config for Runtime {
    type AssetKind = u32;  // ← Uses u32 directly
    ...
}
```

The Substrate node runtime uses `NativeOrWithId<u32>` because it supports native token + asset IDs. Pezkuwichain uses only asset IDs (u32).

## 📝 Next Steps

1. ✅ Add `AssetConversionApi` implementation to runtime
2. 🔨 Rebuild the runtime: `cargo build --release`
3. 🔄 Restart all validators with new runtime
4. ✅ Test USDT→PEZ and USDT→HEZ swaps
5. ✅ Verify swap quote calculations work

## 🎯 Expected Outcome

After adding the runtime API and restarting validators:
- `api.call.assetConversionApi` will be defined
- Swap quote calculations will work
- USDT→PEZ and USDT→HEZ swaps will succeed
- Frontend swap functionality will work correctly

## 📍 Reference

- Substrate node runtime implementation: `/home/mamostehp/Pezkuwi-SDK/substrate/bin/node/runtime/src/lib.rs:3540-3557`
- Diagnostic script: `/home/mamostehp/Pezkuwi-SDK/scripts/beta_testnet/diagnose-usdt-swap-issue.mjs`
