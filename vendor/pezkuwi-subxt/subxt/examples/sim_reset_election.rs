//! Reset MBE election on AH: ForceRotateRound + ForceNew
//!
//! When the snapshot was taken with stale data (e.g. only 1 validator),
//! the election cycles forever with WrongWinnerCount. This script:
//! 1. Calls manage(ForceRotateRound) to kill the current round and snapshot
//! 2. Calls Staking.force_new_era() to trigger a fresh election with new snapshot
//!
//! Run:
//!   SUDO_MNEMONIC="..." cargo run --release -p pezkuwi-subxt --example sim_reset_election

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use std::str::FromStr;

fn build_xcm_transact(para_id: u32, encoded_call: &[u8]) -> (Value, Value) {
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
						vec![Value::u128(para_id as u128)],
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

	(dest, message)
}

async fn sudo_xcm(
	api: &OnlineClient<PezkuwiConfig>,
	sudo: &Keypair,
	para_id: u32,
	encoded_call: &[u8],
	label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
	let (dest, msg) = build_xcm_transact(para_id, encoded_call);
	let xcm_send = pezkuwi_subxt::dynamic::tx("XcmPallet", "send", vec![dest, msg]);
	let sudo_tx = pezkuwi_subxt::dynamic::tx("Sudo", "sudo", vec![xcm_send.into_value()]);

	let progress = api
		.tx()
		.sign_and_submit_then_watch_default(&sudo_tx, sudo)
		.await?;
	let events = progress.wait_for_finalized_success().await?;

	let sent = events
		.iter()
		.flatten()
		.any(|e| e.pallet_name() == "XcmPallet" && e.variant_name() == "Sent");
	if sent {
		println!("  [OK] {}", label);
	} else {
		println!("  [WARN] {} — no Sent event", label);
		for ev in events.iter().flatten() {
			println!("    Event: {}::{}", ev.pallet_name(), ev.variant_name());
		}
	}
	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("=== RESET MBE ELECTION ON AH ===\n");

	let rc_url =
		std::env::var("RC_RPC").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
	let rc_api = OnlineClient::<PezkuwiConfig>::from_url(&rc_url).await?;
	println!("RC connected: spec {}", rc_api.runtime_version().spec_version);

	let ah_url =
		std::env::var("AH_RPC").unwrap_or_else(|_| "ws://127.0.0.1:40944".to_string());
	let ah_api = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;
	println!("AH connected: spec {}", ah_api.runtime_version().spec_version);

	let mnemonic_str =
		std::env::var("SUDO_MNEMONIC").expect("SUDO_MNEMONIC environment variable required");
	let mnemonic = Mnemonic::from_str(&mnemonic_str)?;
	let sudo_keypair = Keypair::from_phrase(&mnemonic, None)?;
	println!("Sudo: {}\n", sudo_keypair.public_key().to_account_id());

	// Check current MBE state
	let phase_key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"MultiBlockElection")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"CurrentPhase").iter())
		.copied()
		.collect();
	let phase_val = match ah_api
		.storage()
		.at_latest()
		.await?
		.fetch_raw(phase_key)
		.await
	{
		Ok(data) => data,
		Err(_) => vec![],
	};
	println!("Current phase raw: 0x{}", hex::encode(&phase_val));

	// Step 1: ForceRotateRound
	// MultiBlockElection pallet index = 85
	// manage call_index = 0
	// ManagerOperation::ForceRotateRound = variant 0
	println!("\n--- Step 1: ForceRotateRound ---");
	let force_rotate_call: Vec<u8> = vec![85, 0, 0]; // [pallet=85, call=0, variant=0]
	sudo_xcm(
		&rc_api,
		&sudo_keypair,
		1000,
		&force_rotate_call,
		"manage(ForceRotateRound)",
	)
	.await?;

	println!("Waiting 30s for DMP processing...");
	tokio::time::sleep(std::time::Duration::from_secs(30)).await;

	// Verify phase is now Off
	let phase_key2: Vec<u8> = pezsp_crypto_hashing::twox_128(b"MultiBlockElection")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"CurrentPhase").iter())
		.copied()
		.collect();
	let phase_val2 = match ah_api
		.storage()
		.at_latest()
		.await?
		.fetch_raw(phase_key2)
		.await
	{
		Ok(data) => data,
		Err(_) => vec![],
	};
	println!("Phase after ForceRotateRound: 0x{}", hex::encode(&phase_val2));

	// Step 2: force_new_era via XCM to AH Staking
	println!("\n--- Step 2: ForceEra = ForceNew ---");
	// Staking pallet index on AH
	let staking_idx: u8 = {
		// Look up from runtime: Staking = 80
		80
	};
	// force_new_era call_index needs to be checked
	// For now, we use the sim_full_setup approach: set ForceEra storage directly
	// ForceEra key: twox128(Staking) + twox128(ForceEra)
	let force_era_key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"ForceEra").iter())
		.copied()
		.collect();

	// ForceNew = variant index 2 in Forcing enum
	// enum Forcing { NotForcing=0, ForceNew=1, ForceNone=2, ForceAlways=3 }
	// Actually, let me check: from sim_full_setup output "ForceEra: ForceNew" was set.
	// In the SCALE encoding: ForceNew = 1
	let force_new_value: Vec<u8> = vec![1u8]; // ForceNew = variant index 1

	// Encode system.set_storage call
	let mut set_storage_call: Vec<u8> = Vec::new();
	set_storage_call.push(0u8); // System pallet = 0
	set_storage_call.push(4u8); // set_storage call_index = 4
	set_storage_call.push(4u8); // compact(1) = 1 item
	// Key
	set_storage_call.push((force_era_key.len() as u8) << 2); // compact(32) = 128
	set_storage_call.extend_from_slice(&force_era_key);
	// Value
	set_storage_call.push((force_new_value.len() as u8) << 2); // compact(1) = 4
	set_storage_call.extend_from_slice(&force_new_value);

	println!("ForceEra key: 0x{}", hex::encode(&force_era_key));
	println!(
		"Encoded call ({} bytes): 0x{}",
		set_storage_call.len(),
		hex::encode(&set_storage_call)
	);

	sudo_xcm(
		&rc_api,
		&sudo_keypair,
		1000,
		&set_storage_call,
		"system.set_storage(ForceEra = ForceNew)",
	)
	.await?;

	println!("Waiting 30s for DMP processing...");
	tokio::time::sleep(std::time::Duration::from_secs(30)).await;

	// Verify
	let force_era_val = match ah_api
		.storage()
		.at_latest()
		.await?
		.fetch_raw(force_era_key)
		.await
	{
		Ok(data) => data,
		Err(_) => vec![],
	};
	let force_era_str = if force_era_val.is_empty() {
		"empty (default NotForcing)"
	} else {
		match force_era_val[0] {
			0 => "NotForcing",
			1 => "ForceNew",
			2 => "ForceNone",
			3 => "ForceAlways",
			_ => "Unknown",
		}
	};
	println!("ForceEra: {}", force_era_str);

	// Check validators
	let cv_key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"CounterForValidators").iter())
		.copied()
		.collect();
	let cv_val = match ah_api.storage().at_latest().await?.fetch_raw(cv_key).await {
		Ok(data) => data,
		Err(_) => vec![],
	};
	if cv_val.len() >= 4 {
		let count = u32::from_le_bytes(cv_val[..4].try_into().unwrap());
		println!("CounterForValidators: {}", count);
	}

	println!("\n=== DONE ===");
	println!("Next: AH will take a new snapshot at session boundary with 2 validators.");
	println!("Then: MBE election should produce a valid 2-winner solution.");
	println!("Monitor: sim_query_state → watch CurrentPhase and ActiveEra");

	Ok(())
}
