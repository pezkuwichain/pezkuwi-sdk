# Claude Code Kuralları - Pezkuwi SDK

## 🚨 GITHUB ACTIONS KURALI - KESİNLİKLE UYULMALI

**Workflow hata verdiğinde veya değişiklik yapılacağında:**
1. **ÖNCE** tüm mevcut workflow run'larını iptal et (`gh run cancel`)
2. **SONRA** hepsini sil (`gh run delete`)
3. **EN SON** tek bir commit/push ile temiz başlat

**ASLA eski workflow'ların üzerine yeni workflow bırakma!**
**ASLA kuyrukta onlarca workflow biriktirme!**

```bash
# Temizlik komutu (her zaman önce bunu çalıştır):
gh run list --limit 100 --json databaseId,status | jq -r '.[] | select(.status == "queued" or .status == "in_progress" or .status == "pending") | .databaseId' | xargs -I{} gh run cancel {} 2>/dev/null
sleep 5
gh run list --limit 100 --json databaseId -q '.[].databaseId' | xargs -I{} gh run delete {} 2>/dev/null
```

---

## ⚠️ PEZKUWI SDK TERMİNOLOJİSİ - KRİTİK

**ASLA POLKADOT SDK TERİMLERİ KULLANMA! Bu bağımsız bir blockchain projesi.**

### Doğru Terminoloji Tablosu:

| YANLIŞ (Polkadot SDK) | DOĞRU (Pezkuwi SDK) |
|-----------------------|---------------------|
| parachain | **teyrchain** |
| rococo | **pezkuwichain** |
| westend | **zagros** |
| kusama | **zagros** |
| polkadot | **pezkuwichain** |
| `[[parachains]]` | **`[[teyrchains]]`** |
| `[[parachains.collators]]` | **`[[teyrchains.collators]]`** |
| `-lparachain=debug` | **`-lteyrchain=debug`** |
| `parachain=debug` | **`teyrchain=debug`** |

### Token'lar:
- **HEZ**: Relay chain native token (200M genesis, inflationary)
- **PEZ**: Asset Hub governance token (5B sabit supply)
- **TYR**: Base unit (1 HEZ = 10^18 TYR)

### System Teyrchains:
- **Asset Hub Teyrchain**: ID 1000
- **People Chain Teyrchain**: ID 1004

### Zombienet Config Örneği (DOĞRU):
```toml
[relaychain]
default_args = ["-lteyrchain=debug"]
chain = "pezkuwichain-dev"

[[teyrchains]]
id = 1000
chain = "asset-hub-pezkuwichain-dev"

[[teyrchains.collators]]
args = ["-lteyrchain=debug"]
```

---

## 🎯 ANA HEDEF VE ÇALIŞMA PRENSİPLERİ

### Hedef
Pezkuwi blockchain'i mainnet'e taşımak. Her test aşamasında (dev → local → alpha → beta → staging → mainnet) tüm bug/hataları kalıcı olarak çözmeden bir sonraki aşamaya GEÇİLMEZ.

### Mevcut Aşama: DEV NETWORK
**Başarı Kriterleri (hepsi sağlanmalı):**
- [ ] 3 runtime çalışmalı (Relay Chain, Asset Hub, People Chain)
- [ ] Birbirini görmeli (peer discovery)
- [ ] Bloklar üretilmeli
- [ ] Finalized olmalı
- [ ] Alice hesabında genesis token'ları görülmeli (HEZ, PEZ)

### Test Aşamaları Sırası
1. **DEV** (1 validator - Alice) <- ŞU AN BURADAYIZ
2. **LOCAL** (2 validator - Alice + Bob)
3. **ALPHA** (4 validator)
4. **BETA** (8 validator)
5. **STAGING** (21 validator)
6. **MAINNET** (100 validator)

### Çalışma Prensibi
```
Her aşamada:
1. Planlanan testleri yap
2. Tüm testlerden başarılı sonuç al
3. Hata/bug varsa → düzelt → tekrar test et
4. Başarılı olunca → blockchain upgrade → sonraki aşama
```

