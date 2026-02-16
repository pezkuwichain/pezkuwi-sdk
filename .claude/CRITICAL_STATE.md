# PEZKUWI MAİNNET KRİTİK DURUM

**Son Guncelleme:** 2026-02-16 UTC
**Bu dosyayi her oturum basinda OKU!**

---

## 1. VPS ERİŞİMİ

**BAĞLANTI:** `ssh root@<IP>` - Tum VPS'lere root erisimi var.
**Toplam:** 18 VPS | 21 Validator (mainnet) + 4 Validator (zagros) | 4 Collator (mainnet) + 2 Collator (zagros)

| VPS | IP | Rol | Calisan Node'lar |
|-----|-----|-----|------------------|
| VPS1 | 37.60.230.9 | Web + CI | Landing page + transfer service + CI runner |
| VPS2 | 62.146.235.186 | Merkez | mainnet val [5,6,7,21] + telemetry + CI runner |
| **VPS3** | **217.77.6.126** | **Ana Sunucu (94GB)** | mainnet val [1,2,3,4] + mainnet col [Azad(AH), Erin(People)] + zagros val [1,2,3,4] + zagros col [Azad(AH), Erin(People)] |
| VPS-A | 217.77.15.51 | Validator | mainnet val [14] |
| VPS-B | 161.97.183.44 | Validator | mainnet val [13] |
| VPS-C | 161.97.185.100 | Validator | mainnet val [12] |
| VPS-D | 109.123.229.159 | Validator | mainnet val [16] |
| VPS-E | 161.97.116.241 | Validator | mainnet val [17] |
| VPS-F | 46.250.241.121 | Validator | mainnet val [18] |
| VPS-G | 164.68.121.181 | Validator | mainnet val [19] |
| VPS-H | 158.220.93.23 | Validator | mainnet val [20] |
| VPS-I | 207.180.194.103 | Validator | mainnet val [11] |
| VPS-J | 173.249.57.228 | Collator | mainnet col [Beritan(AH), Firaz(People)] |
| VPS-K | 173.249.48.125 | Bos | BOS (binary var, kurulmamis) |
| VPS-L | 167.86.70.241 | Validator | mainnet val [10] |
| VPS-M | 167.86.108.190 | Validator | mainnet val [9] |
| VPS-N | 207.180.233.147 | Validator | mainnet val [8] |
| VPS-O | 178.18.252.120 | Validator | mainnet val [15] |

**NOT:** Zagros testnet artik SADECE VPS3'te calisiyor. Diger VPS'lerdeki zagros servisleri ve verileri 2026-02-15'te temizlendi (~68GB bos alan kazanildi).

---

## 2. MEVCUT NETWORK DURUMU

### 2.1 Mainnet (PezkuwiChain)

#### Relay Chain
- **Durum:** CALISIYOR
- **spec_version:** 1_020_004
- **Block:** ~11,553 (15 Subat 2026)
- **Peers:** 24
- **Validator Sayisi:** 21
- **Bootnode:** `/ip4/217.77.6.126/tcp/30333/p2p/12D3KooWHY3k8ksTjT7izsUTbns1QLs8TraFVMcANtYhYKB4N69P`
- **RPC:** VPS3 port 9944

#### Teyrchains
| Para ID | Isim | Durum | spec_version | Block | Peers |
|---------|------|-------|-------------|-------|-------|
| 1000 | Asset Hub | CALISIYOR | 1_020_004 | ~11,009 | 1 |
| 1004 | People Chain | CALISIYOR | 1_020_007 | ~17,200 | 1 |

#### Mainnet Servis Isimleri (VPS3)
- `pez-mainnet-validator-1` ... `pez-mainnet-validator-4`
- `pez-mainnet-azad` (Asset Hub collator)
- `pez-mainnet-erin` (People Chain collator)

### 2.2 Zagros Testnet (Eski Mainnet)

**Genesis Hash:** `0xbb4a61ab...`

