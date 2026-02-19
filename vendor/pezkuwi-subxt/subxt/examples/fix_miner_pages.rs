//! Fix MinerPages storage on AH (2 → 32) via sudo XCM
//!
//! MinerPages=2 causes OCW to mine only the last 2 of 32 snapshot pages,
//! missing all voter data in page 0, resulting in WrongWinnerCount.
//!
//! Run:
//!   SUDO_MNEMONIC="..." cargo run --release -p pezkuwi-subxt --example fix_miner_pages

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("=== FIX MinerPages: 2 → 32 on AH ===\n");

	// Connect to RC
	let rc_url =
		std::env::var("RC_RPC").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
	let rc_api = OnlineClient::<PezkuwiConfig>::from_url(&rc_url).await?;
	println!("RC connected: spec {}", rc_api.runtime_version().spec_version);

	// Connect to AH (for verification)
	let ah_url =
		std::env::var("AH_RPC").unwrap_or_else(|_| "ws://127.0.0.1:40944".to_string());
	let ah_api = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;
	println!("AH connected: spec {}", ah_api.runtime_version().spec_version);

	// Sudo keypair
	let mnemonic_str =
		std::env::var("SUDO_MNEMONIC").expect("SUDO_MNEMONIC environment variable required");
	let mnemonic = Mnemonic::from_str(&mnemonic_str)?;
	let sudo_keypair = Keypair::from_phrase(&mnemonic, None)?;
	println!("Sudo: {}\n", sudo_keypair.public_key().to_account_id());

	// Storage key for MinerPages: twox_128(":MinerPages:")
	// parameter_types! { pub storage MinerPages: u32 = 2; }
	// Key = twox_128(b":MinerPages:") = 16 bytes
	let miner_pages_key: Vec<u8> = {
		let data = b":MinerPages:";
		// Use xxhash via pezsp_crypto_hashing
		pezsp_crypto_hashing::twox_128(data).to_vec()
	};
	println!("MinerPages storage key: 0x{}", hex::encode(&miner_pages_key));

	// Check current value on AH
	let current_val = match ah_api
		.storage()
		.at_latest()
		.await?
		.fetch_raw(miner_pages_key.clone())
		.await
	{
		Ok(data) => data,
		Err(_) => vec![],
	};
	if current_val.is_empty() {
		println!("Current MinerPages: not set (default = 2)");
	} else if current_val.len() >= 4 {
		let val = u32::from_le_bytes(current_val[..4].try_into().unwrap());
		println!("Current MinerPages: {}", val);
		if val == 32 {
			println!("Already set to 32 — nothing to do.");
			return Ok(());
		}
	}

	// Encode System.set_storage call for AH
	// System pallet index = 0 (always first)
	// set_storage call index = 1 (System::set_storage)
	// items: Vec<(Vec<u8>, Vec<u8>)>
	//
	// SCALE encoding:
	// [pallet_idx: u8][call_idx: u8][compact_len: compact<u32>]
	// [compact_key_len][key_bytes][compact_val_len][val_bytes]
	let new_value: u32 = 32;
	let value_bytes = new_value.to_le_bytes(); // [0x20, 0x00, 0x00, 0x00]

	let mut encoded_call: Vec<u8> = Vec::new();
	// Pallet System = index 0
	encoded_call.push(0u8);
	// Call set_storage = call_index 4
	encoded_call.push(4u8);
	// Vec length = 1 item (compact encoding: 1 << 2 | 0 = 4)
	encoded_call.push(4u8); // compact(1)
	// Key: compact length (16 bytes) = 16 << 2 | 0 = 64
	encoded_call.push(64u8); // compact(16)
	encoded_call.extend_from_slice(&miner_pages_key);
	// Value: compact length (4 bytes) = 4 << 2 | 0 = 16
	encoded_call.push(16u8); // compact(4)
	encoded_call.extend_from_slice(&value_bytes);

	println!(
		"\nEncoded call ({} bytes): 0x{}",
		encoded_call.len(),
		hex::encode(&encoded_call)
	);

	// Build XCM message
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

	// Destination: V3 { parents: 0, interior: X1(Teyrchain(1000)) }
	let dest_val = Value::unnamed_variant(
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

	let xcm_send = pezkuwi_subxt::dynamic::tx("XcmPallet", "send", vec![dest_val, message]);
	let sudo_tx = pezkuwi_subxt::dynamic::tx("Sudo", "sudo", vec![xcm_send.into_value()]);

	println!("Submitting sudo(XcmPallet.send(AH, Transact(system.set_storage)))...");
	let progress = rc_api
		.tx()
		.sign_and_submit_then_watch_default(&sudo_tx, &sudo_keypair)
		.await?;
	let events = progress.wait_for_finalized_success().await?;

	let sent = events
		.iter()
		.flatten()
		.any(|e| e.pallet_name() == "XcmPallet" && e.variant_name() == "Sent");
	if sent {
		println!("  [OK] XCM Sent");
	} else {
		println!("  [WARN] No XcmPallet::Sent event found");
		for ev in events.iter().flatten() {
			println!("  Event: {}::{}", ev.pallet_name(), ev.variant_name());
		}
	}

	// Wait for DMP processing
	println!("\nWaiting 30s for DMP processing...");
	tokio::time::sleep(std::time::Duration::from_secs(30)).await;

	// Verify new value
	let new_val = match ah_api
		.storage()
		.at_latest()
		.await?
		.fetch_raw(miner_pages_key)
		.await
	{
		Ok(data) => data,
		Err(_) => vec![],
	};
	if new_val.len() >= 4 {
		let val = u32::from_le_bytes(new_val[..4].try_into().unwrap());
		println!("New MinerPages value: {}", val);
		if val == 32 {
			println!("\n✅ SUCCESS: MinerPages = 32");
		} else {
			println!("\n❌ UNEXPECTED: MinerPages = {} (expected 32)", val);
		}
	} else {
		println!("❌ FAIL: MinerPages still not set (empty storage)");
	}

	Ok(())
}
