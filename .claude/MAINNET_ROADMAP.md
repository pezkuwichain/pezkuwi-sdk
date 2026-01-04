# PEZKUWI MAINNET ROADMAP

**Oluşturulma:** 2026-01-01
**Son Güncelleme:** 2026-01-02
**Hedef:** %100 Mainnet Hazırlık
**Tahmini Süre:** Faz bazlı ilerleme

---

## NETWORK PARAMETRELERI

| Parametre | Değer |
|-----------|-------|
| **Validators (Mainnet)** | 21 |
| **System Teyrchains** | Asset Hub + People Chain |
| **Relay Token** | HEZ (DOT ile aynı ekonomi, inflationary) |
| **Asset Hub Token** | PEZ (5B sabit arz, genesis'te mint) |
| **Audit** | Internal review (Polkadot SDK fork) |

---

## FAZ 0: MEVCUT DURUM ANALİZİ ✅ TAMAMLANDI

### Runtime Compile Durumu:
- [x] pezkuwichain-runtime (Relay Chain) ✅
- [x] zagros-runtime (Testnet) ✅
- [x] asset-hub-pezkuwichain-runtime ✅
- [x] people-pezkuwichain-runtime ✅

### Token Economics Durumu:

#### HEZ Token (Relay Chain) ✅ KODLANDI
```
Total Genesis: 200,000,000 HEZ
├── Founder:   10% =  20,000,000 HEZ
├── Presale:   50% = 100,000,000 HEZ
├── Treasury:  20% =  40,000,000 HEZ
└── Airdrop:   20% =  40,000,000 HEZ

Dosya: pezkuwi/runtime/pezkuwichain/src/genesis_config_presets.rs
Compile-time validation: ✅
Real addresses: ✅ (founder_governance.json'dan)
```

#### PEZ Token (Asset Hub) ✅ KODLANDI
```
Total Supply: 5,000,000,000 PEZ
├── Treasury:     20.25% = 1,012,500,000 PEZ
├── Founder:       1.875% =    93,750,000 PEZ
├── Presale:       1.875% =    93,750,000 PEZ
└── Rewards Pool: 76.00% = 3,800,000,000 PEZ (sentetik halving)

Dosya: pezcumulus/teyrchains/runtimes/assets/asset-hub-pezkuwichain/src/genesis_config_presets.rs
Compile-time validation: ✅
wHEZ Asset ID: 2
```

### Genesis Presets Durumu:
- [x] "genesis" - Mainnet preset ✅
- [x] "development" - Dev preset ✅
- [x] "local_testnet" - Local preset ✅
- [x] "pezstaging_testnet" - Staging preset ✅

### Crates.io Yayın Durumu:
- Toplam yayınlanan: ~516 crate ✅
- pezkuwi-sdk umbrella: v0.1.1 ✅
- pezkuwi-zombienet-sdk: v0.44.0 ✅

---

## FAZ 1: KOD TAMAMLAMA ✅ TAMAMLANDI

### 1.1 Compile Hataları ✅
- [x] teyrchain-template-runtime düzeltildi
- [x] Tüm workspace `cargo check` geçiyor
- [x] `cargo clippy --workspace` temiz (-D warnings ile)

### 1.2 Macro ve Supertrait Düzeltmeleri ✅
**Kalıcı çözümler uygulandı (geçici bypass yok):**

- [x] `is_using_frame_crate()` → `pezframe` ve `pezkuwi_sdk` kontrol ediyor
- [x] `get_frame_crate_path()` → `pezframe` crate'i buluyor
- [x] `disable_pezframe_system_supertrait_check` attribute'ları kaldırıldı
- [x] Test path'leri `pezframe::` olarak güncellendi

**Düzeltilen dosyalar:**
- `bizinikiwi/pezframe/support/procedural/tools/src/lib.rs`
- `bizinikiwi/pezframe/support/procedural/src/pezpallet/parse/config.rs`
- `docs/sdk/src/reference_docs/frame_pallet_coupling.rs`

### 1.3 Warning Cleanup ✅
**-D warnings flag ile tüm uyarılar temizlendi:**

| Dosya | Sorun | Kalıcı Çözüm |
|-------|-------|--------------|
| `chain_spec_runtime/runtime.rs` | dead_code Header | `type _Header` |
| `first-runtime/src/lib.rs` | dead_code Header | `type _Header` |
| `shared/record.rs` | unused methods | `#[cfg(feature)]` |
| `shared/mod.rs` | unused imports | `#[cfg(feature)]` |
| `overhead/command.rs` | unused imports | `#[cfg(feature)]` |
| `pez-minimal-template-runtime` | unused alias | import kaldırıldı |
| `yet-another-teyrchain` | shadowing imports | redundant use kaldırıldı |
| `umbrella/src/lib.rs` | undefined feature | referans yorumlandı |

### 1.4 Binary'ler
- [ ] `pezkuwi` (relay chain node) - build & run test
- [ ] `pezkuwi-teyrchain` (collator) - build & run test
- [ ] `chain-spec-builder` - build test

### 1.5 Template Repo'lar
- [ ] pez-solochain-template → crates.io dependency
- [ ] pez-teyrchain-template → crates.io dependency

### Hedef:
- [x] `cargo check --workspace` başarılı ✅
- [x] `cargo clippy --workspace -D warnings` başarılı ✅
- [ ] `cargo build --release` (pending)

---

## FAZ 2: CHAIN-SPEC & GENESIS

### 2.1 Chain Spec Dosyaları
- [ ] pezkuwichain-dev.json (development)
- [ ] pezkuwichain-local.json (local testnet)
- [ ] zagros.json (public testnet)
- [ ] pezkuwichain.json (mainnet)
- [ ] asset-hub-pezkuwichain.json (mainnet)
- [ ] people-pezkuwichain.json (mainnet)

### 2.2 Genesis Konfigürasyonu

#### Relay Chain (HEZ):
```
- Sudo account: [BELIRLENECEK]
- Validators (initial): 21 accounts
- Staking: DOT parametreleri
- Inflation: DOT modeli
```

#### Asset Hub (PEZ):
```
- Total Supply: 5,000,000,000 PEZ
- Presale Allocation: [MIKTAR]
- Founder Allocation: [MIKTAR]
- Team Allocation: [MIKTAR]
- Treasury: [MIKTAR]
- Synthetic Halving: [SCHEDULE]
```

### 2.3 Account Listesi
- [ ] Validator accounts (21 adet)
- [ ] Sudo/Root account
- [ ] Treasury account
- [ ] Presale wallets
- [ ] Founder wallets
- [ ] Team wallets

### Hedef:
- Tüm chain-spec'ler hazır
- Genesis balances finalize

---

## FAZ 3: NETWORK TEST AŞAMALARI

### 3.1 DEV (1 Validator - Alice)
**Başarı Kriterleri:**
- [ ] Relay chain başlar
- [ ] Bloklar üretilir
- [ ] Finalize olur
- [ ] Asset Hub teyrchain başlar
- [ ] People Chain teyrchain başlar
- [ ] Cross-chain mesajlaşma çalışır
- [ ] HEZ transfer çalışır
- [ ] PEZ transfer çalışır

### 3.2 LOCAL (2 Validator - Alice + Bob)
**Başarı Kriterleri:**
- [ ] Peer discovery çalışır
- [ ] Consensus çalışır
- [ ] Teyrchain slot auction çalışır

### 3.3 ALPHA (4 Validator)
**Başarı Kriterleri:**
- [ ] Uzak node'lar bağlanır
- [ ] Telemetry çalışır
- [ ] RPC endpoints aktif

### 3.4 BETA (8 Validator)
**Başarı Kriterleri:**
- [ ] Stress test geçer
- [ ] Explorer entegrasyonu
- [ ] Faucet çalışır
- [ ] Wallet entegrasyonu

### 3.5 STAGING (21 Validator)
**Başarı Kriterleri:**
- [ ] Mainnet konfigürasyonu ile çalışır
- [ ] 7 gün kesintisiz çalışma
- [ ] Runtime upgrade testi
- [ ] Governance testi

### Hedef:
- Her aşama %100 geçmeden sonrakine geçilmez
- Tüm bug'lar kalıcı olarak çözülür

---

## FAZ 4: MAINNET HAZIRLIK & LANÇ

### 4.1 Final Checklist
- [ ] Tüm chain-spec'ler finalize
- [ ] Genesis balances onaylandı
- [ ] Validator operatörleri hazır
- [ ] Monitoring kuruldu
- [ ] Backup/Recovery planı var
- [ ] Incident response planı var

### 4.2 Launch Sequence
1. Genesis block oluştur
2. Validators başlat (koordineli)
3. Teyrchain'leri kaydet
4. Collator'ları başlat
5. Public RPC aç
6. Explorer aktif et
7. Duyuru

### Hedef:
- MAINNET LIVE!

---

## İLERLEME TAKİBİ

| Faz | Durum | İlerleme |
|-----|-------|----------|
| FAZ 0 | ✅ TAMAMLANDI | %100 |
| FAZ 1 | ✅ TAMAMLANDI | %90 (binary build pending) |
| FAZ 2 | PENDING | %0 |
| FAZ 3 | PENDING | %0 |
| FAZ 4 | PENDING | %0 |
| **TOPLAM** | - | **%38** |

---

## DEĞİŞİKLİK GEÇMİŞİ

| Tarih | Değişiklik |
|-------|------------|
| 2026-01-01 | İlk versiyon oluşturuldu |
| 2026-01-02 | FAZ 1 tamamlandı - workspace derlemesi başarılı, warning cleanup yapıldı |

---

*Bu plan canlı döküman olarak güncellenecektir.*
