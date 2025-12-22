# Pezkuwi SDK - CI Workflow Düzeltme Planı

> Bu dosya Claude oturumları arasında sürekliliği sağlamak için oluşturulmuştur.
> Her oturum başında bu dosyayı oku ve kaldığın yerden devam et.

## Son Güncelleme
- **Tarih:** 2024-12-22
- **Son Tamamlanan:** cargo clippy düzeltildi
- **Aktif Görev:** FAZ 1 - CI Yeşil Işık

---

## DURUM TABLOSU

### checks.yml (3 job)
| Job | Durum | Komut | Notlar |
|-----|-------|-------|--------|
| cargo-clippy | ✅ TAMAMLANDI | `RUSTFLAGS="-D warnings" SKIP_WASM_BUILD=1 cargo clippy --all-targets --all-features --locked --workspace --quiet` | 5 dosya düzeltildi |
| check-try-runtime | ⏳ BEKLEMEDE | Workflow'dan kontrol et | |
| check-core-crypto-features | ⏳ BEKLEMEDE | `check-features-variants.sh` | |

### checks-quick.yml (10 job)
| Job | Durum | Komut | Notlar |
|-----|-------|-------|--------|
| fmt | ⏳ BEKLEMEDE | `cargo fmt --all -- --check` | |
| check-toml-format | ⏳ BEKLEMEDE | `taplo format --check --config .config/taplo.toml` | |
| check-zepter | ⏳ BEKLEMEDE | `zepter run check` | Önce: `cargo install zepter@1.82.1` |
| check-workspace | ⏳ BEKLEMEDE | `python3 .github/scripts/check-workspace.py .` | |
| check-umbrella | ⏳ BEKLEMEDE | `python3 scripts/generate-umbrella.py` | |
| check-dependency-rules | ⏳ BEKLEMEDE | `.gitlab/ensure-deps.sh` | |
| test-rust-features | ⏳ BEKLEMEDE | `.gitlab/rust-features.sh .` | |
| check-markdown | ⏳ BEKLEMEDE | `markdownlint --config .github/.markdownlint.yaml --ignore target .` | |
| check-fail-ci | ⏳ BEKLEMEDE | `rg "FAIL-CI" --type rust` | Basit, muhtemelen OK |
| check-readme | ⏳ BEKLEMEDE | `.github/scripts/check-missing-readme-generation.sh` | |

### check-links.yml (1 job)
| Job | Durum | Komut | Notlar |
|-----|-------|-------|--------|
| link-checker | ⏳ BEKLEMEDE | `lychee --config .config/lychee.toml './**/*.rs'` | 0 hata bekleniyor |

### tests-misc.yml (11 job)
| Job | Durum | Notlar |
|-----|-------|--------|
| test-full-crypto-feature | ⏳ BEKLEMEDE | |
| test-pezframe-examples-compile-to-wasm | ⏳ BEKLEMEDE | |
| test-pezframe-ui | ⏳ BEKLEMEDE | |
| test-deterministic-wasm | ⏳ BEKLEMEDE | |
| cargo-check-benches | ⏳ BEKLEMEDE | |
| test-node-metrics | ⏳ BEKLEMEDE | |
| check-tracing | ⏳ BEKLEMEDE | |
| check-metadata-hash | ⏳ BEKLEMEDE | |
| cargo-check-each-crate | ⏳ BEKLEMEDE | |
| cargo-check-all-crate-macos | ⏳ BEKLEMEDE | macOS gerekli |

### build-misc.yml (3 job)
| Job | Durum | Notlar |
|-----|-------|--------|
| build-runtimes-polkavm | ⏳ BEKLEMEDE | |
| check-revive-stable-uapi-polkavm | ⏳ BEKLEMEDE | |
| build-pez-subkey | ⏳ BEKLEMEDE | |

---

## ÇALIŞMA SİSTEMİ

