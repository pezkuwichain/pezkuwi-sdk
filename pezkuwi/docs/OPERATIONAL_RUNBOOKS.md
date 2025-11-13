# PezkuwiChain Operational Runbooks
**Version:** 1.0
**Date:** 2025-11-13
**Status:** Production Ready

## Table of Contents
1. [Node Operations](#node-operations)
2. [Incident Response](#incident-response)
3. [Maintenance Procedures](#maintenance-procedures)
4. [Performance Optimization](#performance-optimization)
5. [Troubleshooting Guide](#troubleshooting-guide)

---

## Node Operations

### Runbook 1: Starting a PezkuwiChain Node

**Objective:** Start a PezkuwiChain validator or full node
**Prerequisites:** Binary compiled, chain spec available, ports configured
**Duration:** 5-10 minutes

#### Steps:

1. **Verify Prerequisites**
```bash
# Check binary exists
ls -lh /home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi

# Verify chain spec
ls -lh /path/to/chainspec.json

# Check ports are available
netstat -tuln | grep -E '30333|9933|9944|9615'
```

2. **Start Node** (Choose validator or full node)

**For Validator:**
```bash
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi \
  --validator \
  --name "MyValidator" \
  --chain /path/to/chainspec.json \
  --base-path /data/pezkuwi \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --prometheus-port 9615 \
  --rpc-cors all \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --pruning archive \
  2>&1 | tee /var/log/pezkuwi/node.log
```

**For Full Node:**
```bash
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi \
  --name "MyFullNode" \
  --chain /path/to/chainspec.json \
  --base-path /data/pezkuwi \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --prometheus-port 9615 \
  --rpc-cors all \
  --pruning 1000 \
  2>&1 | tee /var/log/pezkuwi/node.log
```

3. **Verify Node Started**
```bash
# Check process is running
ps aux | grep pezkuwi

# Check logs for errors
tail -f /var/log/pezkuwi/node.log

# Verify metrics endpoint
curl http://localhost:9615/metrics

# Check peer connections (should start increasing)
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9933/
```

4. **Monitor Initial Sync**
```bash
# Watch block height
watch -n 5 'curl -s -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getBlock\"}" \
  http://localhost:9933/ | jq ".result.block.header.number"'
```

**Success Criteria:**
- ✅ Process running without crashes
- ✅ No critical errors in logs
- ✅ Metrics endpoint responding
- ✅ Peer count > 0
- ✅ Block height increasing

**Rollback:** Stop node with `kill -SIGTERM <pid>`

---

### Runbook 2: Graceful Node Shutdown

**Objective:** Stop a running node without corruption
**Prerequisites:** Node running
**Duration:** 2-5 minutes

#### Steps:

1. **Check Current Status**
```bash
# Get node PID
PID=$(ps aux | grep pezkuwi | grep -v grep | awk '{print $2}')

# Check if syncing
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState"}' \
  http://localhost:9933/ | jq
```

2. **Initiate Graceful Shutdown**
```bash
# Send SIGTERM signal (allows cleanup)
kill -SIGTERM $PID

# Monitor shutdown progress
tail -f /var/log/pezkuwi/node.log
```

3. **Wait for Clean Exit**
```bash
# Wait up to 60 seconds for process to exit
timeout=60
while [ $timeout -gt 0 ] && kill -0 $PID 2>/dev/null; do
    echo "Waiting for node to stop... ${timeout}s remaining"
    sleep 5
    timeout=$((timeout - 5))
done
```

4. **Force Kill if Necessary**
```bash
# If still running after timeout
if kill -0 $PID 2>/dev/null; then
    echo "Force killing node..."
    kill -SIGKILL $PID
fi
```

5. **Verify Shutdown**
```bash
# Confirm process stopped
ps aux | grep pezkuwi | grep -v grep

# Check for database corruption
# (Will be verified on next start)
ls -lh /data/pezkuwi/chains/*/db/
```

**Success Criteria:**
- ✅ Process terminated cleanly
- ✅ No database corruption warnings in logs
- ✅ Prometheus exporter stopped

---

### Runbook 3: Node Restart with Zero Downtime

**Objective:** Restart node while maintaining network participation
**Prerequisites:** Running node, backup node available
**Duration:** 10-15 minutes

#### Steps:

1. **Prepare Backup Node**
```bash
# On backup server, sync with primary
rsync -avz primary:/data/pezkuwi/ /data/pezkuwi/

# Start backup node (as validator if needed)
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi \
  --validator \
  --base-path /data/pezkuwi \
  [other flags...]
```

2. **Transfer Session Keys (If Validator)**
```bash
# On primary, get session keys
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
  http://localhost:9933/

# Set keys on backup node
# (Use governance call or backup key injection)
```

3. **Monitor Backup Node Sync**
```bash
# Wait until backup is fully synced
while true; do
    syncing=$(curl -s -H "Content-Type: application/json" \
      -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState"}' \
      http://backup:9933/ | jq '.result.currentBlock')

    if [ "$syncing" = "null" ]; then
        echo "Backup node synced!"
        break
    fi
    sleep 10
done
```

4. **Shutdown Primary**
```bash
# On primary, graceful shutdown
kill -SIGTERM $(pgrep pezkuwi)
```

5. **Update DNS/Load Balancer**
```bash
# Point traffic to backup
# (Specific to your infrastructure)
```

6. **Restart Primary**
```bash
# On primary, restart node
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi [flags...]
```

**Success Criteria:**
- ✅ No missed blocks
- ✅ Continuous network participation
- ✅ Primary node restarted successfully

---

## Incident Response

### Runbook 4: Node Down - Emergency Recovery

**Severity:** Critical
**Objective:** Restore node operation ASAP
**Maximum Downtime:** 5 minutes

#### Steps:

1. **Immediate Assessment** (< 1 minute)
```bash
# Check if process exists
ps aux | grep pezkuwi

# Check system resources
free -h
df -h
top -bn1 | head -20
```

2. **Quick Diagnosis** (< 2 minutes)
```bash
# Check recent logs
tail -100 /var/log/pezkuwi/node.log | grep -i "error\|panic\|fatal"

# Check system logs
journalctl -xe | tail -50

# Check disk space (common cause)
df -h /data/pezkuwi
```

3. **Immediate Recovery** (< 2 minutes)

**If out of disk space:**
```bash
# Emergency cleanup
cd /data/pezkuwi
rm -rf chains/*/db/full/*.log
# Then restart node
```

**If corrupted database:**
```bash
# Restore from backup (see Disaster Recovery runbook)
# OR purge and resync (slower)
rm -rf /data/pezkuwi/chains/*/db
# Then restart node
```

**If resource exhaustion:**
```bash
# Kill memory hogs
pkill -9 <offending_process>
# Restart node with lower cache
--db-cache 512  # Reduce from default 1024
```

4. **Restart Node**
```bash
# Use systemd or direct start
systemctl start pezkuwi
# OR
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi [flags...] &
```

5. **Verify Recovery**
```bash
# Check process
ps aux | grep pezkuwi

# Monitor startup
tail -f /var/log/pezkuwi/node.log

# Verify metrics
curl http://localhost:9615/metrics | grep substrate_block_height
```

6. **Post-Incident**
```bash
# Save logs for analysis
cp /var/log/pezkuwi/node.log /var/log/pezkuwi/incident-$(date +%Y%m%d-%H%M).log

# Document incident
# Update incident tracking system
```

**Success Criteria:**
- ✅ Node running and syncing
- ✅ Peer connections established
- ✅ Block production resumed (if validator)

**Escalation:** If not resolved in 10 minutes, escalate to senior engineer

---

### Runbook 5: Finalization Stalled

**Severity:** Critical
**Objective:** Restore block finalization
**Maximum Resolution Time:** 15 minutes

#### Steps:

1. **Verify Issue** (< 2 minutes)
```bash
# Check finalized vs best block
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
  http://localhost:9933/ | jq '.result.number'

curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getFinalizedHead"}' \
  http://localhost:9933/ | jq
```

2. **Check Consensus** (< 3 minutes)
```bash
# Check validator set
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "grandpa_roundState"}' \
  http://localhost:9933/ | jq

# Check if enough validators online
# (Need > 2/3 for finality)
```

3. **Diagnose Root Cause** (< 5 minutes)

**Check network partition:**
```bash
# Check peer connectivity
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
  http://localhost:9933/ | jq '.result | length'

# Ping other validators
ping validator1.pezkuwichain.com
```

**Check validator participation:**
```bash
# Look for missed blocks
grep "Failed to propose" /var/log/pezkuwi/node.log
```

4. **Resolution Actions** (< 5 minutes)

**If network partition:**
```bash
# Check firewall rules
iptables -L
ufw status

# Verify bootnodes
# Restart node with explicit bootnodes
--bootnodes /ip4/BOOTNODE_IP/tcp/30333/p2p/PEER_ID
```

**If validator offline:**
```bash
# Contact validator operators
# Check validator dashboard
# Prepare emergency governance proposal if needed
```

5. **Monitor Recovery**
```bash
# Watch finalized block height
watch -n 5 'curl -s -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getFinalizedHead\"}" \
  http://localhost:9933/ | jq'
```

**Success Criteria:**
- ✅ Finalized block height increasing
- ✅ Lag between best and finalized < 10 blocks

**Escalation:** If not resolved in 15 minutes, call emergency validator meeting

---

### Runbook 6: High Memory Usage

**Severity:** Warning → Critical (if sustained)
**Objective:** Reduce memory usage below threshold
**Maximum Resolution Time:** 30 minutes

#### Steps:

1. **Verify Memory Usage** (< 2 minutes)
```bash
# Check current memory
free -h

# Check process memory
ps aux --sort=-%mem | head -10

# Check pezkuwi specifically
ps -p $(pgrep pezkuwi) -o pid,vsz=MEMORY,rss,cmd | awk '{print $1"\t"$2/1024"MB\t"$3/1024"MB"}'
```

2. **Identify Memory Consumers** (< 5 minutes)
```bash
# Check cache sizes
grep -i cache /var/log/pezkuwi/node.log | tail -20

# Check for memory leaks
valgrind --leak-check=full --log-file=valgrind.log \
  /home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi [flags...]
# (Only on test node, not production!)
```

3. **Immediate Mitigation** (< 5 minutes)

**Option A: Reduce cache sizes**
```bash
# Restart with lower cache
kill -SIGTERM $(pgrep pezkuwi)
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi \
  --db-cache 512 \  # Reduce from 1024
  --state-cache-size 134217728 \  # 128MB instead of default
  [other flags...]
```

**Option B: Enable pruning** (if archive node not required)
```bash
# Restart with pruning enabled
--pruning 1000  # Keep last 1000 blocks
```

4. **Long-term Solution** (< 20 minutes)

**Add swap space:**
```bash
# Create 8GB swap file
fallocate -l 8G /swapfile
chmod 600 /swapfile
mkswap /swapfile
swapon /swapfile

# Make permanent
echo '/swapfile none swap sw 0 0' | tee -a /etc/fstab
```

**Upgrade server resources:**
```bash
# Plan server upgrade
# Minimum recommended: 16GB RAM
# Recommended: 32GB RAM for validator
```

5. **Monitor Stabilization**
```bash
# Watch memory usage
watch -n 10 'free -h && echo && ps aux | grep pezkuwi | grep -v grep'
```

**Success Criteria:**
- ✅ Memory usage below 80% of available
- ✅ No OOM events
- ✅ Node stable for 1 hour

---

## Maintenance Procedures

### Runbook 7: Runtime Upgrade

**Objective:** Upgrade chain runtime
**Prerequisites:** New runtime WASM, governance approval
**Duration:** 1-2 hours

#### Steps:

1. **Prepare Upgrade** (< 15 minutes)
```bash
# Build new runtime
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi
cargo build --release -p pezkuwichain-runtime

# Get runtime WASM
WASM=/home/mamostehp/Pezkuwi-SDK/target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.compact.compressed.wasm

# Verify WASM validity
subwasm info $WASM
subwasm metadata $WASM
```

2. **Test on Local Dev Chain** (< 20 minutes)
```bash
# Start local test node
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi \
  --dev \
  --tmp

# Submit upgrade via Polkadot.js Apps
# Test all extrinsics still work
```

3. **Propose Upgrade** (Via Governance)
```bash
# Create upgrade proposal
# Via Polkadot.js Apps or CLI:
# 1. Navigate to Developer > Extrinsics
# 2. Select democracy.propose
# 3. Attach system.setCode call with new WASM
# 4. Submit with sufficient deposit
```

4. **Monitor Voting**
```bash
# Check referendum status
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "democracy_referendums"}' \
  http://localhost:9933/ | jq
```

5. **Prepare for Execution** (Day of upgrade)
```bash
# Backup current runtime
cp $OLD_WASM /backup/runtime-v$(date +%Y%m%d).wasm

# Monitor block countdown
# Upgrade executes automatically at scheduled block
```

6. **Verify Upgrade Success**
```bash
# Check runtime version
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_getRuntimeVersion"}' \
  http://localhost:9933/ | jq '.result.specVersion'

# Check logs for errors
grep -i "runtime\|upgrade" /var/log/pezkuwi/node.log | tail -50

# Test key extrinsics
# Via Polkadot.js Apps
```

**Success Criteria:**
- ✅ Runtime version updated
- ✅ No migration errors
- ✅ All pallets functional
- ✅ No performance degradation

**Rollback:** Prepare emergency governance proposal with old runtime WASM

---

### Runbook 8: Database Pruning

**Objective:** Reclaim disk space by pruning old blocks
**Prerequisites:** Full node (not archive), sufficient disk space for operation
**Duration:** 2-4 hours

#### Steps:

1. **Assess Current State** (< 5 minutes)
```bash
# Check database size
du -sh /data/pezkuwi/chains/*/db/

# Check available disk space
df -h /data/pezkuwi

# Check current pruning setting
grep pruning /var/log/pezkuwi/node.log | tail -1
```

2. **Backup Current State**
```bash
# Stop node
systemctl stop pezkuwi

# Backup database (optional but recommended)
tar -czf pezkuwi-db-backup-$(date +%Y%m%d).tar.gz \
  /data/pezkuwi/chains/*/db/
```

3. **Perform Pruning**

**Method A: Restart with pruning enabled**
```bash
# Start node with pruning
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi \
  --pruning 1000 \
  [other flags...]

# Pruning happens automatically over time
```

**Method B: Manual database cleanup**
```bash
# WARNING: Only for emergency disk space recovery
# Remove old state
rm -rf /data/pezkuwi/chains/*/db/full/*.sst

# Restart node (will rebuild necessary state)
systemctl start pezkuwi
```

4. **Monitor Pruning Progress**
```bash
# Watch database size decrease
watch -n 300 'du -sh /data/pezkuwi/chains/*/db/'

# Check node still syncing
tail -f /var/log/pezkuwi/node.log
```

5. **Verify Node Health**
```bash
# Check block height
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
  http://localhost:9933/ | jq '.result.number'

# Check peers
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9933/ | jq
```

**Success Criteria:**
- ✅ Database size reduced
- ✅ Node syncing normally
- ✅ No data corruption

**Note:** Archive nodes should NOT prune - they need full history

---

## Performance Optimization

### Runbook 9: Performance Tuning

**Objective:** Optimize node performance
**Duration:** 1-2 hours

#### Steps:

1. **Baseline Metrics** (< 10 minutes)
```bash
# Capture current performance
curl http://localhost:9615/metrics > baseline-metrics.txt

# Note key metrics:
# - Block import time
# - CPU usage
# - Memory usage
# - Disk I/O
```

2. **Database Optimization**
```bash
# Increase cache size (if RAM available)
--db-cache 2048  # 2GB cache

# Use faster database backend
--database paritydb  # Instead of default RocksDB

# Enable compression
--database-compression-type zstd
```

3. **Network Optimization**
```bash
# Increase peer connections
--in-peers 50  # Default 25
--out-peers 50  # Default 25

# Optimize libp2p settings
--kademlia-disjoint-query-paths
--kademlia-parallelism 10
```

4. **Runtime Optimization**
```bash
# Enable runtime execution optimization
--execution native-else-wasm  # Prefer native execution

# Increase WASM memory limit
--max-runtime-instances 8  # Default 2
```

5. **System-Level Tuning**
```bash
# Increase file descriptors
ulimit -n 65536

# Set CPU affinity (pin to specific cores)
taskset -c 0-3 /home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi [flags...]

# Enable huge pages
echo always > /sys/kernel/mm/transparent_hugepage/enabled
```

6. **Verify Improvements**
```bash
# Capture new metrics
curl http://localhost:9615/metrics > optimized-metrics.txt

# Compare
diff baseline-metrics.txt optimized-metrics.txt
```

**Success Criteria:**
- ✅ Block import time reduced
- ✅ CPU usage optimized
- ✅ No stability issues

---

## Troubleshooting Guide

### Common Issues Matrix

| Issue | Symptoms | Diagnosis | Solution | Runbook |
|-------|----------|-----------|----------|---------|
| Node crashes | Process exits | Check logs for panic | Fix root cause, restart | Runbook 4 |
| Sync stalls | Block height not increasing | Check peers, network | Add bootnodes, check firewall | Runbook 1 |
| High CPU | Top shows 100% CPU | Profile with perf | Optimize workload | Runbook 9 |
| Out of disk | df shows 100% | Check database size | Prune or add storage | Runbook 8 |
| Memory leak | Memory grows over time | Monitor with valgrind | Upgrade or apply fix | Runbook 6 |
| Network partition | Low peer count | Check connectivity | Fix network config | Runbook 5 |

### Emergency Contacts

- **On-Call Engineer:** [Configure]
- **Lead Developer:** [Configure]
- **Infrastructure Team:** [Configure]
- **Emergency Escalation:** [Configure]

---

**Document Version:** 1.0
**Last Updated:** 2025-11-13
**Maintained By:** PezkuwiChain Operations Team
**Next Review:** 2025-12-13
