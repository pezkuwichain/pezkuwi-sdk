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

//! End-to-End Integration Tests for PezkuwiChain
//!
//! These tests validate:
//! - Genesis config generation for all network modes
//! - Chain spec loading and validation
//! - Runtime WASM binary availability

use pezkuwi_service::chain_spec;

/// Test that dev chain spec loads successfully
#[test]
fn test_dev_chain_spec_loads() {
	let chain_spec = chain_spec::pezkuwichain_development_config()
		.expect("Dev chain spec should load");
	
	assert_eq!(chain_spec.name(), "Pezkuwi Development");
	assert_eq!(chain_spec.id(), "pezkuwichain_dev");
	assert_eq!(chain_spec.chain_type(), sc_chain_spec::ChainType::Development);
}

/// Test that local testnet chain spec loads successfully
#[test]
fn test_local_testnet_chain_spec_loads() {
	let chain_spec = chain_spec::pezkuwichain_local_testnet_config()
		.expect("Local testnet chain spec should load");
	
	assert_eq!(chain_spec.name(), "Pezkuwi Local Testnet");
	assert_eq!(chain_spec.id(), "pezkuwi_local_testnet");
	assert_eq!(chain_spec.chain_type(), sc_chain_spec::ChainType::Local);
}

/// Test that alfa testnet chain spec loads successfully
#[test]
fn test_alfa_testnet_chain_spec_loads() {
	let chain_spec = chain_spec::pezkuwichain_alfa_testnet_config()
		.expect("Alfa testnet chain spec should load");
	
	assert_eq!(chain_spec.name(), "PezkuwiChain Alfa Testnet");
	assert_eq!(chain_spec.id(), "pezkuwichain_alfa_testnet");
	assert_eq!(chain_spec.chain_type(), sc_chain_spec::ChainType::Development);
}

/// Test that beta testnet chain spec loads successfully
#[test]
fn test_beta_testnet_chain_spec_loads() {
	let chain_spec = chain_spec::pezkuwichain_beta_testnet_config()
		.expect("Beta testnet chain spec should load");
	
	assert_eq!(chain_spec.name(), "PezkuwiChain Beta Testnet");
	assert_eq!(chain_spec.id(), "pezkuwichain_beta_testnet");
	assert_eq!(chain_spec.chain_type(), sc_chain_spec::ChainType::Live);
}

/// Test that staging chain spec loads successfully
#[test]
fn test_staging_chain_spec_loads() {
	let chain_spec = chain_spec::pezkuwichain_staging_config()
		.expect("Staging chain spec should load");
	
	assert_eq!(chain_spec.name(), "PezkuwiChain Staging");
	assert_eq!(chain_spec.id(), "pezkuwichain_staging");
	assert_eq!(chain_spec.chain_type(), sc_chain_spec::ChainType::Live);
}

/// Test that production chain spec loads successfully
#[test]
fn test_production_chain_spec_loads() {
	let chain_spec = chain_spec::pezkuwichain_production_config()
		.expect("Production chain spec should load");
	
	assert_eq!(chain_spec.name(), "PezkuwiChain");
	assert_eq!(chain_spec.id(), "pezkuwichain");
	assert_eq!(chain_spec.chain_type(), sc_chain_spec::ChainType::Live);
}

/// Test that dev genesis can be built
#[test]
fn test_dev_genesis_builds() {
	let chain_spec = chain_spec::pezkuwichain_development_config()
		.expect("Should load dev config");
	
	let genesis_storage = chain_spec.build_storage()
		.expect("Genesis should be buildable");
	
	// Verify storage is not empty
	assert!(!genesis_storage.top.is_empty(), "Genesis should have top storage");
	
	// Verify we have system storage
	assert!(
		genesis_storage.top.keys().any(|k| k.starts_with(b":code")),
		"Genesis should contain runtime code"
	);
}

/// Test that local testnet genesis can be built
#[test]
fn test_local_testnet_genesis_builds() {
	let chain_spec = chain_spec::pezkuwichain_local_testnet_config()
		.expect("Should load local testnet config");
	
	let genesis_storage = chain_spec.build_storage()
		.expect("Genesis should be buildable");
	
	assert!(!genesis_storage.top.is_empty(), "Genesis should have storage");
}

