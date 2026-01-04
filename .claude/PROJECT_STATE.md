# PEZKUWI SDK - PROJECT STATE

**Son Güncelleme:** 2026-01-02
**Bu dosya SessionStart hook tarafından otomatik okunur**

---

## PROJE NEDİR

Pezkuwi SDK, Polkadot SDK'nın tamamen rebrand edilmiş BAĞIMSIZ bir fork'udur.
- Polkadot ile artık HİÇBİR ilişkisi yoktur
- Tüm crate'ler crates.io'da yayınlanmıştır (~516 crate)
- Kendi token ekonomisi vardır (HEZ + PEZ)

---

## KRİTİK KURALLAR - MUTLAKA OKU

1. **ASLA Polkadot.js API önerme** - Rebrand nedeniyle çalışmıyor
2. **ASLA rebrand'i geri alma** - Terminoloji kalıcı
3. **ASLA geçici çözüm önerme** - Kalıcı çözümler bul
4. **ÖNCE mevcut araçları kontrol et** - Aşağıdaki listeye bak

---

## MEVCUT ARAÇLAR (ZATEN MEVCUT - YENİDEN YAZMA)

### 1. pezkuwi-subxt (Token Transfer, RPC)
```
Konum: /home/mamostehp/pezkuwi-sdk/vendor/pezkuwi-subxt/
Durum: TAMAMLANDI - Kullanıma hazır
Kullanım: Token transfer, chain query, extrinsic submit
```
**Polkadot.js yerine BUNU KULLAN!**

### 2. pezkuwi-zombienet-sdk (Network Orchestration)
```
Konum: /home/mamostehp/pezkuwi-sdk/vendor/pezkuwi-zombienet-sdk/
Binary: /home/mamostehp/pezkuwi-sdk/target/release/pezkuwi-zombienet
Durum: TAMAMLANDI - ParaGenesisArgs fix uygulandı
```

### 3. pezkuwi (Relay Chain Node)
```
Binary: /home/mamostehp/pezkuwi-sdk/target/release/pezkuwi
Durum: TAMAMLANDI - Build edildi
```

### 4. pezkuwi-teyrchain (Collator)
```
Binary: /home/mamostehp/pezkuwi-sdk/target/release/pezkuwi-teyrchain
Durum: TAMAMLANDI - Build edildi
```

---

## TERMİNOLOJİ (ASLA ESKİ TERİMLERİ KULLANMA)

| ESKİ (KULLANMA) | YENİ (KULLAN) |
|-----------------|---------------|
| parachain | teyrchain |
| Polkadot | Pezkuwi |
| polkadot | pezkuwi |
| Substrate | Bizinikiwi |
| substrate | bizinikiwi |
| Cumulus | Pezcumulus |
| Westend | Zagros |
| Rococo | Pezkuwichain |
| paritytech | pezkuwichain |
| sp- | pezsp- |
| sc- | pezsc- |
| frame- | pezframe- |
| pallet- | pezpallet- |

---

## TOKEN EKONOMİSİ

### HEZ (Relay Chain Native Token)
- Genesis: 200,000,000 HEZ
- Model: DOT benzeri inflationary
- Unit: 1 HEZ = 10^18 TYR

### PEZ (Asset Hub Governance Token)
- Supply: 5,000,000,000 PEZ (sabit)
- Asset ID: 1 (Asset Hub'da)
- wHEZ Asset ID: 2

---

## MAINNET ROADMAP DURUMU

| Faz | Durum |
|-----|-------|
| FAZ 0: Analiz | ✅ TAMAMLANDI |
| FAZ 1: Kod | ✅ TAMAMLANDI |
| FAZ 2: Chain-spec | ✅ TAMAMLANDI |
| FAZ 3: Network Test | 🔄 IN PROGRESS |
| FAZ 4: Mainnet | PENDING |

### FAZ 3 Detay:
- DEV (1 validator): ✅ TAMAMLANDI
- LOCAL (2 validator): ✅ TAMAMLANDI
- ALPHA (4 validator): BEKLEMEDE
- BETA (8 validator): BEKLEMEDE
- STAGING (21 validator): BEKLEMEDE

---

## ÇÖZÜLEN KRİTİK SORUNLAR

### 1. Teyrchain Genesis Registration (2026-01-02)
**Sorun:** Teyrchain block #0'da kalıyordu
**Kök Neden:** ParaGenesisArgs format uyumsuzluğu (array vs object)
**Çözüm:** `vendor/pezkuwi-zombienet-sdk/crates/orchestrator/src/generators/chain_spec.rs` düzeltildi
**Durum:** ÇÖZÜLDÜ

### 2. Polkadot.js API Uyumsuzluğu → ÇÖZÜLDÜ
**Sorun:** "Expected 32 bytes, found 48 bytes" AccountId hatası
**Kök Neden:** Rebrand edilmiş metadata type mismatch
**Çözüm:** pezkuwi-subxt kullanıldı
**Durum:** ✅ ÇÖZÜLDÜ (2026-01-02)

**Token Transfer Test Sonucu:**
```
✓ Connected to Pezkuwichain node
✓ Transaction finalized!
✓ Transfer event: 1,000,000,000,000 TYR (1 HEZ) Alice → Bob
```

**Test Dosyası:** `vendor/pezkuwi-subxt/subxt/examples/tx_pezkuwichain.rs`
**Çalıştırma:** `cargo run --release -p pezkuwi-subxt --example tx_pezkuwichain`

---

## ZOMBIENET CONFIG DOSYALARI

```
/home/mamostehp/pezkuwi-sdk/zombienet-dev.toml   # 1 validator (DEV)
/home/mamostehp/pezkuwi-sdk/zombienet-local.toml # 2 validator (LOCAL)
```

### Spawn Komutu:
```bash
cd /home/mamostehp/pezkuwi-sdk
./target/release/pezkuwi-zombienet spawn -p native zombienet-dev.toml
```

---

## KRİTİK DOSYA KONUMLARI

| Dosya | Konum |
|-------|-------|
| CLAUDE.md | `/home/mamostehp/pezkuwi-sdk/.claude/CLAUDE.md` |
| MAINNET_ROADMAP.md | `/home/mamostehp/pezkuwi-sdk/.claude/MAINNET_ROADMAP.md` |
| TERMINOLOGY.md | `/home/mamostehp/pezkuwi-sdk/.claude/TERMINOLOGY.md` |
| Genesis presets | `pezkuwi/runtime/pezkuwichain/src/genesis_config_presets.rs` |
| Asset Hub presets | `pezcumulus/teyrchains/runtimes/assets/asset-hub-pezkuwichain/src/genesis_config_presets.rs` |

---

## SONRAKİ ADIMLAR (ÖNCELİK SIRASI)

1. **pezkuwi-subxt ile token transfer testi** - API çalışıyor mu doğrula
2. **XCM messaging testi** - Relay ↔ Asset Hub iletişimi
3. **ALPHA network** - 4 validator konfigürasyonu

---

*Bu dosya SessionStart hook tarafından otomatik yüklenir. Güncel tutun!*