**ÖNEMLİ:** Ekranda geçici başarı görmek yeterli DEĞİL. Kalıcı çözümler, tam testler, sonra ilerleme.

---

## 📁 CODEBASE STRUCTURE

### Ana Dizinler

```
pezkuwi-sdk/
├── pezkuwi/              # Relay chain node ve runtime (ana blockchain)
│   ├── cli/              # Command-line interface
│   ├── node/             # Node implementation (subsystems, networking)
│   ├── runtime/          # Relay chain runtimes
│   │   ├── pezkuwichain/ # Production relay chain runtime
│   │   ├── zagros/       # Test relay chain runtime
│   │   └── common/       # Shared runtime code
│   ├── pallets/          # Relay chain specific pallets
│   │   └── validator-pool/ # Validator pool management
│   ├── primitives/       # Core types and traits
│   ├── xcm/              # Cross-chain messaging
│   └── zombienet_tests/  # E2E tests with Zombienet
│
├── cumulus/              # Teyrchain (parachain) framework
│   ├── client/           # Teyrchain client components
│   ├── pallets/          # Cumulus-specific pallets
│   │   ├── teyrchain-system/   # Core teyrchain pallet
│   │   ├── collator-selection/ # Collator management
│   │   └── xcmp-queue/         # Cross-chain message queue
│   ├── teyrchains/       # Teyrchain runtimes and custom pallets
│   │   ├── runtimes/     # System teyrchain runtimes
│   │   │   ├── assets/   # Asset Hub (asset-hub-pezkuwichain, asset-hub-zagros)
│   │   │   ├── people/   # People Chain (people-pezkuwichain, people-zagros)
│   │   │   ├── bridge-hubs/    # Bridge Hub runtimes
│   │   │   ├── coretime/       # Coretime runtimes
│   │   │   └── collectives/    # Collectives runtime
│   │   ├── pallets/      # Custom Pezkuwi pallets (12 total)
│   │   │   ├── identity-kyc/   # KYC and identity verification
│   │   │   ├── welati/         # Democratic governance
│   │   │   ├── perwerde/       # Educational platform
│   │   │   ├── presale/        # Token presale mechanism
│   │   │   ├── tiki/           # NFT-based citizenship
│   │   │   ├── trust/          # Peer-to-peer trust system
│   │   │   ├── referral/       # Referral incentive system
│   │   │   ├── pez-treasury/   # Community treasury
│   │   │   ├── pez-rewards/    # Staking rewards distribution
│   │   │   ├── staking-score/  # Reputation metrics
│   │   │   ├── token-wrapper/  # Asset wrapping
│   │   │   └── teyrchain-info/ # Teyrchain metadata
│   │   └── integration-tests/  # Emulated integration tests
│   ├── pezkuwi-omni-node/      # Universal teyrchain node
│   └── pezkuwi-teyrchain/      # Teyrchain binary
│
├── substrate/            # Blockchain framework (Substrate)
│   ├── frame/            # FRAME pallets (~100 pallets)
│   │   ├── balances/     # Token balances
│   │   ├── staking/      # NPoS staking
│   │   ├── identity/     # On-chain identity
│   │   ├── democracy/    # Democratic governance
│   │   ├── treasury/     # On-chain treasury
│   │   ├── contracts/    # Smart contracts (ink!)
│   │   └── ...           # Many more standard pallets
│   ├── client/           # Client-side components
│   │   ├── cli/          # Command-line interface
│   │   ├── consensus/    # Consensus implementations (BABE, GRANDPA, Aura)
│   │   ├── network/      # P2P networking
│   │   ├── db/           # Database backend
│   │   └── rpc/          # RPC server
│   ├── primitives/       # Core primitives
│   └── bin/              # Binary utilities (subkey, chain-spec-builder)
│
├── bridges/              # Cross-chain bridges
│   ├── modules/          # Bridge pallets
│   ├── primitives/       # Bridge primitives
│   ├── snowbridge/       # Ethereum bridge
│   └── relays/           # Bridge relay implementations
│
├── templates/            # Project templates
│   ├── teyrchain/        # Teyrchain template
│   ├── solochain/        # Standalone chain template
│   ├── minimal/          # Minimal template
│   └── zombienet/        # Zombienet config templates
│
├── umbrella/             # Umbrella crate (pezkuwi-sdk)
├── docs/                 # Documentation
├── docker/               # Docker configurations
├── scripts/              # Build and utility scripts
└── prdoc/                # PR documentation
```