### Her Workflow Job İçin:
1. **Test Et:** Komutu local'de çalıştır
2. **Hata Analizi:** Çıktıyı analiz et
3. **Düzelt:** Hataları rebrand kurallarına uygun düzelt
4. **Tekrar Test:** Düzeltme sonrası tekrar çalıştır
5. **İşaretle:** Bu dosyada durumu güncelle

### Düzeltme Kuralları (CLAUDE_RULES.md'den):
- ASLA rebrand'i geri alma
- Terminoloji koruması:
  - polkadot → pezkuwi
  - substrate → bizinikiwi
  - cumulus → pezcumulus
  - westend → zagros
  - rococo → pezkuwichain
  - parachain → teyrchain

---

## TAMAMLANAN DÜZELTMELER

### 2024-12-22: Clippy Düzeltmeleri

**Dosya 1:** `pezcumulus/pezkuwi-omni-node/tests/extra_subcommand_parsing_test.rs`
- Sorun: deprecated `cargo_bin` function
- Çözüm: `#![allow(deprecated)]` + `assert_cmd::Command` kullanımı

**Dosya 2-27:** `**/tests/*.rs` (27 dosya)
- Sorun: deprecated `cargo_bin` import
- Çözüm: Tüm dosyalara `#![allow(deprecated)]` eklendi

**Dosya 28:** `pezkuwi/zombienet-sdk-tests/tests/smoke/coretime_revenue.rs`
- Sorun: `uninlined_format_args`
- Çözüm: `log::info!("{:?}", sale)` → `log::info!("{sale:?}")`

**Dosya 29-31:** `pezkuwi/zombienet-sdk-tests/tests/teyrchains/weights.rs`
- Sorun: `uninlined_format_args` (3 yer)
- Çözüm: Format string'leri inline edildi

**Dosya 32:** `bizinikiwi/pezframe/revive/rpc/src/tests.rs`
- Sorun: Subxt API değişiklikleri (fetch signature, StorageValue)
- Çözüm: `fetch(&query, ())` ve `.decode()?` eklendi

**Dosya 33:** `pezkuwi/pezpallets/validator-pool/src/mock.rs`
- Sorun: `advance_era` unused
- Çözüm: `#[allow(dead_code)]` eklendi

**Dosya 34:** `pezkuwi/pezpallets/validator-pool/src/tests.rs`
- Sorun: `len() > 0` → `!is_empty()`
- Çözüm: Clippy önerisine uyuldu

**Dosya 35:** `pezcumulus/teyrchains/pezpallets/identity-kyc/src/mock.rs`
- Sorun: `new_test_ext_empty` unused
- Çözüm: `#[allow(dead_code)]` eklendi

**Dosya 36:** `bizinikiwi/pezframe/examples/tasks/src/tests.rs`
- Sorun: Field name `i` → `_i`
- Çözüm: `AddNumberIntoTotal { i:` → `AddNumberIntoTotal { _i:`

---

## SONRAKİ ADIMLAR

1. **cargo fmt --all -- --check** çalıştır
2. Hataları düzelt (varsa)
3. Bu dosyayı güncelle
4. Sonraki job'a geç

---

## KRİTİK DOSYA YOLLARI

```
/home/mamostehp/pezkuwi-sdk/
├── .github/workflows/          # Workflow dosyaları
│   ├── checks.yml
│   ├── checks-quick.yml
│   └── check-links.yml
├── .config/
│   ├── lychee.toml            # Link checker config
│   └── taplo.toml             # TOML formatter config
├── .claude/
│   ├── CLAUDE_RULES.md        # Rebrand kuralları
│   └── TERMINOLOGY.md         # Terminoloji mapping
├── scripts/
│   └── generate-umbrella.py   # Umbrella crate generator
├── .gitlab/
│   ├── ensure-deps.sh         # Dependency rules
│   └── rust-features.sh       # Feature testing
└── crate_placeholders/        # 150 placeholder crate
```

---

## NOTLAR

- Her büyük değişiklikten önce git commit yap
- Hata düzeltirken rebrand'i bozma
- Çıktıları takip et, 100+ hata varsa kategorize et
