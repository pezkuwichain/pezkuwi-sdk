# Pezkuwi SDK - Workflow Rebranding Raporu

## Genel Bakış

Bu belge, Pezkuwi SDK'daki 74 workflow dosyasının kapsamlı analizini ve rebranding planını içerir.

**Analiz Tarihi:** 2025-12-03
**Toplam Workflow Sayısı:** 74
**Rebranding Gerektiren Dosya Sayısı:** 33 (64 referans)

---

## Terminoloji Dönüşüm Tablosu

| Eski Terim | Yeni Terim | Açıklama |
| --- | --- | --- |
| `polkadot` | `pezkuwi` | Ana zincir referansları |
| `polkadot-sdk` | `pezkuwi-sdk` | SDK referansları |
| `paritytech` | `pezkuwichain` | Organizasyon adı |
| `parachain` | `teyrchain` | Alt zincir terminolojisi |
| `rococo` | `pezkuwichain` | Test ağı |
| `westend` | `zagros` | Test ağı |
| `kusama` | `zagros` | Canary ağı |

---

## Workflow Kategorileri

### 1. CI/CD Kontrolleri (12 dosya)

| Dosya | Amaç | Rebranding Durumu | Öncelik |
| --- | --- | --- | --- |
| `checks.yml` | Clippy, try-runtime kontrolleri | 1 referans | YÜKSEK |
| `checks-quick.yml` | Format, zepter, toml kontrolleri | 3 referans | YÜKSEK |
| `check-prdoc.yml` | PR dokümantasyon kontrolü | 1 referans | ORTA |
| `check-semver.yml` | SemVer kontrolü | 2 referans | ORTA |
| `check-licenses.yml` | Lisans taraması | 8 referans | YÜKSEK |
| `check-labels.yml` | Etiket kontrolü | 1 referans | ORTA |
| `check-links.yml` | Link kontrolü | 0 referans | TAMAM |
| `check-runtime-compatibility.yml` | Runtime uyumluluk | 1 referans | YÜKSEK |
| `check-runtime-migration.yml` | Runtime migration testi | 1 referans | YÜKSEK |
| `check-cargo-check-runtimes.yml` | Cargo check | 0 referans | TAMAM |
| `check-frame-omni-bencher.yml` | Benchmark kontrolü | 0 referans | TAMAM |
| `check-getting-started.yml` | Başlangıç kontrolü | 0 referans | TAMAM |

### 2. Test Workflow'ları (7 dosya)

| Dosya | Amaç | Rebranding Durumu | Öncelik |
| --- | --- | --- | --- |
| `tests.yml` | Ana test suite | 0 referans | TAMAM |
| `tests-linux-stable.yml` | Linux stable testleri | 1 referans | ORTA |
| `tests-linux-stable-coverage.yml` | Coverage testleri | 0 referans | TAMAM |
| `tests-linux-stable-xp.yml` | XP testleri | 0 referans | TAMAM |
| `tests-misc.yml` | Misc testler | 4 referans | YÜKSEK |
| `tests-evm.yml` | EVM testleri | 4 referans | YÜKSEK |
| `cmd-tests.yml` | CMD testleri | 0 referans | TAMAM |

### 3. Build & Publish (5 dosya)

| Dosya | Amaç | Rebranding Durumu | Öncelik |
| --- | --- | --- | --- |
| `build-publish-images.yml` | Docker image build | 5 referans | KRİTİK |
| `build-publish-eth-rpc.yml` | ETH RPC build | 0 referans | TAMAM |
| `build-misc.yml` | Misc build işlemleri | 0 referans | TAMAM |
| `publish-check-compile.yml` | Compile kontrolü | 0 referans | TAMAM |
| `publish-check-crates.yml` | Crate kontrolü | 0 referans | TAMAM |

### 4. Release Workflow'ları (17 dosya)

