# 📘 PezkuwiChain Chainspec Creation Guide

**Version:** 1.0
**Date:** 2025-11-15
**Author:** Production Readiness Team

---

## 🎯 Overview

This guide provides complete instructions for creating production-ready chainspecs (chain specifications) for PezkuwiChain networks that will properly finalize blocks.

**Critical Requirements:**
- ✅ WASM runtime code must be included (2-5MB)
- ✅ Genesis authorities properly configured
- ✅ Session keys correctly embedded
- ✅ Validator JSON files prepared
- ✅ Preset names matching runtime definitions

---

## 📋 Table of Contents

1. [Prerequisites](#prerequisites)
2. [Understanding Chainspecs](#understanding-chainspecs)
3. [Network Configurations](#network-configurations)
4. [Step-by-Step Build Process](#step-by-step-build-process)
5. [Verification Procedures](#verification-procedures)
6. [Common Issues & Troubleshooting](#common-issues--troubleshooting)
7. [Network Deployment](#network-deployment)

---

## 📦 Prerequisites

### Required Software

```bash
# Rust toolchain
rustc --version  # Should be 1.70.0 or later
cargo --version

# Build dependencies (Ubuntu/Debian)
sudo apt update
sudo apt install -y git clang curl libssl-dev llvm libudev-dev protobuf-compiler

# Verification tools
jq --version     # JSON processor
xxd --version    # Hex dump utility
```

### Repository Setup

```bash
# Clone repository
git clone https://github.com/your-org/Pezkuwi-SDK.git
cd Pezkuwi-SDK/pezkuwi

# Ensure clean state
git status
cargo clean
```

---

## 🔍 Understanding Chainspecs

### What is a Chainspec?

A chainspec (chain specification) is a JSON file that defines:
- **Genesis State**: Initial blockchain state (balances, authorities, config)
- **Runtime Code**: WASM binary of the blockchain runtime
- **Network Metadata**: Name, ID, protocol, token symbols
- **Bootnodes**: Initial peer connection points

### Chainspec Types

| Type | Description | Use Case |
|------|-------------|----------|
| **Plain** | Human-readable JSON | Development, debugging |
| **Raw** | Hex-encoded with WASM | Production deployment |

**CRITICAL:** Only raw chainspecs include the runtime WASM code needed for block production!

### Genesis Config Presets

Presets are defined in `runtime/pezkuwichain/src/genesis_config_presets.rs`:

| Preset Name | Validators | Network Type | Validator Keys |
|-------------|------------|--------------|----------------|
| `dev` | 1 (Alice) | Development | Test seeds |
| `local_testnet` | 2 (Alice+Bob) | Local testing | Test seeds |
| `alfa_testnet` | 4 (test seeds) | Early testing | Test seeds |
| `beta_testnet` | 8 validators | Live testnet | Real keys (JSON) |
| `staging` | 20 validators | Staging | Real keys (JSON) |
| `production` | 100 validators | Mainnet | Real keys (JSON) |

---

## 🌐 Network Configurations

### 1. Local Development (dev)

```bash
# File: chainspecs/local.json (plain)
# File: chainspecs/local-raw.json (raw)

Configuration:
- Preset: "dev"
- Chain ID: "pezkuwichain_dev"
- Chain Type: Development
- Validators: 1 (Alice seed)
- Protocol: "pezkuwi"
```

### 2. Local Testnet (local_testnet)

```bash
# File: chainspecs/local.json (plain)
# File: chainspecs/local-raw.json (raw)

Configuration:
- Preset: "local_testnet"
- Chain ID: "pezkuwi_local_testnet"
- Chain Type: Local
- Validators: 2 (Alice+Bob seeds)
- Protocol: "pezkuwi"
```

### 3. Alfa Testnet

```bash
# File: chainspecs/alfa.json (plain)
# File: chainspecs/alfa-raw.json (raw)

Configuration:
- Preset: "alfa_testnet"
- Chain ID: "pezkuwichain_alfa_testnet"
- Chain Type: Development
- Validators: 4 (test seeds)
- Protocol: "pezkuwi"
```

### 4. Beta Testnet ⭐

```bash
# File: chainspecs/beta.json (plain)
# File: chainspecs/beta-raw.json (raw)
# Validator File: runtime/validators/beta_testnet_validators.json

Configuration:
- Preset: "beta_testnet"
- Chain ID: "pezkuwichain_beta_testnet"
- Chain Type: Live
- Validators: 8 (real keys from JSON)
- Protocol: "pezkuwi"
```

### 5. Staging ⭐

```bash
# File: chainspecs/staging.json (plain)
# File: chainspecs/staging-raw.json (raw)
# Validator File: runtime/validators/staging_validators.json

Configuration:
- Preset: "staging"
- Chain ID: "pezkuwichain_staging"
- Chain Type: Live
- Validators: 20 (real keys from JSON)
- Protocol: "pezkuwi"
```

### 6. Mainnet (Production) ⭐

```bash
# File: chainspecs/mainnet.json (plain)
# File: chainspecs/mainnet-raw.json (raw)
# Validator File: runtime/validators/mainnet_validators.json

Configuration:
- Preset: "production"
- Chain ID: "pezkuwichain"
- Chain Type: Live
- Validators: 100 (real keys from JSON)
- Protocol: "pezkuwi"
```

---

## 🛠️ Step-by-Step Build Process

### Phase 1: Prepare Validator Keys (Beta/Staging/Mainnet Only)

For production networks (beta, staging, mainnet), you must create validator JSON files with real keys.

#### Validator JSON Format

```json
{
  "validators": [
    {
      "name": "Validator-1",
      "stash": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "controller": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "session_keys": {
        "babe": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
        "grandpa": "0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee",
        "para_validator": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
        "para_assignment": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
        "authority_discovery": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
        "beefy": "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1"
      }
    }
  ]
}
```

#### Generate Session Keys

You can generate session keys using:

**Option 1: Using Subkey**
```bash
# Install subkey
cargo install --force subkey --git https://github.com/paritytech/polkadot-sdk

# Generate keys for each validator
subkey generate --scheme sr25519  # For BABE, Para, Authority Discovery
subkey generate --scheme ed25519  # For GRANDPA
subkey generate --scheme ecdsa    # For BEEFY

# Convert to SS58 address (format 42 for Substrate)
subkey inspect --scheme sr25519 "YOUR_SEED_PHRASE"
```

**Option 2: Using RPC (Running Node)**
```bash
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9944
```

#### Validator File Locations

```bash
# Beta testnet (8 validators)
runtime/validators/beta_testnet_validators.json

# Staging (20 validators)
runtime/validators/staging_validators.json

# Mainnet (100 validators)
runtime/validators/mainnet_validators.json
```

**IMPORTANT:** These files are compiled into the runtime at build time via `include_str!()`.

---

### Phase 2: Build the Runtime

#### Step 1: Clean Build Environment

```bash
cd /path/to/Pezkuwi-SDK/pezkuwi

# Clean previous builds
cargo clean

# Remove old chainspecs (optional, for fresh start)
rm -rf chainspecs/*.json 2>/dev/null || true
mkdir -p chainspecs
```

#### Step 2: Build Runtime (Production Mode)

```bash
# Build release binary WITHOUT benchmarks (recommended for production)
cargo build --release

# This compiles:
# - Runtime WASM: target/release/wbuild/pezkuwi-runtime/pezkuwi_runtime.wasm
# - Node binary: target/release/pezkuwi
```

**Build Time:** 5-10 minutes on modern hardware

#### Step 3: Verify Build Success

```bash
# Check binary exists
ls -lh target/release/pezkuwi
# Expected: ~140-150MB

# Check WASM exists
ls -lh target/release/wbuild/pezkuwichain/pezkuwichain.wasm
# Expected: ~2-3MB

# Test binary
./target/release/pezkuwi --version
# Expected: pezkuwi 1.0.0-dev-COMMIT_HASH
```

---

### Phase 3: Generate Chainspecs

#### Network-Specific Build Commands

The binary provides built-in chainspec generation functions (defined in `node/service/src/chain_spec.rs`).

**CRITICAL:** The `--chain` parameter must match function names in chain_spec.rs, NOT preset names!

##### Local Development (dev)

```bash
# Generate plain chainspec
./target/release/pezkuwi build-spec \
  --chain=pezkuwichain-dev \
  --disable-default-bootnode \
  > chainspecs/local.json

# Convert to raw (includes WASM)
./target/release/pezkuwi build-spec \
  --chain=chainspecs/local.json \
  --raw \
  --disable-default-bootnode \
  > chainspecs/local-raw.json
```

##### Local Testnet (2 validators)

```bash
# Generate plain chainspec
./target/release/pezkuwi build-spec \
  --chain=pezkuwichain-local \
  --disable-default-bootnode \
  > chainspecs/local.json

# Convert to raw (includes WASM)
./target/release/pezkuwi build-spec \
  --chain=chainspecs/local.json \
  --raw \
  --disable-default-bootnode \
  > chainspecs/local-raw.json
```

##### Alfa Testnet (4 validators)

```bash
# Generate plain chainspec
./target/release/pezkuwi build-spec \
  --chain=pezkuwichain-alfa \
  --disable-default-bootnode \
  > chainspecs/alfa.json

# Convert to raw (includes WASM)
./target/release/pezkuwi build-spec \
  --chain=chainspecs/alfa.json \
  --raw \
  --disable-default-bootnode \
  > chainspecs/alfa-raw.json
```

##### Beta Testnet (8 validators) ⭐

```bash
# PREREQUISITE: Ensure runtime/validators/beta_testnet_validators.json exists!

# Generate plain chainspec
./target/release/pezkuwi build-spec \
  --chain=pezkuwichain-beta \
  --disable-default-bootnode \
  > chainspecs/beta.json

# Convert to raw (includes WASM)
./target/release/pezkuwi build-spec \
  --chain=chainspecs/beta.json \
  --raw \
  --disable-default-bootnode \
  > chainspecs/beta-raw.json
```

##### Staging (20 validators) ⭐

```bash
# PREREQUISITE: Ensure runtime/validators/staging_validators.json exists!

# Generate plain chainspec
./target/release/pezkuwi build-spec \
  --chain=pezkuwichain-staging \
  --disable-default-bootnode \
  > chainspecs/staging.json

# Convert to raw (includes WASM)
./target/release/pezkuwi build-spec \
  --chain=chainspecs/staging.json \
  --raw \
  --disable-default-bootnode \
  > chainspecs/staging-raw.json
```

##### Mainnet (100 validators) ⭐

```bash
# PREREQUISITE: Ensure runtime/validators/mainnet_validators.json exists!

# Generate plain chainspec
./target/release/pezkuwi build-spec \
  --chain=pezkuwichain-production \
  --disable-default-bootnode \
  > chainspecs/mainnet.json

# Convert to raw (includes WASM)
./target/release/pezkuwi build-spec \
  --chain=chainspecs/mainnet.json \
  --raw \
  --disable-default-bootnode \
  > chainspecs/mainnet-raw.json
```

---

### Phase 4: Add Bootnodes (Optional)

After generating raw chainspecs, you can manually add bootnodes for faster peer discovery.

#### Edit Raw Chainspec

```bash
# Open raw chainspec
nano chainspecs/beta-raw.json

# Find "bootNodes" array (near top)
"bootNodes": [
  "/ip4/37.60.230.9/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
  "/ip4/VALIDATOR_2_IP/tcp/30334/p2p/PEER_ID_2",
  "/dns/validator3.pezkuwichain.io/tcp/30333/p2p/PEER_ID_3"
]
```

**How to Get Peer IDs:**

```bash
# Start validator node
./target/release/pezkuwi --chain=chainspecs/beta-raw.json \
  --validator \
  --name="Validator-1" \
  --base-path=/tmp/validator1 \
  --node-key=0000000000000000000000000000000000000000000000000000000000000001

# Look for line in logs:
# 🏷  Local node identity is: 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

---

## ✅ Verification Procedures

### 1. Verify WASM Inclusion (CRITICAL!)

This is the **most important** verification step. Without WASM, validators cannot produce blocks.

```bash
# Check genesis code size (should be 2-5 million characters)
cat chainspecs/beta-raw.json | jq -r '.genesis.runtimeGenesis.code' | wc -c

# Expected output: 2000000 to 5000000
# If output is < 10: ❌ WASM MISSING - Chainspec is broken!
```

**Example Output:**
```bash
# ✅ CORRECT:
4523891   # ~4.5MB WASM code present

# ❌ INCORRECT:
5         # Only 5 bytes - WASM missing!
```

### 2. Verify File Sizes

```bash
ls -lh chainspecs/

# Expected sizes:
# Plain chainspecs: 50-200 KB
# Raw chainspecs: 5-10 MB (includes WASM!)
```

**Example:**
```
-rw-r--r-- 1 user user  98K Nov 15 10:30 beta.json         ✅
-rw-r--r-- 1 user user 5.2M Nov 15 10:31 beta-raw.json     ✅
```

### 3. Verify Genesis Authorities

```bash
# Check BABE authorities count
cat chainspecs/beta-raw.json | jq '.genesis.runtimeGenesis.patch.babe.epochConfig.authorities' | jq length

# Expected: 8 for beta, 20 for staging, 100 for mainnet

# Check GRANDPA authorities count
cat chainspecs/beta-raw.json | jq '.genesis.runtimeGenesis.patch.grandpa.authorities' | jq length

# Expected: Same as BABE count
```

### 4. Verify Session Keys

```bash
# Check session keys configuration
cat chainspecs/beta-raw.json | jq '.genesis.runtimeGenesis.patch.session.keys' | jq length

# Expected: Same as validator count
```

### 5. Verify Staking Configuration

```bash
# Check initial validators in staking
cat chainspecs/beta-raw.json | jq '.genesis.runtimeGenesis.patch.staking.validatorCount'

# Expected: Validator count (8 for beta, 20 for staging, 100 for mainnet)
```

### 6. Test Network Startup

#### Single Node Test

```bash
# Start single validator
./target/release/pezkuwi \
  --chain=chainspecs/beta-raw.json \
  --base-path=/tmp/test-validator \
  --validator \
  --name="Test-Validator" \
  --port=30333 \
  --rpc-port=9944 \
  --rpc-cors=all

# Watch logs for:
# ✅ "Imported #1" - Block production working!
# ✅ "💤 Idle (0 peers)" - Waiting for peers (expected for single node)
# ❌ "Cannot produce blocks" - Session keys or authority config issue
```

#### Multi-Validator Test

```bash
# Start 2 validators minimum for finalization

# Validator 1
./target/release/pezkuwi \
  --chain=chainspecs/beta-raw.json \
  --base-path=/tmp/validator-1 \
  --validator \
  --name="Validator-1" \
  --port=30333 \
  --rpc-port=9944 \
  --node-key=0000000000000000000000000000000000000000000000000000000000000001 \
  > /tmp/validator-1.log 2>&1 &

# Validator 2
./target/release/pezkuwi \
  --chain=chainspecs/beta-raw.json \
  --base-path=/tmp/validator-2 \
  --validator \
  --name="Validator-2" \
  --port=30334 \
  --rpc-port=9945 \
  --node-key=0000000000000000000000000000000000000000000000000000000000000002 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
  > /tmp/validator-2.log 2>&1 &

# Check logs
tail -f /tmp/validator-1.log

# Watch for:
# ✅ "Discovered new external address" - Peer discovery working
# ✅ "Imported #1" - Block production
# ✅ "Finalized #1" - GRANDPA finalization (requires 2/3+ validators)
```

---

## 🐛 Common Issues & Troubleshooting

### Issue 1: Missing WASM in Raw Chainspec

**Symptom:**
```bash
cat chainspecs/beta-raw.json | jq -r '.genesis.runtimeGenesis.code' | wc -c
5  # Only 5 bytes!
```

**Cause:** Build failed or runtime not properly compiled.

**Solution:**
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Verify WASM file exists
ls -lh target/release/wbuild/pezkuwichain/pezkuwichain.wasm

# Regenerate chainspec
./target/release/pezkuwi build-spec \
  --chain=pezkuwichain-beta \
  --disable-default-bootnode \
  > chainspecs/beta.json

./target/release/pezkuwi build-spec \
  --chain=chainspecs/beta.json \
  --raw \
  --disable-default-bootnode \
  > chainspecs/beta-raw.json

# Verify again
cat chainspecs/beta-raw.json | jq -r '.genesis.runtimeGenesis.code' | wc -c
# Expected: 2000000+
```

---

### Issue 2: "No such file or directory" Error

**Symptom:**
```bash
./target/release/pezkuwi build-spec --chain=beta_testnet
Error: No such file or directory (os error 2)
```

**Cause:** Trying to use preset name instead of chain spec function name.

**Solution:**
```bash
# ❌ WRONG: Using preset name
--chain=beta_testnet

# ✅ CORRECT: Using function name from chain_spec.rs
--chain=pezkuwichain-beta

# Check available chains
./target/release/pezkuwi build-spec --help | grep -A20 "CHAIN"
```

---

### Issue 3: Validators Idle, No Block Production

**Symptom:**
```
💤 Idle (11 peers), best: #0 (0x1234…), finalized #0 (0x1234…)
```

**Cause:** Missing session keys or authorities in genesis.

**Diagnosis:**
```bash
# Check if authorities are in genesis
cat chainspecs/beta-raw.json | jq '.genesis.runtimeGenesis.patch.babe.epochConfig.authorities'

# Should show array of 8 authorities for beta testnet
# If empty or null: authorities missing!
```

**Solution:**
```bash
# Ensure validator JSON file exists and is correct
cat runtime/validators/beta_testnet_validators.json

# Rebuild runtime and chainspec
cargo clean
cargo build --release
./target/release/pezkuwi build-spec --chain=pezkuwichain-beta > chainspecs/beta.json
./target/release/pezkuwi build-spec --chain=chainspecs/beta.json --raw > chainspecs/beta-raw.json

# Verify authorities
cat chainspecs/beta-raw.json | jq '.genesis.runtimeGenesis.patch.babe.epochConfig.authorities' | jq length
# Expected: 8
```

---

### Issue 4: "Invalid Chain Spec" Error

**Symptom:**
```
Error: Service(Client(BadData("Invalid chain spec")))
```

**Cause:** Corrupted or malformed JSON in chainspec.

**Solution:**
```bash
# Validate JSON syntax
jq empty chainspecs/beta-raw.json

# If error, regenerate:
./target/release/pezkuwi build-spec --chain=pezkuwichain-beta > chainspecs/beta.json
./target/release/pezkuwi build-spec --chain=chainspecs/beta.json --raw > chainspecs/beta-raw.json
```

---

### Issue 5: Validator JSON Not Loading

**Symptom:**
```
Compiling pezkuwichain v1.0.0
error: couldn't read runtime/validators/beta_testnet_validators.json: No such file or directory
```

**Cause:** Missing validator JSON file at compile time.

**Solution:**
```bash
# Ensure file exists in correct location
ls runtime/validators/beta_testnet_validators.json

# If missing, create it:
cat > runtime/validators/beta_testnet_validators.json <<'EOF'
{
  "validators": [
    {
      "name": "Validator-1",
      "stash": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      ...
    }
  ]
}
EOF

# Rebuild
cargo build --release
```

---

### Issue 6: Finalization Not Happening

**Symptom:**
```
Imported #5
Imported #6
# Blocks imported but never finalized
```

**Cause:** Not enough validators online (need 2/3+ for GRANDPA finality).

**Solution:**
```bash
# For 8 validator network, need minimum 6 validators online
# Start at least 6 validators

# Check connected peers
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_peers"}' \
  http://localhost:9944 | jq '.result | length'

# Should see at least 5 peers for 6 validators (each connects to others)
```

---

## 🚀 Network Deployment

### Local Development Deployment

```bash
# Single validator (dev mode)
./target/release/pezkuwi \
  --dev \
  --rpc-cors=all \
  --rpc-external \
  --rpc-methods=unsafe
```

### Local Testnet Deployment

```bash
# Start Alice
./target/release/pezkuwi \
  --chain=chainspecs/local-raw.json \
  --alice \
  --base-path=/tmp/alice \
  --port=30333 \
  --rpc-port=9944 \
  --validator

# Start Bob (in another terminal)
./target/release/pezkuwi \
  --chain=chainspecs/local-raw.json \
  --bob \
  --base-path=/tmp/bob \
  --port=30334 \
  --rpc-port=9945 \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ALICE_PEER_ID
```

### Beta Testnet Deployment

```bash
# Validator 1 (bootnode)
./target/release/pezkuwi \
  --chain=chainspecs/beta-raw.json \
  --base-path=/data/validator-1 \
  --validator \
  --name="Validator-1" \
  --port=30333 \
  --rpc-port=9944 \
  --rpc-cors=all \
  --rpc-external \
  --rpc-methods=safe \
  --node-key=YOUR_NODE_KEY_1

# Validator 2
./target/release/pezkuwi \
  --chain=chainspecs/beta-raw.json \
  --base-path=/data/validator-2 \
  --validator \
  --name="Validator-2" \
  --port=30334 \
  --rpc-port=9945 \
  --node-key=YOUR_NODE_KEY_2 \
  --bootnodes /ip4/VALIDATOR_1_IP/tcp/30333/p2p/VALIDATOR_1_PEER_ID

# Repeat for validators 3-8
```

### Production Deployment (Systemd Service)

Create `/etc/systemd/system/pezkuwichain.service`:

```ini
[Unit]
Description=PezkuwiChain Validator Node
After=network.target

[Service]
Type=simple
User=pezkuwi
WorkingDirectory=/opt/pezkuwi
ExecStart=/opt/pezkuwi/target/release/pezkuwi \
  --chain=/opt/pezkuwi/chainspecs/mainnet-raw.json \
  --base-path=/data/pezkuwi \
  --validator \
  --name="Production-Validator-1" \
  --port=30333 \
  --rpc-port=9944 \
  --rpc-cors=all \
  --rpc-methods=safe \
  --node-key-file=/etc/pezkuwi/node-key \
  --bootnodes=/ip4/BOOTNODE_IP/tcp/30333/p2p/BOOTNODE_PEER_ID
Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable pezkuwichain
sudo systemctl start pezkuwichain
sudo systemctl status pezkuwichain

# View logs
sudo journalctl -u pezkuwichain -f
```

---

## 📊 Complete Build & Deploy Checklist

### Beta Testnet Checklist

- [ ] **1. Prepare Validator Keys**
  - [ ] Generate 8 validator session keys
  - [ ] Create `runtime/validators/beta_testnet_validators.json`
  - [ ] Verify JSON syntax with `jq`

- [ ] **2. Build Runtime**
  - [ ] Run `cargo clean`
  - [ ] Run `cargo build --release`
  - [ ] Verify binary: `ls -lh target/release/pezkuwi` (~140MB)
  - [ ] Verify WASM: `ls -lh target/release/wbuild/pezkuwichain/pezkuwichain.wasm` (~2-3MB)

- [ ] **3. Generate Chainspecs**
  - [ ] Generate plain: `./target/release/pezkuwi build-spec --chain=pezkuwichain-beta > chainspecs/beta.json`
  - [ ] Generate raw: `./target/release/pezkuwi build-spec --chain=chainspecs/beta.json --raw > chainspecs/beta-raw.json`

- [ ] **4. Verify Chainspecs**
  - [ ] Check WASM size: `cat chainspecs/beta-raw.json | jq -r '.genesis.runtimeGenesis.code' | wc -c` (>2M)
  - [ ] Check authorities: `cat chainspecs/beta-raw.json | jq '.genesis.runtimeGenesis.patch.babe.epochConfig.authorities' | jq length` (=8)
  - [ ] Check file size: `ls -lh chainspecs/beta-raw.json` (>5MB)

- [ ] **5. Test Locally**
  - [ ] Start 2 validators locally
  - [ ] Verify peer discovery (2 peers)
  - [ ] Verify block production (Imported #1, #2, #3...)
  - [ ] Verify finalization (Finalized #1, #2, #3...)

- [ ] **6. Deploy to VPS**
  - [ ] Upload binary to VPS
  - [ ] Upload chainspec to VPS
  - [ ] Start validators 1-8
  - [ ] Verify network health

---

## 📚 Additional Resources

### Chain Spec Functions Reference

From `node/service/src/chain_spec.rs`:

```rust
// Line 280-291: Development config
pub fn pezkuwichain_development_config() -> Result<PezkuwiChainSpec, String>
// CLI: --chain=pezkuwichain-dev

// Line 297-308: Local testnet config
pub fn pezkuwichain_local_testnet_config() -> Result<PezkuwiChainSpec, String>
// CLI: --chain=pezkuwichain-local

// Line 314-326: Alfa testnet config
pub fn pezkuwichain_alfa_testnet_config() -> Result<PezkuwiChainSpec, String>
// CLI: --chain=pezkuwichain-alfa

// Line 331-343: Beta testnet config
pub fn pezkuwichain_beta_testnet_config() -> Result<PezkuwiChainSpec, String>
// CLI: --chain=pezkuwichain-beta

// Line 348-360: Staging config
pub fn pezkuwichain_staging_config() -> Result<PezkuwiChainSpec, String>
// CLI: --chain=pezkuwichain-staging

// Line 365-377: Production config
pub fn pezkuwichain_production_config() -> Result<PezkuwiChainSpec, String>
// CLI: --chain=pezkuwichain-production
```

### Genesis Preset Functions Reference

From `runtime/pezkuwichain/src/genesis_config_presets.rs`:

```rust
// Development (1 validator - Alice)
pub fn development_config_genesis() -> Value

// Local testnet (2 validators - Alice+Bob)
pub fn local_testnet_genesis() -> Value

// Alfa testnet (4 validators)
pub fn alfa_testnet_genesis() -> Value

// Beta testnet (8 validators from JSON)
pub fn beta_testnet_genesis() -> Value

// Staging (20 validators from JSON)
pub fn staging_genesis() -> Value

// Production (100 validators from JSON)
pub fn production_genesis() -> Value
```

### Key Files Summary

| File Path | Purpose | When Modified |
|-----------|---------|---------------|
| `node/service/src/chain_spec.rs` | Chain spec builders | Add new networks |
| `runtime/pezkuwichain/src/genesis_config_presets.rs` | Genesis presets | Change tokenomics/validators |
| `runtime/validators/beta_testnet_validators.json` | Beta validator keys | Before beta deployment |
| `runtime/validators/staging_validators.json` | Staging validator keys | Before staging deployment |
| `runtime/validators/mainnet_validators.json` | Mainnet validator keys | Before mainnet deployment |
| `chainspecs/*.json` | Generated chainspecs | After each build |

---

## 🎯 Quick Reference Commands

```bash
# Clean build
cargo clean && cargo build --release

# Generate beta testnet chainspecs
./target/release/pezkuwi build-spec --chain=pezkuwichain-beta > chainspecs/beta.json
./target/release/pezkuwi build-spec --chain=chainspecs/beta.json --raw > chainspecs/beta-raw.json

# Verify WASM
cat chainspecs/beta-raw.json | jq -r '.genesis.runtimeGenesis.code' | wc -c

# Verify authorities
cat chainspecs/beta-raw.json | jq '.genesis.runtimeGenesis.patch.babe.epochConfig.authorities' | jq length

# Test startup
./target/release/pezkuwi --chain=chainspecs/beta-raw.json --dev
```

---

## ✅ Success Criteria

A properly created chainspec must have:

1. ✅ **WASM Code Present**: `genesis.runtimeGenesis.code` > 2 million characters
2. ✅ **Correct Authorities**: BABE + GRANDPA authorities = validator count
3. ✅ **Session Keys**: Session keys count = validator count
4. ✅ **File Size**: Raw chainspec > 5MB
5. ✅ **Valid JSON**: `jq empty chainspec.json` no errors
6. ✅ **Block Production**: Validators produce blocks when started
7. ✅ **Finalization**: Network finalizes blocks with 2/3+ validators

---

**End of Guide**

For issues or questions, refer to:
- [Substrate Documentation](https://docs.substrate.io)
- [Polkadot SDK Documentation](https://paritytech.github.io/polkadot-sdk/master/)
- PezkuwiChain GitHub Issues
