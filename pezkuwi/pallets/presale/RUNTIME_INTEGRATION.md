# Runtime Integration Guide - pallet-presale

## Prerequisites

- Substrate node with Polkadot SDK stable2407
- pallet-assets already integrated
- Access to runtime source code

## Step 1: Add Dependency

In your runtime's `Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
pallet-presale = { path = "../../pallets/presale", default-features = false }
```

Under `[features]`:

```toml
std = [
    # ... existing std features ...
    "pallet-presale/std",
]
```

## Step 2: Configure Pallet

In your runtime's `lib.rs`, add parameter types:

```rust
parameter_types! {
    pub const PresalePalletId: PalletId = PalletId(*b"py/prsal");
    pub const WUsdtAssetId: u32 = 2; // wUSDT asset ID
    pub const PezAssetId: u32 = 1;   // PEZ asset ID
    pub const ConversionRate: u128 = 100; // 1 wUSDT = 100 PEZ
    pub const PresaleDuration: BlockNumber = 45 * 24 * 60 * 10; // 45 days (assuming 6s blocks)
}
```

## Step 3: Implement Config

```rust
impl pallet_presale::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type PalletId = PresalePalletId;
    type WUsdtAssetId = WUsdtAssetId;
    type PezAssetId = PezAssetId;
    type ConversionRate = ConversionRate;
    type PresaleDuration = PresaleDuration;
}
```

## Step 4: Add to construct_runtime!

```rust
construct_runtime!(
    pub struct Runtime {
        // ... existing pallets ...
        Assets: pallet_assets,
        // Add presale pallet:
        Presale: pallet_presale,
    }
);
```

## Step 5: Fund Treasury

Before starting presale, ensure the presale treasury has enough PEZ tokens.

Get treasury address:
```rust
// Pallet ID: py/prsal
// Treasury AccountId: (derived from PalletId)
```

Transfer PEZ to treasury:
```bash
# Using polkadot.js or extrinsic
assets.transfer(
    1,  // PEZ asset ID
    presale_treasury_address,
    total_pez_needed
)
```

## Step 6: Build Runtime

```bash
cd runtime/pezkuwichain
cargo build --release --features runtime-benchmarks
```

## Step 7: Deploy

1. Stop validator node
2. Replace runtime WASM
3. Restart node
4. Verify pallet exists: `api.query.presale.presaleActive()`

## Step 8: Start Presale

```bash
# Via sudo
sudo.sudo(presale.startPresale())
```

## Presale Flow

1. **Start**: Sudo calls `presale.startPresale()`
2. **Contribute**: Users call `presale.contribute(amount)` with wUSDT
3. **Wait**: 45 days pass
4. **Finalize**: Sudo calls `presale.finalizePresale()` to distribute PEZ

## Queries

- `presale.presaleActive()` - Is presale active?
- `presale.totalRaised()` - Total wUSDT raised
- `presale.contributions(AccountId)` - User's contribution
- `presale.presaleStartBlock()` - Start block number

## Emergency

- `presale.emergencyPause()` - Pause contributions
- `presale.emergencyUnpause()` - Resume contributions

## Example Calculations

### Contribution: 100 wUSDT
- wUSDT (6 decimals): 100_000_000
- Conversion rate: 100
- PEZ units: (100_000_000 * 100) / 1_000_000 = 10_000
- PEZ with 12 decimals: 10_000 * 1_000_000_000_000 = 10_000_000_000_000_000
- Human-readable: 10,000 PEZ

### Presale Duration
- Blocks: 45 * 24 * 60 * 10 = 648,000 blocks
- Seconds (6s blocks): 3,888,000s = 45 days

## Security Notes

1. Treasury must be funded with PEZ before finalization
2. Use sudo account from secure cold storage
3. Test on testnet first
4. Monitor for anomalies during presale
5. Consider multi-sig for sudo operations

## Testing Checklist

- [ ] Pallet compiles
- [ ] Runtime builds
- [ ] Start presale works
- [ ] Contribute works (wUSDT transfer)
- [ ] Contribution tracking works
- [ ] Time calculation correct
- [ ] Finalize works (PEZ distribution)
- [ ] Emergency pause works
- [ ] Frontend integration works
