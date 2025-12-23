# Rebrand İlerleme Listesi

**Son Güncelleme:** 2024-12-23
**Toplam Crate:** 76 (REBRAND_MAP'ten)
**Rebrand Durumu:** 75/76 tamamlandı (98.7%)
**Derleme Durumu:** ✅ WORKSPACE TAM DERLENİYOR

## Durum Açıklamaları
- ⏳ Bekliyor
- ✅ Rebrand Tamamlandı (Cargo.toml name doğru)
- 🔨 Derleme Hatası (rebrand tamam ama compile hatası var)
- ❌ Eksik/Bulunamadı

---

## Özet

| Kategori | Toplam | Tamamlanan | Durum |
|----------|--------|------------|-------|
| pez-* prefix | 29 | 29 | ✅ |
| pezbridge-* | 8 | 8 | ✅ |
| pezsnowbridge-* | 25 | 24 | ⚠️ 1 eksik |
| xcm-pez-* | 8 | 8 | ✅ |
| pezmmr-* | 2 | 2 | ✅ |
| bp/pezbp-* | 2 | 2 | ✅ |
| peztest-* | 1 | 1 | ✅ |
| asset-test-pezutils | 1 | 1 | ✅ |
| **TOPLAM** | **76** | **75** | **98.7%** |

---

## İlerleme Tablosu

| # | Yeni İsim | Durum | Konum |
|---|-----------|-------|-------|
| 1 | asset-test-pezutils | ✅ | pezcumulus/teyrchains/runtimes/assets/test-utils |
| 2 | pez-chain-spec-guide-runtime | ✅ | docs/sdk/src/reference_docs/chain_spec_runtime |
| 3 | pez-equivocation-detector | ✅ | pezbridges/relays/equivocation |
| 4 | pez-erasure-coding-fuzzer | ✅ | pezkuwi/erasure-coding/fuzzer |
| 5 | pez-ethereum-standards | ✅ | bizinikiwi/primitives/pez-ethereum-standards |
| 6 | pez-finality-relay | ✅ | pezbridges/relays/finality |
| 7 | pez-fork-tree | ✅ | bizinikiwi/utils/pez-fork-tree |
| 8 | pez-generate-bags | ✅ | bizinikiwi/utils/pezframe/pez-generate-bags |
| 9 | pez-kitchensink-runtime | ✅ | bizinikiwi/bin/node/runtime |
| 10 | pez-messages-relay | ✅ | pezbridges/relays/messages |
| 11 | pez-minimal-template-node | ✅ | templates/minimal/node |
| 12 | pez-minimal-template-runtime | ✅ | templates/minimal/runtime |
| 13 | pez-node-bench | ✅ | bizinikiwi/bin/node/bench |
| 14 | pez-node-primitives | ✅ | bizinikiwi/bin/node/primitives |
| 15 | pez-node-rpc | ✅ | bizinikiwi/bin/node/rpc |
| 16 | node-runtime-pez-generate-bags | ✅ | bizinikiwi/utils/pezframe/pez-generate-bags/node-runtime |
| 17 | pez-node-template-release | ✅ | bizinikiwi/scripts/ci/pez-node-template-release |
| 18 | pez-node-testing | ✅ | bizinikiwi/bin/node/testing |
| 19 | pez-penpal-emulated-chain | ✅ | pezcumulus/teyrchains/integration-tests/emulated/chains/teyrchains/testing/penpal |
| 20 | pez-penpal-runtime | ✅ | pezcumulus/teyrchains/runtimes/testing/penpal |
| 21 | pez-remote-ext-tests-bags-list | ✅ | pezkuwi/utils/remote-ext-tests/bags-list |
| 22 | pez-revive-dev-node | ✅ | bizinikiwi/pezframe/revive/dev-node/node |
| 23 | pez-revive-dev-runtime | ✅ | bizinikiwi/pezframe/revive/dev-node/runtime |
| 24 | pez-slot-range-helper | ✅ | pezkuwi/runtime/common/pez-slot-range-helper |
| 25 | pez-solochain-template-node | ✅ | templates/solochain/node |
| 26 | pez-solochain-template-runtime | ✅ | templates/solochain/runtime |
| 27 | pez-subkey | ✅ | bizinikiwi/bin/utils/pez-subkey |
| 28 | pez-template-zombienet-tests | ✅ | templates/zombienet |
| 29 | peztest-runtime-constants | ✅ | pezkuwi/runtime/test-runtime/constants |
| 30 | pez-tracing-gum | ✅ | pezkuwi/node/gum |
| 31 | pez-tracing-gum-proc-macro | ✅ | pezkuwi/node/gum/proc-macro |
| 32 | bp-header-pez-chain | ✅ | pezbridges/primitives/header-chain |
| 33 | pezbp-runtime | ✅ | pezbridges/primitives/runtime |
| 34 | pezbridge-hub-pezkuwichain-emulated-chain | ✅ | pezcumulus/teyrchains/integration-tests/emulated/chains/teyrchains/pezbridges/bridge-hub-pezkuwichain |
| 35 | pezbridge-hub-pezkuwichain-integration-tests | ✅ | pezcumulus/teyrchains/integration-tests/emulated/tests/pezbridges/bridge-hub-pezkuwichain |
| 36 | pezbridge-hub-pezkuwichain-runtime | ✅ | pezcumulus/teyrchains/runtimes/bridge-hubs/bridge-hub-pezkuwichain |
| 37 | pezbridge-hub-test-utils | ✅ | pezcumulus/teyrchains/runtimes/bridge-hubs/test-utils |
| 38 | pezbridge-hub-zagros-emulated-chain | ✅ | pezcumulus/teyrchains/integration-tests/emulated/chains/teyrchains/pezbridges/bridge-hub-zagros |
| 39 | pezbridge-hub-zagros-integration-tests | ✅ | pezcumulus/teyrchains/integration-tests/emulated/tests/pezbridges/bridge-hub-zagros |
| 40 | pezbridge-hub-zagros-runtime | ✅ | pezcumulus/teyrchains/runtimes/bridge-hubs/bridge-hub-zagros |
| 41 | pezbridge-runtime-common | ✅ | pezbridges/bin/runtime-common |
| 42 | pezmmr-gadget | ✅ | bizinikiwi/client/merkle-mountain-range |
| 43 | pezmmr-rpc | ✅ | bizinikiwi/client/merkle-mountain-range/rpc |
| 44 | pezsnowbridge-beacon-primitives | ✅ | pezbridges/snowbridge/primitives/beacon |
| 45 | pezsnowbridge-core | ✅ | pezbridges/snowbridge/primitives/core |
| 46 | pezsnowbridge-ethereum | ✅ | pezbridges/snowbridge/primitives/ethereum |
| 47 | pezsnowbridge-inbound-queue-primitives | ✅ | pezbridges/snowbridge/primitives/inbound-queue |
| 48 | pezsnowbridge-merkle-tree | ✅ | pezbridges/snowbridge/primitives/merkle-tree |
| 49 | pezsnowbridge-outbound-queue-primitives | ✅ | pezbridges/snowbridge/primitives/outbound-queue |
| 50 | pezsnowbridge-outbound-queue-runtime-api | ✅ | pezbridges/snowbridge/pezpallets/outbound-queue/runtime-api |
| 51 | pezsnowbridge-outbound-queue-v2-runtime-api | ✅ | pezbridges/snowbridge/pezpallets/outbound-queue-v2/runtime-api |
| 52 | pezsnowbridge-pezpallet-ethereum-client | ✅ | pezbridges/snowbridge/pezpallets/ethereum-client |
| 53 | pezsnowbridge-pezpallet-ethereum-client-fixtures | ✅ | pezbridges/snowbridge/pezpallets/ethereum-client/fixtures |
| 54 | pezsnowbridge-pezpallet-inbound-queue | ✅ | pezbridges/snowbridge/pezpallets/inbound-queue |
| 55 | pezsnowbridge-pezpallet-inbound-queue-fixtures | ✅ | pezbridges/snowbridge/pezpallets/inbound-queue/fixtures |
| 56 | pezsnowbridge-pezpallet-inbound-queue-v2 | ✅ | pezbridges/snowbridge/pezpallets/inbound-queue-v2 |
| 57 | pezsnowbridge-pezpallet-inbound-queue-v2-fixtures | ✅ | pezbridges/snowbridge/pezpallets/inbound-queue-v2/fixtures |
| 58 | pezsnowbridge-pezpallet-outbound-queue | ✅ | pezbridges/snowbridge/pezpallets/outbound-queue |
| 59 | pezsnowbridge-pezpallet-outbound-queue-v2 | ✅ | pezbridges/snowbridge/pezpallets/outbound-queue-v2 |
| 60 | pezsnowbridge-pezpallet-system | ✅ | pezbridges/snowbridge/pezpallets/system |
| 61 | pezsnowbridge-pezpallet-system-frontend | ✅ | pezbridges/snowbridge/pezpallets/system-frontend |
| 62 | pezsnowbridge-pezpallet-system-v2 | ✅ | pezbridges/snowbridge/pezpallets/system-v2 |
| 63 | pezsnowbridge-runtime-common | ❌ | Mevcut değil veya farklı isimle |
| 64 | pezsnowbridge-runtime-test-common | ✅ | pezbridges/snowbridge/runtime/test-common |
| 65 | pezsnowbridge-system-runtime-api | ✅ | pezbridges/snowbridge/pezpallets/system/runtime-api |
| 66 | pezsnowbridge-system-v2-runtime-api | ✅ | pezbridges/snowbridge/pezpallets/system-v2/runtime-api |
| 67 | pezsnowbridge-test-utils | ✅ | pezbridges/snowbridge/test-utils |
| 68 | pezsnowbridge-verification-primitives | ✅ | pezbridges/snowbridge/primitives/verification |
| 69 | xcm-pez-docs | ✅ | pezkuwi/xcm/docs |
| 70 | xcm-pez-emulator | ✅ | pezcumulus/xcm/xcm-pez-emulator |
| 71 | xcm-pez-executor-integration-tests | ✅ | pezkuwi/xcm/xcm-executor/integration-tests |
| 72 | xcm-pez-procedural | ✅ | pezkuwi/xcm/procedural |
| 73 | xcm-runtime-pezapis | ✅ | pezkuwi/xcm/xcm-runtime-pezapis |
| 74 | xcm-pez-simulator | ✅ | pezkuwi/xcm/xcm-pez-simulator |
| 75 | xcm-pez-simulator-example | ✅ | pezkuwi/xcm/xcm-pez-simulator/example |
| 76 | xcm-pez-simulator-fuzzer | ✅ | pezkuwi/xcm/xcm-pez-simulator/fuzzer |

---

## Bilinen Derleme Sorunları

~~Bu crate'lerin rebrand'i tamamlandı ancak derleme hataları var:~~

1. ~~**pez-kitchensink-runtime** - 622 hata~~ ✅ ÇÖZÜLDÜ (derleniyor)
2. ~~**pezkuwi-sdk (umbrella)** - pezframe_benchmarking_pezpallet_pov import düzeltildi~~ ✅ ÇÖZÜLDÜ

**Tüm kritik derleme sorunları çözüldü!**

## ✅ Derleme Tamamlanan Özel Crate'ler

1. **pezpallet-revive-eth-rpc** ✅ (2025-12-19)
   - pezkuwi-subxt fork ile tam uyumlu hale getirildi
   - Tüm substitute_type path'ler pezsp_runtime/pezpallet_revive olarak güncellendi
   - StorageApi → StorageClientAt API güncellemesi yapıldı
   - RuntimeApiError pattern matching düzeltildi
   - SQLX cache yeniden oluşturuldu (eth_to_bizinikiwi_blocks)

2. **bizinikiwi-txtesttool** ✅ (2025-12-19)
   - subxt 0.44 dynamic storage API güncellemesi (try_fetch, Value type)
   - From<ExtrinsicError> for Error eklendi

3. **pezkuwi-omni-node-lib** ✅ (2025-12-19)
   - StorageEntryType → keys()/value_ty() API güncellemesi

### Bu Oturumda Düzeltilen Sorunlar:

1. `pezpallet-contracts-fixtures/build/Cargo.toml` - workspace inheritance kaldırıldı
2. `pezpallet-revive-fixtures/build.rs` - panic_immediate_abort yeni syntax'a güncellendi
3. `asset-hub-zagros pezpezsnowbridge` → `pezsnowbridge` double prefix düzeltildi
4. `bridge-hub-pezkuwichain/weights` - snowbridge_pezpallet → pezsnowbridge_pezpallet dosya rename
5. `bridge-hub-zagros/weights` - aynı rename
6. `umbrella/src/lib.rs` - pezframe_benchmarking_pallet_pov → pezframe_benchmarking_pezpallet_pov

---

## Sonraki Adımlar

1. [x] Derleme hatalarını düzelt (pez-kitchensink-runtime) ✅ (derleniyor)
2. [x] pezpallet-revive-eth-rpc derlenmesi ✅ (2025-12-19)
3. [x] **Workspace-wide cargo check** ✅ (2025-12-19) - BAŞARILI!
4. [ ] **Crates.io publish planına geç** ← SIRADAKİ
5. [ ] CI/CD workflow'larını güncelle

---

## Log

### 2024-12-23
- **Umbrella eksik crate düzeltmeleri:**
  - `pezpallet-root-testing` umbrella'ya eklendi (std, try-runtime, runtime-full features)
  - `pezpallet-xcm-benchmarks` umbrella'ya eklendi (std, runtime-benchmarks, runtime-full features)
  - Her iki crate için lib.rs re-export eklendi
- **pez-kitchensink-runtime** artık tam olarak derleniyor
- **cargo clippy -p pez-kitchensink-runtime -- -D warnings** geçti

### 2025-12-19
- **WORKSPACE CARGO CHECK BAŞARILI!** ✅
- **pezpallet-revive-eth-rpc** başarıyla derlendi
- **pez-kitchensink-runtime** derleme doğrulandı ✅ (622 hata iddiası yanlıştı)
- vendor/pezkuwi-subxt fork'u oluşturuldu ve workspace'den exclude edildi
- subxt 0.44.0 API değişiklikleri uygulandı:
  - StorageClientAt, try_fetch, RuntimeApiError (revive-eth-rpc)
  - dynamic storage API (txtesttool)
  - StorageEntryType → keys()/value_ty() (omni-node-lib)
- Tüm substitute_type path'leri pezsp_runtime/pezpallet_revive prefix'leriyle güncellendi
- SQLX cache yeniden oluşturuldu (bizinikiwi terminolojisi ile)
- kurdistan-sdk dizini düzenlendi (gereksiz scriptler archive/ klasörüne taşındı)

### 2025-12-16
- Rebrand durumu tam olarak değerlendirildi
- 75/76 crate rebrand tamamlanmış bulundu
- pezsnowbridge-runtime-common mevcut değil (muhtemelen merge edilmiş)
- REBRAND_PROGRESS.md gerçek durumu yansıtacak şekilde güncellendi
- Birkaç derleme hatası düzeltildi (fixtures build scripts, weight file renames)

### 2025-12-15
- İlerleme listesi oluşturuldu
- scan_old_words.py scripti hazırlandı
