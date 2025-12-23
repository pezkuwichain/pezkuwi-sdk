# Polkadot SDK Issue Tracking

Bu dosya, pezkuwi-sdk kodunda referans verilen Polkadot SDK issue'larını listeler.
Her biri için kendi repomuzda bir tracking issue açılmalı.

## Workflow

1. Her Polkadot SDK issue için `pezkuwichain/pezkuwi-sdk` reposunda yeni issue aç
2. Issue açıklaması: "Track Polkadot SDK issue #XXX"
3. Bot haftalık olarak Polkadot'u tarar ve issue durumlarını günceller
4. Discord'a bildirim gönderilir

## Toplu Issue Oluşturma Scripti

```bash
# GitHub CLI ile toplu issue oluşturma örneği
for issue_num in 2 3 4 5 7 8 10 11 12 ...; do
  gh issue create \
    --repo pezkuwichain/pezkuwi-sdk \
    --title "Track Polkadot SDK #$issue_num" \
    --body "This issue tracks upstream changes in https://github.com/paritytech/polkadot-sdk/issues/$issue_num"
done
```

## Issue Listesi (143 adet)

| Pezkuwi Issue # | Polkadot SDK Issue # | Context |
|-----------------|---------------------|---------|
| TBD | #2 | bizinikiwi/pezframe/assets/src/mock.rs - Thread local vars |
| TBD | #3 | templates/minimal/runtime/src/lib.rs |
| TBD | #4 | bizinikiwi/pezframe/nomination-pools |
| TBD | #5 | election-provider-support - max voters |
| TBD | #7 | network-gossip - DoS protection |
| TBD | #8 | state-trie-migration - storage |
| TBD | #10 | election-provider-multi-phase |
| TBD | #11 | pezframe/support migrations |
| TBD | #12 | client/service |
| TBD | #13 | trait_based_programming docs |
| TBD | #14 | remote-externalities CI errors |
| TBD | #15 | network service - unimplemented |
| TBD | #16 | election-provider-multi-phase |
| TBD | #17 | pezframe_system_accounts docs |
| TBD | #18 | blockchain backend time complexity |
| TBD | #19 | offchain storage |
| TBD | #20 | network sync tests |
| TBD | #21 | staking ledger |
| TBD | #22 | rpc-api system call future |
| TBD | #23 | async-backing / slot-based collator |
| TBD | #24 | network protocol Status |
| TBD | #25 | templates (minimal, solochain) |
| TBD | #26 | rpc-api system |
| TBD | #27 | network service conflicts |
| TBD | #28 | staking stash account |
| TBD | #29 | tracing types |
| TBD | #31 | consensus inherents |
| TBD | #32 | network sync block request |
| TBD | #33 | lottery pezpallet |
| TBD | #34 | society pezpallet |
| TBD | #35-190 | (remaining issues to be mapped) |

## Öncelik Sıralaması

**P0 - Kritik (CI'ı etkileyen):**
- #97, #143, #177 (OmniNode)
- #174-186 (configuration)

**P1 - Önemli (Core functionality):**
- #23 (async-backing)
- #15, #27 (network)
- #21 (staking)

**P2 - Normal:**
- Diğer tüm issue'lar

---
*Son güncelleme: 2025-12-23*
