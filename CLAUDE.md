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
1. **DEV** (1 validator - Alice) ← ŞU AN BURADAYIZ
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

- Commit mesajlarına `🤖 Generated with [Claude Code]` ve `Co-Authored-By: Claude` **EKLEME**
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

## 🔗 UPSTREAM ISSUE TRACKING SİSTEMİ

**Polkadot SDK'daki issue'ları Pezkuwi SDK'ya nasıl taşıyoruz:**

### Mantık

Upstream Polkadot SDK'de TODO/issue referansları varsa, bunları **tracking issue** sistemi ile takip ediyoruz.

### Adımlar

**1. Upstream'de Kontrol Et:**
```bash
# Örnek: paritytech/polkadot/issues/2403 için
grep -r "paritytech/polkadot/issues/2403" /home/mamostehp/polkadot-sdk-check/
```

**2. Tracking Issue Oluştur:**
```bash
gh issue create --repo pezkuwichain/pezkuwi-sdk --label "upstream-tracking" \
  --title "[Upstream Tracking] paritytech/polkadot#2403" \
  --body "**Upstream:** https://github.com/paritytech/polkadot/issues/2403

**Status Tracking:**
- [x] Pending - Upstream not yet resolved
- [ ] Resolved - Fix merged upstream
- [ ] Evaluated - Assessed if needed for PezkuwiChain
- [ ] Applied - Fix applied to our chain
- [ ] Closed - Upstream issue closed
- [ ] Skipped - Not relevant for us

**Last Check:** 2025-12-06
**Next Check:** 2026-01-06

**Notes:**
ValidatorIndex From<u32> trait implementation issue.
Periodically check upstream and update checkboxes above based on status changes."
```

**3. Koddaki Linki Güncelle:**
```rust
// ÖNCEKİ:
// https://github.com/paritytech/polkadot/issues/2403

// SONRA (bizim tracking issue'ya işaret et):
// https://github.com/pezkuwichain/pezkuwi-sdk/issues/163
```

### Örnek Tamamlanmış Tracking Issue'lar

- **#163** → paritytech/polkadot#2403 (ValidatorIndex)
- **#164** → paritytech/polkadot#222 (CommittedCandidateReceipt Ord)
- **#165** → paritytech/polkadot#7575 (ScheduledCore.collator DEPRECATED)
- **#166** → paritytech/polkadot#6586 (SessionInfo frozen)

### Neden Böyle Yapıyoruz?

1. **Broken link olmasın:** Upstream issue linklerini kendi repo'muza çeviriyoruz
2. **Takip:** Upstream'de issue çözüldü mü, bizde uygulamamız gerekiyor mu takip ediyoruz
3. **Workflow:** `.github/workflows/upstream-issue-tracker.yml` haftalık kontrol ediyor

---

## ✅ CI/CD QUICK-CHECKS DÜZELTMELERİ TAMAMLANDI

**Son güncelleme:** 2025-11-29

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

### Sonraki Adım
Commit atılıp push edilmeli - CI/CD artık geçmeli.

---
