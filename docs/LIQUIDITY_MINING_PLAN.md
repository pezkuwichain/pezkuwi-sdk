# Liquidity Mining Implementation Plan

## Executive Summary

This document outlines the implementation plan for adding a liquidity mining (yield farming) mechanism to PezkuwiChain's DEX. The system will reward liquidity providers (LPs) with HEZ tokens to incentivize deeper liquidity and higher trading volumes.

---

## 1. Objectives

### Primary Goals:
1. **Increase TVL (Total Value Locked)**: Attract more liquidity to pools
2. **Improve Trading Experience**: Reduce slippage through deeper liquidity
3. **Bootstrap Network Effects**: Incentivize early adopters
4. **Sustainable Economics**: Create long-term value for HEZ token

### Success Metrics:
- TVL increase by 5x within 3 months
- Average swap slippage reduced below 2%
- 100+ active LPs participating
- APR competitive with other chains (target: 50-150%)

---

## 2. Technical Design

### 2.1 Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Liquidity Mining Pallet                     │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Reward Pool                                       │  │
│  │  - HEZ token reserve for rewards                  │  │
│  │  - Emission schedule (block-based)                │  │
│  ├───────────────────────────────────────────────────┤  │
│  │  Staking Registry                                  │  │
│  │  - LP token deposits per user                     │  │
│  │  - Reward debt tracking                           │  │
│  │  - Last claim timestamp                           │  │
│  ├───────────────────────────────────────────────────┤  │
│  │  Reward Calculator                                 │  │
│  │  - Pro-rata share calculation                     │  │
│  │  - Time-weighted rewards                          │  │
│  │  - Boost multipliers (optional)                   │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                          │
                          ├─ AssetConversion (LP tokens)
                          ├─ Balances (HEZ rewards)
                          └─ Timestamp (block time tracking)
```

### 2.2 Core Concepts

**Reward Formula (Simplified Synthetix/Sushiswap Model):**

```rust
reward_per_token = total_rewards_emitted / total_lp_tokens_staked