#### Relay Chain
- **Durum:** CALISIYOR
- **spec_version:** 1_020_004
- **Block:** ~224,955 (15 Subat 2026)
- **Peers:** 5
- **Validator Sayisi:** 4 (21'den dusuruldu, 2026-02-15)
- **RPC:** VPS3 port 9948

#### Teyrchains
| Para ID | Isim | Durum | spec_version | Block | Peers |
|---------|------|-------|-------------|-------|-------|
| 1000 | Asset Hub | CALISIYOR | 1_020_003 | ~222,233 | 0 |
| 1004 | People Chain | CALISIYOR | 1_020_003 | ~222,606 | 0 |

#### Zagros Servis Isimleri (VPS3)
- `zagros-validator-1` ... `zagros-validator-4`
- `zagros-azad` (Asset Hub collator)
- `zagros-erin` (People Chain collator)

#### Zagros Validator Azaltma (2026-02-15)
- Onceki: 21 validator (tum VPS'lerde dagitilmis)
- Sonraki: 4 validator (sadece VPS3)
- Yontem: `sudo(system.setStorage)` ile `ValidatorsToRetire` storage'a 17 validator yazildi
- Session 376'da GRANDPA 21→4 degisti
- `deregister_validators` subxt encoding hatasi nedeniyle calismadi, `system.setStorage` kullanildi

---

## 3. COLLATOR KEY'LERİ

### Asset Hub Collator'lari
```
Azad:
  public: 0x7c8c6f463d124a601fbc7d425daad82651193f35730957982519dbcff6d55f71
  ss58: 5Et1WgtNjUdMxyvHjAKGN8Nq1ivhUyANYjwKpCL8a46D8mCp

Beritan:
  public: 0x845fd9541c46c3dc4325ddcbae06596382771d943f49d9659bdbbed4abd4eb09
  ss58: 5F4GeiJE2oBcPdxfeYfWL4bu4iJfduzJk4aHhttemwhpscpQ
```

### People Chain Collator'lari
```
Erin:
  public: 0xb0f474e2f94868485e7269e503d6b327af392449c0878670021365ac7e173206
  ss58: 5G4iuN7MvkhdwN4ikZd9uijBzxV78LUWQro3rc9HrfWWzeuS

Firaz:
  public: 0x7244ec68c6f873e386ef8039ad6e9436e5e97c0d28bab4499090b9443034eb04
  ss58: 5EeXnoiPoXko3Hqggy74oSgxCFKpkNoppqTcV9MWUQAtmZHj
```

---

## 4. SUDO KEY (FOUNDER)

```
Address: 5CyuFfbF95rzBxru7c9yEsX4XmQXUxpLUcbj9RLg9K1cGiiF
Scheme: sr25519
```

---

## 4.1 TEST WALLET

```
Address: 5DXv3Dc5xELckTgcYa2dm1TSZPgqDPxVDW3Cid4ALWpVjY3w
Note: Mnemonic stored locally, NOT in repo. Check /home/mamostehp/res/
```

---

## 5. SONRAKİ ADIMLAR (YAPILMASI GEREKEN)

### Tamamlanan Isler
- [x] Runtime upgrade 1_020_002 → 1_020_003 (uc chain)
- [x] Runtime upgrade 1_020_003 → 1_020_004 (uc chain)
- [x] NominationPoolsApi - Asset Hub runtime'a eklendi ve deploy edildi
- [x] StakingApi - Asset Hub runtime'a eklendi ve deploy edildi
- [x] Trust Score sistemi migration'lari (People Chain)
- [x] MinJoinBond/MinCreateBond sudo ile ayarlandi (Asset Hub)
- [x] Tiki pallet Collection 0 olusturuldu (XCM ile)
- [x] Zagros validator sayisi 21→4 dusuruldu
- [x] Zagros gereksiz validator servisleri temizlendi (~68GB kazanildi)
- [x] Zagros collator'lara --rpc-cors all eklendi
- [x] Zagros nginx Host header duzeltildi
- [x] Noter delegasyonu ile staking score sistemi (People Chain spec 1_020_007)
- [x] 21 validatore Noter tikisi verildi (XCM batch_all ile, 2026-02-16)
- [x] Relay Chain'den pezpallet_staking_score kaldirildi (sonraki relay upgrade icin hazir)

### Bekleyen Isler
1. [ ] Nova Wallet uyumluluk testi (NominationPoolsApi eklenince)
2. [ ] Zagros teyrchain'leri 1_020_004'e upgrade et (relay zaten 1_020_004)
3. [ ] Public RPC endpointlerini test et ve dokumanla
4. [x] People Chain runtime upgrade 1_020_006 → 1_020_007 (noter delegasyonu deploy)
5. [x] Relay Chain: pezpallet_staking_score runtime'dan cikarildi (kod temizlendi, sonraki build'de aktif)
6. [ ] Bot + noter workflow gelistir (staking verisi toplama ve People Chain'e gonderme)

---

## 6. MAINNET WALLETS DOSYASI

**Konum:** `/home/mamostehp/res/MAINNET_WALLETS_20260128_235407.json`

---

## 7. RPC BAGLANTILARI

### Mainnet (Direkt)
| Chain | Endpoint |
|-------|----------|
| Relay | `ws://217.77.6.126:9944` |
| Asset Hub | `ws://217.77.6.126:40944` |
| People Chain | `ws://217.77.6.126:41944` |
| Public RPC | `wss://rpc.pezkuwichain.io` |

### Zagros Testnet (Direkt)
| Chain | Endpoint |
|-------|----------|
| Relay | `ws://217.77.6.126:9948` |
| Asset Hub | `ws://217.77.6.126:40948` |
| People Chain | `ws://217.77.6.126:41948` |

### Zagros Testnet (Public)
| Chain | Endpoint |
|-------|----------|
| Relay | `wss://zagros-rpc.pezkuwichain.io` |
| Asset Hub | `wss://zagros-asset-hub-rpc.pezkuwichain.io` |
| People Chain | `wss://zagros-people-rpc.pezkuwichain.io` |

---

## 8. ONEMLI ARACLAR

### Runtime Upgrade Tool
**Konum:** `/home/mamostehp/pezkuwi-sdk/tools/runtime-upgrade/`
**Kullanim:** `cargo run --release -p runtime-upgrade -- relay --rpc ws://... --wasm path/to/wasm`

### Teyrchain Registration Script
**Konum:** `/home/mamostehp/pezkuwi-sdk/vendor/pezkuwi-subxt/subxt/examples/register_teyrchain.rs`

### Zagros SetStorage Script
**Konum:** `/home/mamostehp/pezkuwi-sdk/vendor/pezkuwi-subxt/subxt/examples/zagros_set_retire.rs`
**Not:** subxt `deregister_validators` dynamic API encoding hatali. `system.setStorage` ile direkt raw SCALE bytes yazilmali.

### Storage Key'ler
| Pallet | Storage | Key (hex) |
|--------|---------|-----------|
| ValidatorManager | Prefix | `084e7f70a295a190e2e33fd3f8cdfcc2` |
| ValidatorManager | ValidatorsToRetire | `084e7f70a295a190e2e33fd3f8cdfcc2b664fa73499821e43a617aa0e82b17b1` |
| Session | QueuedKeys | `cec5070d609dd3497f72bde07fc96ba088dcde934c658227ee1dfafcd6e16903` |

---

*Bu dosya her oturum basinda okunmali ve her onemli degisiklikte guncellenmelidir.*
