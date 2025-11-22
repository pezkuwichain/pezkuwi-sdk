# PezkuwiChain Monitoring & Observability Guide
**Version:** 1.0
**Date:** 2025-11-13
**Status:** Production Ready

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Monitoring Stack](#monitoring-stack)
4. [Metrics Collection](#metrics-collection)
5. [Alerting Rules](#alerting-rules)
6. [Log Aggregation](#log-aggregation)
7. [Dashboards](#dashboards)
8. [Alert Response](#alert-response)
9. [Troubleshooting](#troubleshooting)

---

## Overview

This guide covers the complete monitoring and observability setup for PezkuwiChain, including metrics collection, log aggregation, alerting, and operational dashboards.

### Monitoring Goals
- **Availability:** Ensure 99.9% uptime
- **Performance:** Track block production and finalization
- **Security:** Monitor suspicious activities and potential attacks
- **Operations:** Provide actionable insights for maintenance

### Key Metrics
- Block height and finalization rate
- Peer connections and network health
- Memory and CPU usage
- Transaction throughput
- Consensus participation
- Storage growth rate

---

## Architecture

### Component Overview
```
┌─────────────────────────────────────────────────────────────────┐
│                        PezkuwiChain Node                         │
│                                                                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  Substrate      │  │  Pallets        │  │  Runtime        │ │
│  │  Framework      │  │  (Business      │  │  (State         │ │
│  │                 │  │   Logic)        │  │   Transition)   │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
│                              │                                   │
│                         Metrics Port 9615                        │
└──────────────────────────────┼───────────────────────────────────┘
                               │
              ┌────────────────┴────────────────┐
              │                                  │
       ┌──────▼──────┐                  ┌───────▼────────┐
       │  Prometheus │                  │   Promtail     │
       │   (Metrics) │                  │  (Log Shipper) │
       └──────┬──────┘                  └───────┬────────┘
              │                                  │
              │                         ┌────────▼────────┐
              │                         │      Loki       │
              │                         │ (Log Aggregator)│
              │                         └────────┬────────┘
              │                                  │
       ┌──────▼──────────────────────────────────▼──────┐
       │                 Grafana                         │
       │         (Visualization & Dashboards)            │
       └──────────────────────┬──────────────────────────┘
                              │
                   ┌──────────▼──────────┐
                   │   Alertmanager      │
                   │  (Alert Routing)    │
                   └─────────────────────┘
```

---

## Monitoring Stack

### Installed Components

#### 1. Prometheus (Metrics Database)
- **Port:** 3000 (HTTP), 9090 (API)
- **Location:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/monitoring/prometheus.yml`
- **Purpose:** Scrapes and stores time-series metrics
- **Retention:** 30 days

#### 2. Loki (Log Aggregation)
- **Port:** 3100
- **Location:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/monitoring/loki-config.yml`
- **Purpose:** Aggregates and indexes logs
- **Retention:** 30 days

#### 3. Promtail (Log Shipper)
- **Port:** 9080
- **Location:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/monitoring/promtail-config.yml`
- **Purpose:** Ships logs from node to Loki

#### 4. Grafana (Visualization)
- **Port:** 3001
- **Location:** Docker Compose
- **Purpose:** Dashboards and alerting UI

#### 5. Alertmanager (Alert Routing)
- **Port:** 9093
- **Location:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/monitoring/alertmanager.yml`
- **Purpose:** Routes alerts to appropriate channels

---

## Metrics Collection

### Available Metrics

#### Blockchain Metrics
```prometheus
# Block height - current chain head
substrate_block_height{status="best"}

# Finalized block height
substrate_block_height{status="finalized"}

# Block production time
substrate_proposer_block_constructed_seconds

# Block import time
substrate_block_verification_and_import_time_bucket
```

#### Network Metrics
```prometheus
# Connected peers
substrate_sub_libp2p_peers_count

# Peer connection states
substrate_sub_libp2p_connections_opened_total
substrate_sub_libp2p_connections_closed_total

# Network bandwidth
substrate_sub_libp2p_notifications_sizes_sum
```

#### System Metrics
```prometheus
# CPU usage
process_cpu_seconds_total

# Memory usage
substrate_memory_usage_bytes

# Disk I/O
substrate_database_cache_bytes
```

#### Transaction Pool Metrics
```prometheus
# Pending transactions
substrate_sub_txpool_validations_scheduled
substrate_sub_txpool_validations_finished

# Transaction throughput
substrate_proposer_number_of_transactions
```

---

## Alerting Rules

### Critical Alerts (Immediate Action Required)

#### 1. Node Down
```yaml
alert: NodeDown
expr: up{job="pezkuwi-node"} == 0
for: 1m
severity: critical
description: "PezkuwiChain node is down for more than 1 minute"
action: "Check node logs, restart if necessary"
```

#### 2. Finalization Stalled
```yaml
alert: FinalizationStalled
expr: increase(substrate_block_height{status="finalized"}[5m]) == 0
for: 5m
severity: critical
description: "Block finalization has stopped"
action: "Check consensus, validator connectivity"
```

#### 3. High Memory Usage
```yaml
alert: HighMemoryUsage
expr: (substrate_memory_usage_bytes / 1e9) > 15
for: 5m
severity: critical
description: "Node memory usage above 15GB"
action: "Investigate memory leak, consider restart"
```

### Warning Alerts (Monitor Closely)

#### 4. Low Peer Count
```yaml
alert: LowPeerCount
expr: substrate_sub_libp2p_peers_count < 5
for: 10m
severity: warning
description: "Node has fewer than 5 connected peers"
action: "Check network connectivity, firewall rules"
```

#### 5. Slow Block Production
```yaml
alert: SlowBlockProduction
expr: rate(substrate_block_height{status="best"}[5m]) < 0.15
for: 10m
severity: warning
description: "Block production rate below expected"
action: "Check validator performance, network issues"
```

#### 6. High CPU Usage
```yaml
alert: HighCPUUsage
expr: rate(process_cpu_seconds_total[5m]) * 100 > 80
for: 15m
severity: warning
description: "CPU usage above 80% for 15 minutes"
action: "Investigate resource-intensive operations"
```

### Info Alerts (Informational)

#### 7. Storage Growing Fast
```yaml
alert: StorageGrowingFast
expr: rate(substrate_state_db_cache_bytes[1h]) > 1e8
for: 1h
severity: info
description: "Database growing faster than expected"
action: "Monitor disk space, plan for scaling"
```

---

## Log Aggregation

### Log Levels and Categories

#### Error Logs (Immediate Investigation)
```
ERROR - Runtime errors, panics, crashes
```
**Alert:** Yes
**Action:** Immediate investigation required

#### Warn Logs (Monitor)
```
WARN - Non-fatal issues, degraded performance
```
**Alert:** No (unless pattern detected)
**Action:** Review during maintenance

#### Info Logs (Operational)
```
INFO - Normal operations, block production
```
**Alert:** No
**Action:** Historical analysis

#### Debug Logs (Development)
```
DEBUG - Detailed execution traces
```
**Alert:** No
**Action:** Troubleshooting only

### Log Queries (Loki)

#### Find Error Messages
```logql
{job="pezkuwi-node"} |= "ERROR"
```

#### Track Specific Component
```logql
{job="pezkuwi-node"} |~ "\\[consensus\\]"
```

#### Performance Issues
```logql
{job="pezkuwi-node"} |= "slow" or "timeout"
```

#### Security Events
```logql
{job="pezkuwi-node"} |= "invalid" or "rejected" or "failed"
```

---

## Dashboards

### Main Dashboard Panels

#### 1. Overview Panel
- Current block height
- Finalized block height
- Finalization lag
- Connected peers
- Node uptime

#### 2. Performance Panel
- Blocks per minute
- Block import time
- Transaction throughput
- Memory usage
- CPU usage

#### 3. Network Panel
- Peer count over time
- Connection events
- Bandwidth usage
- Geographic distribution (if available)

#### 4. Consensus Panel
- Validator participation
- Block production rate
- Missed blocks
- Equivocation events

#### 5. System Resources Panel
- Memory usage graph
- CPU utilization
- Disk I/O
- Network I/O

### Dashboard Access
```
URL: http://monitoring-server:3001
Default Login: admin / admin (CHANGE IMMEDIATELY)
```

---

## Alert Response

### Critical Alert Response Matrix

| Alert | Response Time | Actions | Escalation |
|-------|--------------|---------|------------|
| Node Down | < 2 minutes | 1. Check logs<br>2. Restart node<br>3. Verify recovery | If > 10 min down |
| Finalization Stalled | < 5 minutes | 1. Check consensus<br>2. Verify validator set<br>3. Check peer connectivity | If > 15 min |
| High Memory Usage | < 10 minutes | 1. Identify memory leak<br>2. Consider restart<br>3. Review recent changes | If recurring |
| Low Peer Count | < 15 minutes | 1. Check firewall<br>2. Verify network config<br>3. Check bootnodes | If persistent |

### Alert Notification Channels

#### Configured Channels
1. **Email:** Critical alerts only
2. **Slack/Discord:** All severity levels
3. **PagerDuty:** Critical alerts (if configured)
4. **Webhook:** Custom integrations

#### Configuration
Edit `/home/mamostehp/Pezkuwi-SDK/pezkuwi/monitoring/alertmanager.yml`:
```yaml
receivers:
  - name: 'team-ops'
    email_configs:
      - to: 'ops@pezkuwichain.com'
    slack_configs:
      - api_url: 'YOUR_SLACK_WEBHOOK'
        channel: '#pezkuwi-alerts'
```

---

## Troubleshooting

### Common Issues

#### 1. Prometheus Not Scraping Metrics

**Symptoms:**
- No data in Grafana
- Prometheus targets show "down"

**Solution:**
```bash
# Check node is exposing metrics
curl http://localhost:9615/metrics

# Verify Prometheus config
docker logs prometheus

# Restart Prometheus
docker-compose restart prometheus
```

#### 2. Loki Not Receiving Logs

**Symptoms:**
- No logs in Grafana
- Promtail errors in logs

**Solution:**
```bash
# Check Promtail status
docker logs promtail

# Verify log paths exist
ls /pezkuwi/logs/

# Test Loki endpoint
curl http://localhost:3100/ready
```

#### 3. Alertmanager Not Sending Alerts

**Symptoms:**
- Alerts firing in Prometheus but not received
- Alertmanager shows errors

**Solution:**
```bash
# Check Alertmanager status
docker logs alertmanager

# Verify configuration
curl http://localhost:9093/api/v1/status

# Test alert routing
amtool check-config /etc/alertmanager/config.yml
```

#### 4. High Resource Usage by Monitoring

**Symptoms:**
- Prometheus using too much disk
- Grafana slow to load

**Solution:**
```yaml
# Reduce Prometheus retention (prometheus.yml)
storage:
  tsdb:
    retention.time: 15d  # Reduce from 30d

# Reduce scrape frequency
scrape_interval: 30s  # Increase from 15s
```

### Debug Commands

```bash
# Check all monitoring containers
docker-compose -f monitoring/docker-compose.yml ps

# View Prometheus targets
curl http://localhost:9090/api/v1/targets

# View Alertmanager alerts
curl http://localhost:9093/api/v1/alerts

# Export Prometheus data
curl 'http://localhost:9090/api/v1/query?query=substrate_block_height'

# View Loki labels
curl -G http://localhost:3100/loki/api/v1/labels
```

---

## Starting the Monitoring Stack

### Quick Start

```bash
# Navigate to monitoring directory
cd /home/mamostehp/Pezkuwi-SDK/pezkuwi/monitoring

# Start all services
docker-compose up -d

# Verify all containers are running
docker-compose ps

# Check logs
docker-compose logs -f

# Access Grafana
# Open browser: http://localhost:3001
# Login: admin / admin (change immediately)
```

### Service Endpoints

| Service | URL | Purpose |
|---------|-----|---------|
| Grafana | http://localhost:3001 | Dashboards |
| Prometheus | http://localhost:9090 | Metrics UI |
| Loki | http://localhost:3100 | Logs API |
| Alertmanager | http://localhost:9093 | Alerts UI |
| Node Metrics | http://localhost:9615/metrics | Raw metrics |

---

## Maintenance

### Regular Tasks

#### Daily
- ✅ Review critical alerts
- ✅ Check node uptime
- ✅ Verify finalization

#### Weekly
- ✅ Review warning alerts
- ✅ Check storage growth
- ✅ Analyze performance trends
- ✅ Review error logs

#### Monthly
- ✅ Rotate logs
- ✅ Update dashboards
- ✅ Review alert thresholds
- ✅ Backup Prometheus data
- ✅ Audit monitoring configuration

### Backup and Recovery

```bash
# Backup Prometheus data
tar -czf prometheus-backup-$(date +%Y%m%d).tar.gz \
  /var/lib/prometheus/data

# Backup Grafana dashboards
curl -H "Authorization: Bearer $API_KEY" \
  http://localhost:3001/api/dashboards/db > dashboards-backup.json

# Restore Prometheus
tar -xzf prometheus-backup-YYYYMMDD.tar.gz \
  -C /var/lib/prometheus/data
```

---

## Best Practices

### 1. Alert Fatigue Prevention
- Set appropriate thresholds
- Use `for` clauses to avoid flapping
- Group related alerts
- Regular threshold review

### 2. Dashboard Organization
- One dashboard per use case
- Clear panel titles
- Consistent color schemes
- Include alert annotations

### 3. Log Management
- Enable structured logging
- Use consistent log levels
- Implement log rotation
- Regular log analysis

### 4. Security
- Change default credentials
- Use TLS for external access
- Implement authentication
- Regular security audits
- Limit metrics exposure

---

## Related Documentation

- **Alerting Rules:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/monitoring/alert-rules.yml`
- **Operational Runbooks:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/OPERATIONAL_RUNBOOKS.md`
- **Disaster Recovery:** `/home/mamostehp/Pezkuwi-SDK/pezkuwi/docs/DISASTER_RECOVERY.md`
- **Deployment Guide:** (To be created in FAZ 4)

---

## Support and Escalation

### On-Call Procedures
1. Acknowledge alert within 5 minutes
2. Begin investigation within 10 minutes
3. Provide status update within 30 minutes
4. Escalate if unresolved after 1 hour

### Contact Information
- **Primary On-Call:** [Configure in Alertmanager]
- **Secondary On-Call:** [Configure in Alertmanager]
- **Engineering Lead:** [Configure in Alertmanager]
- **Emergency:** [Configure in Alertmanager]

---

**Document Version:** 1.0
**Last Updated:** 2025-11-13
**Maintained By:** PezkuwiChain Operations Team
