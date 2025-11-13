# Broken Scripts Report - Pezkuwi-SDK

**Generated:** 2025-11-11
**Status:** Beta testnet çalışıyor, bloklar finalize oluyor

## Hatalı Scriptler (3 adet)

### 1. `clear-pez-usdt-dust.mjs`
**Lokasyon:** `/home/mamostehp/Pezkuwi-SDK/scripts/beta_testnet/clear-pez-usdt-dust.mjs`

**Hata:**
```
RpcError: 1002: Verification Error: Runtime error: Execution failed:
Execution aborted due to trap: wasm trap: wasm `unreachable` instruction executed
```

**Sebep:** Pool'a dust ratio'da liquidity eklemeye çalışırken runtime panic oluyor. Pool state'i bozuk veya minBalance koşulları karşılanmıyor.

**Önerilen Çözüm:** SİL - Bu script pool'u temizlemeye çalışıyor ama runtime'da panic oluşturuyor. Yeni bir fresh pool create etmek daha güvenli.

---

### 2. `add-fresh-pez-usdt-liquidity.mjs`
**Lokasyon:** `/home/mamostehp/Pezkuwi-SDK/scripts/beta_testnet/add-fresh-pez-usdt-liquidity.mjs`

**Hata:**
```
RpcError: 1002: Verification Error: Runtime error: Execution failed:
Execution aborted due to trap: wasm trap: wasm `unreachable` instruction executed
```

**Sebep:** Büyük miktarda liquidity eklerken runtime panic oluyor. Muhtemelen pool daha önce bozuk bir state'e geçmiş.

**Önerilen Çözüm:** SİL - Pool zaten sorunlu, bu script de aynı hatayı üretiyor.

---

### 3. `sudo-clear-pez-usdt-pool.mjs`
**Lokasyon:** `/home/mamostehp/Pezkuwi-SDK/scripts/beta_testnet/sudo-clear-pez-usdt-pool.mjs`

**Hata:**
```
TypeError: api.tx.assetConversion.removePool is not a function
```

**Sebep:** Runtime'da `assetConversion.removePool` fonksiyonu yok. Bu sudo-only bir fonksiyon olabilir veya hiç implement edilmemiş olabilir.

**Önerilen Çözüm:** SİL - Bu fonksiyon runtime'da mevcut değil.

---

## Çalışan Scriptler

### Beta Testnet Validator Scripts (✅ ÇALIŞIYOR)
- `start-beta-validator-{1-8}.sh` - Tüm validatorler başarıyla çalışıyor
- `stop-beta-validators.sh` - Validatorleri durduruyor
- `insert-all-beta-keys.sh` - Session key'leri başarıyla insert ediyor
- `complete-setup.sh` - Setup scripti çalışıyor
- `start-beta-testnet-clean.sh` - Clean start çalışıyor

### Diğer Scriptler (❓ TEST EDİLMEDİ)
Aşağıdaki scriptler şu an test edilmedi, çalışıp çalışmadığını bilmiyoruz:
- `calculate-min-liquidity.mjs`
- `check-assets.mjs`
- `check-balances.mjs`
- `check-hez-balance.mjs`
- `check-min-deposit.mjs`
- `check-pool-reserves.mjs`
- `create-all-dex-pools.mjs`
- `create-pez-wusdt-pool.mjs`
- `create-wusdt-sudo.mjs`
- `set-session-keys.mjs`
- `setup-initial-pools.mjs`
- `verify-pool-state.mjs`
- `wrap-hez-and-create-all-pools.mjs`
- `wrap-hez.mjs`

## Önerilen Aksiyonlar

1. **Hatalı 3 scripti SİL:**
   ```bash
   cd /home/mamostehp/Pezkuwi-SDK/scripts/beta_testnet
   rm -f clear-pez-usdt-dust.mjs
   rm -f add-fresh-pez-usdt-liquidity.mjs
   rm -f sudo-clear-pez-usdt-pool.mjs
   ```

2. **Yeni pool management stratejisi:**
   - Broken pool'ları temizlemeye çalışma
   - Her zaman fresh pool create et
   - minBalance koşullarına dikkat et

3. **Test edilmemiş scriptleri incele:**
   - Her bir scripti tek tek çalıştır
   - Hata veren varsa dokümante et
   - Gerekirse düzelt veya sil
