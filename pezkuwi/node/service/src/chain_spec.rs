// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Pezkuwi.

// Pezkuwi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Pezkuwi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Pezkuwi.  If not, see <http://www.gnu.org/licenses/>.

//! Pezkuwi chain configurations.
#[cfg(feature = "rococo-native")]
use rococo_runtime as rococo;
#[cfg(feature = "pezkuwi-native")]
use pezkuwichain as pezkuwi_runtime;
use sc_chain_spec::ChainSpecExtension;
#[cfg(any(feature = "westend-native", feature = "rococo-native", feature = "pezkuwi-native"))]
use sc_chain_spec::ChainType;
#[cfg(any(feature = "westend-native", feature = "rococo-native"))]
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
// GEREKLİ EKLEMELER
use pezkuwi_primitives::AccountId;
use pezkuwi_runtime::{AssetsConfig, PezTreasuryPalletId}; // AssetsConfig geri eklendi
use sp_runtime::traits::AccountIdConversion;
use sp_consensus_grandpa::AuthorityId as GrandpaId; // Ed25519 düzeltmesi için gerekli
#[cfg(feature = "westend-native")]
use westend_runtime as westend;


#[cfg(feature = "westend-native")]
const WESTEND_STAGING_TELEMETRY_URL: &str = "wss://telemetry.pezkuwi.io/submit/";
#[cfg(feature = "rococo-native")]
const ROCOCO_STAGING_TELEMETRY_URL: &str = "wss://telemetry.pezkuwi.io/submit/";
#[cfg(feature = "rococo-native")]
const VERSI_STAGING_TELEMETRY_URL: &str = "wss://telemetry.pezkuwi.io/submit/";
#[cfg(any(feature = "westend-native", feature = "rococo-native"))]
const DEFAULT_PROTOCOL_ID: &str = "dot";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<pezkuwi_primitives::Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<pezkuwi_primitives::Block>,
    /// The light sync state.
    ///
    /// This value will be set by the `sync-state rpc` implementation.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

// Generic chain spec, in case when we don't have the native runtime.
pub type GenericChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The `ChainSpec` parameterized for the westend runtime.
#[cfg(feature = "westend-native")]
pub type WestendChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The `ChainSpec` parameterized for the westend runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "westend-native"))]
pub type WestendChainSpec = GenericChainSpec;

/// The `ChainSpec` parameterized for the rococo runtime.
#[cfg(feature = "rococo-native")]
pub type RococoChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The `ChainSpec` parameterized for the rococo runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "rococo-native"))]
pub type RococoChainSpec = GenericChainSpec;
/// The `ChainSpec` parameterized for the pezkuwi runtime.
#[cfg(feature = "pezkuwi-native")]
pub type PezkuwiChainSpec = sc_service::GenericChainSpec<Extensions>;
/// The `ChainSpec` parameterized for the pezkuwi runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "pezkuwi-native"))]
pub type PezkuwiChainSpec = GenericChainSpec;
pub fn pezkuwi_config() -> Result<GenericChainSpec, String> {
    GenericChainSpec::from_json_bytes(&include_bytes!("../chain-specs/pezkuwi.json")[..])
}

pub fn kusama_config() -> Result<GenericChainSpec, String> {
    GenericChainSpec::from_json_bytes(&include_bytes!("../chain-specs/kusama.json")[..])
}

pub fn westend_config() -> Result<WestendChainSpec, String> {
    WestendChainSpec::from_json_bytes(&include_bytes!("../chain-specs/westend.json")[..])
}

pub fn paseo_config() -> Result<GenericChainSpec, String> {
    GenericChainSpec::from_json_bytes(&include_bytes!("../chain-specs/paseo.json")[..])
}

pub fn rococo_config() -> Result<RococoChainSpec, String> {
    RococoChainSpec::from_json_bytes(&include_bytes!("../chain-specs/rococo.json")[..])
}