| Dosya | Amaç | Rebranding Durumu | Öncelik |
| --- | --- | --- | --- |
| `release-10_branchoff-stable.yml` | Stable branch oluşturma | 1 referans | YÜKSEK |
| `release-11_rc-automation.yml` | RC otomasyonu | 1 referans | YÜKSEK |
| `release-20_build-rc.yml` | RC build | 1 referans | YÜKSEK |
| `release-21_build-runtimes.yml` | Runtime build | 1 referans | YÜKSEK |
| `release-22_combined-rc-runtime-builds-release-draft.yml` | Combined RC build | 1 referans | YÜKSEK |
| `release-30_publish_release_draft.yml` | Release draft yayınlama | 2 referans | YÜKSEK |
| `release-31_promote-rc-to-final.yml` | RC'yi finale promote | 1 referans | YÜKSEK |
| `release-40_publish-deb-package.yml` | DEB paket yayınlama | 0 referans | TAMAM |
| `release-41_publish-rpm-package.yml` | RPM paket yayınlama | 0 referans | TAMAM |
| `release-50_publish-docker.yml` | Docker yayınlama | 2 referans | YÜKSEK |
| `release-60_create-old-release-tag.yml` | Eski release tag | 1 referans | ORTA |
| `release-60_post-crates-release-activities.yml` | Post-release aktiviteler | 1 referans | ORTA |
| `release-70_combined-publish-release.yml` | Combined release | 1 referans | YÜKSEK |
| `release-99_notif-published.yml` | Release bildirimi | 3 referans | KRİTİK |
| `release-build-binary.yml` | Binary build | 0 referans | TAMAM |
| `release-check-runtimes.yml` | Runtime kontrolü | 0 referans | TAMAM |
| `release-srtool.yml` | SRTool build | 0 referans | TAMAM |

### 5. Reusable Workflow'lar (5 dosya)

| Dosya | Amaç | Rebranding Durumu | Öncelik |
| --- | --- | --- | --- |
| `reusable-preflight.yml` | Preflight kontrolleri | 2 referans | YÜKSEK |
| `reusable-isdraft.yml` | Draft kontrolü | 0 referans | TAMAM |
| `release-reusable-rc-build.yml` | RC build reusable | 3 referans | YÜKSEK |
| `release-reusable-publish-packages.yml` | Paket yayınlama | 2 referans | YÜKSEK |
| `release-reusable-s3-upload.yml` | S3 upload | 0 referans | TAMAM |
| `release-reusable-promote-to-final.yml` | Promote reusable | 0 referans | TAMAM |

### 6. Zombienet Workflow'ları (5 dosya)

| Dosya | Amaç | Rebranding Durumu | Öncelik |
| --- | --- | --- | --- |
| `zombienet_pezkuwi.yml` | Pezkuwi zombienet testleri | 0 referans | TAMAM |
| `zombienet_cumulus.yml` | Cumulus zombienet testleri | 0 referans | TAMAM |
| `zombienet_substrate.yml` | Substrate zombienet testleri | 0 referans | TAMAM |
| `zombienet_teyrchain-template.yml` | Teyrchain template testleri | 0 referans | TAMAM |
| `zombienet-reusable-preflight.yml` | Zombienet preflight | 0 referans | TAMAM |
| `check-zombienet-flaky-tests.yml` | Flaky test kontrolü | 0 referans | TAMAM |

### 7. Command & Bot Workflow'ları (7 dosya)

| Dosya | Amaç | Rebranding Durumu | Öncelik |
| --- | --- | --- | --- |
| `cmd.yml` | Command işleyici | 1 referans | ORTA |
| `cmd-run.yml` | Command çalıştırıcı | 0 referans | TAMAM |
| `command-backport.yml` | Backport command | 0 referans | TAMAM |
| `command-inform.yml` | Inform command | 0 referans | TAMAM |
| `command-prdoc.yml` | PRDoc command | 0 referans | TAMAM |
| `review-bot.yml` | Review bot | 3 referans | YÜKSEK |
| `review-trigger.yml` | Review trigger | 0 referans | TAMAM |

### 8. Misc Workflow'lar (9 dosya)

