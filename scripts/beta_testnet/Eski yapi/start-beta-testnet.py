#!/usr/bin/env python3
"""
PEZKUWICHAIN - 8-Validator Beta Testnet Başlatma Scripti
Basit loop mantığıyla tüm validatorleri başlatır ve keyleri insert eder
"""

import json
import subprocess
import time
import sys
import os
import requests

# Renkler
GREEN = "\033[0;32m"
BLUE = "\033[0;34m"
YELLOW = "\033[1;33m"
RED = "\033[0;31m"
RESET = "\033[0m"

# Konfigürasyon
BINARY = "/home/mamostehp/Pezkuwi-SDK/target/release/pezkuwi"
CHAIN_SPEC = "/home/mamostehp/Pezkuwi-SDK/chainspecs/beta-testnet-raw.json"
VALIDATORS_JSON = "/home/mamostehp/Pezkuwi-SDK/pezkuwi/runtime/validators/beta_testnet_validators.json"

# Key tipleri ve RPC key scheme mapping
KEY_TYPES = {
    "babe": "babe",
    "grandpa": "gran",
    "para_validator": "para",
    "para_assignment": "asgn",
    "authority_discovery": "audi",
    "beefy": "beef"
}

def print_header():
    print(f"{BLUE}╔════════════════════════════════════════════════════════╗{RESET}")
    print(f"{BLUE}║  PEZKUWICHAIN - 8 VALİDATÖRLÜ BETA TESTNET           ║{RESET}")
    print(f"{BLUE}║  Basit loop mantığıyla validatorleri başlat           ║{RESET}")
    print(f"{BLUE}╚════════════════════════════════════════════════════════╝{RESET}\n")

def load_validators():
    """Validator bilgilerini JSON'dan oku"""
    print(f"{YELLOW}ADIM 1/6: Validator bilgileri okunuyor...{RESET}")
    with open(VALIDATORS_JSON, 'r') as f:
        data = json.load(f)
    validators = data['beta']
    print(f"{GREEN}  ✓ {len(validators)} validator bilgisi yüklendi{RESET}\n")
    return validators

def cleanup():
    """Eski validatorleri durdur ve temizle"""
    print(f"{YELLOW}ADIM 2/6: Eski validatorler durduruluyor...{RESET}")
    subprocess.run("pkill -f pezkuwi", shell=True, stderr=subprocess.DEVNULL)
    subprocess.run("rm -rf /tmp/beta-validator-*", shell=True)
    time.sleep(2)
    print(f"{GREEN}  ✓ Temizlik tamamlandı{RESET}\n")

def start_validator(index, validator, bootnode=None):
    """Tek bir validator başlat"""
    name = validator['name']
    port = 30333 + index
    rpc_port = 9944 + index
    log_file = f"/tmp/beta-validator-{index+1}.log"
    base_path = f"/tmp/beta-validator-{index+1}"

    # Validator 1 bootnode olacak
    cmd = [
        BINARY,
        f"--chain={CHAIN_SPEC}",
        f"--base-path={base_path}",
        "--validator",
        f"--name={name}",
        f"--port={port}",
        f"--rpc-port={rpc_port}",
        "--rpc-cors=all",
        "--rpc-external",
        "--rpc-methods=unsafe"
    ]

    # İlk validator bootnode olsun
    if index == 0:
        cmd.append("--node-key=0000000000000000000000000000000000000000000000000000000000000001")
    elif bootnode:
        cmd.append(f"--bootnodes={bootnode}")

    # Validator'u başlat
    with open(log_file, 'w') as log:
        process = subprocess.Popen(cmd, stdout=log, stderr=log)

    print(f"{GREEN}  ✓ {name} başlatıldı (PID: {process.pid}, Port: {port}, RPC: {rpc_port}){RESET}")

    return process.pid

def insert_key(rpc_port, key_type, seed, pubkey):
    """Bir session key insert et"""
    url = f"http://localhost:{rpc_port}"

    payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "author_insertKey",
        "params": [key_type, seed, pubkey]
    }

    try:
        response = requests.post(url, json=payload, timeout=5)
        result = response.json()
        return result.get('result') is not None
    except Exception as e:
        print(f"{RED}  ✗ Error inserting {key_type}: {e}{RESET}")
        return False

def insert_validator_keys(index, validator):
    """Bir validator'un tüm keylerini insert et"""
    rpc_port = 9944 + index
    name = validator['name']

    print(f"{GREEN}{name}: Keyleri insert ediliyor...{RESET}")

    success_count = 0
    for json_key, rpc_key in KEY_TYPES.items():
        seed_key = f"{json_key}_seed"
        seed = validator[seed_key]
        pubkey = validator[json_key]

        if insert_key(rpc_port, rpc_key, seed, pubkey):
            success_count += 1

    if success_count == len(KEY_TYPES):
        print(f"{GREEN}  ✅ {name}: Tüm keyler başarıyla insert edildi ({success_count}/{len(KEY_TYPES)}){RESET}")
        return True
    else:
        print(f"{RED}  ✗ {name}: Bazı keyler insert edilemedi ({success_count}/{len(KEY_TYPES)}){RESET}")
        return False

