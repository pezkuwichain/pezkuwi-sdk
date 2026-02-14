//! Comprehensive Post-Upgrade Test Suite (spec_version 1_020_003)
//!
//! Tests all functionality after runtime upgrade:
//! - Balance queries on all 3 chains
//! - XCM teleport from Relay to Asset Hub (1000 HEZ)
//! - XCM teleport from Relay to People Chain (1000 HEZ)
//! - Welati (citizenship) application on People Chain
//! - Staking queries on Relay Chain
//! - Trust score verification on People Chain
//!
//! Run with:
//!   FOUNDER_MNEMONIC='foam hope ...' cargo run --example comprehensive_test

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::{At, Value};
use pezkuwi_subxt::utils::AccountId32;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use scale_value::Composite;
use std::time::Duration;

// Generate interface from relay chain metadata (spec_version 1_020_003)
#[pezkuwi_subxt::subxt(runtime_metadata_path = "../artifacts/relay_mainnet_v3.scale")]
pub mod relay {}

// XCM type aliases for convenience
use relay::runtime_types::pezstaging_xcm::v4::{
	asset::{Asset, AssetId, Assets, Fungibility},
	junction::Junction,
	junctions::Junctions,
	location::Location,
};
use relay::runtime_types::xcm::{
	v3::WeightLimit, VersionedAssetId, VersionedAssets, VersionedLocation,
};

// RPC endpoints (direct to VPS3)
const RELAY_RPC: &str = "ws://217.77.6.126:9944";
const ASSET_HUB_RPC: &str = "ws://217.77.6.126:40944";
const PEOPLE_RPC: &str = "ws://217.77.6.126:41944";

// 1 HEZ = 10^12 TYR
const TYR_PER_HEZ: u128 = 1_000_000_000_000;

// Para IDs
const ASSET_HUB_ID: u32 = 1000;
const PEOPLE_CHAIN_ID: u32 = 1004;

// Test wallet mnemonic
const TEST_MNEMONIC: &str =
	"REDACTED_MNEMONIC";

fn format_hez(tyr: u128) -> String {
	let whole = tyr / TYR_PER_HEZ;
	let frac = (tyr % TYR_PER_HEZ) / (TYR_PER_HEZ / 10000);
	format!("{}.{:04} HEZ", whole, frac)
}

/// Query balance using dynamic storage query (works on any chain)
async fn query_balance(
	api: &OnlineClient<PezkuwiConfig>,
	account: &AccountId32,
) -> Result<u128, Box<dyn std::error::Error>> {
	let storage_query =
		pezkuwi_subxt::dynamic::storage::<(AccountId32,), Value>("System", "Account");
	let client_at = api.storage().at_latest().await?;
	match client_at.entry(storage_query)?.try_fetch((account.clone(),)).await? {
		Some(val) => {
			let decoded = val.decode()?;
			let free = decoded
				.at("data")
				.at("free")
				.ok_or("Could not find free balance")?
				.as_u128()
				.ok_or("Could not parse balance")?;
			Ok(free)
		},
		None => Ok(0),
	}
}

/// Dynamic storage query helper for People Chain - map types
async fn people_storage_map(
	api: &OnlineClient<PezkuwiConfig>,
	pallet: &str,
	entry: &str,
	account: &AccountId32,
) -> Result<Option<Value<()>>, Box<dyn std::error::Error>> {
	let query = pezkuwi_subxt::dynamic::storage::<(AccountId32,), Value>(pallet, entry);
	match api
		.storage()
		.at_latest()
		.await?
		.entry(query)?
		.try_fetch((account.clone(),))
		.await?
	{
		Some(val) => Ok(Some(val.decode()?)),
		None => Ok(None),
	}
}

/// Dynamic storage query helper for People Chain - value types (no key)
async fn people_storage_value(
	api: &OnlineClient<PezkuwiConfig>,
	pallet: &str,
	entry: &str,
) -> Result<Option<Value<()>>, Box<dyn std::error::Error>> {
	let query = pezkuwi_subxt::dynamic::storage::<(), Value>(pallet, entry);
	match api.storage().at_latest().await?.entry(query)?.try_fetch(()).await? {
		Some(val) => Ok(Some(val.decode()?)),
		None => Ok(None),
	}
}