### Key Configuration Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Workspace root, all crate members |
| `.config/taplo.toml` | TOML formatting rules |
| `.config/zepter.yaml` | Feature propagation checks |
| `.config/nextest.toml` | Test runner configuration |
| `.config/lychee.toml` | Link checker configuration |
| `.rustfmt.toml` | Rust formatting (tabs, 100 char width) |
| `.cargo/config.toml` | Cargo build settings |
| `.github/.markdownlint.yaml` | Markdown linting rules |

---

## 🛠️ BUILD COMMANDS

### Basic Build
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build specific binary
cargo build --release -p pezkuwi        # Relay chain node
cargo build --release -p pezkuwi-teyrchain  # Teyrchain collator

# Build with features
cargo build --release --features runtime-benchmarks
cargo build --release --features try-runtime
```

### Run Node
```bash
# Dev mode (single validator)
./target/release/pezkuwi --dev

# With specific chain spec
./target/release/pezkuwi --chain pezkuwichain-dev

# Teyrchain collator
./target/release/pezkuwi-teyrchain --dev
```

### Testing
```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p pallet-identity-kyc
cargo test -p asset-hub-pezkuwichain-runtime

# Run with nextest (faster)
cargo nextest run

# Run runtime benchmarks
cargo test --release --features runtime-benchmarks
```

### Formatting & Linting
```bash
# Format Rust code
cargo +nightly fmt --all

# Format TOML files
taplo format --config .config/taplo.toml

# Check TOML format
taplo format --check --config .config/taplo.toml

# Run Clippy
cargo clippy --all-targets --all-features

# Check feature propagation
zepter run check