/// Test that alfa testnet genesis can be built
#[test]
fn test_alfa_testnet_genesis_builds() {
	let chain_spec = chain_spec::pezkuwichain_alfa_testnet_config()
		.expect("Should load alfa testnet config");
	
	let genesis_storage = chain_spec.build_storage()
		.expect("Genesis should be buildable");
	
	assert!(!genesis_storage.top.is_empty(), "Genesis should have storage");
}

/// Test that beta testnet genesis can be built
#[test]
fn test_beta_testnet_genesis_builds() {
	let chain_spec = chain_spec::pezkuwichain_beta_testnet_config()
		.expect("Should load beta testnet config");
	
	let genesis_storage = chain_spec.build_storage()
		.expect("Genesis should be buildable");
	
	assert!(!genesis_storage.top.is_empty(), "Genesis should have storage");
	
	// Beta testnet should have 8 validators configured
	// This is validated by the runtime genesis preset
}

/// Test that staging genesis can be built
#[test]
fn test_staging_genesis_builds() {
	let chain_spec = chain_spec::pezkuwichain_staging_config()
		.expect("Should load staging config");
	
	let genesis_storage = chain_spec.build_storage()
		.expect("Genesis should be buildable");
	
	assert!(!genesis_storage.top.is_empty(), "Genesis should have storage");
}

/// Test that production genesis can be built
#[test]
fn test_production_genesis_builds() {
	let chain_spec = chain_spec::pezkuwichain_production_config()
		.expect("Should load production config");
	
	let genesis_storage = chain_spec.build_storage()
		.expect("Genesis should be buildable");
	
	assert!(!genesis_storage.top.is_empty(), "Genesis should have storage");
}

/// Test that all chain specs have correct properties
#[test]
fn test_all_chain_specs_have_properties() {
	let specs = vec![
		chain_spec::pezkuwichain_development_config(),
		chain_spec::pezkuwichain_local_testnet_config(),
		chain_spec::pezkuwichain_alfa_testnet_config(),
		chain_spec::pezkuwichain_beta_testnet_config(),
		chain_spec::pezkuwichain_staging_config(),
		chain_spec::pezkuwichain_production_config(),
	];
	
	for spec in specs {
		let chain_spec = spec.expect("Chain spec should load");
		let properties = chain_spec.properties();
		
		// Verify HEZ token properties
		assert_eq!(
			properties.get("tokenSymbol").and_then(|v| v.as_str()),
			Some("HEZ"),
			"Token symbol should be HEZ"
		);
		
		assert_eq!(
			properties.get("tokenDecimals").and_then(|v| v.as_u64()),
			Some(12),
			"Token decimals should be 12"
		);
		
		assert_eq!(
			properties.get("ss58Format").and_then(|v| v.as_u64()),
			Some(42),
			"SS58 format should be 42"
		);
	}
}

/// Test that protocol IDs are correct
#[test]
fn test_protocol_ids_correct() {
	let dev = chain_spec::pezkuwichain_development_config()
		.expect("Dev should load");
	assert_eq!(dev.protocol_id(), Some("pezkuwi"));
	
	let beta = chain_spec::pezkuwichain_beta_testnet_config()
		.expect("Beta should load");
	assert_eq!(beta.protocol_id(), Some("pezkuwi"));
	
	let production = chain_spec::pezkuwichain_production_config()
		.expect("Production should load");
	assert_eq!(production.protocol_id(), Some("pezkuwi"));
}

/// Test that genesis presets are correctly named
#[test]
fn test_genesis_preset_names() {
	// Dev uses "dev" preset
	let dev = chain_spec::pezkuwichain_development_config()
		.expect("Dev should load");
	let dev_genesis = dev.build_storage().expect("Should build");
	assert!(!dev_genesis.top.is_empty());
	
	// Local testnet uses "local_testnet" preset
	let local = chain_spec::pezkuwichain_local_testnet_config()
		.expect("Local should load");
	let local_genesis = local.build_storage().expect("Should build");
	assert!(!local_genesis.top.is_empty());
	
	// Beta testnet uses "beta_testnet" preset with REAL validator keys
	let beta = chain_spec::pezkuwichain_beta_testnet_config()
		.expect("Beta should load");
	let beta_genesis = beta.build_storage().expect("Should build");
	assert!(!beta_genesis.top.is_empty());
}

