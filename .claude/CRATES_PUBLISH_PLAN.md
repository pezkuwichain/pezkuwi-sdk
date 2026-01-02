# Crates.io Publish Plan - Pezkuwi SDK

**Son Guncelleme:** 2026-01-01 19:00 UTC
**Toplam Workspace Crate:** 606
**Publishable Crate:** ~516
**Yayinlanan:** ~516 (umbrella + zombienet dahil)
**Registry:** https://crates.io
**Owner:** https://crates.io/users/SatoshiQaziMuhammed

---

## ✅ UMBRELLA CRATE YAYINLANDI!

**pezkuwi-sdk v0.1.1** basariyla crates.io'ya yayinlandi! (zombienet ozellikleri dahil)

```bash
cargo search pezkuwi-sdk
# pezkuwi-sdk = "0.1.1"    # Pezkuwi SDK umbrella crate.
```

---

## ✅ ZOMBIENET CRATE'LERI YAYINLANDI! (2026-01-01 ~19:00 UTC)

6 adet zombienet crate'i rebrand edilip v0.44.0 olarak yayinlandi:

```
[x] pezkuwi-zombienet-support v0.44.0
[x] pezkuwi-zombienet-prom-metrics-parser v0.44.0
[x] pezkuwi-zombienet-configuration v0.44.0
[x] pezkuwi-zombienet-provider v0.44.0
[x] pezkuwi-zombienet-orchestrator v0.44.0
[x] pezkuwi-zombienet-sdk v0.44.0
```

**Yapilan Degisiklikler:**
- `zombienet-*` → `pezkuwi-zombienet-*` rebrand
- Tum workspace referanslari guncellendi
- Kaynak dosyalarindaki `use zombienet_*` → `use pezkuwi_zombienet_*` guncellendi
- Umbrella'ya 6 zombienet crate eklendi

---

## BU OTURUMDA YAYINLANAN CRATE'LER (2026-01-01)

### Yeni Yayinlanan (~36 crate)

```
[x] pezpallet-contracts v27.0.0
[x] xcm-pez-simulator v7.0.0
[x] pezpallet-derivatives v1.0.0
[x] pezpallet-people v1.0.0
[x] pezpallet-dummy-dim v1.0.0
[x] pezpallet-election-provider-multi-block v0.9.0
[x] pezpallet-multi-asset-bounties v1.0.0
[x] pezpallet-oracle v1.0.0
[x] pezpallet-oracle-runtime-api v1.0.0
[x] pezpallet-origin-restriction v1.0.0
[x] pezpallet-root-offences v25.0.0
[x] pezpallet-root-testing v4.0.0
[x] pezpallet-salary v13.0.0
[x] pezpallet-session-benchmarking v28.0.0
[x] pezpallet-skip-feeless-payment v3.0.0
[x] pezpallet-staking-async-rc-client v0.1.0
[x] pezpallet-staking-async v0.1.0
[x] pezpallet-staking-async-ah-client v0.1.0
[x] pezpallet-staking-async-reward-fn v19.0.0
[x] pezpallet-staking-async-runtime-api v14.0.0
[x] pezpallet-staking-runtime-api v14.0.0
[x] pezpallet-state-trie-migration v29.0.0
[x] pezpallet-statement v10.0.0
[x] pezpallet-transaction-storage v27.0.0
[x] pezpallet-xcm-benchmarks v7.0.0
[x] pezpallet-xcm-precompiles v0.1.0
[x] pezkuwichain-runtime-constants v7.0.0
[x] testnet-teyrchains-constants v1.0.0
[x] bizinikiwi-txtesttool v0.7.0
[x] xcm-pez-emulator v0.5.0
[x] pezkuwi-sdk v0.1.0 (UMBRELLA)
[x] pezkuwi-zombienet-support v0.44.0
[x] pezkuwi-zombienet-prom-metrics-parser v0.44.0
[x] pezkuwi-zombienet-configuration v0.44.0
[x] pezkuwi-zombienet-provider v0.44.0
[x] pezkuwi-zombienet-orchestrator v0.44.0
[x] pezkuwi-zombienet-sdk v0.44.0
```

---

## YAPILAN DUZELTMELER

### 1. pezpallet-contracts-fixtures Dependency
- `pezpallet-contracts` icindeki `pezpallet-contracts-fixtures` dev-dependency comment out edildi
- Sebep: Test-only crate, crates.io'ya yayinlanmadi

### 2. Zombienet Vendor Crate'leri ✅ COZULDU
- ~~Umbrella'dan kaldirildi: zombienet-configuration, zombienet-orchestrator, prom-metrics-parser, provider, zombienet-sdk, support~~
- **Cozum:** `zombienet-*` → `pezkuwi-zombienet-*` olarak rebrand edildi ve yayinlandi
- Artik Umbrella'da `pezkuwi-zombienet-*` olarak mevcut

### 3. pezpallet-contracts-mock-network
- Umbrella'dan kaldirildi
- Sebep: Test-only crate, pezpallet-contracts-fixtures'a bagimli

### 4. Crates.io 300 Feature Limiti
- runtime-benchmarks feature'indan ~35 primitives entry'si gecici olarak kaldirildi
- Umbrella yayinlandiktan sonra geri eklendi

---

## YAYINLANAMAYAN CRATE'LER

Bu crate'ler publish=false oldugu icin yayinlanmadi:

| Crate | Sebep |
|-------|-------|
| `pezpallet-contracts-fixtures` | Build script workspace root gerektiriyor |
| `pezpallet-contracts-fixtures-common` | publish=false |
| `pezpallet-contracts-mock-network` | fixtures'a bagimli |
| `pezkuwi-omni-node` | Runtime crate'lere bagimli |
| `pezkuwi-omni-node-lib` | Runtime crate'lere bagimli |
| `pezkuwi-teyrchain-bin` | Runtime crate'lere bagimli |
| Runtime crate'ler (*-runtime) | Genellikle publish=false |

---

## ONEMLI NOTLAR

1. **Rate Limit:** Crates.io yeni crate yayinlama hizini sinirliyor (~10 dakikada 1-2 crate)
2. **Feature Limiti:** Crates.io maksimum 300 feature/dependency limiti var
3. **Zombienet Crate'leri:** `pezkuwi-zombienet-*` olarak yayinlandi (v0.44.0)
4. **Workspace Degisiklikleri:** Bazi test crate'leri dev-dependencies'den comment out edildi

---

## KULLANIM

```toml
# Cargo.toml
[dependencies]
pezkuwi-sdk = "0.1.1"

# Ozel feature'lar icin
pezkuwi-sdk = { version = "0.1.1", features = ["node", "runtime-full"] }

# Zombienet kullanimi icin
pezkuwi-sdk = { version = "0.1.1", features = ["pezkuwi-zombienet-sdk"] }
```

### Zombienet Crate'leri Dogrudan Kullanim

```toml
[dependencies]
pezkuwi-zombienet-sdk = "0.44.0"
pezkuwi-zombienet-configuration = "0.44.0"
pezkuwi-zombienet-orchestrator = "0.44.0"
```

---

*Son guncelleme: 2026-01-01 ~19:00 UTC*
