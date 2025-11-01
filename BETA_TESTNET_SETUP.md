# PezkuwiChain Beta Testnet Setup Guide

Complete guide for launching and managing the PezkuwiChain Beta Testnet with 8 validators.

## 🎯 Overview

The beta testnet consists of:
- **8 validators** total (configured in `runtime/validators/beta_testnet_validators.json`)
- **4 validators** on Computer 1 (this machine)
- **4 validators** on Computer 2 (remote machine)
- **DEX pools** automatically initialized after network stabilization

## 📋 Prerequisites

1. **Built binary**: `cargo build --release` (already done)
2. **Validator seeds**: Actual seed phrases from your cold wallet (replace placeholders in scripts)
3. **Network access**: Both computers must be able to connect via P2P ports
4. **Python environment**: For DEX pool initialization (optional but recommended)

## 🚀 Quick Start

### Step 1: Start Validators (Computer 1)

```bash
cd /home/mamostehp/Pezkuwi-SDK

# Start all 4 local validators
./scripts/start-all-beta-validators.sh
```

This will start:
- Validator 1 (Bootnode): `ws://127.0.0.1:9944`
- Validator 2: `ws://127.0.0.1:9945`
- Validator 3: `ws://127.0.0.1:9946`
- Validator 4: `ws://127.0.0.1:9947`

### Step 2: Insert Validator Keys

**⚠️ IMPORTANT**: Before running this, edit the script and replace placeholder seeds with your actual seeds!

```bash
# Edit the script first
nano ./scripts/insert-all-beta-keys.sh

# Replace all "your-validator-X-*-seed" placeholders with actual seeds
# from: runtime/validators/beta_testnet_validators.json

# Then run it
./scripts/insert-all-beta-keys.sh
```

### Step 3: Start Validators on Computer 2

On the second computer, use similar scripts with:
- Validators 5-8
- Ports: 30337-30340 (P2P), 9948-9951 (RPC)
- Bootnode address: `/ip4/<COMPUTER_1_IP>/tcp/30333/p2p/<PEER_ID>`

Get COMPUTER_1's peer ID:
```bash
tail -100 /tmp/beta-validator-1.log | grep "Local node identity"
```

### Step 4: Wait for Network Consensus

The network needs at least 6 out of 8 validators to produce blocks. Monitor:

```bash
# Watch validator 1 logs
tail -f /tmp/beta-validator-1.log | grep -E "Prepared block|finalized"
```

Expected output:
```
💤 Idle (X peers), best: #N (0x...)
✨ Imported #N (0x...)
🎁 Prepared block for proposing at N
```

### Step 5: Initialize DEX Pools

Once the network is producing blocks (wait ~2 minutes):

```bash
./scripts/init/init_beta_dex_pool.sh
```

This creates:
- HEZ/PEZ liquidity pool
- Initial liquidity provision
- Testing tokens for transactions

## 📁 Directory Structure

```
/home/mamostehp/Pezkuwi-SDK/
├── scripts/
│   ├── start-beta-validator-1.sh         # Validator 1 (bootnode)
│   ├── start-beta-validator-2.sh         # Validator 2
│   ├── start-beta-validator-3.sh         # Validator 3
│   ├── start-beta-validator-4.sh         # Validator 4
│   ├── start-all-beta-validators.sh      # Master startup script
│   ├── stop-beta-validators.sh           # Stop all validators
│   ├── insert-keys-validator-1.sh        # Insert keys for V1
│   ├── insert-all-beta-keys.sh           # Bulk key insertion
│   └── init/
│       └── init_beta_dex_pool.sh         # DEX initialization
├── runtime/validators/
│   └── beta_testnet_validators.json      # Validator configurations
└── /tmp/
    ├── beta-validator-1.log              # Validator 1 logs
    ├── beta-validator-2.log              # Validator 2 logs
    ├── beta-validator-3.log              # Validator 3 logs
    └── beta-validator-4.log              # Validator 4 logs
```

## 🔧 Management Commands

### Start/Stop Validators

```bash
# Start all
./scripts/start-all-beta-validators.sh

# Stop all
./scripts/stop-beta-validators.sh

# Restart a specific validator
kill $(cat /tmp/beta-validator-1.pid)
./scripts/start-beta-validator-1.sh
```

### Monitor Logs

```bash
# Follow all validators in separate terminals
tail -f /tmp/beta-validator-1.log
tail -f /tmp/beta-validator-2.log
tail -f /tmp/beta-validator-3.log
tail -f /tmp/beta-validator-4.log

# Check for errors
grep -i error /tmp/beta-validator-*.log

# Watch block production
tail -f /tmp/beta-validator-1.log | grep "Prepared block"
```

