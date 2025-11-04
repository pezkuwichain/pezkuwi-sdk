#!/usr/bin/env bash
# Insert all validator keys for Beta Testnet (Validators 1-8)
# AUTO-GENERATED from validator JSON file
# DO NOT EDIT MANUALLY - Regenerate using generate-key-insertion-script.py

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

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

    result=$(curl -s -H "Content-Type: application/json" \
        -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"${key_type}\",\"${seed}\",\"${public_key}\"]}" \
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


# =============================================================================
# VALIDATOR 1 - Validator-beta-1 (Port 9944)
# =============================================================================
echo -e "${BLUE}🔑 Validator 1 - Validator-beta-1${NC}"

insert_key 1 9944 "babe" "state open glance maze canyon cable cargo parent blind mystery cheese foot" "0x9e7490cfe0dd32860282dd4e74b5c40c9237fb6e478066c334c931742308e008"
insert_key 1 9944 "gran" "kick surge tube arrange enforce witness mouse spray disease inquiry inch stamp" "0x16da26f049b37e9b03d14439457ac164f14f5174b1f10ddab2e0dc3ef7903675"
insert_key 1 9944 "para" "march cancel try improve lunch phrase rebuild initial theme snap giant dry" "0xc62b764ea84411f550818a41a6abacddb9388cefbc22412f2bee335e9709a717"
insert_key 1 9944 "asgn" "nephew dignity smoke sunny access soon breeze inch dove park fault museum" "0xa4c75d886439ae7d764cc600051af1d2128d2a8a1cdf3d642eda76824ccaf518"
insert_key 1 9944 "audi" "vicious mixed hour wage clog yellow elbow zoo clock better rhythm diagram" "0x66e821a23729878d8b7d04c69ec0fed9b0f5facae48917c1b83a4ad15cf00720"
insert_key 1 9944 "beef" "practice cable fog idea rigid hybrid digital snack setup right advance romance" "0x0281ba169080b261add0cd22bfb47477eddab854caa17423b12ddeed6504b04aef"

echo ""


# =============================================================================
# VALIDATOR 2 - Validator-beta-2 (Port 9945)
# =============================================================================
echo -e "${BLUE}🔑 Validator 2 - Validator-beta-2${NC}"

insert_key 2 9945 "babe" "nature pink baby physical hotel dirt soon meadow coin employ enroll remind" "0xbef3df377441fe6ad64b81e84c8d201bc5c0bc2bb2e1a85f1e84927c62c5572b"
insert_key 2 9945 "gran" "rent little raven auction error goat error water twice defy hard slab" "0x806cfed7a6d025bc2e7a195bcd5ac50317f824ce6971cf30f95c6a621d6e55bf"
insert_key 2 9945 "para" "mystery rescue elbow update effort path sleep rather deer undo school size" "0xfca6fd2976ae3ea397b15cdc5b0b044fe90a2094328edfa013db80cf04ba1a67"
insert_key 2 9945 "asgn" "curious step vast scan couch episode maid trap crazy swap junior slab" "0xfaad0803c450bf084ca3035bd1d69481db58444b32dd0eaa55de69a5314b0559"
insert_key 2 9945 "audi" "race frequent goose clean labor differ credit lawn lab risk ocean black" "0x7aeed77ed10108ead6c58dd45dd987aba28ea6e9cdf90f8893112c08bb0d4251"
insert_key 2 9945 "beef" "industry flush stairs zoo world width dentist special life retire suffer myth" "0x020177ca58f45047c737e6e02adba1ac3520eb881ea38ad39751d32d7c89d5efaa"

echo ""


# =============================================================================
# VALIDATOR 3 - Validator-beta-3 (Port 9946)
# =============================================================================
echo -e "${BLUE}🔑 Validator 3 - Validator-beta-3${NC}"

insert_key 3 9946 "babe" "cycle judge gentle cute spirit crunch build flee popular cube wagon void" "0x2825cb78cb345b078a189e6a280916f7771df991113e587b9a0694df33abe909"
insert_key 3 9946 "gran" "prefer sugar friend wagon about love blouse coast table future lonely slice" "0xcf2862bbfabf42dfc361e2fd0ee860e55e80f093ad767679d9687e822a04c7a4"
insert_key 3 9946 "para" "where reject camp tail clock plate library apple draw once float ranch" "0x4cdb404f66b7409a6e7fee2eab92aa738c8cbd37d1343b0e0818deb4637d3e0e"
insert_key 3 9946 "asgn" "craft hill receive alarm inner use cereal assume boost castle enhance culture" "0xf054e7834c042696fbbf56d17247924836a6957cc6ec8366455c9b459b51fa4d"
insert_key 3 9946 "audi" "fatal hazard federal cushion cousin spend weapon script boil vicious delay fire" "0xd2c4ffc56fd8504f5fde789d3ba895f65c0c7200f68cfe789a4316e68277e72f"
insert_key 3 9946 "beef" "kiss foil assist bind duty concert van fold reveal weird design rescue" "0x023792b160a004c98d56afc29843ee9aa567a80129794373df9b3f4d883cfd4dfc"

echo ""


# =============================================================================
# VALIDATOR 4 - Validator-beta-4 (Port 9947)
# =============================================================================
echo -e "${BLUE}🔑 Validator 4 - Validator-beta-4${NC}"

insert_key 4 9947 "babe" "evoke second recipe turn salad warfare mix sense cry impact demise avocado" "0x9236bf5cde2e1ce44cd2eceece98b876750cb39a8887bcc1d7c0b72c186b4f6c"
insert_key 4 9947 "gran" "sea resemble monitor fetch quit cotton amused settle limit venue frequent electric" "0x4d7787f6d6b189b64286efdf524a5d73be1eaf4a4b1d245529090b698285a0d4"
insert_key 4 9947 "para" "rubber decorate coyote grass solar butter melt ginger smile flush dash monitor" "0xb27fea579fdc321ebac8975717c17a7b4036bb06549c41ac803c888b9104a256"
insert_key 4 9947 "asgn" "tomato glad flower miracle duty hundred filter gain clay butter twist chronic" "0x9ef9bf97dfc3d22cd4e17a331165afbf0a562a9a86e776592c64ba87c4162d0a"
insert_key 4 9947 "audi" "group trial problem lesson angle grief agent harvest pattern identify approve security" "0x6abffb32a990117c3d8c74f090b287d5888af97b61a3964a2c54cddc614e4d71"
insert_key 4 9947 "beef" "ridge way rhythm renew mix city element obtain prepare glass exist hope" "0x02ca2cac4bd7d21c1c3f7b256233827d8546471d83e1eeb37408767adc66bac7d0"

echo ""


# =============================================================================
# VALIDATOR 5 - Validator-beta-5 (Port 9948)
# =============================================================================
echo -e "${BLUE}🔑 Validator 5 - Validator-beta-5${NC}"

insert_key 5 9948 "babe" "clap screen soap once near guilt accuse hamster knife drink purity skull" "0x74270a49be316a233279af6480ac8de5479bc45426e884fbdb50c6247e62d44b"
insert_key 5 9948 "gran" "innocent fix endless engine yellow smoke venture answer before dentist trend pulse" "0x9ae1139d89f5ae5e9a56a7a7d1375c49b742315c25712128fad16c841da2dba9"
insert_key 5 9948 "para" "peanut catch embody spy orbit design occur series cricket ski ketchup impose" "0xf49d1b0ebff5fbf31cba037085552fd78e4218a19045cdd5f2e1f73840156a75"
insert_key 5 9948 "asgn" "orchard globe member blue install rude cement luxury grant cause exit expect" "0xb8f786ac5db52f3713ca1ff58f8d708332f2c0556d55a62195a52d4ca1bb2e7e"
insert_key 5 9948 "audi" "pistol focus above decade weekend matter claw drift glad worry quarter slice" "0x3e1c338ba320b747769d2d40ece3fc3da306b90096ab381bad1be6bc5a813d57"
insert_key 5 9948 "beef" "mother model label zoo mouse detail cost pet umbrella rate proud unable" "0x033cae08ede6b1c3597dfce1f38c5ceddc4b69d045de4b51f17e2e8fce1cda27ca"

echo ""


# =============================================================================
# VALIDATOR 6 - Validator-beta-6 (Port 9949)
# =============================================================================
echo -e "${BLUE}🔑 Validator 6 - Validator-beta-6${NC}"

insert_key 6 9949 "babe" "tired youth educate hover cross plastic gate giraffe gorilla rescue section idle" "0xf4fc5deb49aa9178f622e711a9d453d967507231a6e2d62d726ce7279316970a"
insert_key 6 9949 "gran" "hedgehog isolate jump safe march fame year mosquito smooth lunch portion solution" "0x7eb7cfb7ac00f11ed7f63697cdb5d0562bba6b90c1901fa00365db075bab05e9"
insert_key 6 9949 "para" "industry thank parade tiger reunion usual kidney high beef divide jaguar smile" "0x5a3fa17b77d3afc3dd1e0b7bf1d2faf95ba14eb2ee370476e5f2f78a8175a957"
insert_key 6 9949 "asgn" "cluster outdoor denial disorder donor rich senior glance display barrel theory pupil" "0x4e232e6efb3be2c4f638fac57330f606fc54bbc3b8d490a5bb520c67c0d7f248"
insert_key 6 9949 "audi" "supreme warfare trust assault where model grain advice broccoli nothing key embody" "0x244d0ea5c6b4a4f4e23912aab77498689e9f355edb63a82ebfc9b9bb08176f0d"
insert_key 6 9949 "beef" "cloth lock grocery actor trend patch grace salad fun mass plate notice" "0x0338f2ef200a5048b35fcfae936a2dd5dffdabc77eb61d707de35f9eba8b3d37e3"

echo ""


# =============================================================================
# VALIDATOR 7 - Validator-beta-7 (Port 9950)
# =============================================================================
echo -e "${BLUE}🔑 Validator 7 - Validator-beta-7${NC}"

insert_key 7 9950 "babe" "pave mixed faith vivid nice trust exercise public kind afford fury army" "0x6898b08fe0c6dfe79750598765efe06cbfaef01da9b110599d05a5b4824ddf38"
insert_key 7 9950 "gran" "cliff entire pause adjust parent tissue enhance weasel east pink art meat" "0x57f18c1620086209773c6d8243af3eae873a63cda0c64cc3bcb02ff3ee41a863"
insert_key 7 9950 "para" "return diet teach excess under warm veteran pride exotic talk exhaust mobile" "0x1e0ee1b33614379148cd69bb79fb2d21c98a8837bab912e96b72e5a2bbb46d01"
insert_key 7 9950 "asgn" "gown differ atom rebuild observe museum word nest upset large boring exact" "0x48f66d8b963b576da719b2350a0242c624b720439ed764c206107647d8a6d872"
insert_key 7 9950 "audi" "artwork town pyramid stumble sugar neither clutch void soon wine slab slide" "0xe462edbb26c14d1772570ce751ace21b271b0f0f472fc434c84f708b9bb42309"
insert_key 7 9950 "beef" "curious abstract home session famous canvas spin forest wing appear warm install" "0x03e783496d6ff583a60ec44ef9fe7ea375c9e31841a1f7fe48a8eaec0093826d78"

echo ""


# =============================================================================
# VALIDATOR 8 - Validator-beta-8 (Port 9951)
# =============================================================================
echo -e "${BLUE}🔑 Validator 8 - Validator-beta-8${NC}"

insert_key 8 9951 "babe" "clinic father over industry blame coast trophy walnut panel giant barely aunt" "0x520e1de89062667c8980666fd8cc83914d43ce5dd69fa4db2bec7a23bec16e61"
insert_key 8 9951 "gran" "invite mule belt fix risk piece grant benefit type park best aisle" "0x7bab34ff06d0e24da37e861bc9683381934e390febe05cec2060b5d4b9c8d746"
insert_key 8 9951 "para" "close render flip cable sail drop job drum kiss brief corn wild" "0x4e169330a7089ffc0be8f85edb9b5c35556bee1a0111f14dc7ee00cd43478232"
insert_key 8 9951 "asgn" "pipe crack swing minimum supply divide tenant solution foil prevent depend wedding" "0xdc8556675a56246d3227c4544e58a602df65b154e09086c0b2b0aa879dfab249"
insert_key 8 9951 "audi" "faint tunnel east jeans raise bundle grief unaware gospel solar dish number" "0x1ec224f7441c0f95d817ccbf0cce3d1cdbd0ffb0f8169654672570dbd6e66608"
insert_key 8 9951 "beef" "live impact struggle furnace sentence sail true spin waste museum symbol able" "0x030bd33780a8a632630c3dd75624c4e6d1ff3fdfa58d88ddebe087e237cc0cd11d"

echo ""


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
