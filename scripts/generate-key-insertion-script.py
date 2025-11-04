#!/usr/bin/env python3
"""
Generate insert-all-beta-keys.sh from JSON validator file
This prevents manual copy-paste errors
"""

import json
import sys
from pathlib import Path

def main():
    # Check if JSON file path provided
    if len(sys.argv) < 2:
        print("❌ Error: Please provide the path to validator JSON file")
        print("")
        print("Usage:")
        print("  python3 generate-key-insertion-script.py /path/to/beta_testnet_validators_WITH_SEEDS_BACKUP.json")
        print("")
        print("Example:")
        print("  python3 generate-key-insertion-script.py /media/mamostehp/D/res/beta_testnet_validators_WITH_SEEDS_BACKUP.json")
        sys.exit(1)

    json_file = Path(sys.argv[1])

    if not json_file.exists():
        print(f"❌ Error: File not found: {json_file}")
        sys.exit(1)

    print(f"📖 Reading validators from: {json_file}")

    try:
        with open(json_file, 'r') as f:
            data = json.load(f)
    except Exception as e:
        print(f"❌ Error reading JSON: {e}")
        sys.exit(1)

    # Get validators from "beta" key
    validators = data.get('beta', [])

    if not validators:
        print("❌ Error: No validators found in 'beta' key")
        sys.exit(1)

    print(f"✅ Found {len(validators)} validators")

    # Use all 8 validators for single computer setup
    validators_to_use = validators[:8]

    # Generate the script
    output_file = Path("/home/mamostehp/Pezkuwi-SDK/scripts/insert-all-beta-keys.sh")

    print(f"🔧 Generating script: {output_file}")

    script_content = """#!/usr/bin/env bash
# Insert all validator keys for Beta Testnet (Validators 1-8)
# AUTO-GENERATED from validator JSON file
# DO NOT EDIT MANUALLY - Regenerate using generate-key-insertion-script.py

set -e

GREEN='\\033[0;32m'
BLUE='\\033[0;34m'
YELLOW='\\033[1;33m'
RED='\\033[0;31m'
NC='\\033[0m'

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   Beta Testnet - Bulk Key Insertion${NC}"
echo -e "${BLUE}   Auto-generated from JSON validator file${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}⚠️  SECURITY CHECK${NC}"
echo "This script will insert validator keys for 8 validators."
echo "Make sure all validators are running before proceeding."
echo ""
read -p "Press Ctrl+C to cancel, or Enter to continue..."
echo ""

# Function to insert key
insert_key() {
    local validator_num=$1
    local rpc_port=$2
    local key_type=$3
    local seed=$4
    local public_key=$5

    local rpc_url="http://127.0.0.1:${rpc_port}"

    echo -e "${GREEN}Validator ${validator_num}: Inserting ${key_type} key...${NC}"

    result=$(curl -s -H "Content-Type: application/json" \\
        -d "{\\"id\\":1,\\"jsonrpc\\":\\"2.0\\",\\"method\\":\\"author_insertKey\\",\\"params\\":[\\"${key_type}\\",\\"${seed}\\",\\"${public_key}\\"]}" \\
        "$rpc_url")

    if echo "$result" | grep -q '"result":null'; then
        echo -e "${GREEN}  ✅ Success${NC}"
        return 0
    else
        echo -e "${RED}  ❌ Failed${NC}"
        echo "  Response: $result"
        return 1
    fi
}

echo -e "${BLUE}🔑 Starting key insertion for 8 validators...${NC}"
echo ""

"""

    # Add validator keys
    for idx, validator in enumerate(validators_to_use, 1):
        rpc_port = 9943 + idx  # 9944, 9945, 9946, 9947

        script_content += f"""
# =============================================================================
# VALIDATOR {idx} - {validator.get('name', f'Validator-{idx}')} (Port {rpc_port})
# =============================================================================
echo -e "${{BLUE}}🔑 Validator {idx} - {validator.get('name', f'Validator-{idx}')}${{NC}}"

"""

        # BABE key
        script_content += f"""insert_key {idx} {rpc_port} "babe" "{validator['babe_seed']}" "{validator['babe']}"
"""

        # GRANDPA key
        script_content += f"""insert_key {idx} {rpc_port} "gran" "{validator['grandpa_seed']}" "{validator['grandpa']}"
"""

        # Para Validator key
        script_content += f"""insert_key {idx} {rpc_port} "para" "{validator['para_validator_seed']}" "{validator['para_validator']}"
"""

        # Para Assignment key
        script_content += f"""insert_key {idx} {rpc_port} "asgn" "{validator['para_assignment_seed']}" "{validator['para_assignment']}"
"""

        # Authority Discovery key
        script_content += f"""insert_key {idx} {rpc_port} "audi" "{validator['authority_discovery_seed']}" "{validator['authority_discovery']}"
"""

        # BEEFY key
        script_content += f"""insert_key {idx} {rpc_port} "beef" "{validator['beefy_seed']}" "{validator['beefy']}"
"""

        script_content += f"""
echo ""

"""

    # Add footer
    script_content += """
echo -e "${GREEN}═══════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ Key insertion complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}📝 Next steps:${NC}"
echo "1. Restart all validators to apply keys"
echo "2. Wait for network to start producing blocks"
echo "3. Check logs for 'Prepared block for proposing'"
echo ""
echo "Validator logs:"
echo "  Validator 1: tail -f /tmp/beta-validator-1.log"
echo "  Validator 2: tail -f /tmp/beta-validator-2.log"
echo "  Validator 3: tail -f /tmp/beta-validator-3.log"
echo "  Validator 4: tail -f /tmp/beta-validator-4.log"
echo "  Validator 5: tail -f /tmp/beta-validator-5.log"
echo "  Validator 6: tail -f /tmp/beta-validator-6.log"
echo "  Validator 7: tail -f /tmp/beta-validator-7.log"
echo "  Validator 8: tail -f /tmp/beta-validator-8.log"
"""

    # Write the script
    try:
        with open(output_file, 'w') as f:
            f.write(script_content)

        # Make executable
        output_file.chmod(0o755)

        print(f"✅ Script generated successfully!")
        print(f"")
        print(f"📁 Output: {output_file}")
        print(f"")
        print(f"🚀 Next steps:")
        print(f"   1. Start validators: ./scripts/start-all-beta-validators.sh")
        print(f"   2. Insert keys: ./scripts/insert-all-beta-keys.sh")
        print(f"")
        print(f"✅ Done! No manual copy-paste needed!")

    except Exception as e:
        print(f"❌ Error writing script: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
