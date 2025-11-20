# 🌉 wUSDT Bridge Service Architecture

**Version:** 1.0.0 (Phase 2 - Custodial Bridge MVP)
**Status:** Design Document
**Target Deployment:** Post-Beta Testnet

---

## 📋 Overview

The wUSDT Bridge Service enables users to deposit USDT from external chains (Tron, Ethereum, BSC) and receive wUSDT on Pezkuwichain, and vice versa for withdrawals.

**Key Properties:**
- **Phase 2 Implementation:** Custodial service with multisig controls
- **Asset ID:** 1000 (on Pezkuwichain)
- **Decimals:** 6 (matching USDT standard)
- **Initial Chains:** TRC20 (Tron), ERC20 (Ethereum), BEP20 (BSC)

---

## 🏗️ Architecture Components

```
┌─────────────────────────────────────────────────────────────────┐
│                     External Blockchains                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                      │
│  │  Tron    │  │ Ethereum │  │   BSC    │                      │
│  │ (TRC20)  │  │ (ERC20)  │  │ (BEP20)  │                      │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘                      │
└───────┼─────────────┼─────────────┼──────────────────────────┘
        │             │             │
        │  USDT Deposits/Withdrawals │
        │             │             │
┌───────▼─────────────▼─────────────▼──────────────────────────────┐
│              Bridge Service Backend (Node.js)                    │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Deposit Monitor  │  Withdrawal Processor  │  API Server │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │         PostgreSQL Database (Supabase)                    │   │
│  │  - Deposit Records   - Withdrawal Requests                │   │
│  │  - User Mappings     - Transaction History                │   │
│  └──────────────────────────────────────────────────────────┘   │
└────────────────────────┬──────────────────────────────────────┘
                         │
                         │ Polkadot.js API
                         │
┌────────────────────────▼──────────────────────────────────────┐
│                  Pezkuwichain Runtime                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │ pallet_assets│  │pallet_multisig│  │  Validators  │        │
│  │ (wUSDT: 1000)│  │ (3/5 signers) │  │              │        │
│  └──────────────┘  └──────────────┘  └──────────────┘        │
└───────────────────────────────────────────────────────────────┘
```

---

## 💾 Database Schema

### Table: `usdt_deposits`
```sql
CREATE TABLE usdt_deposits (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  -- External Chain Info
  external_chain VARCHAR(10) NOT NULL, -- 'tron', 'ethereum', 'bsc'
  external_tx_hash VARCHAR(66) NOT NULL UNIQUE,
  external_from_address VARCHAR(128) NOT NULL,
  external_to_address VARCHAR(128) NOT NULL, -- Bridge custody address

  -- Amount Info
  usdt_amount DECIMAL(20, 6) NOT NULL, -- 6 decimals (USDT standard)
  wusdt_amount DECIMAL(20, 6) NOT NULL, -- Should match after fee
  bridge_fee DECIMAL(20, 6) DEFAULT 0,

  -- Pezkuwichain Info
  pezkuwi_address VARCHAR(64) NOT NULL, -- SS58 address
  pezkuwi_tx_hash VARCHAR(66), -- Mint transaction hash

  -- Status Tracking
  status VARCHAR(20) NOT NULL DEFAULT 'pending',
    -- 'pending', 'confirmed', 'minting', 'minted', 'failed'
  confirmations INTEGER DEFAULT 0,
  required_confirmations INTEGER NOT NULL,

  -- Timestamps
  detected_at TIMESTAMP NOT NULL DEFAULT NOW(),
  confirmed_at TIMESTAMP,
  minted_at TIMESTAMP,

  -- Metadata
  user_memo TEXT, -- Optional user note
  admin_notes TEXT,

  CONSTRAINT positive_amount CHECK (usdt_amount > 0),
  CONSTRAINT valid_status CHECK (status IN ('pending', 'confirmed', 'minting', 'minted', 'failed'))
);

CREATE INDEX idx_deposits_status ON usdt_deposits(status);
CREATE INDEX idx_deposits_external_tx ON usdt_deposits(external_tx_hash);
CREATE INDEX idx_deposits_pezkuwi_addr ON usdt_deposits(pezkuwi_address);
```