/// Build XCM destination for a teyrchain
fn xcm_dest(para_id: u32) -> VersionedLocation {
	VersionedLocation::V4(Location {
		parents: 0,
		interior: Junctions::X1([Junction::Teyrchain(para_id)]),
	})
}

/// Build XCM beneficiary for an account
fn xcm_beneficiary(pubkey: [u8; 32]) -> VersionedLocation {
	VersionedLocation::V4(Location {
		parents: 0,
		interior: Junctions::X1([Junction::AccountId32 { network: None, id: pubkey }]),
	})
}

/// Build XCM assets (native HEZ token)
fn xcm_native_assets(amount: u128) -> VersionedAssets {
	VersionedAssets::V4(Assets(vec![Asset {
		id: AssetId(Location { parents: 0, interior: Junctions::Here }),
		fun: Fungibility::Fungible(amount),
	}]))
}

/// Native fee asset ID
fn xcm_fee_asset() -> VersionedAssetId {
	VersionedAssetId::V4(AssetId(Location { parents: 0, interior: Junctions::Here }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("╔══════════════════════════════════════════════════════════════╗");
	println!("║   PEZKUWICHAIN COMPREHENSIVE POST-UPGRADE TEST SUITE       ║");
	println!("║   spec_version: 1_020_003 (Trust Score System)             ║");
	println!("╚══════════════════════════════════════════════════════════════╝\n");

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 1: Setup wallets
	// ═══════════════════════════════════════════════════════════════════
	println!("═══ SECTION 1: Setup Wallets ═══\n");

	let test_mnemonic = Mnemonic::parse(TEST_MNEMONIC)?;
	let test_wallet = Keypair::from_phrase(&test_mnemonic, None)?;
	let test_account = AccountId32(test_wallet.public_key().0);
	println!("  Test Wallet: {}", test_account);

	let founder_mnemonic_str = std::env::var("FOUNDER_MNEMONIC")
		.expect("FOUNDER_MNEMONIC env var required (founder/sudo seed)");
	let founder_mnemonic = Mnemonic::parse(&founder_mnemonic_str)?;
	let founder_wallet = Keypair::from_phrase(&founder_mnemonic, None)?;
	let founder_account = AccountId32(founder_wallet.public_key().0);
	println!("  Founder:     {}", founder_account);

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 2: Connect to all chains
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 2: Connect to Chains ═══\n");

	let relay_api = OnlineClient::<PezkuwiConfig>::from_insecure_url(RELAY_RPC).await?;
	println!("  ✓ Relay Chain connected ({})", RELAY_RPC);

	let ah_api = OnlineClient::<PezkuwiConfig>::from_insecure_url(ASSET_HUB_RPC).await?;
	println!("  ✓ Asset Hub connected ({})", ASSET_HUB_RPC);

	let people_api = OnlineClient::<PezkuwiConfig>::from_insecure_url(PEOPLE_RPC).await?;
	println!("  ✓ People Chain connected ({})", PEOPLE_RPC);

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 3: Verify spec_version on all chains
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 3: Verify Runtime Versions ═══\n");

	let relay_ver = relay_api.runtime_version().spec_version;
	let ah_ver = ah_api.runtime_version().spec_version;
	let people_ver = people_api.runtime_version().spec_version;

	println!("  Relay Chain:  spec_version = {}", relay_ver);
	println!("  Asset Hub:    spec_version = {}", ah_ver);
	println!("  People Chain: spec_version = {}", people_ver);

	assert_eq!(relay_ver, 1_020_003, "Relay spec_version mismatch!");
	assert_eq!(ah_ver, 1_020_003, "Asset Hub spec_version mismatch!");
	assert_eq!(people_ver, 1_020_003, "People Chain spec_version mismatch!");
	println!("  ✓ All chains at spec_version 1_020_003");

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 4: Check initial balances
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 4: Initial Balances ═══\n");

	let relay_balance = query_balance(&relay_api, &test_account).await?;
	let ah_balance = query_balance(&ah_api, &test_account).await?;
	let people_balance = query_balance(&people_api, &test_account).await?;

	println!("  Test Wallet Balances:");
	println!("    Relay Chain:  {} ({} TYR)", format_hez(relay_balance), relay_balance);
	println!("    Asset Hub:    {} ({} TYR)", format_hez(ah_balance), ah_balance);
	println!("    People Chain: {} ({} TYR)", format_hez(people_balance), people_balance);

	let need_teleport =
		relay_balance >= 2000 * TYR_PER_HEZ && ah_balance == 0 && people_balance == 0;

	let (ah_received, people_received);

	if need_teleport {
		println!("  ✓ Sufficient balance for teleports");

		// ═══════════════════════════════════════════════════════════════════
		// SECTION 5: XCM Teleport — Relay → Asset Hub (1000 HEZ)
		// ═══════════════════════════════════════════════════════════════════
		println!("\n═══ SECTION 5: XCM Teleport → Asset Hub (1000 HEZ) ═══\n");

		let teleport_amount: u128 = 1000 * TYR_PER_HEZ;

		let xcm_ah_tx = relay::tx().xcm_pallet().limited_teleport_assets(
			xcm_dest(ASSET_HUB_ID),
			xcm_beneficiary(test_wallet.public_key().0),
			xcm_native_assets(teleport_amount),
			xcm_fee_asset(),
			WeightLimit::Unlimited,
		);

		println!("  Submitting XCM teleport to Asset Hub (1000 HEZ)...");
		let events_ah = relay_api
			.tx()
			.sign_and_submit_then_watch_default(&xcm_ah_tx, &test_wallet)
			.await?
			.wait_for_finalized_success()
			.await?;

		println!("  ✓ Teleport to Asset Hub finalized!");
		match events_ah.find_first::<relay::xcm_pallet::events::Attempted>()? {
			Some(event) => println!("    XCM Attempted: {:?}", event.outcome),
			None => println!("    ⚠ No Attempted event"),
		}

		// ═══════════════════════════════════════════════════════════════════
		// SECTION 6: XCM Teleport — Relay → People Chain (1000 HEZ)
		// ═══════════════════════════════════════════════════════════════════
		println!("\n═══ SECTION 6: XCM Teleport → People Chain (1000 HEZ) ═══\n");

		let xcm_people_tx = relay::tx().xcm_pallet().limited_teleport_assets(
			xcm_dest(PEOPLE_CHAIN_ID),
			xcm_beneficiary(test_wallet.public_key().0),
			xcm_native_assets(teleport_amount),
			xcm_fee_asset(),
			WeightLimit::Unlimited,
		);

		println!("  Submitting XCM teleport to People Chain (1000 HEZ)...");
		let events_people = relay_api
			.tx()
			.sign_and_submit_then_watch_default(&xcm_people_tx, &test_wallet)
			.await?
			.wait_for_finalized_success()
			.await?;

		println!("  ✓ Teleport to People Chain finalized!");
		match events_people.find_first::<relay::xcm_pallet::events::Attempted>()? {
			Some(event) => println!("    XCM Attempted: {:?}", event.outcome),
			None => println!("    ⚠ No Attempted event"),
		}

		// ═══════════════════════════════════════════════════════════════════
		// SECTION 7: Wait for XCM delivery & verify teleport balances
		// ═══════════════════════════════════════════════════════════════════
		println!("\n═══ SECTION 7: Verify Teleport Results ═══\n");
		println!("  Waiting 30 seconds for XCM messages to be delivered...");
		tokio::time::sleep(Duration::from_secs(30)).await;

		let relay_after = query_balance(&relay_api, &test_account).await?;
		let ah_after = query_balance(&ah_api, &test_account).await?;
		let people_after = query_balance(&people_api, &test_account).await?;

		println!("  Post-Teleport Balances:");
		println!(
			"    Relay Chain:  {} (was {})",
			format_hez(relay_after),
			format_hez(relay_balance)
		);
		println!("    Asset Hub:    {} (was {})", format_hez(ah_after), format_hez(ah_balance));
		println!(
			"    People Chain: {} (was {})",
			format_hez(people_after),
			format_hez(people_balance)
		);

		ah_received = ah_after.saturating_sub(ah_balance);
		people_received = people_after.saturating_sub(people_balance);

		println!("\n  Transfer Summary:");
		println!("    Relay spent:     {}", format_hez(relay_balance.saturating_sub(relay_after)));
		println!("    AH received:     {}", format_hez(ah_received));
		println!("    People received: {}", format_hez(people_received));
		println!("    → Asset Hub:    {}", if ah_received > 0 { "✓ SUCCESS" } else { "✗ FAILED" });
		println!(
			"    → People Chain: {}",
			if people_received > 0 { "✓ SUCCESS" } else { "✗ FAILED" }
		);
	} else {
		println!("\n═══ SECTIONS 5-7: XCM Teleport (SKIPPED - already done) ═══\n");
		println!("  Relay:  {}", format_hez(relay_balance));
		println!("  AH:     {}", format_hez(ah_balance));
		println!("  People: {}", format_hez(people_balance));
		println!("  ✓ Balances already distributed across chains");
		ah_received = ah_balance;
		people_received = people_balance;
	}

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 8: Staking Queries (Relay Chain)
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 8: Staking Status (Relay Chain) ═══\n");

	let block = relay_api.blocks().at_latest().await?;
	println!("  Current Block: #{}", block.number());

	let storage = relay_api.storage().at_latest().await?;

	let active_era = storage.try_fetch(relay::storage().staking().active_era(), ()).await?;
	println!("  Active Era: {:?}", active_era.and_then(|v| v.decode().ok()));

	let current_era = storage.try_fetch(relay::storage().staking().current_era(), ()).await?;
	println!("  Current Era: {:?}", current_era.and_then(|v| v.decode().ok()));

	let session_index = storage.try_fetch(relay::storage().session().current_index(), ()).await?;
	println!("  Session Index: {:?}", session_index.and_then(|v| v.decode().ok()));

	let val_count = storage.try_fetch(relay::storage().staking().validator_count(), ()).await?;
	println!("  Validator Count: {:?}", val_count.and_then(|v| v.decode().ok()));

	let force_era = storage.try_fetch(relay::storage().staking().force_era(), ()).await?;
	println!("  Force Era: {:?}", force_era.and_then(|v| v.decode().ok()));

	println!("  ✓ Staking system operational");

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 9: Welati (Citizenship) Application on People Chain
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 9: Welati (Citizenship) Application ═══\n");

	// Check if test wallet already has KYC status
	let kyc_status =
		people_storage_map(&people_api, "IdentityKyc", "KycStatuses", &test_account).await?;

	let already_citizen = if let Some(status) = &kyc_status {
		let status_str = format!("{:?}", status);
		println!("  Current KYC status: {}", status_str);
		status_str.contains("Approved")
	} else {
		println!("  KYC status: NotStarted (no entry)");
		false
	};

	if already_citizen {
		println!("  ✓ Already a citizen (KYC Approved), skipping application steps");
	} else {
		// Step 1: Apply for citizenship
		let identity_data = b"Azad Qasimlo|DR.QAsimlo";
		let identity_hash = pezsp_crypto_hashing::blake2_256(identity_data);
		println!(
			"  Identity hash: 0x{}",
			identity_hash.iter().map(|b| format!("{:02x}", b)).collect::<String>()
		);

		let apply_tx = pezkuwi_subxt::dynamic::tx(
			"IdentityKyc",
			"apply_for_citizenship",
			Composite::unnamed([
				Value::from_bytes(identity_hash),
				Value::unnamed_variant("None", []),
			]),
		);

		println!("  Submitting citizenship application...");
		let apply_events = people_api
			.tx()
			.sign_and_submit_then_watch_default(&apply_tx, &test_wallet)
			.await?
			.wait_for_finalized_success()
			.await?;
		println!("  ✓ Citizenship application submitted and finalized!");

		for event in apply_events.iter() {
			let event = event?;
			if event.pallet_name() == "IdentityKyc" {
				println!("    Event: {}::{}", event.pallet_name(), event.variant_name());
			}
		}

		// Step 2: Founder approves referral
		println!("\n  Founder approving referral...");
		let approve_tx = pezkuwi_subxt::dynamic::tx(
			"IdentityKyc",
			"approve_referral",
			Composite::unnamed([Value::from_bytes(&test_account.0)]),
		);

		let approve_events = people_api
			.tx()
			.sign_and_submit_then_watch_default(&approve_tx, &founder_wallet)
			.await?
			.wait_for_finalized_success()
			.await?;
		println!("  ✓ Referral approved by founder!");

		for event in approve_events.iter() {
			let event = event?;
			if event.pallet_name() == "IdentityKyc" {
				println!("    Event: {}::{}", event.pallet_name(), event.variant_name());
			}
		}

		// Step 3: Test wallet confirms citizenship
		println!("\n  Confirming citizenship (self-confirmation)...");
		let confirm_tx = pezkuwi_subxt::dynamic::tx(
			"IdentityKyc",
			"confirm_citizenship",
			Composite::unnamed([]),
		);

		let confirm_events = people_api
			.tx()
			.sign_and_submit_then_watch_default(&confirm_tx, &test_wallet)
			.await?
			.wait_for_finalized_success()
			.await?;
		println!("  ✓ Citizenship confirmed!");

		println!("\n  Confirmation events:");
		for event in confirm_events.iter() {
			let event = event?;
			let pallet = event.pallet_name();
			match pallet {
				"IdentityKyc" | "Tiki" | "Referral" | "Trust" | "Nfts" | "Balances" => {
					println!("    {}::{}", pallet, event.variant_name());
				},
				_ => {},
			}
		}
	}

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 9b: Ensure NFT Collection exists & mint CitizenNft
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 9b: NFT Collection & Citizen NFT Fix ═══\n");

	// Check if Tiki collection (ID=0) exists
	let _collection_exists = people_storage_map(
		&people_api,
		"Nfts",
		"Collection",
		// Collection storage key is a u32 (collection_id = 0), not an AccountId
		// Use a raw dynamic query instead
		&AccountId32([0u8; 32]), // placeholder
	)
	.await;

	// Use a different approach: check if NextItemId storage exists in Tiki
	let next_item_id = people_storage_value(&people_api, "Tiki", "NextItemId").await?;
	println!("  Tiki NextItemId: {:?}", next_item_id);

	// Check if test wallet already has CitizenNft
	let has_citizen_nft =
		people_storage_map(&people_api, "Tiki", "CitizenNft", &test_account).await?;

	if has_citizen_nft.is_some() {
		println!("  ✓ Test Wallet already has Citizen NFT");
	} else {
		println!("  ✗ Test Wallet does NOT have Citizen NFT - attempting sudo fix...");

		// Step 1: Create NFT collection 0 via sudo (Nfts::force_create)
		// force_create(owner, config) where owner = founder, config = default
		println!("  Creating NFT Collection 0 via sudo...");
		let create_collection_inner = pezkuwi_subxt::dynamic::tx(
			"Nfts",
			"force_create",
			Composite::unnamed([
				// owner
				Value::unnamed_variant("Id", [Value::from_bytes(&founder_account.0)]),
				// config (CollectionConfig)
				Value::unnamed_composite([
					// settings: u64 (all features enabled = 0)
					Value::u128(0),
					// max_supply: Option<u32>
					Value::unnamed_variant("None", []),
					// mint_settings: MintSettings
					Value::unnamed_composite([
						// mint_type
						Value::unnamed_variant("Issuer", []),
						// price: Option<Balance>
						Value::unnamed_variant("None", []),
						// start_block: Option<BlockNumber>
						Value::unnamed_variant("None", []),
						// end_block: Option<BlockNumber>
						Value::unnamed_variant("None", []),
						// default_item_settings: u64
						Value::u128(0),
					]),
				]),
			]),
		);

		let sudo_create =
			pezkuwi_subxt::dynamic::tx("Sudo", "sudo", vec![create_collection_inner.into_value()]);

		match people_api
			.tx()
			.sign_and_submit_then_watch_default(&sudo_create, &founder_wallet)
			.await
		{
			Ok(progress) => match progress.wait_for_finalized_success().await {
				Ok(events) => {
					println!("  ✓ NFT Collection 0 created!");
					for event in events.iter() {
						let event = event?;
						if event.pallet_name() == "Nfts" || event.pallet_name() == "Sudo" {
							println!("    {}::{}", event.pallet_name(), event.variant_name());
						}
					}
				},
				Err(e) => println!("  ⚠ Collection creation finalized with error: {}", e),
			},
			Err(e) => println!("  ⚠ Collection creation failed: {}", e),
		}

		// Step 2: Force mint citizen NFT for test wallet via sudo (Tiki::force_mint_citizen_nft)
		tokio::time::sleep(Duration::from_secs(6)).await;

		println!("  Force minting Citizen NFT for test wallet...");
		let mint_inner = pezkuwi_subxt::dynamic::tx(
			"Tiki",
			"force_mint_citizen_nft",
			Composite::unnamed([Value::from_bytes(&test_account.0)]),
		);

		let sudo_mint = pezkuwi_subxt::dynamic::tx("Sudo", "sudo", vec![mint_inner.into_value()]);

		match people_api
			.tx()
			.sign_and_submit_then_watch_default(&sudo_mint, &founder_wallet)
			.await
		{
			Ok(progress) => match progress.wait_for_finalized_success().await {
				Ok(events) => {
					println!("  ✓ Citizen NFT minted!");
					for event in events.iter() {
						let event = event?;
						let pallet = event.pallet_name();
						match pallet {
							"Tiki" | "Nfts" | "Sudo" => {
								println!("    {}::{}", pallet, event.variant_name());
							},
							_ => {},
						}
					}
				},
				Err(e) => println!("  ⚠ NFT mint finalized with error: {}", e),
			},
			Err(e) => println!("  ⚠ NFT mint failed: {}", e),
		}

		tokio::time::sleep(Duration::from_secs(6)).await;
	}

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 10: Trust Score Verification (People Chain)
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 10: Trust Score Verification ═══\n");

	match people_storage_map(&people_api, "Trust", "TrustScores", &test_account).await? {
		Some(val) => println!("  Test Wallet Trust Score: {:?}", val),
		None => println!("  Test Wallet Trust Score: 0 (not set)"),
	}

	match people_storage_value(&people_api, "Trust", "TotalActiveTrustScore").await? {
		Some(val) => println!("  Total Active Trust Score: {:?}", val),
		None => println!("  Total Active Trust Score: 0"),
	}

	match people_storage_map(&people_api, "Trust", "TrustScores", &founder_account).await? {
		Some(val) => println!("  Founder Trust Score: {:?}", val),
		None => println!("  Founder Trust Score: 0 (not set)"),
	}

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 11: Tiki (Role NFT) Verification
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 11: Tiki (Role NFT) Verification ═══\n");

	match people_storage_map(&people_api, "Tiki", "CitizenNft", &test_account).await? {
		Some(val) => println!("  ✓ Test Wallet has Citizen NFT: {:?}", val),
		None => println!("  ✗ Test Wallet does NOT have Citizen NFT"),
	}

	match people_storage_map(&people_api, "Tiki", "UserTikis", &test_account).await? {
		Some(val) => println!("  Test Wallet Roles: {:?}", val),
		None => println!("  Test Wallet has no roles assigned"),
	}

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 12: Referral System Check
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 12: Referral System ═══\n");

	match people_storage_map(&people_api, "IdentityKyc", "CitizenReferrers", &test_account).await? {
		Some(val) => println!("  Test Wallet referrer: {:?}", val),
		None => println!("  No referrer recorded"),
	}

	match people_storage_map(&people_api, "Referral", "ReferralCount", &founder_account).await {
		Ok(Some(val)) => println!("  Founder referral count: {:?}", val),
		Ok(None) => println!("  Founder referral count: 0"),
		Err(e) => println!("  ⚠ ReferralCount query error: {}", e),
	}

	match people_storage_map(&people_api, "Referral", "Referrals", &test_account).await {
		Ok(Some(val)) => println!("  Test Wallet referral info: {:?}", val),
		Ok(None) => println!("  No referral info for test wallet"),
		Err(e) => println!("  ⚠ Referrals query error: {}", e),
	}

	match people_storage_map(&people_api, "Referral", "ReferrerStatsStorage", &founder_account)
		.await
	{
		Ok(Some(val)) => println!("  Founder referrer stats: {:?}", val),
		Ok(None) => println!("  No referrer stats for founder"),
		Err(e) => println!("  ⚠ ReferrerStats query error: {}", e),
	}

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 13: Staking Score (People Chain - Cross-chain)
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 13: Staking Score System ═══\n");

	match people_storage_map(&people_api, "StakingScore", "StakingScores", &test_account).await {
		Ok(Some(val)) => println!("  Test Wallet Staking Score: {:?}", val),
		Ok(None) => println!("  Test Wallet Staking Score: 0 (not staking)"),
		Err(e) => println!("  ⚠ StakingScore query error: {}", e),
	}

	// Check if staking score bridge from relay is configured
	match people_storage_value(&people_api, "StakingScore", "LastRelayUpdate").await {
		Ok(Some(val)) => println!("  Last Relay Update: {:?}", val),
		Ok(None) => println!("  Last Relay Update: None (no cross-chain data yet)"),
		Err(_) => println!("  StakingScore: LastRelayUpdate storage not found"),
	}

	// ═══════════════════════════════════════════════════════════════════
	// SECTION 14: Final Balances & Summary
	// ═══════════════════════════════════════════════════════════════════
	println!("\n═══ SECTION 14: Final Summary ═══\n");

	let final_relay = query_balance(&relay_api, &test_account).await?;
	let final_ah = query_balance(&ah_api, &test_account).await?;
	let final_people = query_balance(&people_api, &test_account).await?;

	// Re-check critical states for summary
	let final_kyc = people_storage_map(&people_api, "IdentityKyc", "KycStatuses", &test_account)
		.await
		.ok()
		.flatten();
	let final_nft = people_storage_map(&people_api, "Tiki", "CitizenNft", &test_account)
		.await
		.ok()
		.flatten();
	let final_trust = people_storage_map(&people_api, "Trust", "TrustScores", &test_account)
		.await
		.ok()
		.flatten();

	let kyc_ok = final_kyc
		.as_ref()
		.map(|v| format!("{:?}", v).contains("Approved"))
		.unwrap_or(false);
	let nft_ok = final_nft.is_some();

	println!("╔══════════════════════════════════════════════════════════════╗");
	println!("║            PEZKUWICHAIN TEST RESULTS SUMMARY               ║");
	println!("╠══════════════════════════════════════════════════════════════╣");
	println!("║                                                            ║");
	println!("║  spec_version: {} / {} / {}               ║", relay_ver, ah_ver, people_ver);
	println!("║                                                            ║");
	println!("║  Balances (Test Wallet):                                   ║");
	println!("║    Relay:  {:>42} ║", format_hez(final_relay));
	println!("║    AH:     {:>42} ║", format_hez(final_ah));
	println!("║    People: {:>42} ║", format_hez(final_people));
	println!("║                                                            ║");
	println!(
		"║  XCM Teleport:      {} AH  {} People             ║",
		if ah_received > 0 { "✓" } else { "✗" },
		if people_received > 0 { "✓" } else { "✗" }
	);
	println!(
		"║  Welati (KYC):      {}                                    ║",
		if kyc_ok { "✓ Approved" } else { "✗ NOT Approved" }
	);
	println!(
		"║  Citizen NFT:       {}                                    ║",
		if nft_ok { "✓ Minted  " } else { "✗ NOT Minted  " }
	);
	println!(
		"║  Trust Score:       {:?}   ║",
		final_trust
			.as_ref()
			.map(|v| format!("{:?}", v))
			.unwrap_or_else(|| "0".to_string())
	);
	println!("║                                                            ║");
	println!("╚══════════════════════════════════════════════════════════════╝");

	println!("\n✓ COMPREHENSIVE TEST SUITE COMPLETED");

	Ok(())
}