# Fix feature propagation
zepter run default
```

### Umbrella Crate
```bash
# Regenerate umbrella crate
python3 scripts/generate-umbrella.py --sdk . --version 0.1.0
cargo +nightly fmt -p pezkuwi-sdk
```

---

## 🔧 CI/CD WORKFLOWS

### Quick Checks (`checks-quick.yml`)
Fast checks that run on every PR:
- `fmt` - Rust formatting check
- `check-toml-format` - TOML formatting (taplo)
- `check-zepter` - Feature propagation
- `check-workspace` - Workspace integrity
- `check-markdown` - Markdown linting
- `check-umbrella` - Umbrella crate correctness
- `check-dependency-rules` - Dependency constraints

### Main Checks (`checks.yml`)
- Clippy lints
- Documentation builds
- Try-runtime checks

### Tests (`tests-*.yml`)
- `tests-linux-stable.yml` - Main test suite
- `tests-misc.yml` - Miscellaneous tests
- `tests-evm.yml` - EVM-related tests

### Zombienet Tests
- `zombienet_pezkuwi.yml` - Relay chain E2E tests
- `zombienet_cumulus.yml` - Teyrchain E2E tests
- `zombienet_substrate.yml` - Substrate E2E tests

### Release Workflows
- `release-*` - Release automation
- `build-publish-images.yml` - Docker image builds

---

## 📦 CUSTOM PALLETS (12 Total)

Located in `cumulus/teyrchains/pallets/`:

| Pallet | Purpose | Location |
|--------|---------|----------|
| `presale` | Multi-round token launches with vesting | `cumulus/teyrchains/pallets/presale` |
| `identity-kyc` | Decentralized KYC verification | `cumulus/teyrchains/pallets/identity-kyc` |
| `welati` | Democratic governance | `cumulus/teyrchains/pallets/welati` |
| `perwerde` | Educational platform | `cumulus/teyrchains/pallets/perwerde` |
| `pez-treasury` | Community treasury with halving | `cumulus/teyrchains/pallets/pez-treasury` |
| `pez-rewards` | Trust-based staking rewards | `cumulus/teyrchains/pallets/pez-rewards` |
| `staking-score` | Reputation-based metrics | `cumulus/teyrchains/pallets/staking-score` |
| `trust` | Peer-to-peer trust system | `cumulus/teyrchains/pallets/trust` |
| `referral` | Multi-level referral system | `cumulus/teyrchains/pallets/referral` |
| `tiki` | NFT-based citizenship (4-tier) | `cumulus/teyrchains/pallets/tiki` |
| `token-wrapper` | Asset wrapping for XCM | `cumulus/teyrchains/pallets/token-wrapper` |
| `teyrchain-info` | Teyrchain metadata | `cumulus/teyrchains/pallets/teyrchain-info` |

Plus relay chain pallet:
| `validator-pool` | Validator pool management | `pezkuwi/pallets/validator-pool` |

---

## 🌐 RUNTIMES

### Relay Chain Runtimes
| Runtime | Path | Purpose |
|---------|------|---------|
| `pezkuwichain` | `pezkuwi/runtime/pezkuwichain` | Production relay chain |
| `zagros` | `pezkuwi/runtime/zagros` | Test relay chain |

### System Teyrchain Runtimes
| Runtime | Path | Teyrchain ID |
|---------|------|--------------|
| `asset-hub-pezkuwichain` | `cumulus/teyrchains/runtimes/assets/asset-hub-pezkuwichain` | 1000 |
| `asset-hub-zagros` | `cumulus/teyrchains/runtimes/assets/asset-hub-zagros` | 1000 |
| `people-pezkuwichain` | `cumulus/teyrchains/runtimes/people/people-pezkuwichain` | 1004 |
| `people-zagros` | `cumulus/teyrchains/runtimes/people/people-zagros` | 1004 |
| `bridge-hub-pezkuwichain` | `cumulus/teyrchains/runtimes/bridge-hubs/bridge-hub-pezkuwichain` | 1002 |
| `coretime-pezkuwichain` | `cumulus/teyrchains/runtimes/coretime/coretime-pezkuwichain` | 1005 |

---

## Dizin Kuralları

| Dizin | Kullanım |
|-------|----------|
| `/home/mamostehp/Pezkuwi-SDK` | **Tüm işlemler burada yapılır** (edit, commit, push) |

## Ekran Görüntüleri

Kullanıcı "ekran" veya "ekrana bak" dediğinde:
```
/home/mamostehp/DKSweb_ekran/Screenshot.png
```
dosyasını oku.

## Gemini ile Koordinasyon

Gemini mesaj gönderdiğinde veya "gemini mesaj" denildiğinde:
```
/home/mamostehp/Pezkuwi-SDK/.ai-coordination/messages.md
```
dosyasını oku. Diğer koordinasyon dosyaları:
- `claude-status.md` - Claude'un mevcut durumu
- `gemini-status.md` - Gemini'nin mevcut durumu
- `task-board.md` - Görev tablosu

## Commit Kuralları

- Commit mesajlarına `Generated with [Claude Code]` ve `Co-Authored-By: Claude` **EKLEME**
- Sadece düz commit mesajı yaz

## Proje Bilgileri

- **Proje:** Pezkuwi SDK - Bağımsız blockchain projesi
- **Teknoloji:** Polkadot SDK fork'u (ama Polkadot DEĞİL, bağımsız)
- **Ana branch:** `main`
- **GitHub:** `pezkuwichain/pezkuwi-sdk`
- **Discord:** `https://discord.gg/Y3VyEC6h8W` (Server: 1444335345935057049)

## Önemli Notlar

1. `paritytech` referansları `pezkuwichain` olmalı
2. `polkadot-sdk` referansları `pezkuwi-sdk` olmalı
3. Kaliteyi düşüren "kolay çözümler" yerine doğru çözümü uygula
4. Geride iş bırakma - kapsamlı da olsa tamamla

---

## 🧪 ZOMBIENET TESTING

