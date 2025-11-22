# PezkuwiChain Performance Benchmarking Guide
**Version:** 1.0
**Date:** 2025-11-13
**Status:** Production Ready

## Table of Contents
1. [Overview](#overview)
2. [Weight Benchmarking](#weight-benchmarking)
3. [Runtime Benchmarking](#runtime-benchmarking)
4. [Storage Benchmarking](#storage-benchmarking)
5. [Network Benchmarking](#network-benchmarking)
6. [Benchmark Analysis](#benchmark-analysis)

---

## Overview

### Purpose
Performance benchmarking in Substrate/Polkadot chains determines the computational cost (weight) of each extrinsic. Accurate weights are critical for:
- Fee calculation
- Block production limits
- DoS prevention
- Resource allocation

### Weight System
In Substrate, weight represents computational time:
- **1 weight = 1 picosecond of execution time**
- **1 second = 1,000,000,000,000 weights (1 trillion)**
- **Block weight limit** determines how many transactions fit in one block

### Why Benchmark?
- **Accurate Fees:** Users pay fair prices for transactions
- **Security:** Prevent block stuffing attacks
- **Performance:** Optimize block production time
- **Capacity Planning:** Understand network throughput limits

---

## Weight Benchmarking

### Prerequisites

#### 1. Build with Benchmarking Features
```bash
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi

# Build binary with benchmark support
cargo build --release --features runtime-benchmarks

# Verify binary has benchmarking enabled
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi --help | grep benchmark
```

#### 2. Hardware Requirements
**Recommended benchmarking machine:**
- Modern CPU (Intel/AMD, 3.0+ GHz)
- 16GB+ RAM
- SSD storage
- Dedicated machine (minimal background processes)
- **Consistent environment** (same CPU, no thermal throttling)

**Important:** All validators must use weights from same hardware spec!

---

### Benchmarking Individual Pallets

#### Pallet: pez-treasury

**Step 1: Run Benchmark**
```bash
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi

# Benchmark all extrinsics in pez-treasury
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark pallet \
  --chain=dev \
  --pallet=pallet_pez_treasury \
  --extrinsic='*' \
  --steps=50 \
  --repeat=20 \
  --output=./pallets/pez-treasury/src/weights.rs \
  --template=./.maintain/frame-weight-template.hbs
```

**Parameters Explained:**
- `--chain=dev`: Use development chain spec
- `--pallet`: Target pallet name
- `--extrinsic='*'`: Benchmark all extrinsics (or specify one)
- `--steps=50`: Number of steps for component ranges
- `--repeat=20`: Repeat each benchmark 20 times (for accuracy)
- `--output`: Where to write weight file
- `--template`: Custom weight file template (optional)

**Expected Duration:** 30-60 minutes per pallet

**Step 2: Verify Generated Weights**
```bash
# Check generated weight file
cat ./pallets/pez-treasury/src/weights.rs

# Should contain something like:
# impl<T: Config> WeightInfo for SubstrateWeight<T> {
#     fn initialize_treasury() -> Weight {
#         Weight::from_parts(25_000_000, 0)
#             .saturating_add(T::DbWeight::get().reads(1))
#             .saturating_add(T::DbWeight::get().writes(1))
#     }
#     ...
# }
```

**Step 3: Update Pallet to Use Weights**
```rust
// In pallets/pez-treasury/src/lib.rs

#[pallet::config]
pub trait Config: frame_system::Config {
    type WeightInfo: WeightInfo;
    // ... other config types
}

// In your extrinsic
#[pallet::weight(T::WeightInfo::initialize_treasury())]
pub fn initialize_treasury(
    origin: OriginFor<T>,
    initial_amount: BalanceOf<T>,
) -> DispatchResult {
    // ... implementation
}
```

---

#### Pallet: welati (Governance)

```bash
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark pallet \
  --chain=dev \
  --pallet=pallet_welati \
  --extrinsic='*' \
  --steps=50 \
  --repeat=20 \
  --output=./pallets/welati/src/weights.rs
```

**Note:** Governance operations may have complex weights due to:
- Variable proposal sizes
- Different vote counts
- Varying number of parliament members

---

#### Pallet: tiki (Roles & Citizenship)

```bash
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark pallet \
  --chain=dev \
  --pallet=pallet_tiki \
  --extrinsic='*' \
  --steps=50 \
  --repeat=20 \
  --output=./pallets/tiki/src/weights.rs
```

---

### Benchmarking All Pallets (Automated)

```bash
#!/bin/bash
# benchmark-all-pallets.sh

PALLETS=(
  "pallet_pez_treasury"
  "pallet_welati"
  "pallet_tiki"
)

for pallet in "${PALLETS[@]}"; do
  echo "Benchmarking $pallet..."

  /home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark pallet \
    --chain=dev \
    --pallet=$pallet \
    --extrinsic='*' \
    --steps=50 \
    --repeat=20 \
    --output=./pallets/${pallet#pallet_}/src/weights.rs

  if [ $? -eq 0 ]; then
    echo "✅ $pallet benchmarked successfully"
  else
    echo "❌ $pallet benchmarking failed"
    exit 1
  fi

  # Cool down period
  sleep 60
done

echo "All pallets benchmarked!"
```

**Run Script:**
```bash
chmod +x benchmark-all-pallets.sh
./benchmark-all-pallets.sh
```

---

### Update Runtime Configuration

After generating weights, update runtime to use them:

```rust
// In runtime/pezkuwichain/src/lib.rs

// BEFORE (placeholder):
impl pallet_pez_treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Assets = Assets;
    type WeightInfo = (); // ❌ Placeholder
    // ... other types
}

// AFTER (with benchmarked weights):
impl pallet_pez_treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Assets = Assets;
    type WeightInfo = pallet_pez_treasury::weights::SubstrateWeight<Runtime>; // ✅ Real weights
    // ... other types
}
```

**Do this for all three pallets:**
- `pallet_pez_treasury::weights::SubstrateWeight<Runtime>`
- `pallet_welati::weights::SubstrateWeight<Runtime>`
- `pallet_tiki::weights::SubstrateWeight<Runtime>`

---

## Runtime Benchmarking

### Benchmark Entire Runtime

```bash
# Benchmark all pallets in runtime
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark pallet \
  --chain=dev \
  --pallet='*' \
  --extrinsic='*' \
  --steps=50 \
  --repeat=20 \
  --output=./runtime/pezkuwichain/src/weights/
```

**This will:**
- Benchmark ALL pallets (system pallets + custom pallets)
- Generate individual weight files for each
- Place them in `runtime/pezkuwichain/src/weights/`

**Warning:** This can take 4-8 hours for a full runtime!

---

### Overhead Benchmarking

Measure the overhead of the runtime itself:

```bash
# Benchmark block execution overhead
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark overhead \
  --chain=dev \
  --execution=wasm \
  --wasm-execution=compiled
```

**This measures:**
- Empty block production time
- WASM execution overhead
- Host function call costs

---

## Storage Benchmarking

### Database Read/Write Performance

```bash
# Benchmark storage operations
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark storage \
  --chain=dev \
  --mul=2 \
  --weight-path=./runtime/pezkuwichain/src/weights/
```

**This benchmarks:**
- Database read time
- Database write time
- Proof size overhead

**Output:** Weight values for `DbWeight` in runtime

---

### Example DbWeight Configuration

```rust
// In runtime/pezkuwichain/src/lib.rs

parameter_types! {
    pub const DbWeight: RuntimeDbWeight = RuntimeDbWeight {
        read: 25_000_000,  // 25 milliseconds per read
        write: 100_000_000, // 100 milliseconds per write
    };
}
```

**Adjust these based on benchmarks for your storage backend:**
- RocksDB (default): ~25ms read, ~100ms write
- ParityDB: ~20ms read, ~80ms write (faster)

---

## Network Benchmarking

### Block Import Performance

```bash
# Measure block import time
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark block \
  --chain=dev \
  --from 1 \
  --to 1000
```

**Metrics:**
- Average block import time
- Peak block import time
- Import variance

---

### Transaction Pool Performance

Test transaction validation speed:

```javascript
// benchmark-txpool.js
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function benchmarkTxPool() {
  const api = await ApiPromise.create({
    provider: new WsProvider('ws://localhost:9933')
  });

  const startTime = Date.now();
  const txCount = 1000;

  // Submit 1000 transactions rapidly
  const promises = [];
  for (let i = 0; i < txCount; i++) {
    const tx = api.tx.balances.transfer(someAddress, 1000);
    promises.push(tx.signAndSend(sender, { nonce: i }));
  }

  await Promise.all(promises);

  const duration = Date.now() - startTime;
  const tps = (txCount / duration) * 1000;

  console.log(`Transaction pool processed ${txCount} tx in ${duration}ms`);
  console.log(`Throughput: ${tps.toFixed(2)} tx/s`);
}

benchmarkTxPool();
```

---

## Benchmark Analysis

### Interpreting Weight Results

#### Example Weight Function
```rust
fn transfer() -> Weight {
    Weight::from_parts(50_000_000, 0)
        .saturating_add(T::DbWeight::get().reads(2))
        .saturating_add(T::DbWeight::get().writes(2))
}
```

**Breakdown:**
- `50_000_000` = 50 microseconds of computation
- `reads(2)` = 2 database reads (~50 microseconds total)
- `writes(2)` = 2 database writes (~200 microseconds total)
- **Total:** ~300 microseconds per `transfer()` call

#### Weight to Time Conversion
```
Weight: 300_000_000
= 300 microseconds
= 0.3 milliseconds
= 0.0003 seconds
```

#### Transactions Per Block Calculation
```
Block weight limit: 2_000_000_000_000 (2 trillion)
Transfer weight: 300_000_000

Max transfers per block = 2_000_000_000_000 / 300_000_000
                        = 6,666 transactions per block
```

With 6-second block time:
```
TPS = 6,666 / 6 = 1,111 transactions per second
```

---

### Weight Distribution Analysis

Create analysis script:

```python
# analyze-weights.py
import re

def parse_weight_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Extract all weight values
    weights = re.findall(r'Weight::from_parts\((\d+)', content)
    weights = [int(w) for w in weights]

    return {
        'min': min(weights),
        'max': max(weights),
        'avg': sum(weights) / len(weights),
        'count': len(weights)
    }

# Analyze each pallet
pallets = ['pez_treasury', 'welati', 'tiki']
for pallet in pallets:
    stats = parse_weight_file(f'./pallets/{pallet}/src/weights.rs')
    print(f"\n{pallet} weights:")
    print(f"  Min: {stats['min']:,} ({stats['min']/1_000_000:.2f} ms)")
    print(f"  Max: {stats['max']:,} ({stats['max']/1_000_000:.2f} ms)")
    print(f"  Avg: {stats['avg']:,.0f} ({stats['avg']/1_000_000:.2f} ms)")
    print(f"  Functions: {stats['count']}")
```

---

### Benchmark Regression Testing

**Setup Baseline:**
```bash
# Save current benchmarks as baseline
cp pallets/*/src/weights.rs benchmarks/baseline/

# Store commit hash
git rev-parse HEAD > benchmarks/baseline/commit.txt
```

**Run Regression Test:**
```bash
#!/bin/bash
# benchmark-regression-test.sh

echo "Running fresh benchmarks..."
./benchmark-all-pallets.sh

echo "Comparing with baseline..."
for pallet in pez_treasury welati tiki; do
  diff -u benchmarks/baseline/${pallet}_weights.rs \
          pallets/${pallet}/src/weights.rs

  if [ $? -ne 0 ]; then
    echo "⚠️  Weight regression detected in $pallet"
  fi
done
```

---

## Best Practices

### 1. Consistent Benchmarking Environment
- **Same hardware** for all benchmarks
- **Idle system** (no other processes)
- **Same compiler version** (rustc version matters)
- **Same CPU governor** (performance mode, not powersave)

```bash
# Set CPU to performance mode
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

### 2. Multiple Runs for Accuracy
```bash
# Run benchmark 3 times, take median
for i in 1 2 3; do
  /home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark pallet \
    --pallet=pallet_pez_treasury \
    --extrinsic='*' \
    --repeat=20 \
    --output=weights_run${i}.rs
done

# Compare and verify consistency
```

### 3. Version Control Weights
```bash
# Commit weight files
git add pallets/*/src/weights.rs
git commit -m "chore: update pallet weights from benchmarks"
```

### 4. Document Benchmark Environment
```markdown
# Benchmark Environment

**Date:** 2025-11-13
**Hardware:**
- CPU: Intel Xeon E5-2670 @ 2.60GHz
- RAM: 32GB DDR4
- Storage: Samsung 970 EVO Plus NVMe

**Software:**
- rustc: 1.75.0
- cargo: 1.75.0
- OS: Ubuntu 22.04 LTS

**Benchmark Settings:**
- steps: 50
- repeat: 20
- execution: wasm
```

---

## Troubleshooting

### Issue: Benchmark Fails with "Out of Memory"

**Solution:**
```bash
# Increase system limits
ulimit -s unlimited
ulimit -n 65536

# Or benchmark one extrinsic at a time
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi benchmark pallet \
  --pallet=pallet_pez_treasury \
  --extrinsic='initialize_treasury' \  # Single extrinsic
  --repeat=20
```

### Issue: Weights Seem Too High/Low

**Check:**
1. Hardware too slow/fast compared to validators
2. Background processes interfering
3. CPU throttling due to temperature
4. Incorrect database backend configuration

**Verify with:**
```bash
# Monitor CPU frequency during benchmark
watch -n 1 'cat /proc/cpuinfo | grep MHz'

# Check for thermal throttling
sensors
```

### Issue: Benchmark Takes Too Long

**Solutions:**
```bash
# Reduce steps and repeats (less accurate)
--steps=20 \
--repeat=10

# Benchmark specific extrinsics only
--extrinsic='transfer,initialize_treasury'

# Use faster hardware
```

---

## Automation

### CI/CD Benchmark Pipeline

```yaml
# .github/workflows/benchmark.yml
name: Runtime Benchmarks

on:
  pull_request:
    paths:
      - 'pallets/**'
      - 'runtime/**'

jobs:
  benchmark:
    runs-on: benchmark-runner  # Dedicated hardware

    steps:
      - uses: actions/checkout@v2

      - name: Build with benchmarks
        run: cargo build --release --features runtime-benchmarks

      - name: Run benchmarks
        run: ./scripts/benchmark-all-pallets.sh

      - name: Check for regressions
        run: ./scripts/benchmark-regression-test.sh

      - name: Comment results on PR
        run: ./scripts/comment-benchmark-results.sh
```

---

## Related Documentation

- **Load Testing Guide:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/LOAD_TESTING_GUIDE.md`
- **Monitoring Guide:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/MONITORING_GUIDE.md`
- **Deployment Guide:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/DEPLOYMENT_GUIDE.md`

---

**Document Version:** 1.0
**Last Updated:** 2025-11-13
**Next Review:** After major pallet changes
**Maintained By:** PezkuwiChain Performance Team

**IMPORTANT:** Always re-benchmark after significant pallet logic changes!