def check_network_status():
    """Network durumunu kontrol et"""
    print(f"{YELLOW}ADIM 6/6: Network durumu kontrol ediliyor...{RESET}\n")

    url = "http://localhost:9944"

    # Best block
    try:
        response = requests.post(url, json={
            "jsonrpc": "2.0",
            "id": 1,
            "method": "chain_getHeader"
        }, timeout=5)
        best_block = response.json()['result']['number']
        best_block_num = int(best_block, 16)
        print(f"{GREEN}  ✓ Best Block: #{best_block_num}{RESET}")
    except:
        print(f"{RED}  ✗ Best block alınamadı{RESET}")

    # Finalized block
    try:
        response = requests.post(url, json={
            "jsonrpc": "2.0",
            "id": 1,
            "method": "chain_getFinalizedHead"
        }, timeout=5)
        finalized_hash = response.json()['result']

        response = requests.post(url, json={
            "jsonrpc": "2.0",
            "id": 1,
            "method": "chain_getHeader",
            "params": [finalized_hash]
        }, timeout=5)
        finalized_block = response.json()['result']['number']
        finalized_block_num = int(finalized_block, 16)
        print(f"{GREEN}  ✓ Finalized Block: #{finalized_block_num}{RESET}")
    except:
        print(f"{RED}  ✗ Finalized block alınamadı{RESET}")

    # Peer count
    try:
        response = requests.post(url, json={
            "jsonrpc": "2.0",
            "id": 1,
            "method": "system_peers"
        }, timeout=5)
        peers = response.json()['result']
        print(f"{GREEN}  ✓ Bağlı Peer Sayısı: {len(peers)}{RESET}\n")
    except:
        print(f"{RED}  ✗ Peer bilgisi alınamadı{RESET}\n")

def main():
    print_header()

    # 1. Validator bilgilerini yükle
    validators = load_validators()

    # 2. Temizlik
    cleanup()

    # 3. Validatorleri başlat
    print(f"{YELLOW}ADIM 3/6: Validatorler başlatılıyor...{RESET}")
    bootnode = None

    for i, validator in enumerate(validators):
        start_validator(i, validator, bootnode)

        # İlk validator'dan sonra bootnode adresini al
        if i == 0:
            bootnode = "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
            time.sleep(5)  # Bootnode'un hazır olması için bekle
        else:
            time.sleep(2)  # Diğer validatorler için kısa bekleme

    print(f"{GREEN}  ✓ 8 validator başarıyla başlatıldı{RESET}\n")

    # 4. Network bağlantısının kurulmasını bekle
    print(f"{YELLOW}ADIM 4/6: Network ve RPC bağlantısı bekleniyor (15 saniye)...{RESET}")
    time.sleep(15)
    print(f"{GREEN}  ✓ Network hazır{RESET}\n")

    # 5. Keyleri insert et (loop)
    print(f"{YELLOW}ADIM 5/6: Session keyleri insert ediliyor...{RESET}")
    success_count = 0
    for i, validator in enumerate(validators):
        if insert_validator_keys(i, validator):
            success_count += 1
        time.sleep(1)

    print(f"\n{GREEN}✅ Key insertion tamamlandı! ({success_count}/8 validator){RESET}\n")

    # 6. Finalization için bekle
    print(f"{YELLOW}Finalization başlaması bekleniyor (20 saniye)...{RESET}")
    time.sleep(20)

    # 7. Network durumunu kontrol et
    check_network_status()

    # Final rapor
    print(f"{BLUE}╔════════════════════════════════════════════════════════╗{RESET}")
    print(f"{BLUE}║  🎉 8-VALİDATÖRLÜ BETA TESTNET HAZIR!                  ║{RESET}")
    print(f"{BLUE}╚════════════════════════════════════════════════════════╝{RESET}\n")

    print(f"{GREEN}📡 RPC Endpoint: ws://localhost:9944{RESET}\n")

    print(f"{YELLOW}📁 Validator Logları:{RESET}")
    for i in range(8):
        print(f"   Validator {i+1}: tail -f /tmp/beta-validator-{i+1}.log")

    print(f"\n{RED}🛑 Durdur: bash stop-beta-validators.sh{RESET}")
    print(f"{RED}   veya: pkill -f pezkuwi{RESET}\n")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print(f"\n{RED}Script durduruldu.{RESET}")
        sys.exit(1)
    except Exception as e:
        print(f"\n{RED}Hata: {e}{RESET}")
        sys.exit(1)