### Check Validator Status

```bash
# Check if validators are running
ps aux | grep "pezkuwi.*beta"

# Check network connectivity
curl -H "Content-Type: application/json" \
     -d '{"id":1,"jsonrpc":"2.0","method":"system_peers","params":[]}' \
     http://localhost:9944
```

## 🌐 Network Configuration

### Validator Ports

| Validator | P2P Port | RPC Port | WebSocket | Computer |
|-----------|----------|----------|-----------|----------|
| Validator 1 | 30333 | 9944 | ws://127.0.0.1:9944 | Computer 1 |
| Validator 2 | 30334 | 9945 | ws://127.0.0.1:9945 | Computer 1 |
| Validator 3 | 30335 | 9946 | ws://127.0.0.1:9946 | Computer 1 |
| Validator 4 | 30336 | 9947 | ws://127.0.0.1:9947 | Computer 1 |
| Validator 5 | 30337 | 9948 | ws://127.0.0.1:9948 | Computer 2 |
| Validator 6 | 30338 | 9949 | ws://127.0.0.1:9949 | Computer 2 |
| Validator 7 | 30339 | 9950 | ws://127.0.0.1:9950 | Computer 2 |
| Validator 8 | 30340 | 9951 | ws://127.0.0.1:9951 | Computer 2 |

### Firewall Rules (if needed)

```bash
# Allow P2P connections
sudo ufw allow 30333:30340/tcp

# Allow RPC connections (only from trusted IPs)
sudo ufw allow from <TRUSTED_IP> to any port 9944:9951
```

## 🔐 Security Notes

1. **Seed Phrases**: Keep validator seeds in cold storage, never commit to git
2. **RPC Exposure**: Use `--rpc-methods=Unsafe` only for development
3. **Telemetry**: All validators report to `wss://telemetry.pezkuwichain.io`
4. **Network Keys**: Generated automatically with `--unsafe-force-node-key-generation`

## 🐛 Troubleshooting

### Validators Not Producing Blocks

**Symptom**: Network stuck at block #0 or very slow progress

**Solutions**:
1. Verify at least 6 validators are running
2. Check keys are inserted: Look for "Starting consensus session" in logs
3. Ensure validators can communicate (check `system_peers` RPC)
4. Restart validators after key insertion

### Connection Issues Between Computers

**Symptom**: Computer 2 validators show "0 peers"

**Solutions**:
1. Verify bootnode peer ID is correct
2. Check firewall allows P2P port 30333
3. Use public IP address, not 127.0.0.1 in bootnode address
4. Verify network connectivity: `telnet <COMPUTER_1_IP> 30333`

### DEX Pool Initialization Fails

**Symptom**: `init_beta_dex_pool.sh` times out or fails

**Solutions**:
1. Wait for blocks to be produced first (check logs)
2. Verify RPC is accessible: `curl http://localhost:9944`
3. Check Python dependencies: `pip install substrateinterface`
4. Run manually with verbose output: `python3 scripts/init/init_pools.py --rpc-url http://localhost:9944`

## 🔗 Connect DKSweb

Once the beta testnet is running and stable:

1. DKSweb is already configured to use `wss://beta.pezkuwichain.io`
2. Set up DNS/reverse proxy to point `beta.pezkuwichain.io` to Validator 1's IP
3. Or temporarily edit DKSweb's PolkadotContext.tsx to use `ws://localhost:9944`

## 📊 Monitoring & Telemetry

- **Telemetry**: https://telemetry.pezkuwichain.io
- **Local monitoring**: Prometheus metrics on ports 9615-9618
- **Grafana**: Can be configured to visualize validator metrics

## ✅ Success Checklist

- [ ] All 8 validators running
- [ ] Keys inserted for all validators
- [ ] Network producing blocks (check logs)
- [ ] Block finalization occurring (2/3+ validators)
- [ ] DEX pools initialized
- [ ] DKSweb connects successfully
- [ ] Token transfers working
- [ ] Swaps executing correctly

## 📝 Notes

- This is a **beta testnet** - expect resets and upgrades
- Validator 1 acts as the bootnode
- Minimum 6 validators needed for Byzantine Fault Tolerance (BFT)
- Keep validator logs for debugging
- Regular backups not needed (temporary storage in `/tmp`)

---

For issues or questions, check validator logs first, then review this guide.
