//! Fix stuck era on AH — v2 (correct storage keys)
//!
//! The era is stuck because:
//! - CurrentEra=1, ActiveEra=0 → is_planning()=Some(1)
//! - The first election produced empty validator set (no stakers at election time)
//! - RC never activated it → AH can't advance
//!
//! Fix strategy:
//! 1. Use system.killStorage to DELETE CurrentEra key (makes is_planning()=None)
//! 2. Call Staking.force_new_era() to trigger new election
//! Both via XCM Transact from RC.
//!
//! Run:
//!   SUDO_MNEMONIC="..." cargo run --release -p pezkuwi-subxt --example sim_fix_stuck_era_v2

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("=== FIX STUCK ERA ON AH — V2 ===\n");

	let rc_url = std::env::var("RC_RPC").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
	let ah_url = std::env::var("AH_RPC").unwrap_or_else(|_| "ws://127.0.0.1:40944".to_string());

	// First, connect to AH to get the correct storage key from metadata
	println!("--- Step 1: Get correct CurrentEra storage key from AH metadata ---");
	let ah_api = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;

	// Use the metadata to get the correct storage key for Staking.CurrentEra
	let current_era_key = {
		let metadata = ah_api.metadata();
		let pallet = metadata.pallet_by_name("Staking").expect("Staking pallet exists");
		let entry = pallet
			.storage()
			.expect("storage exists")
			.entry_by_name("CurrentEra")
			.expect("CurrentEra exists");

		// Build the key: twox_128(pallet_prefix) + twox_128(entry_name)
		let mut key = Vec::new();
		key.extend_from_slice(&pezsp_crypto_hashing::twox_128(
			pallet.name().as_bytes(),
		));
		key.extend_from_slice(&pezsp_crypto_hashing::twox_128(
			entry.name().as_bytes(),
		));
		println!(
			"  Pallet name in metadata: {:?}",
			pallet.name()
		);
		println!(
			"  Entry name in metadata: {:?}",
			entry.name()
		);
		println!("  Storage key: 0x{}", hex::encode(&key));
		key
	};

	// Verify the key works by querying current value
	let storage = ah_api.storage().at_latest().await?;
	let addr = pezkuwi_subxt::dynamic::storage::<(), Value>("Staking", "CurrentEra");
	match storage.entry(addr) {
		Ok(entry) => match entry.try_fetch(()).await {
			Ok(Some(val)) => println!("  Current value: {:?}", val.decode()),
			Ok(None) => println!("  Current value: None"),
			_ => {},
		},
		_ => {},
	}

	// Also get ForceEra key
	let force_era_key = {
		let metadata = ah_api.metadata();
		let pallet = metadata.pallet_by_name("Staking").expect("Staking pallet exists");
		let entry = pallet
			.storage()
			.expect("storage exists")
			.entry_by_name("ForceEra")
			.expect("ForceEra exists");
		let mut key = Vec::new();
		key.extend_from_slice(&pezsp_crypto_hashing::twox_128(
			pallet.name().as_bytes(),
		));
		key.extend_from_slice(&pezsp_crypto_hashing::twox_128(
			entry.name().as_bytes(),
		));
		println!("  ForceEra key: 0x{}", hex::encode(&key));
		key
	};

	// Connect to RC
	println!("\n--- Step 2: Connect to RC and prepare fix ---");
	let rc_api = OnlineClient::<PezkuwiConfig>::from_url(&rc_url).await?;

	let mnemonic_str =
		std::env::var("SUDO_MNEMONIC").expect("SUDO_MNEMONIC environment variable required");
	let sudo = Keypair::from_phrase(
		&Mnemonic::from_str(&mnemonic_str)?,
		None,
	)?;

	// Build the fix: utility.batch_all([
	//   system.setStorage([(current_era_key, 0x00000000)]),  // CurrentEra = Some(0)
	//   system.setStorage([(force_era_key, 0x01)])  // ForceEra = ForceNew
	// ])
	// Wait — we need to set CurrentEra storage value to u32=0 (4 bytes), NOT Option<u32>
	// The StorageValue OptionQuery stores just the raw type, Option wrapping is done at decode

	println!("\n--- Step 3: Build and send fix via XCM ---");

	// Approach: Use system.setStorage to set CurrentEra=0 and ForceEra=ForceNew
	let mut call_bytes = Vec::new();

	// System pallet index on AH = 0, setStorage call_index = 4
	// NOTE: call_index 1 = set_heap_pages (WRONG!), 4 = set_storage (CORRECT)
	call_bytes.push(0u8); // System pallet index
	call_bytes.push(4u8); // setStorage call index (#[pezpallet::call_index(4)])

	// items: Vec<(Vec<u8>, Vec<u8>)>, 2 items
	encode_compact_u32(&mut call_bytes, 2); // compact(2)

	// Item 1: CurrentEra = 0 (raw u32 LE, NOT Option-wrapped)
	encode_compact_u32(&mut call_bytes, current_era_key.len() as u32);
	call_bytes.extend_from_slice(&current_era_key);
	let current_era_value: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00]; // u32 LE = 0
	encode_compact_u32(&mut call_bytes, current_era_value.len() as u32);
	call_bytes.extend_from_slice(&current_era_value);

	// Item 2: ForceEra = ForceNew (0x01)
	encode_compact_u32(&mut call_bytes, force_era_key.len() as u32);
	call_bytes.extend_from_slice(&force_era_key);
	let force_era_value: Vec<u8> = vec![0x01]; // Forcing::ForceNew
	encode_compact_u32(&mut call_bytes, force_era_value.len() as u32);
	call_bytes.extend_from_slice(&force_era_value);

	println!("Encoded call ({} bytes): 0x{}", call_bytes.len(), hex::encode(&call_bytes));
	println!("  CurrentEra value: 0x{} (raw u32=0)", hex::encode(&current_era_value));
	println!("  ForceEra value:   0x{} (ForceNew)", hex::encode(&force_era_value));

	// Send via XCM
	let (dest, msg) = build_xcm_transact(1000, &call_bytes);
	let xcm_send = pezkuwi_subxt::dynamic::tx("XcmPallet", "send", vec![dest, msg]);
	let sudo_tx = pezkuwi_subxt::dynamic::tx("Sudo", "sudo", vec![xcm_send.into_value()]);

	println!("\nSending XCM Transact to AH...");
	let events = rc_api
		.tx()
		.sign_and_submit_then_watch_default(&sudo_tx, &sudo)
		.await?
		.wait_for_finalized_success()
		.await?;

	let sent = events
		.iter()
		.flatten()
		.any(|e| e.pallet_name() == "XcmPallet" && e.variant_name() == "Sent");
	println!(
		"  [{}] setStorage(CurrentEra=0, ForceEra=ForceNew)",
		if sent { "OK" } else { "WARN" }
	);

	// Wait and verify
	println!("\nWaiting 15 seconds for DMP processing...");
	tokio::time::sleep(std::time::Duration::from_secs(15)).await;

	let ah_api2 = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;
	let storage2 = ah_api2.storage().at_latest().await?;

	let addr = pezkuwi_subxt::dynamic::storage::<(), Value>("Staking", "CurrentEra");
	match storage2.entry(addr) {
		Ok(entry) => match entry.try_fetch(()).await {
			Ok(Some(val)) => println!("CurrentEra after fix: {:?}", val.decode()),
			Ok(None) => println!("CurrentEra after fix: None"),
			_ => {},
		},
		_ => {},
	}

	let addr = pezkuwi_subxt::dynamic::storage::<(), Value>("Staking", "ForceEra");
	match storage2.entry(addr) {
		Ok(entry) => match entry.try_fetch(()).await {
			Ok(Some(val)) => println!("ForceEra after fix:   {:?}", val.decode()),
			Ok(None) => println!("ForceEra after fix:   None"),
			_ => {},
		},
		_ => {},
	}

	let addr = pezkuwi_subxt::dynamic::storage::<(), Value>("Staking", "ActiveEra");
	match storage2.entry(addr) {
		Ok(entry) => match entry.try_fetch(()).await {
			Ok(Some(val)) => println!("ActiveEra:            {:?}", val.decode()),
			Ok(None) => println!("ActiveEra:            None"),
			_ => {},
		},
		_ => {},
	}

	println!("\n=== FIX V2 SENT ===");
	println!("Monitor AH logs for:");
	println!("  1. 'planned None' (CurrentEra cleared)");
	println!("  2. Election starting (MBE phases)");
	println!("  3. 'Sending new validator set of size 2' (Alice+Bob)");

	Ok(())
}

fn encode_compact_u32(buf: &mut Vec<u8>, val: u32) {
	if val < 64 {
		buf.push((val as u8) << 2);
	} else if val < 16384 {
		let v = ((val as u16) << 2) | 0x01;
		buf.extend_from_slice(&v.to_le_bytes());
	} else {
		let v = (val << 2) | 0x02;
		buf.extend_from_slice(&v.to_le_bytes());
	}
}
