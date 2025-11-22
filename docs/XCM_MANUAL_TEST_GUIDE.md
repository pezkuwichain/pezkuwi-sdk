# XCM Manuel Test Rehberi - Frontend ile XCM Testleri

**Tarih:** 2025-11-22
**Network:** Local Zombienet (Rococo Local + Pezkuwi Parachain)
**Test Ortamı:** Alfa Local Testing

---

## 🎯 Amaç

Bu rehber, local zombienet üzerinde çalışan XCM network'ünü frontend'den manuel olarak test etmeniz için hazırlanmıştır.

## 🌐 Network Bilgileri

### RPC Endpoints

| Node | Type | WS Endpoint | RPC HTTP | Chain |
|------|------|-------------|----------|-------|
| **Alice** | Relay Validator | `ws://localhost:9933` | `http://localhost:9933` | Rococo Local |
| **Bob** | Relay Validator | `ws://localhost:9934` | `http://localhost:9934` | Rococo Local |
| **Pezkuwi Collator** | Parachain | `ws://localhost:9977` | `http://localhost:9977` | Rococo Parachain Local |

### Polkadot.js Apps Links

**Relay Chain (Alice):**
```
https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9933#/explorer
```

**Parachain (Pezkuwi):**
```
https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9977#/explorer
```

---

## 🔑 Test Accounts

### Alice Account (Varsayılan)
- **SS58 Address:** `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`
- **Seed:** `//Alice`
- **Balance:** 2000000000000 units (2M tokens pre-funded by zombienet)

### Test için Polkadot.js Extension Kurulumu

1. **Polkadot.js Extension İndir:**
   - Chrome: https://chrome.google.com/webstore (search "Polkadot.js extension")
   - Firefox: https://addons.mozilla.org/en-US/firefox/addon/polkadot-js-extension/

2. **Alice Account'u İçe Aktar:**
   - Extension'u aç
   - "+" butonuna tıkla
   - "Import account from pre-existing seed" seç
   - Seed phrase: `//Alice`
   - Account name: "Alice Local Test"
   - Password belirle

---

## 📋 Test Senaryoları

### Test 1: Network Connectivity (Bağlantı Testi)

**Amaç:** Frontend'in hem relay chain'e hem de parachain'e bağlanabildiğini doğrula.

**Adımlar:**

1. **Frontend'i Aç:**
   ```bash
   # Zaten çalışıyor olmalı, değilse:
   cd /home/mamostehp/pwap/web
   npm run dev
   ```

2. **Browser'da Aç:**
   - http://localhost:8081

3. **Network Seçimi:**
   - Settings → Network
   - Custom RPC URL: `ws://localhost:9977` (Pezkuwi Parachain)
   - Kaydet ve bağlan

4. **Doğrulama:**
   - ✅ Wallet bağlanmalı
   - ✅ Block number artmalı
   - ✅ Account balance görünmeli

---

### Test 2: Balance Kontrolü (Frontend + Polkadot.js)

**Amaç:** Alice account'un her iki chain'de de balance'ı olduğunu doğrula.

#### Polkadot.js Apps ile:

1. **Relay Chain'i Kontrol Et:**
   - https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9933#/accounts
   - Alice'in balance'ını gör (2M ROC)

2. **Parachain'i Kontrol Et:**
   - https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9977#/accounts
   - Alice'in parachain balance'ını gör

#### Frontend ile:

1. **Wallet Sayfasına Git:**
   - Dashboard → Wallet
   - Alice account seç

2. **Token Listesi Görüntüle:**
   - HEZ balance
   - PEZ balance (varsa)
   - wUSDT balance (varsa)

3. **Doğrulama:**
   - ✅ Balance'lar doğru görünüyor mu?
   - ✅ Token sembolleri doğru mu?

---

### Test 3: XCM Pallet Varlığı Kontrolü

**Amaç:** Parachain'de XCM palletlerinin kurulu ve çalışır durumda olduğunu doğrula.

#### Polkadot.js Apps ile:

1. **Parachain'e Bağlan:**
   - https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9977#/extrinsics

2. **XCM Palletlerini Kontrol Et:**
   - Developer → Extrinsics
   - `submit the following extrinsic` dropdown'ı aç
   - Şu palletleri ara:
     - ✅ `polkadotXcm` - XCM messaging pallet
     - ✅ `xcmPallet` - Alternative XCM interface
     - ✅ `assets` - Asset management
     - ✅ `foreignAssets` - XCM bridge için foreign assets

