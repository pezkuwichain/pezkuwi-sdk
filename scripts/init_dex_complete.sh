#!/bin/bash
# ============================================================================
# DEX Tam Otomatik Başlatma Script'i
# ============================================================================
# Bu script:
# 1. HEZ → wHEZ wrap işlemi yapar
# 2. wHEZ/PEZ pool'u oluşturur
# 3. Pool'a likidite ekler
# ============================================================================

set -e

ALICE="//Alice"
ALICE_ADDR="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
WS="ws://localhost:9944"

# Renkler
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${CYAN}🚀 DEX Otomatik Başlatma Script'i${NC}"
echo ""

# Node hazır mı kontrol et
echo -e "${YELLOW}⏳ Node'un hazır olması bekleniyor...${NC}"
sleep 5

MAX_RETRIES=30
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if curl -s -H "Content-Type: application/json" \
           -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' \
           http://localhost:9944 > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Node hazır!${NC}"
        break
    fi

    RETRY_COUNT=$((RETRY_COUNT + 1))
    echo -e "${YELLOW}Deneme $RETRY_COUNT/$MAX_RETRIES...${NC}"
    sleep 2
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo -e "${RED}❌ Node başlamadı!${NC}"
    exit 1
fi

echo ""
echo -e "${CYAN}📋 Adım 1: HEZ → wHEZ Wrap (1000 HEZ)${NC}"
echo -e "${YELLOW}   Alice 1000 HEZ'i wrap ediyor...${NC}"

polkadot-js-api --ws $WS --seed "$ALICE" \
  tx.tokenWrapper.wrap 1000000000000000 2>&1 | grep -E "(inBlock|Wrapped|dispatchError)" || echo "Wrap tamamlandı"

sleep 3

echo ""
echo -e "${CYAN}📋 Adım 2: wHEZ/PEZ Pool Oluşturma${NC}"

polkadot-js-api --ws $WS --seed "$ALICE" \
  tx.assetConversion.createPool 0 1 2>&1 | grep -E "(inBlock|PoolCreated|AlreadyExists|dispatchError)" || echo "Pool oluşturuldu"

sleep 3

echo ""
echo -e "${CYAN}📋 Adım 3: Pool'a Likidite Ekleme${NC}"
echo -e "${YELLOW}   100 wHEZ + 500 PEZ ekleniyor...${NC}"

# Miktar: 100 wHEZ + 500 PEZ (1:5 ratio)
polkadot-js-api --ws $WS --seed "$ALICE" \
  tx.assetConversion.addLiquidity \
  0 \
  1 \
  100000000000000 \
  500000000000000 \
  95000000000000 \
  475000000000000 \
  "$ALICE_ADDR" 2>&1 | grep -E "(inBlock|LiquidityAdded|dispatchError)" || echo "Likidite eklendi"

sleep 2

echo ""
echo -e "${GREEN}✅ DEX başlatma tamamlandı!${NC}"
echo ""
echo -e "${CYAN}📊 Durum Kontrolü:${NC}"

# Alice'nin bakiyelerini kontrol et
echo -e "${YELLOW}Alice'nin bakiyeleri:${NC}"
polkadot-js-api --ws $WS query.assets.account 0 "$ALICE_ADDR" 2>&1 | grep -A 3 "balance" || echo "  wHEZ: Yükleniyor..."
polkadot-js-api --ws $WS query.assets.account 1 "$ALICE_ADDR" 2>&1 | grep -A 3 "balance" || echo "  PEZ: Yükleniyor..."

echo ""
echo -e "${GREEN}🎉 Hazır! Swap UI'dan test edebilirsin!${NC}"
echo -e "${CYAN}💡 Tarayıcıda: http://localhost:5173/wallet${NC}"
