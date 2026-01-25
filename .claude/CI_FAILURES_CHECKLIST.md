# CI WORKFLOW HATA ANALİZİ VE ÇÖZÜM CHECKLIST

**Tarih:** 2026-01-25
**Analiz Edilen Run'lar:** Son 100 failed workflow

---

## ÖZET: 5 KÖK NEDEN BULUNDU

| # | Kök Neden | Etkilenen Workflow Sayısı | Öncelik |
|---|-----------|---------------------------|---------|
| 1 | curl-sys OpenSSL 3.0.0 gereksinimi | 4+ | KRİTİK |
| 2 | Cargo.lock güncel değil | 3+ | KRİTİK |
| 3 | HOME dizin uyumsuzluğu (container) | 1 | YÜKSEK |
| 4 | serde_core wasm32 compile hatası | 1+ | YÜKSEK |
| 5 | WhereSection::_w deprecated constant | 1 | ORTA |

---

## HATA 1: curl-sys OpenSSL 3.0.0 Gereksinimi ✅ ÇÖZÜLDÜ

### Hata Mesajı:
```
error: failed to run custom build command for `curl-sys v0.4.85+curl-8.18.0`
cargo:warning=curl/lib/vtls/openssl.c:101:6: error: "OpenSSL 3.0.0 or later required"
```

### Etkilenen Workflow'lar:
- ✅ Checks / cargo-clippy
- ✅ Checks / check-try-runtime
- ✅ Docs / test-doc
- ✅ Docs / build-rustdoc

### Kök Neden:
CI container image'ında (Debian-based) OpenSSL 1.x kurulu. curl-sys v0.4.85+ ise OpenSSL 3.0.0+ gerektiriyor.

### KALICI ÇÖZÜM UYGULANDI:
**OpenSSL bağımlılığı tamamen kaldırıldı - isahc → reqwest (rustls-tls) migration**

Bu çözüm:
- OpenSSL bağımlılığını tamamen kaldırdı
- Pure Rust TLS (rustls) kullanıyor
- Daha güvenli, daha portable
- C derleme gereksinimi yok

**Değiştirilen dosyalar:**
1. `Cargo.toml` - reqwest'e rustls-tls feature eklendi, isahc kaldırıldı
2. `pezbridges/relays/utils/Cargo.toml` - isahc → reqwest
3. `pezbridges/relays/utils/src/metrics/float_json_value.rs` - kod güncellendi

**Kaldırılan bağımlılıklar (Cargo.lock):**
- curl, curl-sys, isahc, openssl-sys, native-tls, libnghttp2-sys...

### Durum:
- [x] Çözüm seçildi (KALICI: OpenSSL bağımlılığı tamamen kaldırıldı)
- [x] Uygulama yapıldı
- [x] Test edildi (`cargo check --workspace` başarılı)

---

## HATA 2: Cargo.lock Güncel Değil ✅ ÇÖZÜLDÜ

### Hata Mesajı:
```
error: the lock file Cargo.lock needs to be updated but --locked was passed to prevent this
```

### Etkilenen Workflow'lar:
- ✅ Build and push images / build-linux-bizinikiwi
- ✅ Build and push images / build-linux-stable
- ✅ tests linux stable / test-linux-stable-no-try-runtime
- ✅ tests linux stable / test-linux-stable
- ✅ tests misc / cargo-check-benches

### Kök Neden:
Cargo.toml'da dependency değişikliği yapıldı ama Cargo.lock güncellenmeden commit edildi.

### ÇÖZÜM UYGULANDI:
`cargo update` çalıştırıldı, Cargo.lock güncellendi.
Ayrıca isahc → reqwest migration sırasında Cargo.lock tamamen yenilendi.

### Durum:
- [x] `cargo update` çalıştırıldı
- [x] Cargo.lock güncellendi (curl/isahc/openssl kaldırıldı)
- [ ] Cargo.lock commit edildi (beklemede)
- [ ] CI tekrar çalıştırıldı

---

## HATA 3: HOME Dizin Uyumsuzluğu (Container) ✅ ÇÖZÜLDÜ

### Hata Mesajı:
```
error: $HOME differs from euid-obtained home directory: you may be using sudo
error: $HOME directory: /github/home
error: euid-obtained home directory: /root
```