3. **Pallet Methods Kontrol:**
   - `polkadotXcm.send()` - XCM mesajı gönderme
   - `polkadotXcm.limitedReserveTransferAssets()` - Reserve transfer
   - `polkadotXcm.limitedTeleportAssets()` - Teleport

4. **Chain State Kontrol:**
   - Developer → Chain state
   - Selected constant: `polkadotXcm`
   - ✅ Pallet version görmeli
   - ✅ Configuration değerleri görmeli

---

### Test 4: Asset Configuration Kontrolü

**Amaç:** wUSDT (Asset ID 1000) ve diğer asset'lerin metadata'sını doğrula.

#### Polkadot.js ile Asset Metadata Kontrolü:

1. **Chain State'e Git:**
   - https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9977#/chainstate

2. **Asset Metadata Sorgula:**
   ```
   selected storage: assets > metadata(u32)
   assetId: 0 (wHEZ için)
   ```
   - ✅ name: "Wrapped HEZ"
   - ✅ symbol: "wHEZ"
   - ✅ decimals: 12

3. **wUSDT Kontrolü:**
   ```
   assetId: 1000
   ```
   - ✅ name: "Wrapped USDT"
   - ✅ symbol: "wUSDT"
   - ✅ decimals: 6 (ÖNEMLI: 12 değil!)

4. **PEZ Kontrolü:**
   ```
   assetId: 1
   ```
   - ✅ name: "Pezkuwi Utility Token"
   - ✅ symbol: "PEZ"
   - ✅ decimals: 12

---

### Test 5: Parachain Registration Kontrolü

**Amaç:** Pezkuwi parachain'in relay chain'e düzgün register olduğunu doğrula.

#### Relay Chain'de Parachain Bilgisi:

1. **Relay Chain'e Bağlan:**
   - https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9933#/parachains

2. **Parachain Listesi:**
   - Network → Parachains
   - ✅ Para ID 2000 görünmeli
   - ✅ Lifecycle: "Parachain" olmalı (OnBoarding değil!)
   - ✅ Head updates almalı (block finalize oluyor mu?)

3. **Chain State ile Detaylı Kontrol:**
   - Developer → Chain state
   - `paras.paraLifecycles(2000)`
   - ✅ Result: "Parachain"

---

### Test 6: Block Production Monitoring

**Amaç:** Her iki chain'in de block ürettiğini canlı olarak gözlemle.

#### Polkadot.js Explorer:

1. **Relay Chain Block Production:**
   - https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9933#/explorer
   - ✅ Her ~6 saniyede yeni block
   - ✅ Block finalize oluyor (✓ checkmark)
   - ✅ Events görünüyor

2. **Parachain Block Production:**
   - https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9977#/explorer
   - ✅ Her ~12 saniyede yeni block
   - ✅ Block'lar finalize oluyor
   - ✅ `parachainSystem.validationFunctionStored` eventi var mı?

---

### Test 7: XCM Version Support Kontrolü

**Amaç:** Parachain'in hangi XCM versiyonunu desteklediğini öğren.

#### Chain Constants:

1. **Polkadot.js'de:**
   - Developer → Constants
   - `polkadotXcm.safeXcmVersion()`
   - ✅ Expected: `3` (XCM v3) veya `4` (XCM v4)

2. **Runtime Version:**
   - Developer → RPC calls
   - `state.getRuntimeVersion()`
   - ✅ specVersion'ı not et

---

### Test 8: Simple XCM Message Attempt (Advanced)