### Running Zombienet Tests
```bash
# Install zombienet
npm i -g @parity/zombienet

# Run a test
zombienet test --provider native path/to/test.zndsl

# Spawn a network
zombienet spawn path/to/network.toml
```

### Test Configs Location
- Pezkuwi tests: `.github/zombienet-tests/zombienet_pezkuwi_tests.yml`
- Cumulus tests: `.github/zombienet-tests/zombienet_cumulus_tests.yml`
- Substrate tests: `.github/zombienet-tests/zombienet_substrate_tests.yml`

### Example Network Config (DOĞRU TERMINOLOJI):
```toml
[settings]
timeout = 1000

[relaychain]
chain = "pezkuwichain-dev"
default_command = "./target/release/pezkuwi"
default_args = ["-lteyrchain=debug"]

[[relaychain.nodes]]
name = "alice"
validator = true

[[relaychain.nodes]]
name = "bob"
validator = true

[[teyrchains]]
id = 1000
chain = "asset-hub-pezkuwichain-dev"
cumulus_based = true

[[teyrchains.collators]]
name = "asset-hub-collator"
command = "./target/release/pezkuwi-teyrchain"
args = ["-lteyrchain=debug"]
```

---

## ✅ CI/CD QUICK-CHECKS DÜZELTMELERİ TAMAMLANDI

**Son güncelleme:** 2025-12-04

### Tamamlanan İşler

1. **check-workspace.py düzeltmesi** ✅
   - `polkadot-sdk` → `pezkuwi-sdk` değiştirildi
   - Umbrella crate için hem `path` hem `workspace = true` kabul ediliyor

2. **Bridge crate workspace inheritance (16 crate)** ✅
   - Tüm bridge crate'leri `workspace = true` kullanıyor

3. **Markdown lint kuralları** ✅
   - MD004 (ul-style): Devre dışı - çok fazla legacy dosya
   - MD013 (line-length): Devre dışı - URL'ler satırları uzatıyor

4. **TOML format (taplo)** ✅
   - `.config/taplo.toml` path'leri `polkadot` → `pezkuwi` düzeltildi
   - 435+ TOML dosyası formatlandı

5. **Zepter check** ✅
   - `.config/zepter.yaml`: `-p=polkadot-sdk` → `-p=pezkuwi-sdk` düzeltildi
   - Feature propagation: 36+ issue fix edildi
   - Duplicate deps: `pallet-identity-kyc` ve `pallet-tiki` düzeltildi

6. **Umbrella crate** ✅
   - `generate-umbrella.py` çalıştırıldı
   - `umbrella/Cargo.toml` ve `umbrella/src/lib.rs` yeniden oluşturuldu

### Değiştirilen Dosyalar (438 dosya)
- Config dosyaları: `.config/taplo.toml`, `.config/zepter.yaml`, `.github/.markdownlint.yaml`
- Script: `.github/scripts/check-workspace.py`
- Pallet Cargo.toml: `pallet-identity-kyc`, `pallet-tiki` + 12 özel pallet feature propagation
- Tüm Cargo.toml dosyaları (taplo format)
- Umbrella crate dosyaları

---

## 📝 DEVELOPMENT CONVENTIONS

### Rust Style
- **Formatter**: `rustfmt` with nightly
- **Tab style**: Hard tabs (not spaces)
- **Max line width**: 100 characters
- **Import style**: Crate-level granularity

### TOML Style
- **Formatter**: `taplo`
- **Key ordering**: Alphabetical in dependencies
- **Tab style**: Hard tabs

### Commit Messages
- Use conventional commits: `fix:`, `feat:`, `chore:`, `docs:`, etc.
- Keep first line under 72 characters
- Reference issues when applicable

### Feature Flags
Standard features to propagate:
- `std` - Standard library support
- `runtime-benchmarks` - Benchmarking support
- `try-runtime` - Try-runtime testing

### Adding New Crates
1. Add to `Cargo.toml` workspace members
2. Run `python3 scripts/generate-umbrella.py`
3. Run `cargo +nightly fmt -p pezkuwi-sdk`
4. Ensure feature propagation with `zepter run check`

---
