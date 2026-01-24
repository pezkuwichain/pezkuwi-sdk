# PUBLIC TESTNET ROADMAP - Staged Approach

**Oluşturulma:** 2026-01-24
**Son Güncelleme:** 2026-01-24
**Durum:** STAGE 1 - Internal Testnet (BAŞLAMADI)

---

## 🎯 HEDEF

21 validator + 4 collator ile public testnet başlatmak.

**Yaklaşım:** Staged Approach
1. STAGE 1: Internal Testnet (bizim VPS'ler)
2. STAGE 2: Public RPC
3. STAGE 3: Community Validators

---

## 📚 ÖNCEKİ OTURUMLARDAN ÖĞRENMELER (KRİTİK!)

### 1. Ed25519 vs Sr25519 Key Scheme Sorunu (ÇÖZÜLDÜ ✅)

**Problem:** `asset-hub-pezkuwichain-local` yanlışlıkla Ed25519 kullanıyordu.

**Kök Neden:** Prefix matching hatası:
```rust
// HATALI: "asset-hub-pezkuwichain".starts_with("asset-hub-pezkuwi") = TRUE!
if id.starts_with("asset-hub-pezkuwi") { Ed25519 }
else if id.starts_with("asset-hub-pezkuwichain") { Sr25519 }  // Hiç çalışmıyordu!
```

**Çözüm:** Uzun prefix'i ÖNCE kontrol et:
```rust
// DOĞRU: Önce uzun prefix
if id.starts_with("asset-hub-pezkuwichain") { Sr25519 }
else if id.starts_with("asset-hub-pezkuwi") { Ed25519 }
```

**Düzeltilen Dosyalar:**
- `pezcumulus/pezkuwi-teyrchain/src/chain_spec/mod.rs` - RuntimeResolver
- `vendor/pezkuwi-zombienet-sdk/crates/orchestrator/src/generators/chain_spec.rs`
- `vendor/pezkuwi-zombienet-sdk/crates/orchestrator/src/spawner.rs`

**Commit:** `f52eb30abb` - "fix: correct Ed25519/Sr25519 key scheme detection for Asset Hub"

### 2. Zombienet Metric Timeout Sorunu

**Problem:** Network 80 saniye sonra kapanıyor.
```
🧟 One of the nodes crashed: alice: Timeout (5), waiting for metric process_start_time_seconds
```

**Sebep:** Zombienet health check timeout'u - node'lardan metric alamıyor.

**ÖNEMLİ:** Bu bir network/consensus sorunu DEĞİL! Network çalışıyor, sadece monitoring timeout yapıyor.

**Çözüm Seçenekleri:**
1. Zombienet timeout değerlerini artır
2. Systemd service ile deployment (zombienet yerine)
3. Metric endpoint'lerini düzelt

### 3. VPS Build Sorunları

**Problem:** VPS2'de `serde_core` wasm build hatası.
```
error[E0152]: duplicate lang item in crate `alloc`: `exchange_malloc`
```

**Geçici Çözüm:** Local'de build edip binary'leri kopyala.
```bash
scp /home/mamostehp/pezkuwi-sdk/target/release/pezkuwi* root@VPS_IP:/root/pezkuwi-sdk/target/release/
```

**Kalıcı Çözüm:** Rust toolchain versiyonlarını senkronize et.

### 4. Config Path Sorunu

**Problem:** `zombienet-local-21.toml` içinde hardcoded path'ler var.
```toml
# YANLIŞ (local machine):
default_command = "/home/mamostehp/pezkuwi-sdk/target/release/pezkuwi"

# DOĞRU (VPS):
default_command = "/root/pezkuwi-sdk/target/release/pezkuwi"
```

**Çözüm:** Her VPS için path'leri düzelt veya relative path kullan.

---

## 🖥️ VPS KAPASİTELERİ

| | VPS1 | VPS2 |
|---|------|------|
| **IP** | 37.60.230.9 | 62.146.235.186 |
| **Hostname** | - | merkez-server |
| **CPU** | 8 core | 16 core |
| **RAM** | 23 GB | 62 GB |
| **Disk** | 193 GB (115 GB boş) | 581 GB (520 GB boş) |
| **SSH** | `ssh root@37.60.230.9` | `ssh root@62.146.235.186` |

### Önerilen Node Dağılımı

| VPS | Validators | Asset Hub Collators | People Chain Collators |
|-----|------------|---------------------|------------------------|
| VPS1 | 7 | 1 | 1 |
| VPS2 | 14 | 1 | 1 |
| **TOPLAM** | **21** | **2** | **2** |

---

## 📋 STAGE 1: Internal Testnet

### Ön Koşullar
- [x] Ed25519/Sr25519 fix commit edildi
- [x] 21 validator ile local test geçti
- [x] VPS kapasiteleri kontrol edildi
- [ ] Binary'ler VPS'lere deploy edildi

### 1.1 Validator Key Oluşturma
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] 21 validator için Sr25519 session key oluştur
- [ ] Key'leri güvenli şekilde sakla
- [ ] Her validator için ayrı keystore dizini

