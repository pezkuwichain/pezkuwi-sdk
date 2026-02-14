//! Create NFT Collection 0 on People Chain via XCM from Relay Chain
//!
//! This script sends XCM Transact messages from the Relay Chain (via sudo)
//! to the People Chain to:
//! 1. Create NFT Collection 0 using Nfts::force_create
//! 2. Mint NFT Item 0 in that collection for the founder
//!
//! Both calls are executed as XCM Transact with Superuser origin, wrapped
//! in sudo_unchecked_weight on the Relay Chain.
//!
//! Run with:
//!   FOUNDER_MNEMONIC='foam hope ...' cargo run --release --example create_nft_collection

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::utils::AccountId32;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use scale_value::Composite;
use std::time::Duration;

const RELAY_RPC: &str = "ws://217.77.6.126:9944";
const PEOPLE_RPC: &str = "ws://217.77.6.126:41944";
const PEOPLE_CHAIN_PARA_ID: u128 = 1004;

/// Query a single-key storage map on the People Chain.
async fn query_storage_map(
	api: &OnlineClient<PezkuwiConfig>,
	pallet: &str,
	entry: &str,
	key: u32,
) -> Result<Option<scale_value::Value<()>>, Box<dyn std::error::Error>> {
	let query = pezkuwi_subxt::dynamic::storage::<(u32,), Value>(pallet, entry);
	let storage = api.storage().at_latest().await?;
	match storage.entry(query)?.try_fetch((key,)).await? {
		Some(val) => Ok(Some(val.decode()?)),
		None => Ok(None),
	}
}

/// Query a double-key storage map on the People Chain.
async fn query_storage_double_map(
	api: &OnlineClient<PezkuwiConfig>,
	pallet: &str,
	entry: &str,
	key1: u32,
	key2: u32,
) -> Result<Option<scale_value::Value<()>>, Box<dyn std::error::Error>> {
	let query = pezkuwi_subxt::dynamic::storage::<(u32, u32), Value>(pallet, entry);
	let storage = api.storage().at_latest().await?;
	match storage.entry(query)?.try_fetch((key1, key2)).await? {
		Some(val) => Ok(Some(val.decode()?)),
		None => Ok(None),
	}
}