**Amaç:** Basit bir XCM mesajı göndermeyi dene (hata alsan bile configuration'ı test eder).

**⚠️ DİKKAT:** Bu test HRMP channel olmadan başarısız olacak, ama pallet'in çalıştığını gösterir.

#### XCM Reserve Transfer Denemesi:

1. **Polkadot.js Extrinsics:**
   - https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9977#/extrinsics

2. **Reserve Transfer Hazırla:**
   ```
   polkadotXcm.limitedReserveTransferAssets(
     dest: V3 {
       parents: 1,
       interior: Here
     },
     beneficiary: V3 {
       parents: 0,
       interior: X1(AccountId32 {
         network: None,
         id: <Alice's account ID>
       })
     },
     assets: V3([
       {
         id: { Concrete: { parents: 0, interior: Here } },
         fun: { Fungible: 1000000000000 }
       }
     ]),
     feeAssetItem: 0,
     weightLimit: Unlimited
   )
   ```

3. **Submit Transaction:**
   - Alice account ile imzala
   - Submit

4. **Beklenen Sonuç:**
   - ❌ Muhtemelen hata: "NoChannel" veya "Unreachable"
   - ✅ Ama extrinsic oluşturabildin → Pallet çalışıyor!
   - ✅ Events'de `polkadotXcm.Attempted` görüyorsan XCM engine çalışıyor

---

## 🧪 Otomatik Test Script Sonuçları

Test script'i de paralel çalışıyor. Sonuçları kontrol et:

```bash
cat /tmp/xcm-test-results.log
```

**Beklenen Çıktı:**
```
✅ Relay Chain: Rococo Local Testnet
✅ Parachain: Rococo Parachain Local
✅ Network health: OK
✅ Block production: Active
✅ Parachain lifecycle: Parachain
✅ XCM pallets: Available
✅ wUSDT (Asset ID 1000): Configured
```

---

## 📊 Test Checklist

Frontend Manuel Testleri:

- [ ] Relay chain'e bağlanabiliyorum
- [ ] Parachain'e bağlanabiliyorum
- [ ] Alice account balance görünüyor
- [ ] Token listesi doğru (HEZ, PEZ, wUSDT)
- [ ] Block production canlı (block number artıyor)
- [ ] Polkadot.js Apps ile bağlantı çalışıyor

Polkadot.js Apps ile XCM Kontrolleri:

- [ ] `polkadotXcm` pallet mevcut
- [ ] `assets` pallet metadata doğru
- [ ] wUSDT Asset ID 1000, 6 decimals
- [ ] Parachain ID 2000 relay chain'de kayıtlı
- [ ] Para lifecycle: "Parachain"
- [ ] XCM version 3 veya 4 destekleniyor

---

## 🔍 Debugging

### Log Dosyalarını Kontrol Et:

**Zombienet network log:**
```bash
tail -f /tmp/zombienet-xcm.log
```

**Relay chain log (Alice):**
```bash
tail -f /tmp/zombie-*/alice.log
```

**Parachain log:**
```bash
tail -f /tmp/zombie-*/pezkuwi-collator.log
```

### XCM Trace Logging:

Zombienet config'de zaten aktif (`-lxcm=trace`). XCM mesajlarını görmek için:

```bash
tail -f /tmp/zombie-*/pezkuwi-collator.log | grep -i xcm
```

---

## 🚀 İleri Seviye: HRMP Channel Açma (Beta için)

**NOT:** Bu local testnet'te HRMP channel manuel açılamaz çünkü Asset Hub yok. Ama Westend beta testinde yapacağınız adımlar:

### HRMP Channel Açma Süreci:

1. **Para Sovereign Account Hesapla:**
   ```javascript
   const parachainAccount = u8aToHex(
     new Uint8Array([...new TextEncoder().encode("para"), ...encodeAddress(2000)])
   );
   ```

2. **Relay Chain'de Proposal Aç:**
   - `hrmp.hrmpInitOpenChannel(recipient: 1000, ...)`
   - Asset Hub approve etmeli

3. **Channel Açıldıktan Sonra:**
   - XCM transferleri çalışmaya başlar
   - Reserve transfer ile USDT → wUSDT bridge aktif olur

---

## 📝 Test Sonuçlarını Kaydet

Test sırasında ekran görüntüleri al:

1. **Polkadot.js Explorer** - Block production
2. **Polkadot.js Parachains** - Para 2000 registered
3. **Frontend Wallet** - Token balances
4. **XCM Extrinsic** - Attempt screenshot (başarılı ya da başarısız)

Test notlarını buraya ekle:

```
Test Tarihi: 2025-11-22
Tester: [İsminiz]

✅ Başarılı Testler:
-

❌ Başarısız Testler:
-

📝 Notlar:
-
```

---

## 🎓 XCM Terimleri Sözlüğü

- **Relay Chain:** Polkadot'un ana zinciri, parachain'leri koordine eder
- **Parachain:** Relay chain'e bağlı bağımsız blockchain
- **HRMP (Horizontal Relay-routed Message Passing):** Parachain'ler arası mesajlaşma
- **XCM (Cross-Consensus Messaging):** Cross-chain mesaj format standardı
- **Reserve Transfer:** Asset'in relay chain'de tutulup parachain'de wrapped olması
- **Teleport:** Asset'in yakılıp karşı tarafta mint edilmesi (daha güvenli network'ler arası)
- **Sovereign Account:** Parachain'in relay chain'deki account'u

---

**Hazırlayan:** Claude (AI Assistant)
**Doküman Versiyonu:** 1.0
**Son Güncelleme:** 2025-11-22
