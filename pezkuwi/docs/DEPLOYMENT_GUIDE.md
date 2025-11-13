# PezkuwiChain Production Deployment Guide
**Version:** 1.0
**Date:** 2025-11-13
**Status:** Production Ready

## Table of Contents
1. [Pre-Deployment](#pre-deployment)
2. [Infrastructure Setup](#infrastructure-setup)
3. [Node Deployment](#node-deployment)
4. [Validator Setup](#validator-setup)
5. [Network Launch](#network-launch)
6. [Post-Deployment](#post-deployment)
7. [Upgrades & Maintenance](#upgrades--maintenance)

---

## Pre-Deployment

### Checklist Before Launch

#### Code Readiness
- [x] All security fixes applied and tested
- [x] Storage migrations implemented
- [x] Runtime compiles without errors
- [x] All tests passing (152/152)
- [ ] Weight benchmarks generated and applied
- [ ] Load testing completed successfully
- [ ] Code audit completed

#### Documentation
- [x] Security audit report
- [x] API documentation
- [x] Operational runbooks
- [x] Disaster recovery plan
- [x] Monitoring guide
- [ ] User documentation
- [ ] Developer documentation

#### Infrastructure
- [ ] Server hardware provisioned
- [ ] Network topology designed
- [ ] DNS configuration ready
- [ ] SSL certificates obtained
- [ ] Monitoring infrastructure deployed
- [ ] Backup systems configured

---

## Infrastructure Setup

### Hardware Requirements

#### Validator Node (Minimum)
```
CPU: 8 cores @ 3.0+ GHz
RAM: 32 GB
Storage: 1TB NVMe SSD
Network: 1 Gbps dedicated
Backup: Daily snapshots
```

#### Validator Node (Recommended)
```
CPU: 16 cores @ 3.5+ GHz
RAM: 64 GB
Storage: 2TB NVMe SSD (RAID 1)
Network: 10 Gbps dedicated
Backup: Hourly snapshots + offsite
```

#### Full Node (Archive)
```
CPU: 8 cores @ 3.0+ GHz
RAM: 16 GB
Storage: 4TB SSD (grows ~100GB/month)
Network: 1 Gbps
Backup: Weekly snapshots
```

#### RPC Node (Public)
```
CPU: 16 cores @ 3.0+ GHz
RAM: 32 GB
Storage: 1TB NVMe SSD
Network: 10 Gbps (high bandwidth)
Load Balancer: Recommended
Rate Limiting: Required
```

---

### Network Topology

```
┌────────────────────────────────────────────────────────────────┐
│                      Production Network                         │
├────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐      ┌──────────────┐      ┌──────────────┐│
│  │  Validator 1 │◄────►│  Validator 2 │◄────►│  Validator 3 ││
│  │   Private    │      │   Private    │      │   Private    ││
│  └──────────────┘      └──────────────┘      └──────────────┘│
│          ▲                     ▲                     ▲         │
│          │                     │                     │         │
│  ┌───────┴─────────────────────┴─────────────────────┴───────┐│
│  │              Sentry Nodes (Public-facing)                  ││
│  └────────────────────────────────────────────────────────────┘│
│                             ▲                                   │
│                             │                                   │
│  ┌──────────────────────────┴──────────────────────────────┐  │
│  │              Load Balancer (RPC/WS)                      │  │
│  └───────────────────────────────────────────────────────────┘ │
│                             ▲                                   │
│                             │                                   │
│                      Public Internet                            │
└────────────────────────────────────────────────────────────────┘
```

**Key Points:**
- Validators NEVER directly exposed to internet
- Sentry nodes protect validators
- Load balancer for RPC endpoints
- Geographic distribution recommended

---

### Server Provisioning

#### Option 1: Cloud Providers

**AWS:**
```bash
# Launch EC2 instance
aws ec2 run-instances \
  --image-id ami-ubuntu-22.04 \
  --instance-type c5.4xlarge \
  --key-name pezkuwi-validator \
  --security-group-ids sg-validator \
  --subnet-id subnet-private \
  --block-device-mappings DeviceName=/dev/sda1,Ebs={VolumeSize=2048,VolumeType=gp3}
```

**Google Cloud:**
```bash
gcloud compute instances create pezkuwi-validator-1 \
  --machine-type=n2-standard-16 \
  --zone=us-central1-a \
  --boot-disk-size=2TB \
  --boot-disk-type=pd-ssd
```

**Digital Ocean:**
```bash
doctl compute droplet create pezkuwi-validator-1 \
  --size c-16 \
  --image ubuntu-22-04-x64 \
  --region nyc3 \
  --volumes volume-pezkuwi-data
```

#### Option 2: Dedicated Servers

**Hetzner, OVH, Vultr, etc.**
- Choose bare metal for best performance
- Ensure root access for system tuning
- Configure RAID for redundancy

---

### System Configuration

#### 1. Initial Server Setup
```bash
# Update system
apt-get update && apt-get upgrade -y

# Install dependencies
apt-get install -y \
  build-essential \
  git \
  curl \
  wget \
  clang \
  libssl-dev \
  llvm \
  libudev-dev \
  pkg-config

# Create pezkuwi user
useradd -m -s /bin/bash pezkuwi
usermod -aG sudo pezkuwi

# Set up firewall
ufw allow 22/tcp  # SSH
ufw allow 30333/tcp  # P2P
ufw allow 9615/tcp  # Prometheus (from monitoring server only)
ufw enable
```

#### 2. Storage Configuration
```bash
# Format and mount data disk
mkfs.ext4 /dev/nvme1n1
mkdir -p /data/pezkuwi
mount /dev/nvme1n1 /data/pezkuwi

# Add to fstab for persistence
echo '/dev/nvme1n1 /data/pezkuwi ext4 defaults,noatime 0 2' >> /etc/fstab

# Set ownership
chown -R pezkuwi:pezkuwi /data/pezkuwi
```

#### 3. Network Optimization
```bash
# Optimize TCP settings for blockchain
cat >> /etc/sysctl.conf <<EOF
# Network optimizations for PezkuwiChain
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 67108864
net.ipv4.tcp_wmem = 4096 65536 67108864
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_congestion_control = bbr
EOF

sysctl -p
```

#### 4. System Limits
```bash
# Increase file descriptors
cat >> /etc/security/limits.conf <<EOF
pezkuwi soft nofile 65536
pezkuwi hard nofile 65536
EOF
```

---

## Node Deployment

### Binary Deployment

#### Option 1: Pre-built Binary
```bash
# Download pre-built binary
wget https://github.com/pezkuwichain/pezkuwi-sdk/releases/download/v1.0.0/pezkuwi

# Verify checksum
sha256sum pezkuwi
# Compare with published checksum

# Install binary
chmod +x pezkuwi
mv pezkuwi /usr/local/bin/

# Verify installation
pezkuwi --version
```

#### Option 2: Build from Source
```bash
# Clone repository
su - pezkuwi
cd ~
git clone https://github.com/pezkuwichain/pezkuwi-sdk.git
cd pezkuwi-sdk/pezkuwi

# Checkout specific version
git checkout v1.0.0

# Build release binary
cargo build --release

# Install binary
sudo cp target/release/pezkuwi /usr/local/bin/

# Verify
pezkuwi --version
```

---

### Chain Specification

#### 1. Generate Chain Spec
```bash
# On a development machine, not production!
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi

# Generate base chain spec
./target/release/pezkuwi build-spec \
  --chain mainnet \
  --disable-default-bootnode \
  > pezkuwi-mainnet-spec.json
```

#### 2. Customize Chain Spec
```json
{
  "name": "PezkuwiChain Mainnet",
  "id": "pezkuwi",
  "chainType": "Live",
  "bootNodes": [
    "/ip4/BOOTNODE1_IP/tcp/30333/p2p/PEER_ID",
    "/ip4/BOOTNODE2_IP/tcp/30333/p2p/PEER_ID"
  ],
  "telemetryEndpoints": [
    ["wss://telemetry.pezkuwichain.com/submit", 0]
  ],
  "properties": {
    "tokenDecimals": 18,
    "tokenSymbol": "PEZ"
  },
  "genesis": {
    // Genesis configuration...
  }
}
```

**Critical Genesis Settings:**
- Initial validators and their session keys
- Initial balances (founders, treasury, presale)
- Sudo key (if applicable)
- Protocol parameters (block time, epoch duration, etc.)

#### 3. Convert to Raw Format
```bash
# Convert to raw format for deployment
./target/release/pezkuwi build-spec \
  --chain pezkuwi-mainnet-spec.json \
  --raw \
  --disable-default-bootnode \
  > pezkuwi-mainnet-raw.json

# Distribute this file to all validators
```

---

### Systemd Service Configuration

```bash
# Create systemd service file
sudo tee /etc/systemd/system/pezkuwi.service > /dev/null <<EOF
[Unit]
Description=PezkuwiChain Validator Node
After=network.target

[Service]
Type=simple
User=pezkuwi
Group=pezkuwi
WorkingDirectory=/home/pezkuwi
ExecStart=/usr/local/bin/pezkuwi \\
  --base-path /data/pezkuwi \\
  --chain /home/pezkuwi/pezkuwi-mainnet-raw.json \\
  --validator \\
  --name "ValidatorName" \\
  --port 30333 \\
  --rpc-port 9933 \\
  --prometheus-external \\
  --prometheus-port 9615 \\
  --pruning archive \\
  --db-cache 8192 \\
  --execution wasm \\
  --wasm-execution compiled \\
  --max-runtime-instances 8
Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd
sudo systemctl daemon-reload

# Enable service to start on boot
sudo systemctl enable pezkuwi

# Start the service
sudo systemctl start pezkuwi

# Check status
sudo systemctl status pezkuwi

# View logs
sudo journalctl -fu pezkuwi
```

---

## Validator Setup

### 1. Generate Session Keys

```bash
# After node is running, generate session keys
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9933/

# Response will be a hex string - SAVE THIS SECURELY!
# Example: "0x1234567890abcdef..."
```

### 2. Bond Tokens (Via Governance or Sudo)

Using Polkadot.js Apps:
1. Connect to your node
2. Go to Developer > Extrinsics
3. Select `staking.bond`
4. Bond sufficient tokens
5. Set controller account
6. Submit transaction

### 3. Set Session Keys

```javascript
// Using Polkadot.js
await api.tx.session.setKeys(
  sessionKeys,  // From author_rotateKeys
  proof
).signAndSend(validatorAccount);
```

### 4. Validate

```javascript
// Start validating
await api.tx.staking.validate({
  commission: 10000000  // 10% commission (in perbill: 10% = 10,000,000 / 100,000,000)
}).signAndSend(validatorAccount);
```

---

### Session Key Backup

```bash
# CRITICAL: Backup session keys securely!

# Encrypt keystore
tar -czf keystore-backup.tar.gz \
  /data/pezkuwi/chains/pezkuwi/keystore/

gpg --encrypt --recipient validator@pezkuwichain.com \
  keystore-backup.tar.gz

# Store encrypted backup in multiple locations:
# 1. Hardware Security Module (HSM)
# 2. Offline USB drive in safe
# 3. Encrypted cloud backup
# 4. Paper wallet (seed phrases)
```

---

## Network Launch

### Launch Sequence

#### Phase 1: Genesis Block (Day 0)
1. Distribute chain spec to all initial validators
2. All validators start nodes simultaneously
3. Monitor for block production start
4. Verify all validators online

```bash
# Monitor genesis
watch -n 5 'curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\"}" | jq ".result.number"'
```

#### Phase 2: Network Stabilization (Week 1)
- Monitor validator performance
- Verify finalization working
- Check peer connectivity
- Test governance proposals
- Monitor for any issues

#### Phase 3: Public Access (Week 2-4)
- Deploy public RPC nodes
- Enable web wallet connections
- Announce network officially
- Begin community onboarding

---

### Coordinated Genesis Launch

**Pre-launch Checklist (All Validators):**
- [ ] Node compiled and tested
- [ ] Chain spec file distributed
- [ ] Session keys generated and set
- [ ] Monitoring operational
- [ ] Communication channel active (Discord/Telegram)
- [ ] Backup systems tested

**Launch Command (Execute Simultaneously):**
```bash
# All validators run at agreed time (e.g., 2025-12-01 00:00:00 UTC)
systemctl start pezkuwi
```

**Post-Launch Monitoring:**
```bash
# Check block production
watch -n 1 'curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\"}" | jq ".result.number"'

# Check finalized blocks
watch -n 5 'curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getFinalizedHead\"}" | jq'

# Check validator set
curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "session_validators"}' | jq
```

---

## Post-Deployment

### Verification Steps

#### 1. Node Health Check
```bash
# System health
curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' | jq

# Expected: {"isSyncing":false,"peers":X,"shouldHavePeers":true}
```

#### 2. Block Production
```bash
# Monitor block authoring (if validator)
grep "Prepared block" /var/log/syslog | tail -20

# Check for missed blocks
grep "could not author" /var/log/syslog
```

#### 3. Finalization
```bash
# Check finalization is progressing
watch -n 10 'curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getFinalizedHead\"}" | jq'
```

#### 4. Monitoring Stack
```bash
# Verify Prometheus scraping
curl http://localhost:9615/metrics | grep substrate_block_height

# Check Grafana dashboards
open http://monitoring-server:3001
```

---

### Security Hardening

#### 1. Firewall Rules
```bash
# Lock down validator node
ufw deny incoming
ufw allow from SENTRY_NODE_IP to any port 30333
ufw allow from MONITORING_SERVER_IP to any port 9615
ufw allow 22/tcp  # SSH (consider restricting to specific IPs)
ufw enable
```

#### 2. SSH Hardening
```bash
# Disable password authentication
sed -i 's/PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
sed -i 's/#PubkeyAuthentication yes/PubkeyAuthentication yes/' /etc/ssh/sshd_config

# Disable root login
sed -i 's/PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config

# Restart SSH
systemctl restart sshd
```

#### 3. Automatic Security Updates
```bash
# Enable unattended upgrades
apt-get install unattended-upgrades
dpkg-reconfigure -plow unattended-upgrades
```

#### 4. Intrusion Detection
```bash
# Install fail2ban
apt-get install fail2ban

# Configure for SSH
cat > /etc/fail2ban/jail.local <<EOF
[sshd]
enabled = true
port = 22
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600
EOF

systemctl restart fail2ban
```

---

## Upgrades & Maintenance

### Runtime Upgrade Procedure

#### 1. Prepare New Runtime
```bash
# Build new runtime
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi
cargo build --release -p pezkuwichain-runtime

# Get WASM blob
WASM=target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.compact.compressed.wasm
```

#### 2. Submit Upgrade Proposal
```javascript
// Via Polkadot.js Apps or script
const wasm = fs.readFileSync('pezkuwichain_runtime.compact.compressed.wasm');

await api.tx.system.setCode(wasm)
  .signAndSend(sudoAccount);  // Or via governance
```

#### 3. Monitor Upgrade
```bash
# Watch for runtime upgrade event
journalctl -fu pezkuwi | grep "runtime"

# Verify new runtime version
curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_getRuntimeVersion"}' | jq
```

---

### Node Binary Upgrade

#### Zero-Downtime Upgrade (With Backup Node)
```bash
# 1. Start backup validator with same keys
# 2. Wait for backup to sync
# 3. Stop primary validator
# 4. Upgrade primary binary
# 5. Restart primary
# 6. Stop backup
```

#### Standard Upgrade (Brief Downtime)
```bash
# 1. Stop node
systemctl stop pezkuwi

# 2. Backup current binary
cp /usr/local/bin/pezkuwi /usr/local/bin/pezkuwi.backup

# 3. Install new binary
wget https://github.com/pezkuwichain/pezkuwi-sdk/releases/download/v1.1.0/pezkuwi
chmod +x pezkuwi
mv pezkuwi /usr/local/bin/

# 4. Restart node
systemctl start pezkuwi

# 5. Monitor startup
journalctl -fu pezkuwi
```

---

## Troubleshooting

### Common Issues

#### Node Won't Start
```bash
# Check logs
journalctl -u pezkuwi -n 100

# Common causes:
# - Corrupted database: rm -rf /data/pezkuwi/chains/*/db/
# - Wrong chain spec: verify pezkuwi-mainnet-raw.json
# - Port conflicts: netstat -tuln | grep 30333
```

#### Not Producing Blocks
```bash
# Check if validator is in active set
curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "session_validators"}' | jq

# Verify session keys are set correctly
# Check logs for errors
journalctl -fu pezkuwi | grep -i "error\|failed"
```

#### High Memory Usage
```bash
# Reduce cache size
# Edit /etc/systemd/system/pezkuwi.service
# Change --db-cache 8192 to --db-cache 4096

systemctl daemon-reload
systemctl restart pezkuwi
```

---

## Related Documentation

- **Monitoring Guide:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/MONITORING_GUIDE.md`
- **Operational Runbooks:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/OPERATIONAL_RUNBOOKS.md`
- **Disaster Recovery:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/DISASTER_RECOVERY.md`
- **Load Testing:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/LOAD_TESTING_GUIDE.md`

---

**Document Version:** 1.0
**Last Updated:** 2025-11-13
**Maintained By:** PezkuwiChain Operations Team

**CRITICAL:** Follow security best practices. Never expose validator nodes directly to internet!