user_reward = user_lp_balance × (reward_per_token - user_reward_debt)
```

**Emission Schedule:**
- Fixed emission per block: 10 HEZ/block
- Distributed across all pools (can be weighted)
- Total annual emission: ~5M HEZ (assuming 6s block time)

---

## 3. Implementation Steps

### Phase 1: Core Pallet Development (Week 1-2)

#### 3.1 Create Liquidity Mining Pallet

**File**: `pezkuwi/pallets/liquidity-mining/src/lib.rs`

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Get},
    };
    use frame_system::pallet_prelude::*;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type RewardPerBlock: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// LP token deposits per user per pool
    #[pallet::storage]
    #[pallet::getter(fn user_deposits)]
    pub type UserDeposits<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, T::AccountId,   // User
        Blake2_128Concat, u32,            // Pool LP token ID
        BalanceOf<T>,                     // Amount deposited
        ValueQuery,
    >;

    /// Reward debt tracking (for reward calculation)
    #[pallet::storage]
    #[pallet::getter(fn reward_debt)]
    pub type RewardDebt<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, T::AccountId,
        Blake2_128Concat, u32,
        BalanceOf<T>,
        ValueQuery,
    >;

    /// Total LP tokens staked per pool
    #[pallet::storage]
    #[pallet::getter(fn total_staked)]
    pub type TotalStaked<T: Config> = StorageMap<
        _,
        Blake2_128Concat, u32,
        BalanceOf<T>,
        ValueQuery,
    >;

    /// Accumulated reward per LP token (scaled by 1e12)
    #[pallet::storage]
    #[pallet::getter(fn acc_reward_per_share)]
    pub type AccRewardPerShare<T: Config> = StorageMap<
        _,
        Blake2_128Concat, u32,
        u128,
        ValueQuery,
    >;

    /// Last reward update block
    #[pallet::storage]
    #[pallet::getter(fn last_reward_block)]
    pub type LastRewardBlock<T: Config> = StorageMap<
        _,
        Blake2_128Concat, u32,
        T::BlockNumber,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// LP tokens deposited for mining
        Deposited { user: T::AccountId, pool_id: u32, amount: BalanceOf<T> },
        /// LP tokens withdrawn
        Withdrawn { user: T::AccountId, pool_id: u32, amount: BalanceOf<T> },
        /// Rewards claimed
        RewardsClaimed { user: T::AccountId, pool_id: u32, amount: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        NoRewardsToClaim,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Deposit LP tokens for mining
        #[pallet::weight(10_000)]
        pub fn deposit(
            origin: OriginFor<T>,
            pool_id: u32,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Update pool rewards before deposit
            Self::update_pool(pool_id)?;

            // Claim pending rewards before adding new deposit
            Self::internal_claim(who.clone(), pool_id)?;

            // Transfer LP tokens from user to pallet
            // (Requires integration with PoolAssets pallet)

            // Update user deposit
            UserDeposits::<T>::mutate(&who, pool_id, |balance| {
                *balance = balance.saturating_add(amount);
            });

            // Update total staked
            TotalStaked::<T>::mutate(pool_id, |total| {
                *total = total.saturating_add(amount);
            });

            // Update reward debt
            let acc_reward = AccRewardPerShare::<T>::get(pool_id);
            RewardDebt::<T>::mutate(&who, pool_id, |debt| {
                let user_balance = UserDeposits::<T>::get(&who, pool_id);
                *debt = (user_balance.saturated_into::<u128>() * acc_reward / 1e12 as u128).saturated_into();
            });

            Self::deposit_event(Event::Deposited { user: who, pool_id, amount });
            Ok(())
        }

        /// Withdraw LP tokens
        #[pallet::weight(10_000)]
        pub fn withdraw(
            origin: OriginFor<T>,
            pool_id: u32,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Update pool rewards
            Self::update_pool(pool_id)?;

            // Claim pending rewards
            Self::internal_claim(who.clone(), pool_id)?;

            // Check user balance
            let user_balance = UserDeposits::<T>::get(&who, pool_id);
            ensure!(user_balance >= amount, Error::<T>::InsufficientBalance);

            // Update balances
            UserDeposits::<T>::mutate(&who, pool_id, |balance| {
                *balance = balance.saturating_sub(amount);
            });

            TotalStaked::<T>::mutate(pool_id, |total| {
                *total = total.saturating_sub(amount);
            });

            // Transfer LP tokens back to user

            Self::deposit_event(Event::Withdrawn { user: who, pool_id, amount });
            Ok(())
        }

        /// Claim rewards
        #[pallet::weight(10_000)]
        pub fn claim_rewards(
            origin: OriginFor<T>,
            pool_id: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::update_pool(pool_id)?;
            Self::internal_claim(who, pool_id)?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Update pool reward accumulator
        fn update_pool(pool_id: u32) -> DispatchResult {
            let current_block = <frame_system::Pallet<T>>::block_number();
            let last_block = LastRewardBlock::<T>::get(pool_id);

            if current_block <= last_block {
                return Ok(());
            }

            let total_staked = TotalStaked::<T>::get(pool_id);

            if total_staked.is_zero() {
                LastRewardBlock::<T>::insert(pool_id, current_block);
                return Ok(());
            }

            // Calculate rewards
            let blocks_elapsed = current_block.saturating_sub(last_block);
            let reward_per_block = T::RewardPerBlock::get();
            let total_reward = reward_per_block.saturating_mul(blocks_elapsed.saturated_into());

            // Update accumulator (scaled by 1e12)
            let reward_per_token = (total_reward.saturated_into::<u128>() * 1e12 as u128) / total_staked.saturated_into::<u128>();

            AccRewardPerShare::<T>::mutate(pool_id, |acc| {
                *acc = acc.saturating_add(reward_per_token);
            });

            LastRewardBlock::<T>::insert(pool_id, current_block);

            Ok(())
        }

        /// Internal claim function
        fn internal_claim(who: T::AccountId, pool_id: u32) -> DispatchResult {
            let user_balance = UserDeposits::<T>::get(&who, pool_id);
            if user_balance.is_zero() {
                return Ok(());
            }

            let acc_reward = AccRewardPerShare::<T>::get(pool_id);
            let reward_debt = RewardDebt::<T>::get(&who, pool_id);

            let pending_reward = (user_balance.saturated_into::<u128>() * acc_reward / 1e12 as u128)
                .saturating_sub(reward_debt.saturated_into::<u128>());

            if pending_reward > 0 {
                let reward: BalanceOf<T> = pending_reward.saturated_into();

                // Transfer rewards to user
                T::Currency::deposit_creating(&who, reward);

                Self::deposit_event(Event::RewardsClaimed { user: who.clone(), pool_id, amount: reward });
            }

            // Update reward debt
            RewardDebt::<T>::mutate(&who, pool_id, |debt| {
                *debt = (user_balance.saturated_into::<u128>() * acc_reward / 1e12 as u128).saturated_into();
            });

            Ok(())
        }

        /// Calculate pending rewards for a user
        pub fn pending_rewards(who: T::AccountId, pool_id: u32) -> BalanceOf<T> {
            let user_balance = UserDeposits::<T>::get(&who, pool_id);
            if user_balance.is_zero() {
                return Zero::zero();
            }

            let acc_reward = AccRewardPerShare::<T>::get(pool_id);
            let total_staked = TotalStaked::<T>::get(pool_id);

            // Calculate additional rewards since last update
            let current_block = <frame_system::Pallet<T>>::block_number();
            let last_block = LastRewardBlock::<T>::get(pool_id);
            let blocks_elapsed = current_block.saturating_sub(last_block);

            let additional_reward = if !total_staked.is_zero() {
                let reward_per_block = T::RewardPerBlock::get();
                let total_reward = reward_per_block.saturating_mul(blocks_elapsed.saturated_into());
                (total_reward.saturated_into::<u128>() * 1e12 as u128) / total_staked.saturated_into::<u128>()
            } else {
                0
            };

            let final_acc_reward = acc_reward.saturating_add(additional_reward);
            let reward_debt = RewardDebt::<T>::get(&who, pool_id);

            let pending = (user_balance.saturated_into::<u128>() * final_acc_reward / 1e12 as u128)
                .saturating_sub(reward_debt.saturated_into::<u128>());

            pending.saturated_into()
        }
    }
}
```

