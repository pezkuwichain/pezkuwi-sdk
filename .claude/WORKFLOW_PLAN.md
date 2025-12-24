# Pezkuwi SDK - CI Workflow Mühendislik Planı

> **KURAL:** Bu dosyadaki komutları birebir çalıştır. Kendi komutunu uydurma.

## Son CI Durumu (main branch)

| Durum | Sayı |
|-------|------|
| Skipped | 25 |
| Successful | 58 |
| **Failing** | **42** |
| Cancelled | 10 |

---

## FAZ 1: Başarısız Workflow'lar (10 workflow, ~30 job)

### 1.1 quick-checks.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| `check-toml-format` | ✅ | `taplo format --check --config .config/taplo.toml` |
| `check-markdown` | ✅ | `markdownlint --config .github/.markdownlint.yaml --ignore target --ignore vendor .` |
| `fmt` | ✅ | `cargo fmt --all -- --check` |
| `check-zepter` | ✅ | `zepter run check` |
| `check-workspace` | ✅ | `python3 .github/scripts/check-workspace.py .` |
| `check-dependency-rules` | ✅ | `python3 .github/scripts/deny-git-deps.py .` |
| `check-umbrella` | ✅ | `python3 scripts/generate-umbrella.py --check` |
| `test-rust-features` | ✅ | `.gitlab/rust-features.sh .` |
| `check-fail-ci` | ✅ | `! grep -r "FAIL-CI" --include="*.rs" . --exclude-dir=target` |
| `check-readme` | ✅ | `.github/scripts/check-missing-readme-generation.sh` |

**Düzeltme:**
```bash
# TOML format düzelt
taplo format --config .config/taplo.toml

# Markdown düzelt
markdownlint --config .github/.markdownlint.yaml --ignore target --ignore vendor . --fix
```

---

### 1.2 checks.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| `cargo-clippy` | ✅ | `RUSTFLAGS="-D warnings" SKIP_WASM_BUILD=1 cargo clippy --all-targets --all-features --locked --workspace --quiet` |
| `check-try-runtime` | ✅ | Aşağıdaki 3 komut sırayla çalıştırılmalı |
| `check-core-crypto-features` | ✅ | Aşağıdaki 3 script çalıştırılmalı |

**check-try-runtime komutları:**
```bash
cargo check --locked --all --features try-runtime --quiet
cargo check --locked -p teyrchain-template-node --features try-runtime
cargo check --locked --all --features try-runtime,experimental --quiet
```

**check-core-crypto-features komutları:**
```bash
./bizinikiwi/primitives/core/check-features-variants.sh
./bizinikiwi/primitives/application-crypto/check-features-variants.sh
./bizinikiwi/primitives/keyring/check-features-variants.sh
```

---

### 1.3 docs.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| `test-doc` | ❌ | `time cargo test --doc --workspace --locked --release --no-fail-fast --features=runtime-benchmarks,try-runtime` |
| `build-rustdoc` | ❌ | `time cargo doc --workspace --all-features --no-deps` |

---

### 1.4 tests.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| `cargo-check-all-benches` | ✅ | `SKIP_WASM_BUILD=1 cargo check --workspace --benches --features runtime-benchmarks --quiet` |
| `test-syscalls` | ❌ | CI-specific (syscall binary testi) |
| `quick-benchmarks` | ❌ | `cargo run --release -p pez-frame-benchmarking-cli` |

---

### 1.5 tests-linux-stable.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| `test-linux-stable` | ❌ | `cargo nextest run --workspace --locked --release --no-fail-fast` |
| `test-linux-stable-int` | ❌ | `cargo nextest run -p "*-integration-tests*" -j1 --locked --release --no-fail-fast` |
| `test-linux-stable-no-try-runtime` | ❌ | `cargo check --workspace --locked --no-default-features` |
| `test-linux-stable-runtime-benchmarks` | ❌ | `cargo check --workspace --locked --features runtime-benchmarks` |

---

### 1.6 tests-misc.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| `test-node-metrics` | ❌ | `cargo test -p pezsc-network --features "substrate-prometheus"` |
| `cargo-check-all-crate-macos` | ❌ | macOS runner gerekli |
| `test-pezframe-ui` | ❌ | `cargo test -p pezframe-ui-tests --locked` |
| `cargo-check-each-crate` | ❌ | `cargo check -p <crate-name> --locked` (her crate için ayrı) |

---

### 1.7 tests-evm.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| `evm-test-suite (test:pvm)` | ❌ | PolkaVM toolchain gerekli |
| `differential-tests` | ❌ | `resolc` compiler gerekli |

---

### 1.8 build-misc.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| `build-runtimes-polkavm` | ❌ | RISC-V target gerekli (`riscv64emac-unknown-none-polkavm`) |

---

