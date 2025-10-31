#!/bin/bash
set -e

echo "🚀 Starting Dev Network Test Suite..."
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create reports directory
mkdir -p test-reports

# 1. Build chain spec
echo -e "${BLUE}📋 Step 1: Building chain spec...${NC}"
./target/release/pezkuwi build-spec --chain=dev > chain-specs/dev/spec.json
./target/release/pezkuwi build-spec --chain=dev --raw > chain-specs/dev/spec-raw.json
echo -e "${GREEN}✅ Chain spec created${NC}\n"

# 2. Start node
echo -e "${BLUE}🔧 Step 2: Starting dev node...${NC}"
./target/release/pezkuwi \
    --chain=chain-specs/dev/spec-raw.json \
    --alice \
    --tmp \
    --rpc-port 9944 \
    --rpc-external \
    --rpc-cors all \
    --unsafe-rpc-external \
    > test-reports/dev-node.log 2>&1 &

NODE_PID=$!
echo -e "${GREEN}✅ Node started with PID: $NODE_PID${NC}"

# 3. Wait for node to be ready
echo -e "${YELLOW}⏳ Waiting for node to be ready...${NC}"
sleep 15
echo -e "${GREEN}✅ Node is ready${NC}\n"

# 4. Run tests
echo -e "${BLUE}🧪 Step 3: Running tests...${NC}\n"
npm run test:dev

TEST_EXIT_CODE=$?

# 5. Cleanup
echo -e "\n${BLUE}🧹 Step 4: Cleaning up...${NC}"
kill $NODE_PID 2>/dev/null || true
echo -e "${GREEN}✅ Node stopped${NC}\n"

# 6. Final result
if [ $TEST_EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}═══════════════════════════════════════${NC}"
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    echo -e "${GREEN}═══════════════════════════════════════${NC}"
    exit 0
else
    echo -e "${RED}═══════════════════════════════════════${NC}"
    echo -e "${RED}❌ Some tests failed${NC}"
    echo -e "${RED}═══════════════════════════════════════${NC}"
    exit 1
fi
