# PEZKUWI MAİNNET KRİTİK DURUM

**Son Guncelleme:** 2026-02-14 UTC
**Bu dosyayi her oturum basinda OKU!**

---

## 1. VPS ERİŞİMİ

**BAĞLANTI:** `ssh root@<IP>` - Tum VPS'lere root erisimi var.
**Toplam:** 18 VPS | 21 Validator | 4 Collator

| VPS | IP | Rol | Calisan Node'lar |
|-----|-----|-----|------------------|
| VPS1 | 37.60.230.9 | Web + CI | Landing page + transfer service + CI runner |
| VPS2 | 62.146.235.186 | Merkez | val [5,6,7,21] + telemetry + CI runner |
| **VPS3** | **217.77.6.126** | **Ana Sunucu (94GB)** | val [1,2,3,4] + col [Azad(AH), Erin(People)] + bridge + zagros + CI runner |
| VPS-A | 217.77.15.51 | Validator | val [14] |
| VPS-B | 161.97.183.44 | Validator | val [13] |
| VPS-C | 161.97.185.100 | Validator | val [12] |
| VPS-D | 109.123.229.159 | Validator | val [16] |
| VPS-E | 161.97.116.241 | Validator | val [17] |
| VPS-F | 46.250.241.121 | Validator | val [18] |
| VPS-G | 164.68.121.181 | Validator | val [19] |
| VPS-H | 158.220.93.23 | Validator | val [20] |
| VPS-I | 207.180.194.103 | Validator | val [11] |
| VPS-J | 173.249.57.228 | Collator | col [Beritan(AH), Firaz(People)] |
| VPS-K | 173.249.48.125 | Bos | BOS (binary var, kurulmamis) |
| VPS-L | 167.86.70.241 | Validator | val [10] |
| VPS-M | 167.86.108.190 | Validator | val [9] |
| VPS-N | 207.180.233.147 | Validator | val [8] |
| VPS-O | 178.18.252.120 | Validator | val [15] |

---

## 2. MEVCUT NETWORK DURUMU

### Relay Chain (PezkuwiChain Mainnet)
- **Durum:** CALISIYOR
- **spec_version:** 1_020_003
- **Validator Sayisi:** 21
- **Bootnode:** `/ip4/217.77.6.126/tcp/30333/p2p/12D3KooWHY3k8ksTjT7izsUTbns1QLs8TraFVMcANtYhYKB4N69P`
- **RPC:** VPS3 port 9944

### Teyrchains
| Para ID | Isim | Durum | spec_version |
|---------|------|-------|-------------|
| 1000 | Asset Hub | CALISIYOR | 1_020_003 |
| 1004 | People Chain | CALISIYOR | 1_020_003 |

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
Mnemonic: REDACTED_MNEMONIC
Address: 5DXv3Dc5xELckTgcYa2dm1TSZPgqDPxVDW3Cid4ALWpVjY3w
```

---

## 5. SONRAKİ ADIMLAR (YAPILMASI GEREKEN)

### Runtime Upgrade Gerektiren Degisiklikler
1. [ ] **NominationPoolsApi** - Asset Hub runtime'a eklendi (kod hazir, deploy gerekli)
2. [ ] **StakingApi** - Asset Hub runtime'a eklendi (kod hazir, deploy gerekli)
3. [ ] Trust Score sistemi migration'lari (People Chain)

### Sudo ile Yapilmasi Gereken Ayarlar (Runtime Upgrade Sonrasi)
4. [ ] **MinJoinBond** - NominationPools palet'inde sudo ile ayarlanmali (Asset Hub)
5. [ ] **MinCreateBond** - NominationPools palet'inde sudo ile ayarlanmali (Asset Hub)

### Diger
6. [ ] Tiki pallet Collection 0 olusturma (XCM script hazir)
7. [ ] Nova Wallet uyumluluk testi (NominationPoolsApi eklenince)

---

## 6. MAINNET WALLETS DOSYASI

**Konum:** `/home/mamostehp/res/MAINNET_WALLETS_20260128_235407.json`

---

## 7. RPC BAGLANTILARI

| Chain | Endpoint |
|-------|----------|
| Relay | `ws://217.77.6.126:9944` |
| Asset Hub | `ws://217.77.6.126:40944` |
| People Chain | `ws://217.77.6.126:41944` |
| Public RPC | `wss://rpc.pezkuwichain.io` |

---

*Bu dosya her oturum basinda okunmali ve her onemli degisiklikte guncellenmelidir.*
