# PezkuwiChain Disaster Recovery Plan
**Version:** 1.0
**Date:** 2025-11-13
**Status:** Production Ready
**Review Frequency:** Quarterly

## Table of Contents
1. [Overview](#overview)
2. [Backup Strategy](#backup-strategy)
3. [Recovery Procedures](#recovery-procedures)
4. [Failure Scenarios](#failure-scenarios)
5. [Testing & Validation](#testing--validation)

---

## Overview

### Purpose
This document outlines procedures for recovering PezkuwiChain operations in the event of catastrophic failures, including hardware failure, data corruption, security breaches, and natural disasters.

### Recovery Objectives

| Metric | Target | Notes |
|--------|--------|-------|
| **RTO** (Recovery Time Objective) | 4 hours | Maximum acceptable downtime |
| **RPO** (Recovery Point Objective) | 15 minutes | Maximum acceptable data loss |
| **MTTR** (Mean Time To Recovery) | 2 hours | Average recovery time |

### Critical Components

1. **Chain State** - Most critical, irreplaceable
2. **Validator Keys** - Critical for block production
3. **Configuration Files** - Chain spec, node config
4. **Historical Data** - Logs, metrics (nice to have)

---

## Backup Strategy

### What to Backup

#### 1. Chain Database (CRITICAL)
**Priority:** Highest
**Frequency:** Every 6 hours
**Retention:** 7 days full + 30 days incremental

```bash
# Automated backup script
#!/bin/bash
BACKUP_DIR="/backup/pezkuwi/$(date +%Y%m%d-%H%M)"
mkdir -p $BACKUP_DIR

# Stop node for consistent backup
systemctl stop pezkuwi

# Backup database
tar -czf $BACKUP_DIR/chaindb.tar.gz \
  /data/pezkuwi/chains/*/db/

# Backup keystore
tar -czf $BACKUP_DIR/keystore.tar.gz \
  /data/pezkuwi/chains/*/keystore/

# Restart node
systemctl start pezkuwi

# Verify backup
tar -tzf $BACKUP_DIR/chaindb.tar.gz > /dev/null
echo "Backup completed: $BACKUP_DIR"
```

#### 2. Validator Keys (CRITICAL)
**Priority:** Highest
**Frequency:** On creation, then weekly
**Retention:** Permanent (encrypted, offline)

```bash
# Backup validator keys
gpg --encrypt --recipient ops@pezkuwichain.com \
  /data/pezkuwi/chains/*/keystore/* \
  > validator-keys-$(date +%Y%m%d).gpg

# Store in multiple locations:
# 1. Encrypted cloud storage (AWS S3, Google Cloud)
# 2. Offline USB drive in secure location
# 3. Paper backup (seed phrases) in safe
```

#### 3. Configuration Files
**Priority:** High
**Frequency:** On change
**Retention:** Version controlled (git)

```bash
# Files to backup
/home/mamostehp/Pezkuwi-SDK/pezkuwi/chainspec.json
/etc/systemd/system/pezkuwi.service
/home/mamostehp/Pezkuwi-SDK/pezkuwi/monitoring/*
/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/*
```

#### 4. Operational Data
**Priority:** Medium
**Frequency:** Daily
**Retention:** 30 days

```bash
# Logs
/var/log/pezkuwi/

# Metrics (Prometheus)
/var/lib/prometheus/data/

# Grafana dashboards
/var/lib/grafana/
```

### Backup Locations

#### Primary Backup
- **Location:** On-site NAS or attached storage
- **Purpose:** Quick recovery
- **Retention:** 7 days

#### Secondary Backup
- **Location:** Cloud storage (S3, Google Cloud)
- **Purpose:** Geographic redundancy
- **Retention:** 30 days
- **Encryption:** AES-256

#### Offline Backup
- **Location:** Offline storage in secure facility
- **Purpose:** Disaster recovery
- **Retention:** 90 days
- **Frequency:** Weekly

### Automated Backup Schedule

```cron
# Crontab entries for automated backups

# Chain database - every 6 hours
0 */6 * * * /opt/pezkuwi/scripts/backup-chaindb.sh

# Keystore - weekly (Sunday 2 AM)
0 2 * * 0 /opt/pezkuwi/scripts/backup-keys.sh

# Logs - daily (1 AM)
0 1 * * * /opt/pezkuwi/scripts/backup-logs.sh

# Sync to cloud - every 12 hours
0 */12 * * * /opt/pezkuwi/scripts/sync-to-cloud.sh
```

---

## Recovery Procedures

### Procedure 1: Full Node Recovery from Backup

**Scenario:** Complete server failure, need to restore from backup
**RTO:** 2-4 hours
**Prerequisites:** Backup available, new server provisioned

#### Steps:

**1. Prepare New Server** (30 minutes)
```bash
# Update system
apt-get update && apt-get upgrade -y

# Install dependencies
apt-get install -y build-essential git curl wget

# Create data directory
mkdir -p /data/pezkuwi
chown pezkuwi:pezkuwi /data/pezkuwi
```

**2. Install PezkuwiChain Binary** (15 minutes)
```bash
# Copy from backup or rebuild
cd /home/mamostehp/Pezkuwi-SDK
git clone https://github.com/pezkuwichain/pezkuwi-sdk.git
cd pezkuwi-sdk/pezkuwi
cargo build --release

# Verify binary
/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi --version
```

**3. Restore Chain Database** (60-120 minutes)
```bash
# Get latest backup
LATEST_BACKUP=$(ls -t /backup/pezkuwi/ | head -1)

# Extract database
tar -xzf /backup/pezkuwi/$LATEST_BACKUP/chaindb.tar.gz \
  -C /data/pezkuwi/

# Verify extraction
ls -lh /data/pezkuwi/chains/*/db/
```

**4. Restore Keystore** (5 minutes)
```bash
# Extract keystore
tar -xzf /backup/pezkuwi/$LATEST_BACKUP/keystore.tar.gz \
  -C /data/pezkuwi/

# Set permissions
chown -R pezkuwi:pezkuwi /data/pezkuwi/chains/*/keystore/
chmod 600 /data/pezkuwi/chains/*/keystore/*
```

**5. Restore Configuration** (10 minutes)
```bash
# Copy chainspec
cp /backup/pezkuwi/config/chainspec.json \
  /home/mamostehp/Pezkuwi-SDK/pezkuwi/

# Restore systemd service
cp /backup/pezkuwi/config/pezkuwi.service \
  /etc/systemd/system/

systemctl daemon-reload
```

**6. Start Node** (5 minutes)
```bash
# Start node
systemctl start pezkuwi

# Monitor startup
journalctl -u pezkuwi -f
```

**7. Verify Recovery** (15 minutes)
```bash
# Check node status
systemctl status pezkuwi

# Verify block height
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
  http://localhost:9933/ | jq '.result.number'

# Check peer connections
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9933/ | jq

# For validators: verify block production
# Should see authored blocks in logs
grep "Prepared block" /var/log/pezkuwi/node.log | tail -10
```

**Success Criteria:**
- ✅ Node process running
- ✅ Syncing or caught up to network
- ✅ Peer connections established
- ✅ For validators: producing blocks

---

### Procedure 2: Validator Key Recovery

**Scenario:** Lost or corrupted validator keys
**RTO:** 1-2 hours
**Prerequisites:** Encrypted key backup available

#### Steps:

**1. Stop Current Node** (2 minutes)
```bash
# Stop node to prevent key conflicts
systemctl stop pezkuwi
```

**2. Decrypt Backup** (5 minutes)
```bash
# Decrypt validator keys
gpg --decrypt validator-keys-YYYYMMDD.gpg > keys.tar.gz

# OR restore from seed phrase
# Use Subkey tool to regenerate keys
```

**3. Restore Keys** (10 minutes)
```bash
# Clear existing keystore
rm -rf /data/pezkuwi/chains/*/keystore/*

# Extract keys
tar -xzf keys.tar.gz -C /data/pezkuwi/chains/*/keystore/

# Set strict permissions
chmod 600 /data/pezkuwi/chains/*/keystore/*
chown pezkuwi:pezkuwi /data/pezkuwi/chains/*/keystore/*
```

**4. Verify Keys** (5 minutes)
```bash
# Check keys exist
ls -l /data/pezkuwi/chains/*/keystore/

# Verify key format (should be hex)
cat /data/pezkuwi/chains/*/keystore/* | head -1
```

**5. Restart Node** (5 minutes)
```bash
# Start node
systemctl start pezkuwi

# Monitor for key-related errors
journalctl -u pezkuwi -f | grep -i "key\|session"
```

**6. Verify Validator Status** (15 minutes)
```bash
# Check if validator is in active set
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "session_validators"}' \
  http://localhost:9933/ | jq

# Wait for next era and verify block production
# Should see "Prepared block" in logs
tail -f /var/log/pezkuwi/node.log | grep "Prepared block"
```

**Success Criteria:**
- ✅ Keys restored and readable
- ✅ Node started without key errors
- ✅ Validator producing blocks

---

### Procedure 3: Data Corruption Recovery

**Scenario:** Database corruption detected
**RTO:** 1-3 hours (depending on method)

#### Option A: Restore from Backup (1-2 hours)
```bash
# Use Procedure 1, steps 3-7
```

#### Option B: Resync from Genesis (24-72 hours)
```bash
# Only if backup not available or corrupted

# 1. Stop node
systemctl stop pezkuwi

# 2. Delete corrupted database
rm -rf /data/pezkuwi/chains/*/db/

# 3. Keep keystore intact
ls /data/pezkuwi/chains/*/keystore/  # Verify keys safe

# 4. Restart node (will resync from genesis)
systemctl start pezkuwi

# 5. Monitor sync progress
tail -f /var/log/pezkuwi/node.log | grep "Syncing"
```

#### Option C: Fast Sync from Snapshot (4-8 hours)
```bash
# Use trusted snapshot from another node

# 1. Stop node
systemctl stop pezkuwi

# 2. Download snapshot (if available)
wget https://snapshots.pezkuwichain.com/latest.tar.gz

# 3. Extract
tar -xzf latest.tar.gz -C /data/pezkuwi/

# 4. Verify integrity
# Check block hash matches known good hash

# 5. Restart
systemctl start pezkuwi
```

---

## Failure Scenarios

### Scenario 1: Single Validator Node Failure

**Impact:** Medium
**RTO:** 30 minutes

**Response:**
1. Failover to backup validator node (hot standby)
2. Recover primary node using Procedure 1
3. Sync primary, then decommission backup

**Prevention:**
- Maintain hot standby validator
- Use session key rotation
- Monitor validator health

---

### Scenario 2: Network Partition

**Impact:** High
**RTO:** 2 hours

**Response:**
1. Identify partition (check peer connectivity)
2. Reconnect isolated nodes
3. Verify finalization resumes
4. Monitor for equivocation

**Prevention:**
- Geographic distribution of validators
- Multiple network paths
- Peer diversity

---

### Scenario 3: Security Breach

**Impact:** Critical
**RTO:** 4-8 hours

**Response:**
1. **Immediate** (5 minutes):
   - Isolate compromised systems
   - Revoke compromised keys
   - Alert all validators

2. **Assessment** (30 minutes):
   - Identify attack vector
   - Determine data exposure
   - Check for backdoors

3. **Recovery** (2-4 hours):
   - Rebuild from clean backups
   - Rotate all keys
   - Update security measures

4. **Post-Incident** (ongoing):
   - Forensic analysis
   - Security audit
   - Update procedures

**Prevention:**
- Regular security audits
- Key rotation schedule
- Multi-signature governance
- Intrusion detection systems

---

### Scenario 4: Natural Disaster / Data Center Loss

**Impact:** Critical
**RTO:** 8-24 hours

**Response:**
1. Activate disaster recovery site
2. Restore from offsite backups
3. Reconfigure network topology
4. Resume operations with remaining validators

**Prevention:**
- Geographic redundancy
- Offsite backups
- Multi-datacenter architecture
- Documented failover procedures

---

## Testing & Validation

### Recovery Testing Schedule

| Test Type | Frequency | Duration | Pass Criteria |
|-----------|-----------|----------|---------------|
| Backup verification | Weekly | 30 min | Backup extractable & complete |
| Key restoration | Monthly | 1 hour | Keys restored & functional |
| Full node recovery | Quarterly | 4 hours | Node fully operational |
| Disaster simulation | Annually | 1 day | All systems recovered |

### Test Procedure: Quarterly Recovery Drill

**Objective:** Validate full recovery capability
**Duration:** 4 hours
**Participants:** Operations team, on-call engineer

#### Test Steps:

1. **Preparation** (30 minutes)
   - Select test node (non-critical)
   - Notify team of drill
   - Prepare test environment

2. **Simulated Failure** (5 minutes)
   - Stop test node
   - Simulate data loss (move database)

3. **Execute Recovery** (2-3 hours)
   - Follow Procedure 1
   - Document all steps
   - Note any issues

4. **Validation** (30 minutes)
   - Verify node operational
   - Check all functionality
   - Compare metrics to baseline

5. **Review** (30 minutes)
   - Team debrief
   - Update procedures
   - Document lessons learned

**Success Criteria:**
- ✅ Recovery completed within RTO
- ✅ Data loss within RPO
- ✅ All systems functional
- ✅ No critical issues

---

## Backup Scripts

### Script: Automated Chaindb Backup

```bash
#!/bin/bash
# /opt/pezkuwi/scripts/backup-chaindb.sh

set -e

BACKUP_ROOT="/backup/pezkuwi"
TIMESTAMP=$(date +%Y%m%d-%H%M)
BACKUP_DIR="$BACKUP_ROOT/$TIMESTAMP"
DATA_DIR="/data/pezkuwi"
LOG_FILE="/var/log/pezkuwi/backup.log"

# Create backup directory
mkdir -p $BACKUP_DIR

echo "[$(date)] Starting backup..." | tee -a $LOG_FILE

# Stop node for consistent backup
echo "[$(date)] Stopping node..." | tee -a $LOG_FILE
systemctl stop pezkuwi

# Backup database
echo "[$(date)] Backing up database..." | tee -a $LOG_FILE
tar -czf $BACKUP_DIR/chaindb.tar.gz \
  -C $DATA_DIR chains/*/db/ 2>&1 | tee -a $LOG_FILE

# Backup keystore
echo "[$(date)] Backing up keystore..." | tee -a $LOG_FILE
tar -czf $BACKUP_DIR/keystore.tar.gz \
  -C $DATA_DIR chains/*/keystore/ 2>&1 | tee -a $LOG_FILE

# Restart node
echo "[$(date)] Restarting node..." | tee -a $LOG_FILE
systemctl start pezkuwi

# Verify backups
echo "[$(date)] Verifying backups..." | tee -a $LOG_FILE
tar -tzf $BACKUP_DIR/chaindb.tar.gz > /dev/null
tar -tzf $BACKUP_DIR/keystore.tar.gz > /dev/null

# Calculate sizes
DB_SIZE=$(du -sh $BACKUP_DIR/chaindb.tar.gz | cut -f1)
KEY_SIZE=$(du -sh $BACKUP_DIR/keystore.tar.gz | cut -f1)

echo "[$(date)] Backup completed successfully" | tee -a $LOG_FILE
echo "Database: $DB_SIZE, Keystore: $KEY_SIZE" | tee -a $LOG_FILE

# Cleanup old backups (keep last 7 days)
find $BACKUP_ROOT -type d -mtime +7 -exec rm -rf {} + 2>/dev/null || true

echo "[$(date)] Old backups cleaned up" | tee -a $LOG_FILE

# Send notification (optional)
# curl -X POST https://hooks.slack.com/services/YOUR/WEBHOOK/URL \
#   -d '{"text":"Pezkuwi backup completed: '"$TIMESTAMP"'"}'
```

---

## Emergency Contacts

### Incident Response Team

| Role | Primary Contact | Secondary Contact | Escalation |
|------|----------------|-------------------|------------|
| Incident Commander | [Configure] | [Configure] | CTO |
| On-Call Engineer | [Configure] | [Configure] | Lead Engineer |
| Security Lead | [Configure] | [Configure] | CISO |
| Infrastructure | [Configure] | [Configure] | VP Engineering |

### External Contacts

| Service | Contact | Purpose |
|---------|---------|---------|
| Hosting Provider | [Configure] | Infrastructure issues |
| Security Vendor | [Configure] | Security incidents |
| Legal | [Configure] | Breach notification |
| Community Manager | [Configure] | Public communication |

---

## Post-Incident Procedures

### Incident Report Template

```markdown
# Incident Report: [Title]

**Date:** YYYY-MM-DD
**Severity:** Critical / High / Medium / Low
**Duration:** HH:MM
**Status:** Resolved / Ongoing

## Summary
[Brief description of incident]

## Timeline
- HH:MM - Incident detected
- HH:MM - Response initiated
- HH:MM - Root cause identified
- HH:MM - Fix applied
- HH:MM - Service restored

## Impact
- Affected systems: [List]
- Downtime: [Duration]
- Data loss: [Amount]
- Users affected: [Number]

## Root Cause
[Detailed analysis]

## Resolution
[Steps taken to resolve]

## Prevention
- [ ] Action item 1
- [ ] Action item 2
- [ ] Update documentation

## Lessons Learned
[Key takeaways]
```

---

## Related Documentation

- **Monitoring Guide:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/MONITORING_GUIDE.md`
- **Operational Runbooks:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/OPERATIONAL_RUNBOOKS.md`
- **Security Audit:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/SECURITY_AUDIT_REPORT.md`
- **Security Fixes:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/SECURITY_FIXES_COMPLETED.md`

---

**Document Version:** 1.0
**Last Updated:** 2025-11-13
**Next Review:** 2025-12-13
**Maintained By:** PezkuwiChain Operations Team

**CRITICAL:** Test recovery procedures quarterly. Update as infrastructure changes.
