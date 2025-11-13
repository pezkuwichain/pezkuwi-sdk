# Security Best Practices for PezkuwiChain Development

**Document Version**: 1.0
**Last Updated**: 2025-11-13
**Target Audience**: Developers, Security Auditors, Runtime Engineers

## Table of Contents

1. [Substrate-Specific Security](#substrate-specific-security)
2. [Arithmetic Safety](#arithmetic-safety)
3. [Access Control](#access-control)
4. [Storage Management](#storage-management)
5. [Economic Security](#economic-security)
6. [Testing & Validation](#testing--validation)
7. [Deployment Security](#deployment-security)
8. [Incident Response](#incident-response)

---

## Substrate-Specific Security

### Pallet Development Guidelines

#### Storage Versioning

**ALWAYS** use storage versioning for pallets:

```rust
pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
pub struct Pallet<T>(_);
```

**Why**: Enables safe runtime upgrades and storage migrations.

#### Weight Annotations

**ALWAYS** provide accurate weight annotations:

```rust
#[pallet::weight(<T as Config>::WeightInfo::my_function())]
pub fn my_function(origin: OriginFor<T>) -> DispatchResult {
    // implementation
}
```

**Why**: Prevents DoS attacks through resource exhaustion.

#### Bounded Collections

**ALWAYS** use `BoundedVec`, `BoundedBTreeMap`, etc.:

```rust
// ❌ NEVER
pub type UnboundedList<T> = StorageValue<_, Vec<T::AccountId>>;

// ✅ ALWAYS
pub type BoundedList<T> = StorageValue<_, BoundedVec<T::AccountId, T::MaxSize>>;
```

**Why**: Prevents unbounded storage growth and state bloat.

---

## Arithmetic Safety

### Checked Arithmetic

**Rule #1**: Use checked arithmetic for all financial calculations and counter increments.

#### Integer Operations

```rust
// ❌ UNSAFE - Can overflow
let total = a + b;
let counter = counter + 1;

// ✅ SAFE - Explicit overflow handling
let total = a.checked_add(b).ok_or(Error::<T>::Overflow)?;
let counter = counter.saturating_add(1);
```

#### Division Operations

```rust
// ❌ UNSAFE - Can panic on division by zero
let ratio = numerator / denominator;

// ✅ SAFE - Explicit zero check
let ratio = numerator.checked_div(denominator).ok_or(Error::<T>::DivisionByZero)?;
```

#### Multiplication and Division

```rust
// ❌ UNSAFE - Can overflow then divide
let result = amount * percentage / 100;

// ✅ SAFE - Checked operations
let result = amount
    .checked_mul(percentage)
    .and_then(|v| v.checked_div(100))
    .ok_or(Error::<T>::ArithmeticOverflow)?;
```

### Type Conversions

```rust
// ❌ UNSAFE - Silent failure
let value: u32 = big_number.try_into().unwrap_or(0);

// ✅ SAFE - Explicit error handling
let value: u32 = big_number.try_into()
    .map_err(|_| Error::<T>::ConversionOverflow)?;
```

### Saturating vs Checked Arithmetic

**Use saturating** for:
- Counters that shouldn't fail (logs, metrics)
- Non-critical calculations
- User-facing values

**Use checked** for:
- Financial calculations
- Token amounts
- Vote counts
- Critical system counters

```rust
// Saturating for non-critical
let participation_count = count.saturating_add(1);

// Checked for critical
let total_votes = aye_votes.checked_add(nay_votes)
    .ok_or(Error::<T>::VoteOverflow)?;
```

---

## Access Control

### Origin Validation

**ALWAYS** validate origin for privileged operations:

```rust
// ❌ DANGEROUS - No origin check
pub fn critical_operation(origin: OriginFor<T>) -> DispatchResult {
    // Missing: ensure_root, ensure_signed, or custom origin check
}

// ✅ SECURE - Proper validation
pub fn critical_operation(origin: OriginFor<T>) -> DispatchResult {
    T::AdminOrigin::ensure_origin(origin)?;
    // or
    let who = ensure_signed(origin)?;
    ensure!(Self::is_authorized(&who), Error::<T>::Unauthorized);
    // implementation
}
```

### Custom Origins

Define custom origins for role-based access:

```rust
pub struct EnsureSerok<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> EnsureOrigin<T::RuntimeOrigin> for EnsureSerok<T> {
    type Success = T::AccountId;

    fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
        match o.clone().into() {
            Ok(frame_system::RawOrigin::Signed(who)) => {
                if let Some(serok) = CurrentOfficials::<T>::get(GovernmentPosition::Serok) {
                    if who == serok {
                        return Ok(who);
                    }
                }
                Err(o)
            }
            _ => Err(o),
        }
    }
}
```

### Privilege Escalation Prevention

**NEVER** allow regular users to call privileged functions indirectly:

```rust
// ❌ DANGEROUS - User can escalate privileges
pub fn user_function(origin: OriginFor<T>) -> DispatchResult {
    let who = ensure_signed(origin)?;
    // Calls privileged function without proper checks
    Self::admin_function(frame_system::RawOrigin::Root.into())?;
    Ok(())
}

// ✅ SECURE - Proper authorization
pub fn user_function(origin: OriginFor<T>) -> DispatchResult {
    let who = ensure_signed(origin)?;
    // Check if user is authorized for this specific action
    ensure!(Self::can_perform_action(&who), Error::<T>::Unauthorized);
    // Perform action with user's authority, not root
    Self::internal_action(&who)?;
    Ok(())
}
```

---

## Storage Management

### Storage Efficiency

Minimize storage reads/writes:

```rust
// ❌ INEFFICIENT - Multiple reads
let value1 = MyStorage::<T>::get(&key);
let value2 = MyStorage::<T>::get(&key);
let value3 = MyStorage::<T>::get(&key);

// ✅ EFFICIENT - Single read
let value = MyStorage::<T>::get(&key);
// Use value multiple times
```

### Storage Deposits

Require deposits for unbounded storage:

```rust
#[pallet::call_index(0)]
#[pallet::weight(T::WeightInfo::store_data())]
pub fn store_data(
    origin: OriginFor<T>,
    data: BoundedVec<u8, T::MaxDataSize>
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // Require deposit proportional to data size
    let deposit = Self::calculate_deposit(data.len());
    T::Currency::reserve(&who, deposit)?;

    MyData::<T>::insert(&who, data);
    Ok(())
}
```

### Storage Cleanup

**ALWAYS** clean up storage when items are removed:

```rust
// ❌ MEMORY LEAK - Storage not cleaned
pub fn remove_item(origin: OriginFor<T>, id: u32) -> DispatchResult {
    ensure_root(origin)?;
    // Removed item but not associated data
    Items::<T>::remove(id);
    // ItemMetadata::<T>::remove(id); // MISSING!
    Ok(())
}

// ✅ PROPER CLEANUP
pub fn remove_item(origin: OriginFor<T>, id: u32) -> DispatchResult {
    ensure_root(origin)?;
    Items::<T>::remove(id);
    ItemMetadata::<T>::remove(id);
    ItemOwners::<T>::remove(id);
    // Release deposits
    if let Some(deposit) = ItemDeposits::<T>::take(id) {
        let _ = T::Currency::unreserve(&deposit.owner, deposit.amount);
    }
    Ok(())
}
```

---

## Economic Security

### Token Economics

#### Prevent Overflow in Token Operations

```rust
// ❌ UNSAFE
let new_balance = old_balance + transfer_amount;

// ✅ SAFE
let new_balance = old_balance.checked_add(transfer_amount)
    .ok_or(Error::<T>::BalanceOverflow)?;
```

#### Verify Sufficient Balance

```rust
// ❌ MISSING CHECK
T::Currency::transfer(&from, &to, amount, Preservation::Preserve)?;

// ✅ WITH VERIFICATION
ensure!(
    T::Currency::free_balance(&from) >= amount,
    Error::<T>::InsufficientBalance
);
T::Currency::transfer(&from, &to, amount, Preservation::Preserve)?;
```

### Deposit Requirements

Prevent spam with deposits:

```rust
#[pallet::constant]
type ProposalDeposit: Get<BalanceOf<Self>>;

pub fn submit_proposal(origin: OriginFor<T>, ...) -> DispatchResult {
    let proposer = ensure_signed(origin)?;

    // Reserve deposit
    T::Currency::reserve(&proposer, T::ProposalDeposit::get())?;

    // Store proposal with deposit info
    let proposal = Proposal {
        proposer: proposer.clone(),
        deposit: T::ProposalDeposit::get(),
        // ... other fields
    };

    Proposals::<T>::insert(proposal_id, proposal);
    Ok(())
}
```

### Slashing Mechanisms

Implement penalties for malicious behavior:

```rust
pub fn slash_misbehaving_candidate(
    origin: OriginFor<T>,
    candidate: T::AccountId,
    reason: SlashReason
) -> DispatchResult {
    T::AdminOrigin::ensure_origin(origin)?;

    let deposit = CandidateDeposits::<T>::get(&candidate)
        .ok_or(Error::<T>::NoDepositFound)?;

    // Slash percentage based on severity
    let slash_amount = match reason {
        SlashReason::Withdrawal => deposit / 2,      // 50%
        SlashReason::Misconduct => deposit * 3 / 4,  // 75%
        SlashReason::Fraud => deposit,               // 100%
    };

    // Slash and send to treasury
    let _ = T::Currency::slash_reserved(&candidate, slash_amount);

    Self::deposit_event(Event::CandidateSlashed {
        candidate,
        amount: slash_amount,
        reason,
    });

    Ok(())
}
```

---

## Testing & Validation

### Unit Testing

**ALWAYS** test edge cases:

```rust
#[test]
fn test_counter_overflow() {
    new_test_ext().execute_with(|| {
        // Set counter to max value
        NextId::<Test>::put(u32::MAX);

        // Attempt to increment
        assert_noop!(
            MyPallet::create_item(RuntimeOrigin::signed(1)),
            Error::<Test>::IdOverflow
        );
    });
}

#[test]
fn test_arithmetic_overflow() {
    new_test_ext().execute_with(|| {
        let max_balance = BalanceOf::<Test>::max_value();

        assert_noop!(
            MyPallet::add_balance(RuntimeOrigin::signed(1), max_balance, 1),
            Error::<Test>::BalanceOverflow
        );
    });
}
```

### Integration Testing

Test cross-pallet interactions:

```rust
#[test]
fn test_election_and_role_assignment() {
    new_test_ext().execute_with(|| {
        // Setup election
        assert_ok!(Governance::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Presidential,
            None,
            None
        ));

        // Register and vote
        // ...

        // Finalize
        assert_ok!(Governance::finalize_election(RuntimeOrigin::root(), 0));

        // Verify role was assigned in Tiki pallet
        assert!(Tiki::has_tiki(&winner, &Tiki::Serok));
    });
}
```

### Fuzzing

Implement property-based testing:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_halving_never_overflows(
        initial_amount in 1u128..u128::MAX/2
    ) {
        new_test_ext().execute_with(|| {
            let mut amount = initial_amount;

            // Test 50 halving periods
            for _ in 0..50 {
                amount = amount / 2;
                assert!(amount >= 0);
            }
        });
    }
}
```

### try-runtime Testing

**ALWAYS** test migrations with try-runtime:

```bash
# Build with try-runtime
cargo build --release --features try-runtime

# Test migration on live state
./target/release/pezkuwi try-runtime \
    --runtime ./target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.wasm \
    on-runtime-upgrade \
    live --uri wss://pezkuwi.network:443 \
    --checks all
```

---

## Deployment Security

### Pre-Deployment Checklist

```markdown
## Code Security
- [ ] All arithmetic uses checked operations
- [ ] All origins properly validated
- [ ] Bounded collections used throughout
- [ ] Storage migrations tested with try-runtime
- [ ] Weight benchmarks regenerated
- [ ] No TODO or FIXME comments in production code

## Testing
- [ ] All unit tests passing
- [ ] Integration tests passing
- [ ] Overflow tests added
- [ ] try-runtime validation successful
- [ ] Testnet deployment successful (48h+ stable)

## Documentation
- [ ] Security audit completed
- [ ] API documentation updated
- [ ] Migration guide prepared
- [ ] Changelog published

## Operational
- [ ] Monitoring configured
- [ ] Alerts set up
- [ ] Rollback plan documented
- [ ] Emergency contacts verified
```

### Runtime Upgrade Safety

Follow the upgrade checklist from RUNTIME_UPGRADES.md:

1. **2 weeks before**: Deploy to testnet
2. **1 week before**: Validator coordination
3. **24 hours before**: Database snapshots
4. **Upgrade**: Monitor closely
5. **Post-upgrade**: Verify all functions

### Secret Management

**NEVER** hardcode secrets:

```rust
// ❌ NEVER
const ADMIN_SEED: &str = "//Alice";

// ✅ CORRECT - Use keystore
// Secrets managed through keystore, never in code
```

---

## Incident Response

### Security Incident Levels

| Level | Description | Response Time | Actions |
|-------|-------------|---------------|---------|
| **P0 - Critical** | Funds at risk, chain halted | Immediate | Emergency rollback, halt chain |
| **P1 - High** | Major vulnerability found | 1 hour | Coordinate validators, prepare fix |
| **P2 - Medium** | Minor vulnerability, no immediate risk | 24 hours | Schedule patch release |
| **P3 - Low** | Documentation issue, low-risk bug | 1 week | Normal release cycle |

### Emergency Response Procedures

#### 1. Detection
- Automated monitoring alerts
- User reports
- Security researcher disclosure

#### 2. Assessment
```markdown
## Immediate Questions (5 minutes)
- Is the chain halted?
- Are funds at risk?
- How many users affected?
- Can we reproduce the issue?
```

#### 3. Containment
```markdown
## For Critical Issues
1. Notify all validators immediately
2. Consider chain halt if funds at risk
3. Prevent new transactions (if needed)
4. Document all actions
```

#### 4. Mitigation
```markdown
## Fix Implementation
1. Identify root cause
2. Develop fix
3. Test fix thoroughly
4. Prepare rollback plan
5. Coordinate deployment
```

#### 5. Communication
```markdown
## Stakeholder Updates
- 15 min: Initial notification
- Hourly: Status updates
- Post-incident: Full report
```

### Post-Incident Review

Conduct retrospective after every incident:

```markdown
## Incident Report Template

### Summary
- What happened?
- When was it detected?
- What was the impact?

### Timeline
- Discovery: [timestamp]
- Response initiated: [timestamp]
- Fixed: [timestamp]
- Verified: [timestamp]

### Root Cause
- Technical cause
- Contributing factors

### Lessons Learned
- What went well?
- What could be improved?
- Action items

### Follow-up Actions
- [ ] Code fixes
- [ ] Documentation updates
- [ ] Monitoring improvements
- [ ] Team training
```

---

## Security Resources

### Internal Documentation
- [Storage Migrations Guide](./STORAGE_MIGRATIONS.md)
- [Runtime Upgrade Guide](./RUNTIME_UPGRADES.md)
- [Security Audit Report](./SECURITY_AUDIT_REPORT.md)
- [API Documentation](./API_PALLET_TIKI.md)

### External Resources
- [Substrate Security Best Practices](https://docs.substrate.io/build/runtime-security/)
- [Polkadot Security](https://wiki.polkadot.network/docs/learn-security)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)

### Tools
- [try-runtime](https://paritytech.github.io/try-runtime-cli/)
- [cargo-audit](https://github.com/RustSec/cargo-audit)
- [cargo-geiger](https://github.com/rust-secure-code/cargo-geiger)
- [clippy](https://github.com/rust-lang/rust-clippy)

---

## Quick Reference

### Arithmetic Safety Cheat Sheet

| Operation | Unsafe | Safe (Checked) | Safe (Saturating) |
|-----------|--------|----------------|-------------------|
| Addition | `a + b` | `a.checked_add(b)?` | `a.saturating_add(b)` |
| Subtraction | `a - b` | `a.checked_sub(b)?` | `a.saturating_sub(b)` |
| Multiplication | `a * b` | `a.checked_mul(&b)?` | `a.saturating_mul(b)` |
| Division | `a / b` | `a.checked_div(&b)?` | `a.saturating_div(b)` |
| Increment | `x += 1` | `x = x.checked_add(1)?` | `x = x.saturating_add(1)` |
| Conversion | `x as u32` | `x.try_into()?` | - |

### Origin Validation Cheat Sheet

| Check | Code | Use Case |
|-------|------|----------|
| Root | `ensure_root(origin)?` | System-level operations |
| Signed | `ensure_signed(origin)?` | User transactions |
| Custom Origin | `T::MyOrigin::ensure_origin(origin)?` | Role-based access |
| None (unsigned) | `ensure_none(origin)?` | Validators, offchain workers |

---

## Contributing to Security

### Reporting Security Issues

**DO NOT** open public GitHub issues for security vulnerabilities.

**Instead**:
1. Email: security@pezkuwi.network
2. Include: Detailed description, steps to reproduce, impact assessment
3. Allow 90 days for fix before public disclosure

### Security Bounty Program

Coming soon: Details on responsible disclosure and bounty rewards.

---

**Document Maintenance**: This document should be reviewed and updated quarterly or after any security incident.

**Last Review**: 2025-11-13
**Next Review**: 2025-02-13
