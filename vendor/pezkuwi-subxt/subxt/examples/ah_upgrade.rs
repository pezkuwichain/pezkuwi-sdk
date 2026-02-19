//! Asset Hub Runtime Upgrade (Local Simulation)
//!
//! Two-step process:
//! 1. RC → XCM → AH: System.authorize_upgrade(blake2_256(wasm))
//! 2. AH direct: System.apply_authorized_upgrade(wasm)
//!
//! Run:
//!   SUDO_MNEMONIC="..." \
//!   WASM_FILE="target/release/wbuild/asset-hub-pezkuwichain-runtime/asset_hub_pezkuwichain_runtime.compact.compressed.wasm" \
//!   cargo run --release -p pezkuwi-subxt --example ah_upgrade

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("=== ASSET HUB RUNTIME UPGRADE ===\n");

	let rc_url =
		std::env::var("RC_RPC").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
	let ah_url =
		std::env::var("AH_RPC").unwrap_or_else(|_| "ws://127.0.0.1:40944".to_string());
	let wasm_path = std::env::var("WASM_FILE").expect("WASM_FILE environment variable required");

	let mnemonic_str =
		std::env::var("SUDO_MNEMONIC").expect("SUDO_MNEMONIC environment variable required");
	let mnemonic = Mnemonic::from_str(&mnemonic_str)?;
	let sudo_keypair = Keypair::from_phrase(&mnemonic, None)?;
	println!("Sudo: {}", sudo_keypair.public_key().to_account_id());

	// Load WASM
	let wasm_data = std::fs::read(&wasm_path)?;
	println!(
		"WASM: {} ({:.2} MB)",
		wasm_path,
		wasm_data.len() as f64 / 1_048_576.0
	);

	// Blake2-256 hash of WASM
	let code_hash = pezsp_crypto_hashing::blake2_256(&wasm_data);
	println!("Code hash: 0x{}", hex::encode(code_hash));

	// Connect to RC
	let rc_api = OnlineClient::<PezkuwiConfig>::from_url(&rc_url).await?;
	println!("RC connected: {} (spec {})", rc_url, rc_api.runtime_version().spec_version);

	// Connect to AH
	let ah_api = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;
	println!(
		"AH connected: {} (spec {})\n",
		ah_url,
		ah_api.runtime_version().spec_version
	);

	// ═══════════════════════════════════════════
	// STEP 1: Authorize upgrade via XCM from RC
	// ═══════════════════════════════════════════
	println!("=== STEP 1: Authorize upgrade (RC → XCM → AH) ===");

	// Encode System::authorize_upgrade_without_checks(code_hash)
	// System pallet index = 0, call_index = 10
	let mut encoded_call = Vec::with_capacity(34);
	encoded_call.push(0x00); // System pallet
	encoded_call.push(0x0a); // authorize_upgrade_without_checks (10)
	encoded_call.extend_from_slice(&code_hash);
	println!("  Encoded call: {} bytes", encoded_call.len());

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
						vec![Value::u128(1000)],
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
					("call", Value::from_bytes(&encoded_call)),
				],
			),
		])],
	);

	let xcm_send =
		pezkuwi_subxt::dynamic::tx("XcmPallet", "send", vec![dest, message]);
	let sudo_tx = pezkuwi_subxt::dynamic::tx(
		"Sudo",
		"sudo_unchecked_weight",
		vec![
			xcm_send.into_value(),
			Value::named_composite([
				("ref_time", Value::u128(1u128)),
				("proof_size", Value::u128(1u128)),
			]),
		],
	);

	let progress = rc_api
		.tx()
		.sign_and_submit_then_watch_default(&sudo_tx, &sudo_keypair)
		.await?;
	let events = progress.wait_for_finalized_success().await?;

	let mut sent = false;
	for event in events.iter() {
		let event = event?;
		if event.pallet_name() == "XcmPallet" && event.variant_name() == "Sent" {
			sent = true;
		}
		if event.pallet_name() == "Sudo" || event.pallet_name() == "XcmPallet" {
			println!("  {}::{}", event.pallet_name(), event.variant_name());
		}
	}
	if !sent {
		println!("  WARNING: No XcmPallet::Sent event!");
		println!("  Aborting.");
		return Ok(());
	}
	println!("  XCM authorize_upgrade sent!\n");

	// Wait for AH to process the XCM — poll AuthorizedUpgrade storage
	println!("Waiting for AH to process XCM authorize_upgrade...");
	let mut authorized = false;
	for attempt in 1..=30 {
		tokio::time::sleep(std::time::Duration::from_secs(6)).await;

		// Reconnect to get fresh state
		let ah_check = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;
		let block = ah_check.blocks().at_latest().await?;
		let block_num = block.number();

		// Check System::AuthorizedUpgrade storage via raw key
		// twox128("System") ++ twox128("AuthorizedUpgrade")
		let auth_key = pezsp_crypto_hashing::twox_128(b"System")
			.iter()
			.chain(pezsp_crypto_hashing::twox_128(b"AuthorizedUpgrade").iter())
			.copied()
			.collect::<Vec<u8>>();
		let result = ah_check
			.storage()
			.at_latest()
			.await?
			.fetch_raw(auth_key)
			.await?;
		if !result.is_empty() {
			println!(
				"  AuthorizedUpgrade found on AH at block {} (attempt {})!",
				block_num, attempt
			);
			authorized = true;
			break;
		}
		println!(
			"  Attempt {}/30: AH block {} — AuthorizedUpgrade not yet set...",
			attempt, block_num
		);
	}

	if !authorized {
		println!("  ERROR: AuthorizedUpgrade not set after 3 minutes. Aborting.");
		return Ok(());
	}

	// ═══════════════════════════════════════════
	// STEP 1.5: Fund sudo account on AH via XCM
	// ═══════════════════════════════════════════
	println!("\n=== STEP 1.5: Fund sudo account on AH ===");
	let sudo_account_id = sudo_keypair.public_key().to_account_id();
	let account_bytes: [u8; 32] = *sudo_account_id.as_ref();

	// Encode Balances::force_set_balance(who, new_free)
	// Balances pallet = 10, call_index = 8
	// who = MultiAddress::Id(AccountId32) = variant 0 + 32 bytes
	// new_free = Compact<u128> = 100 HEZ = 100 * 10^18
	let mut fund_call: Vec<u8> = Vec::new();
	fund_call.push(10u8); // Balances pallet
	fund_call.push(8u8); // force_set_balance
	fund_call.push(0u8); // MultiAddress::Id variant
	fund_call.extend_from_slice(&account_bytes);
	// Compact<u128> for 100_000_000_000_000_000_000 (100 HEZ)
	// For compact: values > 2^30 use BigInt mode: (byte_len - 4) << 2 | 0b11, then LE bytes
	let amount: u128 = 100_000_000_000_000_000_000u128; // 100 HEZ
	let amount_bytes = amount.to_le_bytes();
	// Trim trailing zeros for compact encoding
	let significant = amount_bytes.iter().rposition(|&b| b != 0).map(|i| i + 1).unwrap_or(1);
	let byte_len = significant.max(4); // minimum 4 bytes for BigInt mode
	fund_call.push(((byte_len as u8 - 4) << 2) | 0b11);
	fund_call.extend_from_slice(&amount_bytes[..byte_len]);

	println!(
		"  Encoded force_set_balance ({} bytes): 0x{}",
		fund_call.len(),
		hex::encode(&fund_call)
	);

	let fund_dest = Value::unnamed_variant(
		"V3",
		vec![Value::named_composite([
			("parents", Value::u128(0)),
			(
				"interior",
				Value::unnamed_variant(
					"X1",
					vec![Value::unnamed_variant(
						"Teyrchain",
						vec![Value::u128(1000)],
					)],
				),
			),
		])],
	);

	let fund_msg = Value::unnamed_variant(
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
					("call", Value::from_bytes(&fund_call)),
				],
			),
		])],
	);

	let fund_xcm = pezkuwi_subxt::dynamic::tx("XcmPallet", "send", vec![fund_dest, fund_msg]);
	let fund_sudo = pezkuwi_subxt::dynamic::tx("Sudo", "sudo", vec![fund_xcm.into_value()]);

	let progress = rc_api
		.tx()
		.sign_and_submit_then_watch_default(&fund_sudo, &sudo_keypair)
		.await?;
	let events = progress.wait_for_finalized_success().await?;
	let fund_sent = events
		.iter()
		.flatten()
		.any(|e| e.pallet_name() == "XcmPallet" && e.variant_name() == "Sent");
	if fund_sent {
		println!("  [OK] Force set balance XCM sent");
	} else {
		println!("  [WARN] No XcmPallet::Sent event for funding");
	}

	println!("  Waiting 12s for DMP processing...");
	tokio::time::sleep(std::time::Duration::from_secs(12)).await;

	// ═══════════════════════════════════════════
	// STEP 2: Enact upgrade on AH directly
	// ═══════════════════════════════════════════
	println!("\n=== STEP 2: Apply authorized upgrade on AH ===");
	println!("  Submitting {} bytes WASM...", wasm_data.len());

	let enact_call = pezkuwi_subxt::dynamic::tx(
		"System",
		"apply_authorized_upgrade",
		vec![Value::from_bytes(&wasm_data)],
	);

	let progress = ah_api
		.tx()
		.sign_and_submit_then_watch_default(&enact_call, &sudo_keypair)
		.await?;
	let events = progress.wait_for_finalized_success().await?;

	let mut code_updated = false;
	for event in events.iter() {
		let event = event?;
		println!("  {}::{}", event.pallet_name(), event.variant_name());
		if event.pallet_name() == "System" && event.variant_name() == "CodeUpdated" {
			code_updated = true;
		}
	}

	if code_updated {
		println!("\n  UPGRADE SUCCESS!");
	} else {
		println!("\n  WARNING: No CodeUpdated event!");
	}

	// ═══════════════════════════════════════════
	// STEP 3: Verify
	// ═══════════════════════════════════════════
	println!("\nWaiting 6 seconds for new runtime...");
	tokio::time::sleep(std::time::Duration::from_secs(6)).await;

	let ah_api2 = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;
	println!(
		"AH spec_version: {} → {}",
		ah_api.runtime_version().spec_version,
		ah_api2.runtime_version().spec_version
	);

	println!("\n=== DONE ===");

	Ok(())
}