### 1.9 build-publish-images.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| `build-linux-stable` | ✅ | `cargo build --locked --release -p pezkuwi` |
| `build-linux-stable-pezcumulus` | ✅ | `cargo build --locked --release -p pezkuwi-teyrchain-bin` |
| `build-linux-bizinikiwi` | ✅ | `cargo build --locked --release -p pezstaging-node-cli` |
| `build-templates-node` | ✅ | `cargo build --locked --release -p teyrchain-template-node` |
| `build-malus` | ✅ | `cargo build --locked --release -p pezkuwi-test-malus` |
| `build-test-collators` | ✅ | `cargo build --locked --release -p test-teyrchain-adder-collator -p test-teyrchain-undying-collator` |
| `prepare-bridges-zombienet-artifacts` | ❌ | Bridge binary'leri |
| `build-push-image-test-teyrchain` | ❌ | Docker image build |

---

### 1.10 check-getting-started.yml

| Job | Durum | Lokal Test Komutu |
|-----|-------|-------------------|
| Tüm distro testleri | ✅ | Template repo'ları mevcut, script doğru |

**Template Repo'ları (GitHub'da mevcut):**
- ✅ `github.com/pezkuwichain/pezkuwi-sdk-minimal-template`
- ✅ `github.com/pezkuwichain/pezkuwi-sdk-solochain-template`
- ✅ `github.com/pezkuwichain/pezkuwi-sdk-teyrchain-template`
- ✅ `github.com/pezkuwichain/pezkuwi-runtime-templates`

---

## FAZ 2: Düzeltme Sırası

```
AŞAMA 1 - Kritik (PR merge için ZORUNLU):
├── quick-checks.yml
│   ├── taplo format --config .config/taplo.toml
│   └── markdownlint --fix
└── checks.yml
    ├── cargo clippy düzeltmeleri
    └── try-runtime feature düzeltmeleri

AŞAMA 2 - Derleme Hataları:
├── build-publish-images.yml binary build'ler
├── build-misc.yml PolkaVM build
└── tests.yml benchmark compile

AŞAMA 3 - Test Hataları:
├── tests-linux-stable.yml unit/integration tests
├── tests-misc.yml çeşitli testler
└── docs.yml doc tests

AŞAMA 4 - EVM/PolkaVM:
├── tests-evm.yml PolkaVM toolchain
└── resolc compiler kurulumu
```

---

## Tamamlanan Düzeltmeler

| Tarih | Düzeltme | Etki |
|-------|----------|------|
| 2025-12-25 | pezpallet-welati EnsureOrigin try_successful_origin düzeltmeleri (3 fix) | pezkuwi-teyrchain-bin build |
| 2025-12-25 | asset-hub-zagros governance origins macro düzeltmeleri (2 macro) | pezkuwi-teyrchain-bin build |
| 2025-12-25 | Binary build'ler test edildi: pezkuwi, pezkuwi-teyrchain-bin, pezstaging-node-cli, teyrchain-template-node | build-publish-images |
| 2025-12-24 | Bench check'e --features runtime-benchmarks eklendi, yanlış exclusion'lar kaldırıldı | cargo-check-all-benches |
| 2024-12-24 | Workflow'larda pezstaging-node-cli paket adı düzeltildi | build-publish-images, tests |
| 2024-12-24 | Umbrella node feature'ına subxt native propagation eklendi | check-try-runtime |
| 2024-12-24 | Vendor crate rebrand düzeltmeleri (sp_* -> pezsp_*) | cargo-clippy |
| 2024-12-24 | Subxt examples artifact paths düzeltildi | cargo-clippy |
| 2024-12-24 | Subxt examples type conversion düzeltmeleri | cargo-clippy |
| 2024-12-24 | Zombienet-sdk test type düzeltmeleri | cargo-clippy |
| 2024-12-24 | TOML format düzeltildi (umbrella/Cargo.toml) | check-toml-format |
| 2024-12-23 | Umbrella'ya pezpallet-root-testing eklendi | check-try-runtime |
| 2024-12-23 | Umbrella'ya pezpallet-xcm-benchmarks eklendi | Runtime build |
| 2024-12-23 | Subxt runtime-full'dan çıkarıldı | getrandom WASM fix |
| 2024-12-23 | Template URL'leri düzeltildi | check-getting-started |
| 2024-12-22 | rustfmt.toml stable-only | fmt job |
| 2024-12-22 | Zepter feature propagation | check-zepter |

---

## Şu An Yapılması Gereken

1. ✅ `taplo format --config .config/taplo.toml` - TAMAMLANDI
2. ✅ `cargo clippy` düzeltmeleri - TAMAMLANDI
3. ✅ `check-try-runtime` komutları - TAMAMLANDI (subxt native feature propagation eklendi)
4. ✅ `cargo-check-all-benches` - TAMAMLANDI (--features runtime-benchmarks eklendi)
5. ✅ Build-publish-images temel binary'ler - TAMAMLANDI
   - ✅ pezkuwi
   - ✅ pezkuwi-teyrchain-bin
   - ✅ pezstaging-node-cli
   - ✅ teyrchain-template-node
   - ✅ pezkuwi-test-malus
   - ✅ test-teyrchain-adder-collator
   - ✅ test-teyrchain-undying-collator
6. ✅ Template repo'ları - MEVCUT (GitHub'da kontrol edildi)
7. ✅ Kalan binary build'leri - TAMAMLANDI
8. Commit ve push yap
9. CI'da test et

---

*Son Güncelleme: 2025-12-25*