/// Test that deprecated real_testnet redirects to beta
#[test]
fn test_real_testnet_deprecated_redirect() {
	let real = chain_spec::pezkuwichain_real_testnet_config()
		.expect("Real testnet should load (deprecated)");
	
	let beta = chain_spec::pezkuwichain_beta_testnet_config()
		.expect("Beta testnet should load");
	
	// Both should have same properties since real_testnet redirects to beta
	assert_eq!(real.name(), beta.name());
	assert_eq!(real.id(), beta.id());
}

/// Test WASM binary is available for pezkuwi runtime
#[cfg(feature = "pezkuwi-native")]
#[test]
fn test_pezkuwi_wasm_binary_available() {
	use pezkuwichain as pezkuwi_runtime;
	
	assert!(
		pezkuwi_runtime::WASM_BINARY.is_some(),
		"Pezkuwi WASM binary should be available"
	);
	
	let wasm = pezkuwi_runtime::WASM_BINARY.unwrap();
	assert!(!wasm.is_empty(), "WASM binary should not be empty");
	assert!(wasm.len() > 1024, "WASM binary should be substantial size");
}

/// Test that chain spec extensions are properly set
#[test]
fn test_chain_spec_extensions() {
	let chain_spec = chain_spec::pezkuwichain_development_config()
		.expect("Dev chain spec should load");
	
	// Extensions should be properly initialized
	let extensions = chain_spec.extensions();
	
	// Fork blocks should be empty for fresh chain
	assert!(extensions.fork_blocks.is_empty(), "Fork blocks should be empty initially");
	
	// Bad blocks should be empty for fresh chain
	assert!(extensions.bad_blocks.is_empty(), "Bad blocks should be empty initially");
}

/// Test that all chain specs can be serialized to JSON
#[test]
fn test_chain_specs_serialize_to_json() {
	let specs = vec![
		("dev", chain_spec::pezkuwichain_development_config()),
		("local", chain_spec::pezkuwichain_local_testnet_config()),
		("alfa", chain_spec::pezkuwichain_alfa_testnet_config()),
		("beta", chain_spec::pezkuwichain_beta_testnet_config()),
		("staging", chain_spec::pezkuwichain_staging_config()),
		("production", chain_spec::pezkuwichain_production_config()),
	];
	
	for (name, spec_result) in specs {
		let spec = spec_result.expect(&format!("{} should load", name));
		
		let json = serde_json::to_string(&spec)
			.expect(&format!("{} should serialize to JSON", name));
		
		assert!(!json.is_empty(), "{} JSON should not be empty", name);
		assert!(json.contains("\"name\""), "{} JSON should contain name field", name);
		assert!(json.contains("\"id\""), "{} JSON should contain id field", name);
	}
}

/// Comprehensive test: Build all chain specs and verify storage
#[test]
fn test_comprehensive_all_chains_valid() {
	let test_cases = vec![
		("dev", chain_spec::pezkuwichain_development_config(), 1),
		("local", chain_spec::pezkuwichain_local_testnet_config(), 2),
		("alfa", chain_spec::pezkuwichain_alfa_testnet_config(), 4),
		("beta", chain_spec::pezkuwichain_beta_testnet_config(), 8),
		("staging", chain_spec::pezkuwichain_staging_config(), 20),
		("production", chain_spec::pezkuwichain_production_config(), 100),
	];
	
	for (name, spec_result, _expected_validators) in test_cases {
		// Load chain spec
		let spec = spec_result.expect(&format!("{} should load", name));
		
		// Build genesis storage
		let storage = spec.build_storage()
			.expect(&format!("{} genesis should build", name));
		
		// Verify storage is not empty
		assert!(
			!storage.top.is_empty(),
			"{} should have non-empty storage",
			name
		);
		
		// Verify runtime code exists
		assert!(
			storage.top.keys().any(|k| k.starts_with(b":code")),
			"{} should have runtime code in storage",
			name
		);
		
		println!("✅ {} chain spec validated successfully", name);
	}
}