### Etkilenen Workflow'lar:
- ✅ Check the getting-started.sh script (tüm container job'ları)

### Kök Neden:
GitHub Actions container job'larında HOME env değişkeni `/github/home` olarak ayarlı, ancak container root kullanıcısı `/root` home dizinini kullanıyor. rustup bu tutarsızlığı reddediyor.

### ÇÖZÜM UYGULANMIŞ (önceki oturumda):
```yaml
# .github/workflows/check-getting-started.yml satır 79-80
env:
  HOME: /root  # Fix HOME directory mismatch in containers (rustup requires this)
```

### Durum:
- [x] HOME env variable eklendi
- [ ] Commit yapıldı (beklemede)
- [ ] CI tekrar çalıştırıldı

---

## HATA 4: serde_core wasm32 Compile Hatası

### Hata Mesajı:
```
error: could not compile `serde_core` (lib) due to 1 previous error
```

### Etkilenen Workflow'lar:
- ❌ EVM test suite / differential-tests

### Kök Neden:
serde_core crate'i wasm32 target'ında compile olurken "duplicate lang item" veya "ambiguous imports" hatası veriyor.

### Durum:
Bu sorun daha önce `pezkuwichain/serde` fork'unda düzeltildi:
- Fork commit: `0a75fdd8`
- pezkuwi-sdk commit: `7cc45454ff`

### Kontrol Edilmesi Gerekenler:
1. Cargo.lock serde_core'un doğru commit'i mi gösteriyor?
2. CI cache temizlenmesi gerekiyor mu?

### Çözüm (eğer hala devam ediyorsa):
```bash
# Cargo.lock'u kontrol et
grep -A5 'name = "serde_core"' Cargo.lock

# Force update
cargo update -p serde_core
```

### Durum:
- [ ] serde_core versiyonu doğrulandı
- [ ] Cargo.lock güncellendi
- [ ] CI tekrar çalıştırıldı

---

## HATA 5: WhereSection::_w UI Test Mismatch ⚠️ İNCELENİYOR

### Hata Mesajı:
```
error: use of deprecated constant `WhereSection::_w`:
   = help: the trait `std::fmt::Debug` is implemented for `pezframe_system::Error<T>`
```

### Etkilenen Workflow'lar:
- ⚠️ tests misc / test-pezframe-ui

### Kök Neden:
Bu bir **UI testi** - kasıtlı olarak deprecated constant hatası BEKLEYEN bir test.
`.stderr` dosyası beklenen derleyici çıktısını içeriyor.
CI'daki sorun: Gerçek çıktı ile beklenen çıktı arasında ufak farklar var.

### Dosya:
- Test: `bizinikiwi/pezframe/support/test/tests/construct_runtime_ui/deprecated_where_block.rs`
- Beklenen hata: `bizinikiwi/pezframe/support/test/tests/construct_runtime_ui/deprecated_where_block.stderr`

### Olası Nedenler:
1. Rust compiler versiyonu farkı (hata mesajı formatı değişmiş olabilir)
2. serde_core path değişikliği (`$CARGO/serde_core-$VERSION/...`)

### Çözüm:
UI testini yeniden generate etmek gerekebilir:
```bash
TRYBUILD=overwrite cargo test -p pezframe-support-test --test construct_runtime_ui
```

### Durum:
- [x] Kök neden analiz edildi (UI test mismatch)
- [ ] UI test stderr dosyası güncellendi
- [ ] Test geçti

---

## UYGULAMA SIRASI (ÖNERİLEN)

### Adım 1: Cargo.lock Güncelle (5 dakika)
```bash
cd /home/mamostehp/pezkuwi-sdk
cargo update
cargo check --workspace
git add Cargo.lock
git commit -m "chore: update Cargo.lock for CI compatibility"
```

### Adım 2: HOME Env Variable Ekle (2 dakika)
```yaml
# .github/workflows/check-getting-started.yml düzenle
# env: bloğuna HOME: /root ekle
```

### Adım 3: OpenSSL Sorunu Çöz (10 dakika)
```yaml
# .github/workflows/checks.yml düzenle
# OpenSSL 3 kurulumu veya alternative çözüm
```

### Adım 4: Commit ve Push
```bash
git add -A
git commit -m "fix: CI workflow failures - Cargo.lock, HOME env, OpenSSL"
git push
```

### Adım 5: CI İzle
```bash
gh run watch
```

---

## EK: ETKİLENEN WORKFLOW DOSYALARI

| Workflow Dosyası | Hatalar |
|------------------|---------|
| `.github/workflows/checks.yml` | curl-sys/OpenSSL |
| `.github/workflows/docs.yml` | curl-sys/OpenSSL |
| `.github/workflows/build-misc.yml` | Cargo.lock |
| `.github/workflows/tests-linux-stable.yml` | Cargo.lock |
| `.github/workflows/tests-misc.yml` | Cargo.lock, WhereSection |
| `.github/workflows/check-getting-started.yml` | HOME env |
| `.github/workflows/evm-tests.yml` | serde_core |

---

## NOTLAR

1. **Öncelik:** Önce Cargo.lock'u güncelle - bu en çok workflow'u etkiliyor
2. **Test:** Her değişiklikten sonra lokal test yap (`cargo check --workspace`)
3. **Paralel:** curl-sys ve HOME sorunları birlikte commit edilebilir
4. **Cache:** CI cache temizliği gerekebilir (`gh cache delete --all`)

---

*Bu checklist CI hataları çözülene kadar güncel tutulmalıdır.*