/// Query an AccountId-keyed storage map on the People Chain.
async fn query_storage_by_account(
	api: &OnlineClient<PezkuwiConfig>,
	pallet: &str,
	entry: &str,
	account: &AccountId32,
) -> Result<Option<scale_value::Value<()>>, Box<dyn std::error::Error>> {
	let query = pezkuwi_subxt::dynamic::storage::<(AccountId32,), Value>(pallet, entry);
	let storage = api.storage().at_latest().await?;
	match storage.entry(query)?.try_fetch((account.clone(),)).await? {
		Some(val) => Ok(Some(val.decode()?)),
		None => Ok(None),
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("=== CREATE NFT COLLECTION ON PEOPLE CHAIN VIA XCM ===\n");

	// Step 1: Load founder keypair from environment
	let mnemonic_str =
		std::env::var("FOUNDER_MNEMONIC").expect("FOUNDER_MNEMONIC environment variable required");
	let mnemonic = Mnemonic::parse(&mnemonic_str)?;
	let founder_keypair = Keypair::from_phrase(&mnemonic, None)?;
	let founder_account = AccountId32(founder_keypair.public_key().0);
	println!("Founder account: {}", founder_account);

	// Step 2: Connect to Relay Chain and People Chain
	println!("\nConnecting to Relay Chain at {} ...", RELAY_RPC);
	let relay_api = OnlineClient::<PezkuwiConfig>::from_insecure_url(RELAY_RPC).await?;
	println!(
		"Connected to Relay Chain (spec_version: {})",
		relay_api.runtime_version().spec_version
	);

	println!("Connecting to People Chain at {} ...", PEOPLE_RPC);
	let people_api = OnlineClient::<PezkuwiConfig>::from_insecure_url(PEOPLE_RPC).await?;
	println!(
		"Connected to People Chain (spec_version: {})",
		people_api.runtime_version().spec_version
	);

	// Step 3: Check if Collection 0 already exists
	println!("\n--- Checking if Collection 0 already exists ---");
	match query_storage_map(&people_api, "Nfts", "Collection", 0).await? {
		Some(_val) => {
			println!("Collection 0 ALREADY EXISTS on People Chain!");
			println!("Skipping collection creation.");
		},
		None => {
			println!("Collection 0 does NOT exist. Creating...\n");

			// Encode Nfts::force_create call bytes via People Chain metadata
			let force_create = pezkuwi_subxt::dynamic::tx(
				"Nfts",
				"force_create",
				Composite::unnamed([
					// owner: MultiAddress::Id(founder)
					Value::unnamed_variant("Id", [Value::from_bytes(&founder_account.0)]),
					// config: CollectionConfig { settings, max_supply, mint_settings }
					Value::unnamed_composite([
						Value::u128(0),                     // settings: u64
						Value::unnamed_variant("None", []), // max_supply
						Value::unnamed_composite([
							// mint_settings
							Value::unnamed_variant("Issuer", []), // mint_type
							Value::unnamed_variant("None", []),   // price
							Value::unnamed_variant("None", []),   // start_block
							Value::unnamed_variant("None", []),   // end_block
							Value::u128(0),                       // default_item_settings
						]),
					]),
				]),
			);

			let force_create_bytes = people_api.tx().call_data(&force_create)?;
			println!(
				"force_create encoded: {} bytes (0x{}...)",
				force_create_bytes.len(),
				hex::encode(&force_create_bytes[..force_create_bytes.len().min(16)])
			);

			// Send force_create via XCM Transact from Relay Chain
			let sudo_create = build_xcm_sudo_transact(&force_create_bytes);

			let call_bytes = relay_api.tx().call_data(&sudo_create)?;
			println!(
				"sudo XCM encoded: {} bytes (0x{}...)",
				call_bytes.len(),
				hex::encode(&call_bytes[..call_bytes.len().min(32)])
			);

			println!("Submitting sudo XCM transact for force_create...");
			let create_progress = relay_api
				.tx()
				.sign_and_submit_then_watch_default(&sudo_create, &founder_keypair)
				.await?;

			println!("Transaction submitted. Waiting for finalization...");
			let create_events = create_progress.wait_for_finalized_success().await?;
			println!("Transaction finalized on Relay Chain!");

			for event in create_events.iter() {
				let event = event?;
				let pallet = event.pallet_name();
				if pallet == "Sudo" || pallet == "XcmPallet" {
					println!("  Event: {}::{}", pallet, event.variant_name());
				}
			}

			println!("\nWaiting 18 seconds for XCM delivery...");
			tokio::time::sleep(Duration::from_secs(18)).await;

			match query_storage_map(&people_api, "Nfts", "Collection", 0).await? {
				Some(_) => println!("Collection 0 created successfully!"),
				None => {
					println!("WARNING: Collection 0 NOT found yet.");
					println!("Check People Chain logs. Continuing anyway...");
				},
			}
		},
	}

	// Step 4: Check if NFT #0 already exists
	println!("\n--- Checking if NFT #0 already exists ---");
	match query_storage_double_map(&people_api, "Nfts", "Item", 0, 0).await? {
		Some(_val) => {
			println!("NFT #0 ALREADY EXISTS in Collection 0!");
			println!("No minting needed.");
		},
		None => {
			println!("NFT #0 does NOT exist. Minting for founder...\n");

			let force_mint = pezkuwi_subxt::dynamic::tx(
				"Nfts",
				"force_mint",
				Composite::unnamed([
					Value::u128(0), // collection
					Value::u128(0), // item
					Value::unnamed_variant("Id", [Value::from_bytes(&founder_account.0)]),
					Value::unnamed_composite([Value::u128(0)]), // item_config
				]),
			);

			let force_mint_bytes = people_api.tx().call_data(&force_mint)?;
			println!(
				"force_mint encoded: {} bytes (0x{}...)",
				force_mint_bytes.len(),
				hex::encode(&force_mint_bytes[..force_mint_bytes.len().min(16)])
			);

			let sudo_mint = build_xcm_sudo_transact(&force_mint_bytes);

			println!("Submitting sudo XCM transact for force_mint...");
			let mint_progress = relay_api
				.tx()
				.sign_and_submit_then_watch_default(&sudo_mint, &founder_keypair)
				.await?;

			println!("Transaction submitted. Waiting for finalization...");
			let mint_events = mint_progress.wait_for_finalized_success().await?;
			println!("Transaction finalized on Relay Chain!");

			for event in mint_events.iter() {
				let event = event?;
				let pallet = event.pallet_name();
				if pallet == "Sudo" || pallet == "XcmPallet" {
					println!("  Event: {}::{}", pallet, event.variant_name());
				}
			}

			println!("\nWaiting 18 seconds for XCM delivery...");
			tokio::time::sleep(Duration::from_secs(18)).await;
		},
	}

	// Step 5: Final verification
	println!("\n--- Final Verification ---");

	match query_storage_map(&people_api, "Nfts", "Collection", 0).await? {
		Some(val) => println!("Collection 0: EXISTS - {:?}", val),
		None => println!("Collection 0: NOT FOUND"),
	}

	match query_storage_double_map(&people_api, "Nfts", "Item", 0, 0).await? {
		Some(val) => println!("NFT #0: EXISTS - {:?}", val),
		None => println!("NFT #0: NOT FOUND"),
	}

	match query_storage_by_account(&people_api, "Tiki", "CitizenNft", &founder_account).await? {
		Some(val) => println!("Tiki CitizenNft for founder: {:?}", val),
		None => {
			println!("Tiki CitizenNft for founder: None");
			println!(
				"  (Expected - Tiki storage is populated by confirm_citizenship, not force_mint)"
			);
		},
	}

	println!("\n=== NFT COLLECTION CREATION COMPLETE ===");
	Ok(())
}

/// Build an XCM V3 Transact message wrapped in sudo_unchecked_weight.
///
/// Uses the same proven pattern as the runtime-upgrade tool.
fn build_xcm_sudo_transact(encoded_call: &[u8]) -> pezkuwi_subxt_core::tx::payload::DynamicPayload {
	let dest = Value::unnamed_variant(
		"V3",
		vec![Value::named_composite([
			("parents", Value::u128(0)),
			(
				"interior",
				Value::unnamed_variant(
					"X1",
					vec![Value::unnamed_variant(
						"Teyrchain",
						vec![Value::u128(PEOPLE_CHAIN_PARA_ID)],
					)],
				),
			),
		])],
	);

	let message = Value::unnamed_variant(
		"V3",
		vec![Value::unnamed_composite(vec![
			Value::named_variant(
				"UnpaidExecution",
				[
					("weight_limit", Value::unnamed_variant("Unlimited", vec![])),
					("check_origin", Value::unnamed_variant("None", vec![])),
				],
			),
			Value::named_variant(
				"Transact",
				[
					("origin_kind", Value::unnamed_variant("Superuser", vec![])),
					(
						"require_weight_at_most",
						Value::named_composite([
							("ref_time", Value::u128(5_000_000_000u128)),
							("proof_size", Value::u128(500_000u128)),
						]),
					),
					("call", Value::from_bytes(encoded_call)),
				],
			),
		])],
	);

	let xcm_send = pezkuwi_subxt::dynamic::tx("XcmPallet", "send", vec![dest, message]);

	pezkuwi_subxt::dynamic::tx(
		"Sudo",
		"sudo_unchecked_weight",
		vec![
			xcm_send.into_value(),
			Value::named_composite([
				("ref_time", Value::u128(1u128)),
				("proof_size", Value::u128(1u128)),
			]),
		],
	)
}
