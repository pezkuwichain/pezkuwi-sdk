//! Relay Chain Runtime Upgrade
//!
//! Deploys new WASM via sudo(sudoUncheckedWeight(system.setCodeWithoutChecks)).
//! Does NOTHING else — no storage changes, no validator count, no ForceEra.
//!
//! Run:
//!   SUDO_MNEMONIC="..." \
//!   WASM_FILE="target/release/wbuild/pezkuwichain-runtime/pezkuwichain_runtime.compact.compressed.wasm" \
//!   cargo run --release -p pezkuwi-subxt --example rc_upgrade
//!
//! Optional:
//!   RC_RPC="ws://127.0.0.1:9944"  (default: ws://127.0.0.1:9944)

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::tx::TxStatus;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("=== RELAY CHAIN RUNTIME UPGRADE ===\n");

	let rc_url =
		std::env::var("RC_RPC").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
	let wasm_path = std::env::var("WASM_FILE").expect("WASM_FILE environment variable required");

	// Load WASM
	let wasm_data = std::fs::read(&wasm_path)?;
	println!(
		"WASM: {} ({:.2} MB)",
		wasm_path,
		wasm_data.len() as f64 / 1_048_576.0
	);
	let code_hash = pezsp_crypto_hashing::blake2_256(&wasm_data);
	println!("Code hash: 0x{}", hex::encode(code_hash));

	// Connect
	let api = OnlineClient::<PezkuwiConfig>::from_url(&rc_url).await?;
	let old_spec = api.runtime_version().spec_version;
	println!("RC connected: {} (spec {})", rc_url, old_spec);

	// Load sudo key
	let mnemonic_str =
		std::env::var("SUDO_MNEMONIC").expect("SUDO_MNEMONIC environment variable required");
	let mnemonic = Mnemonic::from_str(&mnemonic_str)?;
	let sudo_keypair = Keypair::from_phrase(&mnemonic, None)?;
	println!("Sudo: {}\n", sudo_keypair.public_key().to_account_id());

	// Deploy WASM via sudo(sudoUncheckedWeight(system.setCodeWithoutChecks))
	println!("Deploying WASM...");
	let set_code = pezkuwi_subxt::dynamic::tx(
		"System",
		"set_code_without_checks",
		vec![Value::from_bytes(&wasm_data)],
	);
	let sudo_tx = pezkuwi_subxt::dynamic::tx(
		"Sudo",
		"sudo_unchecked_weight",
		vec![
			set_code.into_value(),
			Value::named_composite([
				("ref_time", Value::u128(1u128)),
				("proof_size", Value::u128(1u128)),
			]),
		],
	);

	let tx_progress = api
		.tx()
		.sign_and_submit_then_watch_default(&sudo_tx, &sudo_keypair)
		.await?;
	println!(
		"  TX: 0x{}",
		hex::encode(tx_progress.extrinsic_hash().as_ref())
	);

	let mut progress = tx_progress;
	let mut upgrade_ok = false;
	loop {
		let status = progress.next().await;
		match status {
			Some(Ok(TxStatus::InBestBlock(details))) => {
				match details.wait_for_success().await {
					Ok(events) => {
						for ev in events.iter().flatten() {
							println!("  {}::{}", ev.pallet_name(), ev.variant_name());
							if ev.pallet_name() == "System"
								&& ev.variant_name() == "CodeUpdated"
							{
								upgrade_ok = true;
							}
						}
					},
					Err(e) => println!("  DISPATCH ERROR: {}", e),
				}
				break;
			},
			Some(Ok(TxStatus::Error { message })) => {
				println!("  TX ERROR: {}", message);
				break;
			},
			Some(Ok(TxStatus::Invalid { message })) => {
				println!("  TX INVALID: {}", message);
				break;
			},
			Some(Ok(TxStatus::Dropped { message })) => {
				println!("  TX DROPPED: {}", message);
				break;
			},
			Some(Err(e)) => {
				println!("  STREAM ERROR: {}", e);
				break;
			},
			None => {
				println!("  STREAM ENDED");
				break;
			},
			_ => {},
		}
	}

	if !upgrade_ok {
		println!("\n  UPGRADE FAILED!");
		return Ok(());
	}

	// Verify
	println!("\nWaiting 12 seconds for new runtime...");
	tokio::time::sleep(std::time::Duration::from_secs(12)).await;

	let api2 = OnlineClient::<PezkuwiConfig>::from_url(&rc_url).await?;
	let new_spec = api2.runtime_version().spec_version;
	println!("spec_version: {} → {}", old_spec, new_spec);

	if new_spec > old_spec {
		println!("\n=== UPGRADE SUCCESS ===");
	} else {
		println!("\n=== WARNING: spec_version did not increase ===");
	}

	Ok(())
}