#### 3.2 Integration with Runtime

**File**: `pezkuwi/runtime/pezkuwichain/src/lib.rs`

```rust
// Add to runtime
parameter_types! {
    pub const RewardPerBlock: Balance = 10 * UNITS; // 10 HEZ per block
}

impl pallet_liquidity_mining::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type RewardPerBlock = RewardPerBlock;
}

construct_runtime!(
    pub struct Runtime {
        // ... existing pallets ...
        LiquidityMining: pallet_liquidity_mining = 77,
    }
);
```

### Phase 2: Testing (Week 3)

#### Test Cases:
1. **Deposit LP tokens**: Verify balance updates
2. **Withdraw LP tokens**: Ensure correct amount returned
3. **Claim rewards**: Validate reward calculation
4. **Multiple users**: Test pro-rata distribution
5. **Edge cases**:
   - Zero deposits
   - Zero total staked
   - Rapid deposits/withdrawals
   - Reward overflow protection

### Phase 3: Frontend Integration (Week 4)

#### 3.1 Staking Interface Component

```typescript
// src/components/LiquidityMining.tsx
- Display APR for each pool
- Stake LP tokens button
- Unstake LP tokens button
- Claim rewards button
- Show pending rewards
- Transaction history
```

#### 3.2 Pool Dashboard Updates

Add liquidity mining metrics to PoolDashboard.tsx:
- Current APR from mining rewards
- Your staked LP tokens
- Pending rewards
- Total rewards earned

### Phase 4: Deployment & Monitoring (Week 5)

1. **Genesis Configuration**:
   - Allocate reward reserve (e.g., 50M HEZ for mining)
   - Set initial emission rate
   - Configure pool weights

2. **Monitoring Dashboard**:
   - Total HEZ emitted
   - Total LP tokens staked
   - Active stakers count
   - Average APR
   - TVL trending

3. **Runtime Upgrade**:
   - Test on local network
   - Deploy to beta testnet
   - Monitor for 1 week
   - Deploy to mainnet

---

## 4. Economic Parameters

### Initial Configuration:

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| Emission per block | 10 HEZ | Sustainable long-term emission |
| Annual emission | ~5M HEZ | ~5% of initial supply |
| Pool allocation | 100% to wHEZ/PEZ | Bootstrap main pool first |
| Reward duration | Infinite (decreasing) | Long-term sustainability |
| Min stake amount | 0.01 LP tokens | Low barrier to entry |

