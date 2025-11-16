# Beta Testnet Scripts

Bu klasördeki scriptler beta testnet'i düzgün çalıştırmak için gerekli, test edilmiş scriptlerdir.

## 📋 İçerik

### 1. `start-all-beta-validators.sh`
8 validatörü başlatan ana script.

**Kullanım:**
```bash
bash start-all-beta-validators.sh
```

**Ne yapar:**
- Validator 1'i bootnode olarak başlatır (RPC: 9944, P2P: 30333)
- Diğer 7 validatörü bootnode'a bağlayarak başlatır
- Her validator için ayrı log dosyası oluşturur: `/tmp/beta-validator-{1..8}.log`

### 2. `start-beta-validator-{1..8}.sh`
Her validator için ayrı başlatma scripti.

**Not:** Normalde `start-all-beta-validators.sh` kullanılır, ama ihtiyaç durumunda tek tek başlatmak için kullanılabilir.

### 3. `insert-all-beta-keys.sh`
Tüm validatörlerin blockchain'e block üretebilmesi için gerekli anahtarları ekler.

**Kullanım:**
```bash
bash insert-all-beta-keys.sh
```

**⚠️ ÖNEMLİ:** Bu script validatörler çalışırken çalıştırılmalıdır!

**Ne yapar:**
- Her validator için 6 anahtar tipini ekler: babe, gran, para, asgn, audi, beef
- Anahtarlar eklenmeden validatörler block üretemez

### 4. `stop-beta-validators.sh`
Tüm beta validatörlerini durdurur.

**Kullanım:**
```bash
bash stop-beta-validators.sh
```

## 🚀 Tam Başlatma Prosedürü

### İlk Kez Başlatma (Temiz Başlangıç)

1. **Eski verileri temizle:**
```bash
rm -rf /tmp/beta-validator-{1..8} /tmp/beta-validator-*.log
```

2. **Validatörleri başlat:**
```bash
cd /home/mamostehp/Pezkuwi-SDK/scripts/beta_testnet
bash start-all-beta-validators.sh
```

3. **Anahtarları ekle:**
```bash
bash insert-all-beta-keys.sh
```
(Onay için 'y' tuşuna bas)

4. **Block üretimini kontrol et:**
```bash
tail -f /tmp/beta-validator-1.log
```

Şunları görmelisin:
- ✅ "7 peers" - Diğer validatörlerle bağlantı kuruldu
- ✅ "Prepared block for proposing" - Block üretiliyor
- ✅ "best: #XXX" sayısı artıyor - Blockchain ilerliyor

### Mevcut Veriyle Tekrar Başlatma

Eğer `/tmp/beta-validator-{1..8}` verileri varsa ve korumak istiyorsan:

1. **Validatörleri durdur:**
```bash
bash stop-beta-validators.sh
```

2. **Tekrar başlat:**
```bash
bash start-all-beta-validators.sh
```

**Not:** Anahtarlar zaten ekliyse tekrar eklemeye gerek yok.

## 🔧 Önemli Teknik Detaylar

### Bootnode Peer ID
Validator 2-8, Validator 1'e (bootnode) şu peer ID ile bağlanır:
```
grep "Local node identity" /tmp/beta-validator-1.log | tail -1
```

⚠️ **Dikkat:** Eğer database temizlenirse (`rm -rf /tmp/beta-validator-1`), bu peer ID değişir ve validator 2-8'in scriptleri güncellenmeli!

Yeni peer ID'yi bulmak için:
```bash
grep "Local node identity" /tmp/beta-validator-1.log | tail -1
```

Validator 2-8 scriptlerinde güncellemek için:
```bash
cd /home/mamostehp/Pezkuwi-SDK/scripts/beta_testnet
sed -i 's/ESKİ_PEER_ID/YENİ_PEER_ID/g' start-beta-validator-{2..8}.sh
```

### Validator Endpoints

| Validator | RPC Port | P2P Port | Endpoint |
|-----------|----------|----------|----------|
| Validator 1 (Bootnode) | 9944 | 30333 | ws:// degeri yaz |
| Validator 2 | 9945 | 30334 | ws://127.0.0.1:9945 |
| Validator 3 | 9946 | 30335 | ws://127.0.0.1:9946 |
| Validator 4 | 9947 | 30336 | ws://127.0.0.1:9947 |
| Validator 5 | 9948 | 30337 | ws://127.0.0.1:9948 |
| Validator 6 | 9949 | 30338 | ws://127.0.0.1:9949 |
| Validator 7 | 9950 | 30339 | ws://127.0.0.1:9950 |
| Validator 8 | 9951 | 30340 | ws://127.0.0.1:9951 |

### Log Dosyaları
Her validator'ın logu ayrı dosyada: ( eger yoksa olustur )
```
/tmp/beta-validator-1.log
/tmp/beta-validator-2.log
...
/tmp/beta-validator-8.log
```

## ❗ Sorun Giderme

### Problem: Validatörler "0 peers" gösteriyor

**Sebep:** Bootnode peer ID yanlış veya validatörler birbirini bulamıyor.

**Çözüm:**
1. Bootnode peer ID'yi kontrol et
2. Tüm validatörleri durdur ve anahtarları tekrar ekle

### Problem: Block üretilmiyor (best: #0 stuck)

**Sebep:** Anahtarlar eklenmemiş.

**Çözüm:**
```bash
bash insert-all-beta-keys.sh
```

### Problem: "Low connectivity" uyarısı

Bu normal. Anahtarlar eklendikten ve birkaç block üretildikten sonra kaybolur.

## 📝 Geliştirici Notları

Bu scriptler şu düzeltmeleri içeriyor:
- ✅ Doğru bootnode peer ID (12D3KooWRuAqJ3w5U7yJPcMXERqMPHVUWACCqhwgD7WwvfUjAhMW)
- ✅ Validator 1 bootnode olarak yapılandırılmış
- ✅ Diğer validatörler bootnode'a bağlanacak şekilde yapılandırılmış
- ✅ Anahtar ekleme scripti tüm 6 anahtar tipini ekliyor
- ✅ Her validator ayrı RPC ve P2P portuna sahip

Son güncelleme: 2025-11-03
