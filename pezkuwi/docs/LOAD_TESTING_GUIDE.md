# PezkuwiChain Load Testing Guide
**Version:** 1.0
**Date:** 2025-11-13
**Status:** Production Ready

## Table of Contents
1. [Overview](#overview)
2. [Test Environment Setup](#test-environment-setup)
3. [Load Test Scenarios](#load-test-scenarios)
4. [Performance Metrics](#performance-metrics)
5. [Test Execution](#test-execution)
6. [Results Analysis](#results-analysis)
7. [Optimization Recommendations](#optimization-recommendations)

---

## Overview

### Purpose
This guide provides comprehensive procedures for load testing PezkuwiChain to ensure it can handle production workloads, identify performance bottlenecks, and validate scalability requirements.

### Testing Goals
- Validate block production under load
- Measure transaction throughput capacity
- Identify resource bottlenecks (CPU, memory, network, disk)
- Test finalization under stress
- Validate network resilience
- Establish performance baselines

### Performance Targets

| Metric | Target | Critical Threshold |
|--------|--------|-------------------|
| Block Time | 6 seconds | < 10 seconds |
| Transaction Throughput | 100 tx/s | > 50 tx/s |
| Block Finalization | < 30 seconds | < 60 seconds |
| Peer Connections | > 50 | > 25 |
| Memory Usage | < 8GB | < 12GB |
| CPU Usage | < 60% | < 80% |
| Disk I/O Wait | < 10% | < 20% |

---

## Test Environment Setup

### Infrastructure Requirements

#### Minimum Test Cluster
```
┌─────────────────────────────────────────────────────┐
│                 Test Network                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  4 Validator Nodes (8 CPU, 16GB RAM, SSD)         │
│  2 Full Nodes (4 CPU, 8GB RAM, SSD)               │
│  1 Load Generator (8 CPU, 16GB RAM)               │
│  1 Monitoring Server (4 CPU, 8GB RAM)             │
│                                                     │
│  Network: 1Gbps interconnect                       │
│  Storage: 500GB SSD per node                       │
└─────────────────────────────────────────────────────┘
```

#### Recommended Production-Like Setup
- 6-8 validator nodes
- 4-6 full nodes
- 2 load generators for distributed load
- Dedicated monitoring infrastructure
- Geographic distribution (if testing cross-region)

### Environment Configuration

#### 1. Build Optimized Binary
```bash
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi

# Build with optimizations
cargo build --release --features runtime-benchmarks

# Verify binary
ls -lh /home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi
```

#### 2. Create Test Chain Spec
```bash
# Generate chain spec for testing
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi build-spec \
  --chain local \
  --disable-default-bootnode \
  > test-chainspec.json

# Convert to raw format
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi build-spec \
  --chain test-chainspec.json \
  --raw \
  --disable-default-bootnode \
  > test-chainspec-raw.json
```

#### 3. Start Test Network

**Validator 1 (Alice):**
```bash
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi \
  --base-path /tmp/pezkuwi-test/alice \
  --chain test-chainspec-raw.json \
  --alice \
  --port 30333 \
  --rpc-port 9933 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --validator \
  --rpc-cors all \
  --prometheus-external \
  --prometheus-port 9615
```

**Validator 2 (Bob):**
```bash
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi \
  --base-path /tmp/pezkuwi-test/bob \
  --chain test-chainspec-raw.json \
  --bob \
  --port 30334 \
  --rpc-port 9934 \
  --validator \
  --rpc-cors all \
  --prometheus-external \
  --prometheus-port 9616 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/PEER_ID_OF_ALICE
```

**Additional validators:** Repeat with Charlie, Dave, etc.

**Full Node:**
```bash
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi \
  --base-path /tmp/pezkuwi-test/fullnode1 \
  --chain test-chainspec-raw.json \
  --port 30335 \
  --rpc-port 9935 \
  --rpc-cors all \
  --prometheus-external \
  --prometheus-port 9617 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/PEER_ID_OF_ALICE
```

#### 4. Verify Network Health
```bash
# Check all nodes are connected
for port in 9933 9934 9935; do
  echo "Checking node on port $port..."
  curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
    http://localhost:$port/ | jq
done

# Verify block production
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
  http://localhost:9933/ | jq '.result.number'
```

---

## Load Test Scenarios

### Scenario 1: Baseline Performance Test

**Objective:** Establish baseline metrics without load
**Duration:** 30 minutes
**Load:** Natural network activity only

**Metrics to Capture:**
- Average block time
- Idle resource usage (CPU, memory, disk)
- Network bandwidth at rest
- Finalization lag

**Expected Results:**
- Block time: ~6 seconds
- CPU usage: < 20%
- Memory usage: < 4GB
- Finalization lag: < 10 blocks

---

### Scenario 2: Sustained Transaction Load

**Objective:** Test system under sustained transaction load
**Duration:** 2 hours
**Load:** 50 transactions per second (constant)

#### Test Script
```javascript
// load-test-sustained.js
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function sustainedLoadTest() {
  const wsProvider = new WsProvider('ws://localhost:9933');
  const api = await ApiPromise.create({ provider: wsProvider });
  const keyring = new Keyring({ type: 'sr25519' });

  // Generate test accounts
  const testAccounts = [];
  for (let i = 0; i < 100; i++) {
    testAccounts.push(keyring.addFromUri(`//Test//${i}`));
  }

  console.log('Starting sustained load test...');
  const startTime = Date.now();
  const duration = 2 * 60 * 60 * 1000; // 2 hours
  const targetTps = 50;
  const intervalMs = 1000 / targetTps;

  let txCount = 0;
  let successCount = 0;
  let failCount = 0;

  const interval = setInterval(async () => {
    if (Date.now() - startTime > duration) {
      clearInterval(interval);
      console.log(`Test complete. Total: ${txCount}, Success: ${successCount}, Failed: ${failCount}`);
      process.exit(0);
    }

    try {
      const sender = testAccounts[txCount % testAccounts.length];
      const recipient = testAccounts[(txCount + 1) % testAccounts.length];

      const transfer = api.tx.balances.transfer(recipient.address, 1000);
      await transfer.signAndSend(sender, ({ status }) => {
        if (status.isInBlock || status.isFinalized) {
          successCount++;
        }
      });

      txCount++;
    } catch (error) {
      failCount++;
      console.error('Transaction failed:', error.message);
    }
  }, intervalMs);
}

sustainedLoadTest().catch(console.error);
```

**Run Test:**
```bash
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi
npm install @polkadot/api
node load-test-sustained.js
```

**Expected Results:**
- Transaction success rate: > 99%
- Block time: 6-8 seconds
- CPU usage: 40-60%
- Memory usage: 5-7GB

---

### Scenario 3: Spike Load Test

**Objective:** Test system response to sudden traffic spikes
**Duration:** 30 minutes
**Load:** Alternating between 10 tps and 200 tps every 2 minutes

#### Test Script
```javascript
// load-test-spike.js
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function spikeLoadTest() {
  const wsProvider = new WsProvider('ws://localhost:9933');
  const api = await ApiPromise.create({ provider: wsProvider });
  const keyring = new Keyring({ type: 'sr25519' });

  const testAccounts = [];
  for (let i = 0; i < 100; i++) {
    testAccounts.push(keyring.addFromUri(`//Test//${i}`));
  }

  console.log('Starting spike load test...');
  const duration = 30 * 60 * 1000; // 30 minutes
  const spikeDuration = 2 * 60 * 1000; // 2 minutes per phase
  let currentPhase = 'low'; // 'low' or 'high'
  let phaseStart = Date.now();

  const sendTransactions = async (tps) => {
    const intervalMs = 1000 / tps;

    return new Promise((resolve) => {
      const interval = setInterval(async () => {
        if (Date.now() - phaseStart > spikeDuration) {
          clearInterval(interval);
          resolve();
        }

        try {
          const sender = testAccounts[Math.floor(Math.random() * testAccounts.length)];
          const recipient = testAccounts[Math.floor(Math.random() * testAccounts.length)];

          const transfer = api.tx.balances.transfer(recipient.address, 1000);
          await transfer.signAndSend(sender);
        } catch (error) {
          // Silent fail for spike test
        }
      }, intervalMs);
    });
  };

  while (Date.now() < duration) {
    if (currentPhase === 'low') {
      console.log('Low load phase: 10 tps');
      await sendTransactions(10);
      currentPhase = 'high';
    } else {
      console.log('High load phase: 200 tps');
      await sendTransactions(200);
      currentPhase = 'low';
    }
    phaseStart = Date.now();
  }

  console.log('Spike test complete');
  process.exit(0);
}

spikeLoadTest().catch(console.error);
```

**Expected Results:**
- No crashes during spikes
- Block time remains stable (< 10 seconds)
- Transaction pool manages backlog gracefully
- Recovery time after spike: < 30 seconds

---

### Scenario 4: Stress Test (Breaking Point)

**Objective:** Find maximum sustainable throughput
**Duration:** 1 hour
**Load:** Incrementally increase from 50 to 500 tps

#### Test Approach
```bash
# Start with 50 tps, increase by 50 every 10 minutes
for tps in 50 100 150 200 250 300 350 400 450 500; do
  echo "Testing at ${tps} tps..."
  node load-test-constant.js --tps $tps --duration 600
  sleep 60  # Cool down period
done
```

**Metrics to Monitor:**
- Transaction pool depth
- Block import time
- Finalization lag
- Memory growth rate
- Error rate

**Expected Breaking Point:**
- Target: > 200 tps sustained
- Warning level: Transaction success rate < 95%
- Critical level: Block production slows > 10s

---

### Scenario 5: Multi-Pallet Stress Test

**Objective:** Test realistic mixed workload
**Duration:** 1 hour
**Load:** Mixed transactions across all pallets

#### Transaction Mix
- 40% Balance transfers
- 20% Treasury operations
- 20% Governance votes
- 10% Citizenship operations
- 10% Role management

#### Test Script
```javascript
// load-test-mixed.js
async function mixedWorkloadTest() {
  const transactionTypes = [
    { weight: 0.40, fn: () => generateBalanceTransfer() },
    { weight: 0.20, fn: () => generateTreasuryOp() },
    { weight: 0.20, fn: () => generateGovernanceVote() },
    { weight: 0.10, fn: () => generateCitizenshipOp() },
    { weight: 0.10, fn: () => generateRoleManagement() }
  ];

  const selectTransaction = () => {
    const rand = Math.random();
    let cumulative = 0;

    for (const tx of transactionTypes) {
      cumulative += tx.weight;
      if (rand < cumulative) {
        return tx.fn();
      }
    }
  };

  // Run for 1 hour at 100 tps
  const targetTps = 100;
  const duration = 60 * 60 * 1000;

  setInterval(async () => {
    try {
      const tx = selectTransaction();
      await tx.signAndSend(testAccount);
    } catch (error) {
      console.error('Transaction failed:', error);
    }
  }, 1000 / targetTps);
}
```

**Expected Results:**
- All pallet types handle load gracefully
- No specific pallet becomes a bottleneck
- Resource usage proportional to transaction mix

---

## Performance Metrics

### Key Metrics to Collect

#### Blockchain Metrics
```prometheus
# Block production rate
rate(substrate_block_height{status="best"}[5m])

# Block import time
substrate_block_verification_and_import_time

# Finalization lag
substrate_block_height{status="best"} - substrate_block_height{status="finalized"}

# Transaction pool size
substrate_sub_txpool_validations_scheduled
```

#### System Metrics
```bash
# CPU usage
mpstat 1 10

# Memory usage
free -h && vmstat 1 10

# Disk I/O
iostat -x 1 10

# Network throughput
iftop -t -s 60
```

#### Network Metrics
```prometheus
# Peer connections
substrate_sub_libp2p_peers_count

# Network bytes
rate(substrate_sub_libp2p_notifications_sizes_sum[5m])
```

---

## Test Execution

### Pre-Test Checklist

- [ ] Test network deployed and stable
- [ ] Monitoring stack operational
- [ ] Baseline metrics captured
- [ ] Test accounts funded
- [ ] Load generator scripts validated
- [ ] Storage space available (> 100GB free)
- [ ] Test duration and schedule confirmed
- [ ] Team notified of test window

### Execution Procedure

#### 1. Baseline Capture (30 minutes)
```bash
# Start metric collection
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi/monitoring
docker-compose up -d

# Capture baseline
curl http://localhost:9615/metrics > baseline-metrics.txt

# Monitor for 30 minutes
watch -n 60 'curl -s http://localhost:9615/metrics | grep substrate_block_height'
```

#### 2. Execute Load Tests
```bash
# Run each scenario sequentially
for scenario in sustained spike stress mixed; do
  echo "Running $scenario test..."
  node load-test-${scenario}.js 2>&1 | tee ${scenario}-test.log

  # Cool down between tests
  echo "Cooling down for 10 minutes..."
  sleep 600
done
```

#### 3. Continuous Monitoring
```bash
# Watch key metrics in separate terminals

# Terminal 1: Block production
watch -n 5 'curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\"}" | jq ".result.number"'

# Terminal 2: System resources
watch -n 5 'ps aux | grep pezkuwi | grep -v grep'

# Terminal 3: Transaction pool
watch -n 5 'curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_pendingExtrinsics\"}" | jq "length"'
```

---

## Results Analysis

### Performance Report Template

```markdown
# Load Test Results - [Date]

## Test Environment
- **Network:** 4 validators, 2 full nodes
- **Hardware:** [Specs]
- **Duration:** [Total test time]
- **Load Generator:** [Tool and version]

## Scenario Results

### Baseline Performance
- Block time: X.XX seconds (avg)
- CPU usage: XX%
- Memory usage: X.XGB
- Finalization lag: X blocks

### Sustained Load (50 tps)
- Success rate: XX.X%
- Block time: X.XX seconds
- CPU usage: XX%
- Memory usage: X.XGB
- Peak transaction pool: XXX transactions

### Spike Test
- Max throughput sustained: XXX tps
- Recovery time: XX seconds
- Errors during spike: XX

### Breaking Point
- Maximum sustained throughput: XXX tps
- Breaking point: XXX tps
- Primary bottleneck: [CPU/Memory/Disk/Network]

## Bottlenecks Identified
1. [Description of bottleneck]
2. [Description of bottleneck]

## Recommendations
1. [Optimization recommendation]
2. [Hardware recommendation]
3. [Configuration tuning]
```

### Analysis Queries

#### Calculate Average Block Time
```bash
# From Prometheus
avg_over_time(rate(substrate_block_height{status="best"}[5m])[1h])
```

#### Find Peak Memory Usage
```bash
# From system metrics
max_over_time(substrate_memory_usage_bytes[1h])
```

#### Transaction Success Rate
```bash
# Calculate from logs
grep "Transaction included" test.log | wc -l
# vs
grep "Transaction" test.log | wc -l
```

---

## Optimization Recommendations

### Based on Common Bottlenecks

#### CPU-Bound
**Symptoms:** High CPU usage (> 80%), slow block import
**Solutions:**
```bash
# Increase execution workers
--execution-wasm WasmExecutor --execution-native-else-wasm

# Optimize runtime execution
--max-runtime-instances 8

# Use faster CPU (higher clock speed preferred)
```

#### Memory-Bound
**Symptoms:** High memory usage (> 12GB), swap activity
**Solutions:**
```bash
# Reduce cache sizes
--db-cache 1024  # 1GB cache

# Enable pruning
--pruning 1000

# Upgrade to more RAM (32GB recommended for validators)
```

#### Disk I/O Bound
**Symptoms:** High I/O wait, slow database operations
**Solutions:**
```bash
# Use NVMe SSD storage
# Enable compression
--database-compression-type zstd

# Use ParityDB instead of RocksDB
--database paritydb

# Separate database and node directories
--base-path /fast-ssd/data --database /ultra-fast-nvme/db
```

#### Network-Bound
**Symptoms:** High peer connection churn, slow block propagation
**Solutions:**
```bash
# Optimize peer limits
--in-peers 50 --out-peers 50

# Use reserved peers for validators
--reserved-nodes /ip4/VALIDATOR_IP/tcp/30333/p2p/PEER_ID

# Optimize libp2p
--kademlia-disjoint-query-paths
```

---

## Continuous Performance Testing

### Automated Testing Schedule

```cron
# Run nightly performance regression tests
0 2 * * * /opt/pezkuwi/scripts/nightly-performance-test.sh

# Weekly full load test
0 1 * * 0 /opt/pezkuwi/scripts/weekly-load-test.sh

# Monthly stress test
0 1 1 * * /opt/pezkuwi/scripts/monthly-stress-test.sh
```

### Performance Regression Detection

```bash
#!/bin/bash
# performance-regression-check.sh

CURRENT_TPS=$(get_current_throughput)
BASELINE_TPS=$(cat baseline-tps.txt)

if [ $CURRENT_TPS -lt $((BASELINE_TPS * 90 / 100)) ]; then
  echo "WARNING: Performance regression detected!"
  echo "Current: ${CURRENT_TPS} tps, Baseline: ${BASELINE_TPS} tps"
  exit 1
fi
```

---

## Related Documentation

- **Monitoring Guide:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/MONITORING_GUIDE.md`
- **Performance Benchmarking:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/PERFORMANCE_BENCHMARKING.md`
- **Operational Runbooks:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/OPERATIONAL_RUNBOOKS.md`

---

**Document Version:** 1.0
**Last Updated:** 2025-11-13
**Next Review:** 2025-12-13
**Maintained By:** PezkuwiChain Performance Team