| Dosya | Amaç | Rebranding Durumu | Öncelik |
| --- | --- | --- | --- |
| `docs.yml` | Dokümantasyon | 1 referans | ORTA |
| `fork-sync-action.yml` | Fork senkronizasyonu | 1 referans | ORTA |
| `gitspiegel-trigger.yml` | Git mirror trigger | 0 referans | TAMAM |
| `issues-auto-add-teyrchain.yml` | Issue otomatik ekleme | 1 referans | ORTA |
| `issues-auto-label.yml` | Issue otomatik etiketleme | 0 referans | TAMAM |
| `misc-notify-burnin-label.yml` | Burnin label bildirimi | 0 referans | TAMAM |
| `misc-review-bot-merge-queue.yml` | Merge queue review | 0 referans | TAMAM |
| `misc-sync-templates.yml` | Template senkronizasyonu | 2 referans | YÜKSEK |
| `misc-update-wishlist-leaderboard.yml` | Wishlist güncelleme | 0 referans | TAMAM |
| `publish-claim-crates.yml` | Crate claim | 0 referans | TAMAM |

### 9. Benchmark Workflow'ları (3 dosya)

| Dosya | Amaç | Rebranding Durumu | Öncelik |
| --- | --- | --- | --- |
| `bench-all-runtimes.yml` | Tüm runtime benchmarks | 0 referans | TAMAM |
| `benchmarks-networking.yml` | Network benchmarks | 0 referans | TAMAM |
| `benchmarks-subsystem.yml` | Subsystem benchmarks | 0 referans | TAMAM |

---

## Detaylı Değişiklik Listesi

### KRİTİK - Hemen Değiştirilmeli

#### 1. `build-publish-images.yml` (5 değişiklik)

```yaml
# Satır 359: polkadot klasör yolu
cargo nextest --manifest-path polkadot/zombienet-sdk-tests/Cargo.toml ...
# DEĞİŞTİR: polkadot → pezkuwi (klasör yeniden adlandırılmadıysa bırakılabilir)

# Satır 363: Artifact adı
cp polkadot-zombienet-tests.tar.zst ./artifacts
# DEĞİŞTİR: pezkuwi-zombienet-tests.tar.zst

# Satır 463: Job adı
build-push-image-polkadot-debug:
# DEĞİŞTİR: build-push-image-pezkuwi-debug

# Satır 481: Image adı
image-name: "polkadot-debug"
# DEĞİŞTİR: image-name: "pezkuwi-debug"

# Satır 482: Dockerfile yolu
dockerfile: "docker/dockerfiles/polkadot/polkadot_injected_debug.Dockerfile"
# DEĞİŞTİR: dockerfile: "docker/dockerfiles/pezkuwi/pezkuwi_injected_debug.Dockerfile"
```

#### 2. `release-99_notif-published.yml` (3 değişiklik - Matrix odaları)

```yaml
# Satır 26-32: Matrix odaları - PEZKUWI ODALARINA DEĞİŞTİRİLMELİ
- name: '#polkadotvalidatorlounge:web3.foundation'  # KALDIR
- name: '#polkadot-announcements:parity.io'         # KALDIR
- name: '#kusama-announce:parity.io'                # KALDIR

# YENİ: Pezkuwi Discord/Matrix odaları eklenecek
- name: '#pezkuwi-announcements:pezkuwichain.io'
```

#### 3. `check-licenses.yml` (8 değişiklik - NPM paketi)

```yaml
# Satır 32, 70: scope
scope: "@paritytech"
# DEĞİŞTİR: scope: "@pezkuwichain"

# Satır 37, 45, 53, 75, 83, 91: License scanner
npx @paritytech/license-scanner scan ...
# NOT: Bu paket paritytech'e ait. Alternatif bulunmalı veya fork edilmeli.
```

### YÜKSEK ÖNCELİK

#### 4. `reusable-preflight.yml` (2 değişiklik - Yorum)

```yaml
# Satır 31, 99: Yorum satırları - runner dokümantasyonu
# https://github.com/paritytech/ci_cd/wiki/GitHub#paritytech-self-hosted-runners
# DEĞİŞTİR: Yorum güncellenebilir veya kaldırılabilir (işlevsel değil)
```

#### 5. `checks-quick.yml` (3 değişiklik)

```yaml
# Satır 116: npm scope
scope: "@paritytech"
# DEĞİŞTİR: scope: "@pezkuwichain"

# Satır 183-184: Docker image
image: "paritytech/tools:latest"
# paritytech/tools uses "nonroot" user
# NOT: Bu image paritytech'e ait. Alternatif gerekli.
```