**Komutlar:**
```bash
# Session key oluştur
./target/release/pezkuwi key generate --scheme Sr25519 --output-type json

# Veya subkey ile
./target/release/pez-subkey generate --scheme Sr25519
```

**Dosyalar:**
- `mainnet/keys/validators-21-session-keys.json` (GİTİGNORE'DA!)

### 1.2 Chain Spec Oluşturma
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] Testnet chain spec adı belirle (`zagros` veya `pezkuwichain-testnet`)
- [ ] Genesis config'i düzenle (validator listesi)
- [ ] Raw chain spec oluştur

**Komutlar:**
```bash
# Plain chain spec
./target/release/pezkuwi build-spec --chain pezkuwichain-local --disable-default-bootnode > testnet-plain.json

# Validator'ları ekle (genesis config düzenle)

# Raw chain spec
./target/release/pezkuwi build-spec --chain testnet-plain.json --raw > testnet-raw.json
```

### 1.3 Systemd Service Dosyaları
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] Validator service template oluştur
- [ ] Collator service template oluştur
- [ ] Her VPS için service dosyalarını deploy et

**Template:** `/etc/systemd/system/pezkuwi-validator@.service`
```ini
[Unit]
Description=Pezkuwi Validator %i
After=network.target

[Service]
Type=simple
User=root
ExecStart=/root/pezkuwi-sdk/target/release/pezkuwi \
    --chain /root/pezkuwi-sdk/testnet-raw.json \
    --name validator-%i \
    --validator \
    --base-path /root/pezkuwi-data/validator-%i \
    --port 3033%i \
    --rpc-port 994%i \
    --prometheus-port 961%i \
    --telemetry-url "wss://telemetry.pezkuwichain.io/submit/ 1"
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### 1.4 VPS Deployment
**Durum:** ❌ BAŞLAMADI

**VPS1 Görevleri:**
- [ ] Binary'leri kopyala
- [ ] Chain spec kopyala
- [ ] Keystore'ları kopyala (validator 1-7)
- [ ] Service dosyalarını kur
- [ ] Validator'ları başlat
- [ ] Collator'ları başlat

**VPS2 Görevleri:**
- [ ] Binary'leri kopyala
- [ ] Chain spec kopyala
- [ ] Keystore'ları kopyala (validator 8-21)
- [ ] Service dosyalarını kur
- [ ] Validator'ları başlat
- [ ] Collator'ları başlat

### 1.5 Bootnode Yapılandırması
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] İlk validator'ü bootnode olarak belirle
- [ ] Bootnode peer ID'sini al
- [ ] Diğer node'lara bootnode bilgisini ekle

**Komut:**
```bash
# Peer ID öğren
./target/release/pezkuwi key inspect-node-key --file /root/pezkuwi-data/validator-1/chains/*/network/secret_ed25519
```

### 1.6 Network Test (24 Saat)
**Durum:** ❌ BAŞLAMADI

**Kontrol Listesi:**
- [ ] Tüm validator'ler peer'larını buluyor
- [ ] Block üretimi devam ediyor
- [ ] Finality çalışıyor (GRANDPA)
- [ ] Teyrchain'ler block üretiyor
- [ ] Hiçbir node crash olmuyor
- [ ] Memory leak yok
- [ ] Disk dolmuyor

---

## 📋 STAGE 2: Public RPC

### 2.1 RPC Node Kurulumu
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] Validator OLMAYAN RPC node kur
- [ ] `--rpc-external` flag'i ekle
- [ ] `--rpc-cors all` flag'i ekle
- [ ] Pruning ayarla (archive vs pruned)

### 2.2 Domain ve SSL
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] Domain belirle (rpc.pezkuwichain.io?)
- [ ] DNS ayarla
- [ ] Nginx reverse proxy kur
- [ ] Let's Encrypt SSL sertifikası

### 2.3 Rate Limiting
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] Nginx rate limiting
- [ ] IP başına istek limiti
- [ ] WebSocket bağlantı limiti

---

## 📋 STAGE 3: Community Validators

### 3.1 Dokümantasyon
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] Validator kurulum rehberi
- [ ] Donanım gereksinimleri
- [ ] Session key oluşturma
- [ ] Staking süreci

### 3.2 Telemetry
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] Telemetry backend kur
- [ ] telemetry.pezkuwichain.io domain
- [ ] Dashboard

### 3.3 Block Explorer
**Durum:** ❌ BAŞLAMADI

**Seçenekler:**
- Subscan self-hosted
- Polkadot.js Apps
- Custom explorer

### 3.4 Faucet
**Durum:** ❌ BAŞLAMADI

**Görevler:**
- [ ] Faucet backend
- [ ] Rate limiting (IP/hesap başına)
- [ ] Captcha

---

## 🔧 YARARLI KOMUTLAR

### Binary'leri VPS'e Kopyala
```bash
# Tüm binary'ler
scp /home/mamostehp/pezkuwi-sdk/target/release/pezkuwi \
    /home/mamostehp/pezkuwi-sdk/target/release/pezkuwi-teyrchain \
    /home/mamostehp/pezkuwi-sdk/target/release/pezkuwi-zombienet \
    root@VPS_IP:/root/pezkuwi-sdk/target/release/
```

### VPS'te Repo Sync
```bash
ssh root@VPS_IP "cd /root/pezkuwi-sdk && git fetch origin && git reset --hard origin/main"
```

### Process Kontrol
```bash
# Çalışan node'ları gör
ssh root@VPS_IP "pgrep -af pezkuwi"

# Hepsini durdur
ssh root@VPS_IP "pkill -9 -f pezkuwi"
```

### Log Kontrol
```bash
# Systemd log
ssh root@VPS_IP "journalctl -u pezkuwi-validator@1 -f"

# Zombienet log
ssh root@VPS_IP "tail -f /tmp/zombienet-21.log"
```

### Block Numarası Kontrol
```bash
curl -s -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
  http://VPS_IP:9944 | jq '.result.number'
```

---

## ⚠️ BİLİNEN SORUNLAR VE ÇÖZÜMLER

### 1. "CannotSign" Hatası
**Sebep:** Key scheme mismatch (Ed25519 vs Sr25519)
**Çözüm:** Fix zaten uygulandı (commit f52eb30abb)

### 2. "WASM binary was not built"
**Sebep:** SKIP_WASM_BUILD=1 ile build edilmiş
**Çözüm:** WASM ile build et veya local'den kopyala

### 3. Metric Timeout (Zombienet)
**Sebep:** Node metric endpoint'e geç cevap veriyor
**Çözüm:** Systemd ile deployment (zombienet bypass)

### 4. Path Hatası (VPS)
**Sebep:** Config'de `/home/mamostehp/...` path'leri var
**Çözüm:** `/root/pezkuwi-sdk/...` olarak değiştir

---

## 📅 ZAMAN ÇİZELGESİ (TAHMİNİ)

| Stage | Süre | Bitiş |
|-------|------|-------|
| STAGE 1.1-1.3 (Hazırlık) | 1-2 gün | - |
| STAGE 1.4-1.5 (Deployment) | 1 gün | - |
| STAGE 1.6 (24h Test) | 1-2 gün | - |
| STAGE 2 (Public RPC) | 1-2 gün | - |
| STAGE 3 (Community) | 1 hafta+ | - |

---

## 📝 DEĞİŞİKLİK GEÇMİŞİ

| Tarih | Değişiklik |
|-------|------------|
| 2026-01-24 | İlk versiyon oluşturuldu |
| 2026-01-24 | VPS kapasiteleri eklendi |
| 2026-01-24 | Ed25519/Sr25519 fix dokümante edildi |

---

*Bu dosya SessionStart hook ile otomatik yüklenir. Her stage tamamlandığında güncelleyin!*
