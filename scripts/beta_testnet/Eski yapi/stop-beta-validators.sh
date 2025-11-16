#!/usr/bin/env bash
# Stop all Beta Testnet validators

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}🛑 Stopping Beta Testnet Validators...${NC}"
echo ""

for i in 1 2 3 4 5 6 7 8; do
    PID_FILE="/tmp/beta-validator-$i.pid"
    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        if ps -p "$PID" > /dev/null 2>&1; then
            echo -e "${GREEN}Stopping Validator $i (PID: $PID)...${NC}"
            kill "$PID"
            rm "$PID_FILE"
        else
            echo -e "${YELLOW}Validator $i not running${NC}"
            rm "$PID_FILE"
        fi
    else
        echo -e "${YELLOW}Validator $i PID file not found${NC}"
    fi
done

echo ""
echo -e "${GREEN}✅ All validators stopped${NC}"