#### 6. `review-bot.yml` (3 değişiklik - DEVRE DIŞI)

```yaml
# Satır 31, 33, 41: Review bot devre dışı
# Bu workflow zaten devre dışı bırakılmış - paritytech bağımlılığı nedeniyle
# ÖNERİ: Pezkuwi için kendi review bot'u kurulabilir
```

#### 7. `tests-misc.yml` (4 değişiklik)

```yaml
# Satır 24: Yorum - substrate PR referansı
# https://github.com/paritytech/substrate/pull/3778
# DEĞİŞTİR: https://github.com/paritytech/polkadot-sdk/pull/XXX (veya kaldır)

# Satır 204: Docker image
paritytech/node-bench-regression-guard:latest
# NOT: Bu image paritytech'e ait. Fork veya alternatif gerekli.

# Satır 248: Yorum
# https://github.com/paritytech/substrate/pull/6916
# DEĞİŞTİR: Güncellenebilir

# Satır 385: Revive URL (BIRAKILMALI - harici dependency)
https://github.com/paritytech/revive/releases/...
```

#### 8. `tests-evm.yml` (4 değişiklik - Harici Dependency)

```yaml
# Satır 43, 53, 63, 128: Revive ve EVM test suite
# BIRAKILMALI - Bunlar harici Parity projeleri, fork edilmemiş
# Revive compiler ve EVM test suite Parity'ye ait
```

### ORTA ÖNCELİK

#### 9. `misc-sync-templates.yml` (2 değişiklik)

```yaml
# Satır 7: Yorum - parachain template
# - https://github.com/pezkuwichain/pezkuwi-sdk-teyrchain-template
# DEĞİŞTİR: - https://github.com/pezkuwichain/pezkuwi-sdk/issues/25

# Satır 132: PSVM aracı
cargo install --git https://github.com/paritytech/psvm psvm
# NOT: PSVM Parity aracı - kullanılıyorsa fork edilmeli
```

#### 10. `check-semver.yml` (2 değişiklik - Yorum/Link)

```yaml
# Satır 151, 174: Forum linkleri
- https://forum.polkadot.network/t/psa-polkadot-sdk-to-use-semver
# DEĞİŞTİR: Pezkuwi forum linki veya kaldırılabilir
```

#### 11. `check-runtime-compatibility.yml` (1 değişiklik)

```yaml
# Satır 89: Polkadot API npm paketi
npx @polkadot-api/check-runtime@latest ...
# NOT: Bu paket Polkadot için. Pezkuwi uyumluluğu kontrol edilmeli.
```

#### 12. `check-runtime-migration.yml` (1 değişiklik)

```yaml
# Satır 89: Try-runtime CLI
curl -sL https://github.com/paritytech/try-runtime-cli/releases/...
# NOT: Try-runtime CLI Parity'ye ait. Fork veya uyumluluk kontrolü gerekli.
```

---

## Geliştirme ve Testnet Aşamaları İçin Davranış Planı

### DEV Aşaması (Mevcut)

**Trigger Kuralları:**
- `push` to `main` branch
- `pull_request` events
- `merge_group` events

**Önerilen Değişiklikler:**
1. Tüm workflow'lar `main` branch'te tetiklenmeli
2. Ağır testler (zombienet, benchmarks) sadece manuel tetiklenebilir
3. Quick-checks her PR'da zorunlu

```yaml
# DEV aşaması için önerilen trigger
on:
  push:
    branches: [main, develop]
  pull_request:
    types: [opened, synchronize, reopened, ready_for_review]
  merge_group:
```

### LOCAL Aşaması (2 Validator)

**Ek Workflow'lar:**
- Dual validator testleri
- Peer discovery testleri
- Consensus testleri

### ALPHA/BETA Aşamaları

**Değişiklikler:**
1. Release workflow'ları aktif
2. Docker image'ları DockerHub'a push
3. Binary artifact'lar release'lere eklenir

```yaml
# ALPHA/BETA için release trigger
on:
  push:
    tags:
      - 'v*-alpha*'
      - 'v*-beta*'
```

### STAGING/TESTNET Aşaması

**Değişiklikler:**
1. Tam zombienet test suite çalışır
2. Runtime upgrade testleri
3. Migration testleri zorunlu

