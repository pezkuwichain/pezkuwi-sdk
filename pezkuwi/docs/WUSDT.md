# wUSDT (Wrapped USDT) on Pezkuwichain

## Overview

wUSDT is a bridged stablecoin on Pezkuwichain pegged 1:1 to USDT (Tether). It allows users to bring USDT liquidity from external chains (Tron, Ethereum, BSC) into the Pezkuwichain ecosystem.

## Technical Specifications

- **Asset ID**: `1000`
- **Symbol**: `wUSDT`
- **Decimals**: `6` (matching USDT standard)
- **Minimum Balance**: `1,000` (0.001 USDT) - prevents dust
- **Pallet**: `pallet_assets`

## Implementation Phases

### Phase 1: Local Development (Current)
- ✅ Asset constants defined in `constants/src/lib.rs`
- 🔄 Genesis configuration
- 🔄 Test mint functionality
- 🔄 Frontend integration

### Phase 2: Custodial Bridge MVP
- Bridge service monitors external USDT deposits
- Automated minting on Pezkuwichain
- User deposits tracked in database
- Manual admin controls for security

### Phase 3: Semi-Decentralized
- Multisig custody (3/5 or 5/7 threshold)
- Validator-controlled bridge keys
- On-chain governance for parameters

### Phase 4: Fully Decentralized
- Light client bridge
- Trustless USDT ↔ wUSDT conversion
- On-chain proof verification

## Usage

### For Users

**Deposit USDT → Get wUSDT:**
1. Go to bridge.pezkuwichain.io
2. Generate deposit address
3. Send USDT (TRC20/ERC20/BEP20) to address
4. Receive wUSDT on Pezkuwichain (5-10 min)

**Withdraw wUSDT → Get USDT:**
1. Submit withdrawal request
2. Specify destination address
3. Pay bridge fee
4. Receive USDT on external chain

### For Developers

**Check wUSDT Balance:**
```typescript
const balance = await api.query.assets.account(WUSDT_ASSET_ID, userAddress);
const wusdtAmount = balance.unwrap().balance.toString();
// Convert to human-readable (6 decimals)
const readableAmount = Number(wusdtAmount) / 1_000_000;
```

**Transfer wUSDT:**
```typescript
await api.tx.assets.transfer(
  WUSDT_ASSET_ID,
  recipientAddress,
  amount * 1_000_000 // Convert to 6 decimals
).signAndSend(sender);
```

## Security Considerations

### Phase 1-2 (Custodial)
- ⚠️ Bridge operator controls minting
- ✅ Rate limiting implemented
- ✅ Maximum daily deposit limits
- ✅ Emergency pause functionality
- ✅ Reserve ratio monitoring (1:1 backing)

### Phase 3-4 (Decentralized)
- ✅ Multisig key management
- ✅ On-chain governance
- ✅ Audit trail for all mints/burns
- ✅ Cross-chain proof verification

## Audits

- [ ] Phase 2: Internal security review
- [ ] Phase 3: External audit (TBD)
- [ ] Phase 4: Full security audit by reputable firm

## Integration Support

**RPC Endpoint:** `wss://rpc.pezkuwichain.io`

**Asset Metadata:**
```json
{
  "assetId": 1000,
  "symbol": "wUSDT",
  "name": "Wrapped USDT",
  "decimals": 6,
  "isFrozen": false
}
```

## FAQ

**Q: Is wUSDT backed 1:1 by real USDT?**
A: Yes, every wUSDT is backed by USDT held in the bridge custody address. Reserve can be verified on-chain.

**Q: Can I use wUSDT in smart contracts?**
A: Yes, wUSDT is a standard asset on Pezkuwichain and can be used in any dApp or smart contract.

**Q: What are the fees?**
A: Bridge fee: 0.1% (min $1, max $100). On-chain transfer fee: ~$0.01 in HEZ.

**Q: How long does bridging take?**
A: Deposits: 5-10 minutes (depends on external chain confirmations)
   Withdrawals: 10-30 minutes (manual approval in Phase 1-2)

## Links

- Bridge UI: https://bridge.pezkuwichain.io (coming soon)
- Documentation: https://docs.pezkuwichain.io/wusdt
- Status Page: https://status.pezkuwichain.io/bridge

## Contact

For bridge-related issues:
- Email: bridge@pezkuwichain.io
- Telegram: @pezkuwi_bridge_support
- Discord: discord.gg/pezkuwi #bridge-support