### Future Adjustments (Via Governance):
- Add more pools (wHEZ/USDT, wHEZ/DAI)
- Adjust emission rates
- Implement boost multipliers
- Add lock-up periods for higher rewards

---

## 5. Security Considerations

### Risks & Mitigations:

1. **Reward Inflation**:
   - **Risk**: Excessive emission devalues HEZ
   - **Mitigation**: Capped annual emission, governance control

2. **Flash Loan Attacks**:
   - **Risk**: Manipulate rewards via instant stake/unstake
   - **Mitigation**: Minimum stake duration (e.g., 1 block)

3. **Integer Overflow**:
   - **Risk**: Reward calculation overflow
   - **Mitigation**: Use saturating arithmetic, scale by 1e12

4. **Front-running**:
   - **Risk**: MEV on reward claims
   - **Mitigation**: Auto-claim on deposit/withdraw

---

## 6. Success Metrics & KPIs

### Track Monthly:

1. **TVL Growth**: Target +20% MoM
2. **Number of LPs**: Target 100+ active stakers
3. **Average Stake Duration**: Target 30+ days
4. **Trading Volume**: Should correlate with TVL growth
5. **APR Competitiveness**: Maintain 50-150% range

---

## 7. Timeline

| Phase | Duration | Deliverables |
|-------|----------|--------------|
| **Phase 1** | 2 weeks | Liquidity mining pallet, runtime integration |
| **Phase 2** | 1 week | Comprehensive test suite, security audit |
| **Phase 3** | 1 week | Frontend staking interface, dashboard updates |
| **Phase 4** | 1 week | Beta testnet deployment, monitoring setup |
| **Phase 5** | 1 week | Mainnet deployment, documentation |

**Total**: 6 weeks from start to mainnet

---

## 8. Future Enhancements

### V2 Features:
1. **Multiple Pool Support**: Weighted allocations
2. **Boost Multipliers**: Lock tokens for higher APR
3. **NFT Staking**: Stake special NFTs for bonus rewards
4. **Referral Program**: Earn rewards for referred LPs
5. **Auto-compounding**: Reinvest rewards automatically

### V3 Features:
1. **Governance Voting**: Stake LP tokens to vote
2. **Dual Rewards**: Earn both HEZ + partner tokens
3. **Impermanent Loss Insurance**: Partial coverage fund
4. **Dynamic Emissions**: Adjust based on market conditions

---

## 9. Implementation Checklist

- [ ] Create `pallet-liquidity-mining` crate
- [ ] Implement core storage and logic
- [ ] Add to runtime configuration
- [ ] Write unit tests (90%+ coverage)
- [ ] Security audit by external firm
- [ ] Create frontend staking UI
- [ ] Update pool dashboard
- [ ] Deploy to local testnet
- [ ] Deploy to beta testnet
- [ ] Monitor for 1 week
- [ ] Deploy to mainnet
- [ ] Launch announcement & documentation

---

## 10. Resources

### Documentation:
- [Synthetix Staking Rewards](https://github.com/Synthetixio/synthetix/blob/master/contracts/StakingRewards.sol)
- [Sushiswap MasterChef](https://github.com/sushiswap/sushiswap/blob/master/contracts/MasterChef.sol)
- [Substrate Pallets](https://docs.substrate.io/reference/frame-pallets/)

### Team Requirements:
- 1x Runtime Developer (Rust/Substrate)
- 1x Frontend Developer (React/TypeScript)
- 1x Security Auditor (External)
- 1x DevOps Engineer (Monitoring)

### Budget Estimate:
- Development: 4-6 weeks @ $10k/week = $40-60k
- Security audit: $15-25k
- Monitoring infrastructure: $2k/year
- **Total**: $57-87k

---

## Conclusion

This liquidity mining implementation will significantly boost PezkuwiChain's DEX competitiveness by:
1. Attracting deep liquidity → lower slippage → better UX
2. Incentivizing early adopters → network effects
3. Creating sustainable HEZ token utility

**Recommended Next Step**: Approve budget and begin Phase 1 development.