### Table: `usdt_withdrawals`
```sql
CREATE TABLE usdt_withdrawals (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  -- Pezkuwichain Info
  pezkuwi_address VARCHAR(64) NOT NULL,
  pezkuwi_tx_hash VARCHAR(66) NOT NULL UNIQUE, -- Burn transaction
  wusdt_burned DECIMAL(20, 6) NOT NULL,

  -- External Chain Info
  target_chain VARCHAR(10) NOT NULL, -- 'tron', 'ethereum', 'bsc'
  target_address VARCHAR(128) NOT NULL,
  usdt_amount DECIMAL(20, 6) NOT NULL,
  bridge_fee DECIMAL(20, 6) DEFAULT 0,
  external_tx_hash VARCHAR(66), -- USDT send transaction

  -- Status & Timing
  status VARCHAR(20) NOT NULL DEFAULT 'pending',
    -- 'pending', 'approved', 'processing', 'sent', 'failed'
  withdrawal_tier VARCHAR(10), -- 'instant', 'standard', 'large'
  delay_seconds INTEGER,
  available_at TIMESTAMP,

  -- Timestamps
  requested_at TIMESTAMP NOT NULL DEFAULT NOW(),
  approved_at TIMESTAMP,
  sent_at TIMESTAMP,

  -- Metadata
  admin_notes TEXT,
  failure_reason TEXT,

  CONSTRAINT positive_burned CHECK (wusdt_burned > 0),
  CONSTRAINT valid_withdrawal_status CHECK (status IN ('pending', 'approved', 'processing', 'sent', 'failed'))
);

CREATE INDEX idx_withdrawals_status ON usdt_withdrawals(status);
CREATE INDEX idx_withdrawals_pezkuwi ON usdt_withdrawals(pezkuwi_address);
CREATE INDEX idx_withdrawals_available ON usdt_withdrawals(available_at);
```

### Table: `bridge_reserves`
```sql
CREATE TABLE bridge_reserves (
  id SERIAL PRIMARY KEY,
  chain VARCHAR(10) NOT NULL,
  address VARCHAR(128) NOT NULL,
  usdt_balance DECIMAL(20, 6) NOT NULL,
  wusdt_minted DECIMAL(20, 6) NOT NULL,
  collateral_ratio DECIMAL(10, 4), -- Calculated: balance / minted * 100
  last_checked_at TIMESTAMP NOT NULL DEFAULT NOW(),

  UNIQUE(chain, address)
);

CREATE INDEX idx_reserves_chain ON bridge_reserves(chain);
```

---

## 🔄 Deposit Flow

### 1. User Initiates Deposit

**User actions:**
1. Goes to `bridge.pezkuwichain.io/deposit`
2. Selects chain (Tron/Ethereum/BSC)
3. Enters Pezkuwichain address (SS58 format)
4. Gets unique deposit address (or uses shared custody address with memo)

**Frontend generates:**
```json
{
  "pezkuwi_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "deposit_address": "TXYz...abc",
  "chain": "tron",
  "minimum_deposit": "10.00 USDT",
  "estimated_time": "5-10 minutes"
}
```

### 2. External Chain Monitor Detects Deposit

**Polling service** (runs every 30s):
```typescript
// Pseudocode
async function monitorDeposits() {
  for (const chain of ['tron', 'ethereum', 'bsc']) {
    const recentTxs = await getRecentTransactions(chain, CUSTODY_ADDRESS);

    for (const tx of recentTxs) {
      if (isUSDTTransfer(tx) && !existsInDatabase(tx.hash)) {
        const deposit = {
          external_chain: chain,
          external_tx_hash: tx.hash,
          external_from_address: tx.from,
          external_to_address: tx.to,
          usdt_amount: tx.value / 1e6, // Convert to decimal
          wusdt_amount: tx.value / 1e6 * 0.999, // 0.1% bridge fee
          bridge_fee: tx.value / 1e6 * 0.001,
          pezkuwi_address: extractPezkuwiAddress(tx.memo || tx.from),
          status: 'pending',
          confirmations: tx.confirmations,
          required_confirmations: chain === 'tron' ? 20 : 12,
        };

        await db.insert('usdt_deposits', deposit);
        await notifyUser(deposit);
      }
    }
  }
}
```