```yaml
# TESTNET için trigger
on:
  push:
    tags:
      - 'v*-staging*'
      - 'v*-testnet*'
```

### MAINNET Aşaması

**Değişiklikler:**
1. Tüm güvenlik kontrolleri zorunlu
2. Dual sign-off gerekli
3. Canary deployment workflow'u aktif

---

## Harici Bağımlılıklar ve Alternatifler

### Değiştirilemeyecek Bağımlılıklar

Bu bağımlılıklar Parity ekosisteminin parçası ve fork edilmesi pratik değil:

| Dependency | Kullanım Yeri | Öneri |
| --- | --- | --- |
| `@polkadot-api/check-runtime` | Runtime uyumluluk | Uyumluluk testi yap, muhtemelen çalışır |
| `paritytech/revive` | EVM testleri | Harici olarak bırak |
| `paritytech/try-runtime-cli` | Migration testleri | Substrate fork'u olduğu için muhtemelen uyumlu |
| `paritytech/evm-test-suite` | EVM testleri | Harici olarak bırak |

### Fork Edilmesi Gerekenler

| Tool | Öncelik | Sebep |
| --- | --- | --- |
| `paritytech/tools` Docker image | YÜKSEK | CI container'ı |
| `paritytech/node-bench-regression-guard` | ORTA | Benchmark regresyon kontrolü |
| `@paritytech/license-scanner` | DÜŞÜK | Lisans taraması |
| `paritytech/psvm` | DÜŞÜK | Version yönetimi |

### Oluşturulması Gerekenler

| Kaynak | Öncelik | Açıklama |
| --- | --- | --- |
| Pezkuwi Matrix/Discord odaları | YÜKSEK | Release bildirimleri için |
| `pezkuwichain/tools` Docker image | YÜKSEK | CI için |
| Pezkuwi DockerHub registry | YÜKSEK | Image depolama |

---

## Uygulama Planı

### Faz 1 - Kritik Değişiklikler (Hemen)

1. [ ] `build-publish-images.yml` - polkadot→pezkuwi değişiklikleri
2. [ ] `release-99_notif-published.yml` - Matrix odaları güncelle/devre dışı bırak
3. [ ] Runner konfigürasyonu (ubuntu-latest kullanımı) - ZATEN YAPILDI

### Faz 2 - Yüksek Öncelik (1 Hafta İçinde)

1. [ ] `check-licenses.yml` - License scanner alternatifi
2. [ ] `checks-quick.yml` - NPM scope ve Docker image
3. [ ] `reusable-preflight.yml` - Yorum güncellemeleri
4. [ ] `review-bot.yml` - Pezkuwi review bot kurulumu (opsiyonel)

### Faz 3 - Orta Öncelik (2 Hafta İçinde)

1. [ ] Tüm yorum satırlarında paritytech→pezkuwichain
2. [ ] GitHub PR/issue link güncellemeleri
3. [ ] `misc-sync-templates.yml` - Template senkronizasyonu

### Faz 4 - Düşük Öncelik (İleride)

1. [ ] Forum linkleri (kendi forum kurulduktan sonra)
2. [ ] Harici tool fork'ları
3. [ ] NPM paket fork'ları

---

## Özet

| Kategori | Dosya Sayısı | Değişiklik Gereken | Tamam |
| --- | --- | --- | --- |
| CI/CD Kontrolleri | 12 | 7 | 5 |
| Test Workflow'ları | 7 | 3 | 4 |
| Build & Publish | 5 | 1 | 4 |
| Release Workflow'ları | 17 | 11 | 6 |
| Reusable Workflow'lar | 6 | 3 | 3 |
| Zombienet | 6 | 0 | 6 |
| Command & Bot | 7 | 2 | 5 |
| Misc | 10 | 4 | 6 |
| Benchmark | 3 | 0 | 3 |
| **TOPLAM** | **74** | **33** | **41** |

**Sonuç:** 74 workflow dosyasından 33'ü rebranding gerektiriyor. Bunların çoğu yorum satırları ve harici bağımlılıklar. Kritik işlevsel değişiklik gerektiren sadece 5-6 dosya var.
