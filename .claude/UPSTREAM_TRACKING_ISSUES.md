# Upstream Tracking Issues

Bu dosya, pezkuwi-sdk'da referans verilen TÜM upstream kaynakları listeler.
Her biri için `pezkuwichain/pezkuwi-sdk` reposunda bir tracking issue açılmalı.

## Issue Template

```markdown
## Upstream Reference Tracking

**Upstream URL:** https://github.com/paritytech/polkadot-sdk/issues/XXX
**Type:** Issue / Repository / Documentation

---

### Status Tracking:

- [ ] Pending - Upstream not yet resolved
- [ ] Resolved - Fix merged upstream
- [ ] Evaluated - Assessed if needed for PezkuwiChain
- [ ] Applied - Fix applied to our chain
- [ ] Closed - Upstream issue closed
- [ ] Skipped - Not relevant for us

**Last Check:** YYYY-MM-DD
**Next Check:** YYYY-MM-DD

---

### Notes:
(Add context about this upstream reference)
```

---

## BÖLÜM 1: Polkadot SDK Issues (143 adet)

Bu issue'lar kodda `https://github.com/pezkuwichain/pezkuwi-sdk/issues/XXX` olarak referans verilmiş.
Gerçek upstream URL: `https://github.com/paritytech/polkadot-sdk/issues/XXX`

### Yüksek Öncelikli (8+ referans)
| Pezkuwi Issue # | Upstream # | Referans Sayısı | Açıklama |
|-----------------|------------|-----------------|----------|
| TBD | 23 | 8 | async-backing, slot-based collator |
| TBD | 40 | 8 | (context needed) |

### Orta Öncelikli (4-7 referans)
| Pezkuwi Issue # | Upstream # | Referans Sayısı |
|-----------------|------------|-----------------|
| TBD | 160 | 7 |
| TBD | 149 | 5 |
| TBD | 147 | 5 |
| TBD | 128 | 5 |
| TBD | 122 | 5 |
| TBD | 102 | 5 |
| TBD | 96 | 4 |
| TBD | 25 | 4 |
| TBD | 158 | 4 |
| TBD | 15 | 4 |

### Düşük Öncelikli (1-3 referans)
Toplam 131 adet daha issue var (2-190 arası).

Tam liste için çalıştır:
```bash
grep -rho "github.com/pezkuwichain/pezkuwi-sdk/issues/[0-9]*" --include="*.rs" \
  | grep -v target | grep -v vendor \
  | sed 's|github.com/pezkuwichain/pezkuwi-sdk/issues/||' \
  | sort -n | uniq
```

---

## BÖLÜM 2: External Repository References

### A. xcm-format (Polkadot Fellows)
**Upstream:** https://github.com/polkadot-fellows/xcm-format
**Referans sayısı:** 8

Dosyalar:
- pezbridges/snowbridge/primitives/inbound-queue/src/v1.rs
- pezkuwi/xcm/docs/src/lib.rs
- pezkuwi/xcm/docs/src/fundamentals.rs
- docs/sdk/src/pezkuwi_sdk/xcm.rs

### B. try-runtime-cli
**Upstream:** https://github.com/paritytech/try-runtime-cli
**Referans sayısı:** 2

### C. trie
**Upstream:** https://github.com/paritytech/trie
**Referans sayısı:** 1

### D. parity-db
**Upstream:** https://github.com/paritytech/parity-db
**Referans sayısı:** 3

### E. parity-common
**Upstream:** https://github.com/paritytech/parity-common
**Referans sayısı:** 1

### F. arkworks-extensions
**Upstream:** https://github.com/paritytech/arkworks-extensions
**Referans sayısı:** 1

### G. chainextension-registry
**Upstream:** https://github.com/parity/chainextension-registry (verify)
**Referans sayısı:** 2

### H. polkadot-blockchain-academy
**Upstream:** https://github.com/polkadot-blockchain-academy
**Referans sayısı:** 2

### I. srtool
**Upstream:** https://github.com/paritytech/srtool
**Referans sayısı:** (verify count)

### J. rustc-rv32e-toolchain
**Upstream:** https://github.com/parity/rustc-rv32e-toolchain
**Referans sayısı:** 1

### K. radium-0.7-fork
**Upstream:** Verify original source
**Referans sayısı:** 1

---

## BÖLÜM 3: Diğer Broken Links

### A. shawntabrizi.com
- `www.shawntabrizi.com/bizinikiwi-*` → `www.shawntabrizi.com/substrate-*`

### B. substrate-developer-hub
- `github.com/bizinikiwi-developer-hub/awesome-bizinikiwi` → `github.com/substrate-developer-hub/awesome-substrate`

### C. paritytech/bizinikiwi-archive
- Bu repo artık mevcut değil, alternatif bul veya sil

---

## Toplu Issue Oluşturma Scripti

```bash
#!/bin/bash
# Polkadot SDK issues için tracking issue oluştur

ISSUES=(2 3 4 5 7 8 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 31 32 33 34 35 36 37 38 39 40 41 43 44 45 47 48 49 50 51 53 55 57 60 74 76 77 78 79 80 81 82 83 84 86 87 88 89 90 91 92 93 94 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 115 116 117 118 119 120 121 122 123 124 125 126 127 128 129 130 131 132 133 134 135 136 139 140 141 142 143 144 145 146 147 148 149 150 151 152 153 154 155 156 157 158 159 160 161 162 163 164 165 166 167 168 169 170 171 172 173 174 175 176 177 178 179 180 181 182 183 184 185 186 187 188 189 190)

for issue_num in "${ISSUES[@]}"; do
  gh issue create \
    --repo pezkuwichain/pezkuwi-sdk \
    --title "[Upstream Tracking] Polkadot SDK #$issue_num" \
    --body "## Upstream Reference Tracking

**Upstream URL:** https://github.com/paritytech/polkadot-sdk/issues/$issue_num
**Type:** Issue

---

### Status Tracking:

- [x] Pending - Upstream not yet resolved
- [ ] Resolved - Fix merged upstream
- [ ] Evaluated - Assessed if needed for PezkuwiChain
- [ ] Applied - Fix applied to our chain
- [ ] Closed - Upstream issue closed
- [ ] Skipped - Not relevant for us

**Last Check:** $(date +%Y-%m-%d)
**Next Check:** $(date -d '+1 month' +%Y-%m-%d)

---

### Notes:
Automatically created tracking issue.
" \
    --label "upstream-tracking"
done
```

---

## Kod Güncellemesi (Issue'lar oluşturulduktan sonra)

Issue'lar oluşturulduktan sonra, mapping dosyası oluşturun:

```bash
# mapping.txt formatı:
# UPSTREAM_NUM PEZKUWI_NUM
# 23 1001
# 40 1002
# ...
```

Sonra bu script ile güncelleme yapın:
```bash
while read upstream pezkuwi; do
  find . -name "*.rs" -not -path "./target/*" -not -path "./vendor/*" \
    -exec sed -i "s|pezkuwichain/pezkuwi-sdk/issues/$upstream|pezkuwichain/pezkuwi-sdk/issues/$pezkuwi|g" {} \;
done < mapping.txt
```

---

*Son güncelleme: 2025-12-23*