### 3. Confirmation Tracking

**Confirmation monitor** (runs every 60s):
```typescript
async function trackConfirmations() {
  const pending = await db.query(`
    SELECT * FROM usdt_deposits
    WHERE status IN ('pending', 'confirmed')
    AND confirmations < required_confirmations
  `);

  for (const deposit of pending) {
    const currentConfs = await getConfirmations(
      deposit.external_chain,
      deposit.external_tx_hash
    );

    await db.update('usdt_deposits', deposit.id, {
      confirmations: currentConfs,
      status: currentConfs >= deposit.required_confirmations ? 'confirmed' : 'pending'
    });

    if (currentConfs >= deposit.required_confirmations) {
      await queueForMinting(deposit.id);
    }
  }
}
```

### 4. Multisig Minting Process

**Mint processor** (runs every 120s):
```typescript
import { createMintWUSDTTx } from '@pezkuwi/lib/usdt';

async function processMinting() {
  const confirmed = await db.query(`
    SELECT * FROM usdt_deposits
    WHERE status = 'confirmed'
    ORDER BY confirmed_at ASC
    LIMIT 10
  `);

  for (const deposit of confirmed) {
    try {
      // Update status
      await db.update('usdt_deposits', deposit.id, { status: 'minting' });

      // Create multisig mint transaction
      const mintTx = await createMintWUSDTTx(
        api,
        deposit.pezkuwi_address,
        deposit.wusdt_amount,
        MULTISIG_SIGNER_ADDRESS,
        MULTISIG_MEMBER_ADDRESSES
      );

      // Sign and submit
      const txHash = await mintTx.signAndSend(
        MULTISIG_SIGNER_ADDRESS,
        { signer: injector.signer }
      );

      // Record hash
      await db.update('usdt_deposits', deposit.id, {
        pezkuwi_tx_hash: txHash.toHex(),
        status: 'minted',
        minted_at: new Date()
      });

      await notifyUserMintComplete(deposit);

    } catch (error) {
      await db.update('usdt_deposits', deposit.id, {
        status: 'failed',
        admin_notes: error.message
      });

      await alertAdmins(deposit, error);
    }
  }
}
```

---

## 🔙 Withdrawal Flow

### 1. User Burns wUSDT

**On-chain action:**
```typescript
// User calls through frontend
const burnTx = api.tx.assets.burn(
  1000, // wUSDT asset ID
  userAddress,
  amount * 1_000_000 // Convert to 6 decimals
);

await burnTx.signAndSend(userAddress, { signer });
```

**Event emitted:**
```rust
pallet_assets::Event::Burned {
  asset_id: 1000,
  owner: AccountId,
  balance: 100_000_000 // 100 wUSDT
}
```

### 2. Bridge Service Detects Burn

**Event listener:**
```typescript
api.query.system.events((events) => {
  events.forEach(async (record) => {
    const { event } = record;

    if (api.events.assets.Burned.is(event)) {
      const [assetId, owner, balance] = event.data;

      if (assetId.toNumber() === 1000) { // wUSDT
        const withdrawal = {
          pezkuwi_address: owner.toString(),
          pezkuwi_tx_hash: record.hash.toHex(),
          wusdt_burned: Number(balance) / 1e6,
          target_chain: await getUserPreferredChain(owner),
          target_address: await getUserExternalAddress(owner),
          status: 'pending',
          withdrawal_tier: calculateTier(Number(balance) / 1e6),
          delay_seconds: calculateDelay(Number(balance) / 1e6),
          available_at: new Date(Date.now() + calculateDelay(...) * 1000),
        };

        await db.insert('usdt_withdrawals', withdrawal);
        await notifyUserWithdrawalPending(withdrawal);
      }
    }
  });
});
```

### 3. Withdrawal Approval & Processing

