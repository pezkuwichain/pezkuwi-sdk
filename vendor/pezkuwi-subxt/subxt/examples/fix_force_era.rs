//! Fix ForceEra: set from ForceAlways back to NotForcing
//!
//! Run with:
//!   SUDO_MNEMONIC="..." RPC_URL="ws://217.77.6.126:9944" \
//!   cargo run --release --example fix_force_era -p pezkuwi-subxt

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let url = std::env::var("RPC_URL").unwrap_or_else(|_| "ws://217.77.6.126:9944".to_string());

	println!("=== FIX ForceEra: ForceAlways -> NotForcing ===\n");
	println!("RPC: {}", url);

	let api = OnlineClient::<PezkuwiConfig>::from_insecure_url(&url).await?;
	println!("Connected!");

	let mnemonic_str =
		std::env::var("SUDO_MNEMONIC").expect("SUDO_MNEMONIC environment variable required");
	let mnemonic = Mnemonic::from_str(&mnemonic_str)?;
	let keypair = Keypair::from_phrase(&mnemonic, None)?;
	println!("Sudo account: {}\n", keypair.public_key().to_account_id());

	// Staking::ForceEra storage key (verified twox128)
	let force_era_key =
		hex::decode("5f3e4907f716ac89b6347d15ececedcaf7dad0317324aecae8744b87fc95f2f3")?;
	// NotForcing = enum variant 0 = 0x00
	let not_forcing_value = vec![0x00u8];

	println!("Storage key: 0x{}", hex::encode(&force_era_key));
	println!("New value:   0x{} (NotForcing)", hex::encode(&not_forcing_value));

	// Build: system.setStorage(items: Vec<(Key, Value)>)
	let set_storage_call = pezkuwi_subxt::dynamic::tx(
		"System",
		"set_storage",
		vec![Value::unnamed_composite(vec![Value::unnamed_composite(vec![
			Value::from_bytes(&force_era_key),
			Value::from_bytes(&not_forcing_value),
		])])],
	);

	// Wrap in sudo
	let sudo_call = pezkuwi_subxt::dynamic::tx("Sudo", "sudo", vec![set_storage_call.into_value()]);

	println!("\nSubmitting sudo(system.setStorage)...");

	use pezkuwi_subxt::tx::TxStatus;
	let tx_progress = api.tx().sign_and_submit_then_watch_default(&sudo_call, &keypair).await?;

	println!("TX: 0x{}", hex::encode(tx_progress.extrinsic_hash().as_ref()));

	let mut progress = tx_progress;
	loop {
		let status = progress.next().await;
		match status {
			Some(Ok(TxStatus::InBestBlock(details))) => {
				match details.wait_for_success().await {
					Ok(events) => {
						let mut sudid = false;
						for event in events.iter() {
							if let Ok(ev) = event {
								println!("  Event: {}::{}", ev.pallet_name(), ev.variant_name());
								if ev.pallet_name() == "Sudo" && ev.variant_name() == "Sudid" {
									sudid = true;
								}
							}
						}
						if sudid {
							println!("\nSUCCESS: ForceEra set to NotForcing");
						} else {
							println!("\nWARNING: Sudo::Sudid event not found");
						}
					},
					Err(e) => println!("DISPATCH ERROR: {}", e),
				}
				break;
			},
			Some(Ok(TxStatus::Error { message })) => {
				println!("TX ERROR: {}", message);
				break;
			},
			Some(Ok(TxStatus::Invalid { message })) => {
				println!("TX INVALID: {}", message);
				break;
			},
			Some(Ok(TxStatus::Dropped { message })) => {
				println!("TX DROPPED: {}", message);
				break;
			},
			Some(Err(e)) => {
				println!("STREAM ERROR: {}", e);
				break;
			},
			None => {
				println!("STREAM ENDED");
				break;
			},
			_ => {},
		}
	}

	Ok(())
}
