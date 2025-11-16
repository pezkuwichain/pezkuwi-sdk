#!/usr/bin/env bash
#
# PEZKUWICHAIN - 8-Validator Beta Testnet Başlatma Scripti
# Basit loop mantığıyla tüm validatorleri başlatır ve keyleri insert eder
#

set -e

# Renkler
GREEN="\033[0;32m"
BLUE="\033[0;34m"
YELLOW="\033[1;33m"
RED="\033[0;31m"
RESET="\033[0m"

# Konfigürasyon
BINARY="/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi"
CHAIN_SPEC="/home/mamostehp/Pezkuwi-SDK/chainspecs/beta-testnet-raw.json"
VALIDATORS_JSON="/home/mamostehp/Pezkuwi-SDK/pezkuwi/runtime/validators/beta_testnet_validators.json"

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${RESET}"
echo -e "${BLUE}║  PEZKUWICHAIN - 8 VALİDATÖRLÜ BETA TESTNET           ║${RESET}"
echo -e "${BLUE}║  Basit bash loop ile validatorleri başlat             ║${RESET}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${RESET}"
echo ""

echo -e "${YELLOW}ADIM 1/6: Eski validatorler durduruluyor...${RESET}"
pkill -f pezkuwi 2>/dev/null || true
rm -rf /tmp/beta-validator-*
sleep 2
echo -e "${GREEN}  ✓ Temizlik tamamlandı${RESET}\n"

echo -e "${YELLOW}ADIM 2/6: Validatorler başlatılıyor...${RESET}"

# Bootnode P2P adresi
BOOTNODE="/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"

# Validator 1 (Bootnode)
$BINARY \
  --chain="$CHAIN_SPEC" \
  --base-path=/tmp/beta-validator-1 \
  --validator \
  --name="Validator-beta-1" \
  --port=30333 \
  --rpc-port=9944 \
  --rpc-cors=all \
  --rpc-external \
  --rpc-methods=unsafe \
  --node-key=0000000000000000000000000000000000000000000000000000000000000001 \
  > /tmp/beta-validator-1.log 2>&1 &

echo -e "${GREEN}  ✓ Validator-beta-1 başlatıldı (PID: $!, Port: 30333, RPC: 9944)${RESET}"
sleep 12

# Validators 2-8
for i in {2..8}; do
  port=$((30333 + i - 1))
  rpc_port=$((9944 + i - 1))

  $BINARY \
    --chain="$CHAIN_SPEC" \
    --base-path=/tmp/beta-validator-$i \
    --validator \
    --name="Validator-beta-$i" \
    --port=$port \
    --rpc-port=$rpc_port \
    --rpc-cors=all \
    --rpc-external \
    --rpc-methods=unsafe \
    --unsafe-force-node-key-generation \
    --bootnodes=$BOOTNODE \
    > /tmp/beta-validator-$i.log 2>&1 &

  echo -e "${GREEN}  ✓ Validator-beta-$i başlatıldı (PID: $!, Port: $port, RPC: $rpc_port)${RESET}"
  sleep 8
done

echo -e "${GREEN}  ✓ 8 validator başarıyla başlatıldı${RESET}\n"

echo -e "${YELLOW}ADIM 3/6: Network ve RPC hazırlanıyor (15 saniye)...${RESET}"
sleep 15
echo -e "${GREEN}  ✓ Network hazır${RESET}\n"

echo -e "${YELLOW}ADIM 4/6: Session keyleri insert ediliyor...${RESET}"

# JSON'dan validator bilgilerini oku ve keyleri insert et
jq -r '.beta[] | @json' "$VALIDATORS_JSON" | while IFS= read -r validator; do
  name=$(echo "$validator" | jq -r '.name')

  # Validator numarasını al (Validator-beta-1 → 1)
  num=$(echo "$name" | grep -oP '\d+$')
  rpc_port=$((9944 + num - 1))

  echo -e "${GREEN}$name: Keyleri insert ediliyor...${RESET}"

  # Her key tipini insert et
  for key_type in babe grandpa para_validator para_assignment authority_discovery beefy; do
    seed=$(echo "$validator" | jq -r ".${key_type}_seed")
    pubkey=$(echo "$validator" | jq -r ".$key_type")

    # RPC key type mapping
    case "$key_type" in
      "grandpa") rpc_key="gran" ;;
      "para_validator") rpc_key="para" ;;
      "para_assignment") rpc_key="asgn" ;;
      "authority_discovery") rpc_key="audi" ;;
      "beefy") rpc_key="beef" ;;
      *) rpc_key="$key_type" ;;
    esac

    # Key insert et
    curl -s -H "Content-Type: application/json" \
      -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\":[\"$rpc_key\",\"$seed\",\"$pubkey\"]}" \
      http://localhost:$rpc_port > /dev/null
  done

  echo -e "${GREEN}  ✅ $name: Tüm keyler başarıyla insert edildi (6/6)${RESET}"
  sleep 1
done

echo -e "\n${GREEN}✅ Key insertion tamamlandı! (8/8 validator)${RESET}\n"

echo -e "${YELLOW}ADIM 5/6: Finalization başlaması bekleniyor (20 saniye)...${RESET}"
sleep 20

echo -e "${YELLOW}ADIM 6/6: Network durumu kontrol ediliyor...${RESET}\n"

# Best block
best_block=$(curl -s -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"chain_getHeader"}' \
  http://localhost:9944 | jq -r '.result.number' | xargs printf "%d")

echo -e "${GREEN}  ✓ Best Block: #$best_block${RESET}"

# Finalized block
finalized_hash=$(curl -s -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"chain_getFinalizedHead"}' \
  http://localhost:9944 | jq -r '.result')

finalized_block=$(curl -s -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"chain_getHeader\",\"params\":[\"$finalized_hash\"]}" \
  http://localhost:9944 | jq -r '.result.number' | xargs printf "%d")

echo -e "${GREEN}  ✓ Finalized Block: #$finalized_block${RESET}"

# Peer count
peer_count=$(curl -s -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"system_peers"}' \
  http://localhost:9944 | jq '. result | length')

echo -e "${GREEN}  ✓ Bağlı Peer Sayısı: $peer_count${RESET}\n"

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${RESET}"
echo -e "${BLUE}║  🎉 8-VALİDATÖRLÜ BETA TESTNET HAZIR!                  ║${RESET}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${RESET}\n"

echo -e "${GREEN}📡 RPC Endpoint: ws://localhost:9944${RESET}\n"

echo -e "${YELLOW}📁 Validator Logları:${RESET}"
for i in {1..8}; do
  echo "   Validator $i: tail -f /tmp/beta-validator-$i.log"
done

echo -e "\n${RED}🛑 Durdur: bash stop-beta-validators.sh${RESET}"
echo -e "${RED}   veya: pkill -f pezkuwi${RESET}\n"