**Withdrawal processor** (runs every 300s):
```typescript
async function processWithdrawals() {
  const ready = await db.query(`
    SELECT * FROM usdt_withdrawals
    WHERE status = 'pending'
    AND available_at <= NOW()
    ORDER BY requested_at ASC
    LIMIT 5
  `);

  for (const withdrawal of ready) {
    try {
      // Check reserve balance
      const reserve = await getReserveBalance(withdrawal.target_chain);
      if (reserve < withdrawal.usdt_amount) {
        await alertLowReserves(withdrawal.target_chain, reserve);
        continue;
      }

      // Update status
      await db.update('usdt_withdrawals', withdrawal.id, {
        status: 'processing'
      });

      // Send USDT on external chain
      const externalTx = await sendUSDT(
        withdrawal.target_chain,
        withdrawal.target_address,
        withdrawal.usdt_amount,
        CUSTODY_PRIVATE_KEY
      );

      // Record success
      await db.update('usdt_withdrawals', withdrawal.id, {
        status: 'sent',
        external_tx_hash: externalTx.hash,
        sent_at: new Date()
      });

      await notifyUserWithdrawalComplete(withdrawal, externalTx.hash);

    } catch (error) {
      await db.update('usdt_withdrawals', withdrawal.id, {
        status: 'failed',
        failure_reason: error.message
      });

      await alertAdmins(withdrawal, error);
    }
  }
}
```

---

## 🔐 Security Measures

### Rate Limiting
```typescript
// Max 10 deposits per address per hour
const DEPOSIT_RATE_LIMIT = {
  maxDeposits: 10,
  windowSeconds: 3600
};

// Max daily minting volume
const DAILY_MINT_LIMIT = {
  maxAmount: 1_000_000, // 1M USDT
  windowSeconds: 86400
};
```

### Multisig Configuration
```rust
// runtime/src/lib.rs
type BridgeMultisig = pallet_multisig::Config {
  type Threshold = ConstU16<3>; // 3/5 signatures required
  type MaxSignatories = ConstU16<5>;
};
```

### Reserve Monitoring
```typescript
// Alert if collateral ratio drops below 100%
async function monitorReserves() {
  const reserves = await db.query('SELECT * FROM bridge_reserves');

  for (const reserve of reserves) {
    if (reserve.collateral_ratio < 100) {
      await alertCritical(`
        Reserve shortfall detected!
        Chain: ${reserve.chain}
        Ratio: ${reserve.collateral_ratio}%
        Deficit: ${reserve.wusdt_minted - reserve.usdt_balance} USDT
      `);

      // Pause new deposits
      await pauseDeposits(reserve.chain);
    }
  }
}
```

---

## 📊 Admin Dashboard

### Key Metrics
- Total wUSDT minted
- Total wUSDT burned
- Active deposits (pending/confirmed)
- Active withdrawals (pending/processing)
- Reserve balances per chain
- Collateral ratio per chain
- 24h volume (deposits + withdrawals)
- Bridge fee earnings

### Admin Actions
1. **Approve Withdrawal** - Manual approval for large withdrawals
2. **Pause Bridge** - Emergency stop for deposits/withdrawals
3. **Adjust Fees** - Update bridge fee percentage
4. **Add Reserve** - Top up custody addresses
5. **Export Reports** - Daily/weekly/monthly transaction reports

---

## 🚀 Deployment Checklist

### Phase 2 MVP Launch

- [ ] Deploy backend service (Node.js + PostgreSQL)
- [ ] Configure multisig (3/5 signers)
- [ ] Set up monitoring (Prometheus + Grafana)
- [ ] Deploy custody addresses (Tron/ETH/BSC)
- [ ] Fund initial reserves (100K USDT minimum)
- [ ] Configure rate limits
- [ ] Set up alerting (Slack/Telegram)
- [ ] Deploy frontend UI
- [ ] Test deposit flow (Tron testnet)
- [ ] Test withdrawal flow (Tron testnet)
- [ ] Security review
- [ ] Deploy to Beta testnet
- [ ] Public announcement

---

## 📈 Future Enhancements (Phase 3-4)

### Phase 3: Semi-Decentralized
- Validator-controlled bridge keys
- On-chain governance for parameters
- Automated oracle price feeds
- Transparent reserve proofs

### Phase 4: Fully Decentralized
- Light client bridge (trustless)
- On-chain proof verification
- Automated reserve management
- Cross-chain messaging (XCM)

---

**Document Version:** 1.0.0
**Last Updated:** 2025-11-20
**Status:** Design Phase
**Next Review:** Post-Beta Launch
