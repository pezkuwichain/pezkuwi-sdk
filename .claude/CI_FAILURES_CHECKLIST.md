# CI WORKFLOW HATA ANALİZİ VE ÇÖZÜM CHECKLIST

**Tarih:** 2026-01-26 (Güncellendi)
**Analiz Edilen Run'lar:** Son push (main branch)

---

## ÖZET: 1 KÖK NEDEN - 5 WORKFLOW HATASI

| # | Workflow | Job | Durum |
|---|----------|-----|-------|
| 1 | Build and push images | build-test-collators | ❌ BAŞARISIZ |
| 2 | tests misc | test-node-metrics | ❌ BAŞARISIZ |
| 3 | Checks | cargo-clippy | ❌ BAŞARISIZ |
| 4 | EVM test suite | evm-test-suite (test:evm) | ❌ BAŞARISIZ |
| 5 | EVM test suite | All test misc tests passed | ❌ BAŞARISIZ (bağımlılık) |

---

## KÖK NEDEN ANALİZİ

**TÜM HATALAR AYNI SORUNDAN KAYNAKLIYOR:**

```
error[E0152]: duplicate lang item in crate `alloc` (which `serde_core` depends on): `exchange_malloc`
```

### Neden Oluşuyor?

1. CI Docker image'larında `wasm32v1-none` target yüklü DEĞİL
2. wasm-builder fallback olarak `wasm32-unknown-unknown` + `-Z build-std=core,alloc` kullanıyor
3. Bu durumda:
   - Standard library'nin kendi `alloc` crate'i var
   - `-Z build-std` ayrıca bir `alloc` crate'i build ediyor
   - İki farklı `alloc` = duplicate lang item hatası

### Neden serde Fork'u Yeterli Değil?

- serde fork'umuz (`fix-wasm32v1-none` branch) **wasm32v1-none** target için düzeltilmiş
- Ancak **wasm32-unknown-unknown + build-std** senaryosu farklı
- wasm32v1-none yüklenirse, wasm-builder bu target'i kullanır ve `-Z build-std` KULLANMAZ
- Problem çözülür

---

## ÇÖZÜM: wasm32v1-none Target Kurulumu

**Etkilenen workflow dosyalarına `rustup target add wasm32v1-none` eklenecek:**

### Dosya 1: `.github/workflows/build-publish-images.yml`
- `build-test-collators` job'ına target kurulumu ekle

### Dosya 2: `.github/workflows/tests-misc.yml`
- `test-node-metrics` job'ını etkileyen WASM build'ler için target kurulumu ekle
- `test-deterministic-wasm` job'ına target kurulumu ekle

### Dosya 3: `.github/workflows/checks.yml`
- `cargo-clippy` job'ına target kurulumu ekle (zombienet-sdk-tests WASM build)

### Dosya 4: `.github/workflows/tests-evm.yml`
- EVM test'leri için target kurulumu ekle

---

## HATA DETAYLARI

### HATA 1: build-test-collators

**Log:**
```
error[E0152]: duplicate lang item in crate `alloc` (which `serde_core` depends on): `exchange_malloc`
= note: first definition in `alloc` loaded from wasm32-unknown-unknown/lib/liballoc-81e4e4ffae91d46d.rlib
= note: second definition in `alloc` loaded from target/testnet/wbuild/test-teyrchain-adder/target/wasm32-unknown-unknown/release/deps/liballoc-21f70be7f9695a73.rmeta
```

**Çözüm:** `rustup target add wasm32v1-none` ekle

### HATA 2: test-node-metrics

**Log:**
```
error[E0152]: duplicate lang item in crate `alloc` (which `serde_core` depends on): `exchange_malloc`
```

**Çözüm:** `rustup target add wasm32v1-none` ekle

### HATA 3: cargo-clippy

**Log:**
```
error[E0152]: duplicate lang item in crate `alloc` (which `serde_core` depends on): `exchange_malloc`
= note: second definition in `alloc` loaded from target/debug/build/pezkuwi-zombienet-sdk-tests-981c0c2de47cd6a0/out/runtimes/x86_64-unknown-linux-gnu/release/wbuild/pezkuwichain-runtime/target/wasm32-unknown-unknown/release/deps/liballoc-b6b797d641a22516.rmeta

thread 'main' panicked at pezkuwi/zombienet-sdk-tests/build.rs:92:42:
Failed to read WASM file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

**Çözüm:** `rustup target add wasm32v1-none` ekle

### HATA 4: evm-test-suite (test:evm)

**Log:**
```
error: (in promise) Error: No platform detected. Start the chain manually or use START_GETH or START_REVIVE_DEV_NODE and START_ETH_RPC to start the chain from the test runner.
```

**Kök Neden:** dev-node WASM build hatası nedeniyle başlatılamıyor
**Çözüm:** WASM build düzeltilince bu da düzelecek

### HATA 5: All test misc tests passed

**Kök Neden:** test-node-metrics başarısız olduğu için bağımlılık hatası
**Çözüm:** test-node-metrics düzeltilince bu da düzelecek

---

## UYGULAMA PLANI

1. [ ] `.github/workflows/build-publish-images.yml` - wasm32v1-none ekle
2. [ ] `.github/workflows/tests-misc.yml` - wasm32v1-none ekle
3. [ ] `.github/workflows/checks.yml` - wasm32v1-none ekle
4. [ ] `.github/workflows/tests-evm.yml` - wasm32v1-none ekle
5. [ ] Commit ve push
6. [ ] CI sonuçlarını bekle

---

## NOTLAR

- serde fork'u doğru kullanılıyor (Cargo.lock kontrol edildi)
- wasm32v1-none target kurulumu daha önce PR #346'da kaldırılmıştı
- Şimdi tekrar eklenmesi gerekiyor çünkü fallback mekanizması çalışmıyor

---

*Son güncelleme: 2026-01-26 ~00:30 UTC*