/// Westend staging testnet config.
#[cfg(feature = "westend-native")]
pub fn westend_staging_testnet_config() -> Result<WestendChainSpec, String> {
    Ok(WestendChainSpec::builder(
        westend::WASM_BINARY.ok_or("Westend development wasm not available")?,
        Default::default(),
    )
    .with_name("Westend Staging Testnet")
    .with_id("westend_staging_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_preset_name("staging_testnet")
    .with_telemetry_endpoints(
        TelemetryEndpoints::new(vec![(WESTEND_STAGING_TELEMETRY_URL.to_string(), 0)])
            .expect("Westend Staging telemetry url is valid; qed"),
    )
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

/// Rococo staging testnet config.
#[cfg(feature = "rococo-native")]
pub fn rococo_staging_testnet_config() -> Result<RococoChainSpec, String> {
    Ok(RococoChainSpec::builder(
        rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?,
        Default::default(),
    )
    .with_name("Rococo Staging Testnet")
    .with_id("rococo_staging_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_preset_name("staging_testnet")
    .with_telemetry_endpoints(
        TelemetryEndpoints::new(vec![(ROCOCO_STAGING_TELEMETRY_URL.to_string(), 0)])
            .expect("Rococo Staging telemetry url is valid; qed"),
    )
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

pub fn versi_chain_spec_properties() -> serde_json::map::Map<String, serde_json::Value> {
    serde_json::json!({
        "ss58Format": 42,
        "tokenDecimals": 12,
        "tokenSymbol": "VRS",
    })
    .as_object()
    .expect("Map given; qed")
    .clone()
}

/// Versi staging testnet config.
#[cfg(feature = "rococo-native")]
pub fn versi_staging_testnet_config() -> Result<RococoChainSpec, String> {
    Ok(RococoChainSpec::builder(
        rococo::WASM_BINARY.ok_or("Versi development wasm not available")?,
        Default::default(),
    )
    .with_name("Versi Staging Testnet")
    .with_id("versi_staging_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_preset_name("staging_testnet")
    .with_telemetry_endpoints(
        TelemetryEndpoints::new(vec![(VERSI_STAGING_TELEMETRY_URL.to_string(), 0)])
            .expect("Versi Staging telemetry url is valid; qed"),
    )
    .with_protocol_id("versi")
    .with_properties(versi_chain_spec_properties())
    .build())
}

/// Westend development config (single validator Alice)
#[cfg(feature = "westend-native")]
pub fn westend_development_config() -> Result<WestendChainSpec, String> {
    Ok(WestendChainSpec::builder(
        westend::WASM_BINARY.ok_or("Westend development wasm not available")?,
        Default::default(),
    )
    .with_name("Development")
    .with_id("westend_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

/// Rococo development config (single validator Alice)
#[cfg(feature = "rococo-native")]
pub fn rococo_development_config() -> Result<RococoChainSpec, String> {
    Ok(RococoChainSpec::builder(
        rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?,
        Default::default(),
    )
    .with_name("Development")
    .with_id("rococo_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

/// `Versi` development config (single validator Alice)
#[cfg(feature = "rococo-native")]
pub fn versi_development_config() -> Result<RococoChainSpec, String> {
    Ok(RococoChainSpec::builder(
        rococo::WASM_BINARY.ok_or("Versi development wasm not available")?,
        Default::default(),
    )
    .with_name("Development")
    .with_id("versi_dev")
    .with_chain_type(ChainType::Development)
    .with_protocol_id("versi")
    .build())
}

/// Westend local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "westend-native")]
pub fn westend_local_testnet_config() -> Result<WestendChainSpec, String> {
    Ok(WestendChainSpec::builder(
        westend::fast_runtime_binary::WASM_BINARY
            .ok_or("Westend development wasm not available")?,
        Default::default(),
    )
    .with_name("Westend Local Testnet")
    .with_id("westend_local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

/// Rococo local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "rococo-native")]
pub fn rococo_local_testnet_config() -> Result<RococoChainSpec, String> {
    Ok(RococoChainSpec::builder(
        rococo::fast_runtime_binary::WASM_BINARY.ok_or("Rococo development wasm not available")?,
        Default::default(),
    )
    .with_name("Rococo Local Testnet")
    .with_id("rococo_local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .build())
}

/// `Versi` local testnet config (multivalidator Alice + Bob + Charlie + Dave)
#[cfg(feature = "rococo-native")]
pub fn versi_local_testnet_config() -> Result<RococoChainSpec, String> {
    Ok(RococoChainSpec::builder(
        rococo::WASM_BINARY.ok_or("Rococo development wasm (used for versi) not available")?,
        Default::default(),
    )
    .with_name("Versi Local Testnet")
    .with_id("versi_local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_preset_name("versi_local_testnet")
    .with_protocol_id("versi")
    .build())
}

// Helper function to generate Pezkuwi properties
fn pezkuwi_properties() -> sc_service::Properties {
    let mut p = sc_service::Properties::new();
    let _ = p.insert("tokenSymbol".into(), "HEZ".into());
    let _ = p.insert("tokenDecimals".into(), 12.into());
    let _ = p.insert("ss58Format".into(), 42.into());
    p
}
/// PezkuwiChain development config (single validator Alice)
#[cfg(feature = "pezkuwi-native")]
pub fn pezkuwichain_development_config() -> Result<PezkuwiChainSpec, String> {
    use sp_consensus_babe::AuthorityId as BabeId;
    use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
    use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
    use pezkuwi_primitives::{ValidatorId, AssignmentId};

    const TOTAL_SUPPLY: u128 = 5_000_000_000 * 1_000_000_000_000; // 5 Milyar PEZ
    const ALICE_PEZ: u128 = 1_000_000_000 * 1_000_000_000_000;     // 1 Milyar PEZ (test için)
    const TREASURY_PEZ: u128 = TOTAL_SUPPLY - ALICE_PEZ;           // 4 Milyar PEZ

    let root_key = sp_keyring::Sr25519Keyring::Alice.to_account_id();
    let alice_account = sp_keyring::Sr25519Keyring::Alice.to_account_id();
    let pez_treasury_account: AccountId = PezTreasuryPalletId::get().into_account_truncating();

    // Authority keys
    let alice_babe_id: BabeId = sp_keyring::Sr25519Keyring::Alice.public().into();
    let alice_grandpa_id: GrandpaId = sp_keyring::Ed25519Keyring::Alice.public().into();
    let alice_validator_id: ValidatorId = sp_keyring::Sr25519Keyring::Alice.public().into();
    let alice_assignment_id: AssignmentId = sp_keyring::Sr25519Keyring::Alice.public().into();
    let alice_discovery_id: AuthorityDiscoveryId = sp_keyring::Sr25519Keyring::Alice.public().into();
    let alice_beefy_id: BeefyId = sp_core::ecdsa::Public::from_raw([0u8; 33]).into();

    // 1. Assets patch
    let assets_config = AssetsConfig {
        assets: vec![(1, root_key.clone(), true, 1)],
        metadata: vec![(1, "Pez".into(), "PEZ".into(), 12)],
        accounts: vec![
            (1, pez_treasury_account.clone(), TREASURY_PEZ),
            (1, root_key.clone(), ALICE_PEZ),
        ],
        next_asset_id: Some(2),
    };

    let assets_patch = serde_json::json!({
        "assets": assets_config
    });

    // 2. Grandpa patch
    /* 
    let grandpa_patch = serde_json::json!({
        "grandpa": {
            "authorities": vec![(alice_grandpa_id.clone(), 1u64)]
        }
    }); */

    // 3. Balances patch
    let balances_patch = serde_json::json!({
        "balances": {
            "balances": vec![(alice_account.clone(), 10_000_000_000_000_000_000_000u128)]
        }
    });

    // 4. BABE patch - Sadece epochConfig (authorities Session'dan gelir)
    // BABE config - Session ve Runtime'dan gelsin
    // let babe_patch = serde_json::json!({ ... });

    // 5. Session patch
    let session_patch = serde_json::json!({
        "session": {
            "keys": vec![(
                alice_account.clone(),
                alice_account.clone(),
                serde_json::json!({
                    "babe": alice_babe_id.clone(),
                    "grandpa": alice_grandpa_id.clone(),
                    "para_validator": alice_validator_id.clone(),
                    "para_assignment": alice_assignment_id.clone(),
                    "authority_discovery": alice_discovery_id.clone(),
                    "beefy": alice_beefy_id.clone(),
                })
            )]
        }
    });

    // 6. Staking patch
    let staking_patch = serde_json::json!({
        "staking": {
            "validatorCount": 1u32,
            "minimumValidatorCount": 1u32,
            "stakers": serde_json::json!([
                [
                    alice_account.clone(),
                    alice_account.clone(),
                    1_000_000_000_000_000_000_000u128,
                    "Validator"
                ]
            ]),
            "invulnerables": serde_json::json!([]),
            "minNominatorBond": 1_000_000_000_000_000_000u128,
            "minValidatorBond": 1_000_000_000_000_000_000u128
        }
    });

    // 7. Tüm patch'leri birleştir
    let mut patch_map = serde_json::Map::new();
    
    if let serde_json::Value::Object(obj) = assets_patch {
        patch_map.extend(obj);
    }
    // if let serde_json::Value::Object(obj) = grandpa_patch {
    //     patch_map.extend(obj);
    // }
    if let serde_json::Value::Object(obj) = balances_patch {
        patch_map.extend(obj);
    }
    // if let serde_json::Value::Object(obj) = babe_patch {
    //     patch_map.extend(obj);
    // }
    if let serde_json::Value::Object(obj) = session_patch {
        patch_map.extend(obj);
    }
    if let serde_json::Value::Object(obj) = staking_patch {
        patch_map.extend(obj);
    }

    let patch = serde_json::Value::Object(patch_map);

    // 8. Chain spec oluştur
    Ok(PezkuwiChainSpec::builder(
        pezkuwi_runtime::WASM_BINARY.ok_or("Pezkuwi development wasm not available")?,
        Default::default(),
    )
    .with_name("Pezkuwi Development")
    .with_id("pezkuwichain_dev")
    .with_chain_type(ChainType::Development)
    // .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .with_protocol_id("pezkuwi")
    .with_properties(pezkuwi_properties())
    .with_genesis_config_patch(patch)
    .build())
}

/// PezkuwiChain production config
#[cfg(feature = "pezkuwi-native")]
pub fn pezkuwichain_production_config() -> Result<PezkuwiChainSpec, String> {
    Ok(PezkuwiChainSpec::builder(
        pezkuwi_runtime::WASM_BINARY.ok_or("Pezkuwi production wasm not available")?,
        Default::default(),
    )
    .with_name("PezkuwiChain")
    .with_id("pezkuwichain")
    .with_chain_type(ChainType::Live) // LIVE chain
    .with_genesis_config_preset_name("production")
    .with_protocol_id("pezkuwi")
    .with_properties(pezkuwi_properties())
    .build())
}

/// PezkuwiChain beta testnet config
#[cfg(feature = "pezkuwi-native")]
pub fn pezkuwichain_beta_testnet_config() -> Result<PezkuwiChainSpec, String> {
    Ok(PezkuwiChainSpec::builder(
        pezkuwi_runtime::WASM_BINARY.ok_or("Pezkuwi beta wasm not available")?,
        Default::default(),
    )
    .with_name("PezkuwiChain Beta Testnet")
    .with_id("pezkuwichain_beta_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_preset_name("beta_testnet")
    .with_protocol_id("pezkuwi")
    .with_properties(pezkuwi_properties())
    .build())
}

/// PezkuwiChain local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "pezkuwi-native")]
pub fn pezkuwichain_local_testnet_config() -> Result<PezkuwiChainSpec, String> {
    Ok(PezkuwiChainSpec::builder(
        pezkuwi_runtime::WASM_BINARY.ok_or("Pezkuwi local testnet wasm not available")?,
        Default::default(),
    )
    .with_name("PezkuwiChain Local Testnet")
    .with_id("pezkuwichain_local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .with_protocol_id("pezkuwi")
    .with_properties(pezkuwi_properties())
    .build())
}


/// PezkuwiChain real testnet config (8 validators, real token economics)
#[cfg(feature = "pezkuwi-native")]
pub fn pezkuwichain_real_testnet_config() -> Result<PezkuwiChainSpec, String> {
    Ok(PezkuwiChainSpec::builder(
        pezkuwi_runtime::WASM_BINARY.ok_or("Pezkuwi real testnet wasm not available")?,
        Default::default(),
    )
    .with_name("PezkuwiChain Real Testnet")
    .with_id("pezkuwichain_real_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_preset_name("real_testnet")
    .with_protocol_id("pezkuwi")
    .with_properties(pezkuwi_properties())
    .build())
}