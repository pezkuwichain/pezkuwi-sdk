//! Initialize AH Staking-Async with Validator Data
//!
//! This script populates the AH staking-async pallet with validator data
//! for the local mainnet simulation. Without this, AH cannot plan new eras
//! or send validator sets back to RC.
//!
//! Steps:
//! 1. Give Bob balance on AH (via XCM from RC)
//! 2. Alice + Bob: bond on AH directly
//! 3. Alice + Bob: validate on AH directly
//! 4. set_validator_count(2) via XCM Transact (root on AH)
//! 5. force_new_era() via XCM Transact (root on AH)
//!
//! Run:
//!   SUDO_MNEMONIC="..." cargo run --release -p pezkuwi-subxt --example init_ah_staking
//!
//! Optional:
//!   RC_RPC="ws://127.0.0.1:9944"   (default)
//!   AH_RPC="ws://127.0.0.1:40944"  (default)

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::tx::TxStatus;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use std::str::FromStr;

const HEZ: u128 = 1_000_000_000_000_000_000; // 10^18

const AH_PARA_ID: u32 = 1000;

const ALICE_PUBKEY: [u8; 32] = [
	0xd4, 0x35, 0x93, 0xc7, 0x15, 0xfd, 0xd3, 0x1c, 0x61, 0x14, 0x1a, 0xbd, 0x04, 0xa9, 0x9f,
	0xd6, 0x82, 0x2c, 0x85, 0x58, 0x85, 0x4c, 0xcd, 0xe3, 0x9a, 0x56, 0x84, 0xe7, 0xa5, 0x6d,
	0xa2, 0x7d,
];
const BOB_PUBKEY: [u8; 32] = [
	0x8e, 0xaf, 0x04, 0x15, 0x16, 0x87, 0x73, 0x63, 0x26, 0xc9, 0xfe, 0xa1, 0x7e, 0x25, 0xfc,
	0x52, 0x87, 0x61, 0x36, 0x93, 0xc9, 0x12, 0x90, 0x9c, 0xb2, 0x26, 0xaa, 0x47, 0x94, 0xf2,
	0x6a, 0x48,
];

/// AH pallet indices
const AH_BALANCES: u8 = 10;
const AH_STAKING: u8 = 80;

/// SCALE Compact<u128> encoding
fn encode_compact_u128(buf: &mut Vec<u8>, val: u128) {
	if val < 64 {
		buf.push((val as u8) << 2);
	} else if val < 16384 {
		let v = ((val as u16) << 2) | 0x01;
		buf.extend_from_slice(&v.to_le_bytes());
	} else if val < (1u128 << 30) {
		let v = ((val as u32) << 2) | 0x02;
		buf.extend_from_slice(&v.to_le_bytes());
	} else {
		let bytes = val.to_le_bytes();
		let len = 16 - val.leading_zeros() as usize / 8;
		let len = if len == 0 { 1 } else { len };
		buf.push(((len as u8 - 4) << 2) | 0x03);
		buf.extend_from_slice(&bytes[..len]);
	}
}

/// Encode Balances::force_set_balance(who, #[compact] new_free)
fn encode_force_set_balance(account: &[u8; 32], amount: u128) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(AH_BALANCES);
	data.push(8); // call_index for force_set_balance
	// MultiAddress::Id
	data.push(0x00);
	data.extend_from_slice(account);
	encode_compact_u128(&mut data, amount);
	data
}

/// Encode Staking::set_validator_count(#[compact] new)
fn encode_set_validator_count(count: u32) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(AH_STAKING);
	data.push(9); // call_index for set_validator_count
	encode_compact_u128(&mut data, count as u128);
	data
}

/// Encode Staking::force_new_era()
fn encode_force_new_era() -> Vec<u8> {
	let mut data = Vec::new();
	data.push(AH_STAKING);
	data.push(13); // call_index for force_new_era
	data
}

/// Build XCM V3 dest + message for a teyrchain transact
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

/// Send XCM via sudo on relay chain
async fn sudo_xcm_send(
	api: &OnlineClient<PezkuwiConfig>,
	sudo: &Keypair,
	dest: Value,
	message: Value,
	label: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
	let xcm_send = pezkuwi_subxt::dynamic::tx("XcmPallet", "send", vec![dest, message]);
	let sudo_tx = pezkuwi_subxt::dynamic::tx("Sudo", "sudo", vec![xcm_send.into_value()]);

	let progress = api
		.tx()
		.sign_and_submit_then_watch_default(&sudo_tx, sudo)
		.await?;
	let events = progress.wait_for_finalized_success().await?;

	let mut sent = false;
	for event in events.iter() {
		let event = event?;
		if event.pallet_name() == "XcmPallet" && event.variant_name() == "Sent" {
			sent = true;
		}
	}
	if sent {
		println!("  [OK] {}", label);
	} else {
		println!("  [WARN] {} - no XcmPallet::Sent event", label);
	}
	Ok(sent)
}

/// Wait for a tx on AH
async fn wait_ah_tx(
	mut progress: pezkuwi_subxt::tx::TxProgress<PezkuwiConfig, OnlineClient<PezkuwiConfig>>,
	label: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
	loop {
		let status = progress.next().await;
		match status {
			Some(Ok(TxStatus::InBestBlock(details))) => {
				match details.wait_for_success().await {
					Ok(events) => {
						let mut ok = false;
						for ev in events.iter().flatten() {
							if ev.pallet_name() == "System" && ev.variant_name() == "ExtrinsicSuccess" {
								ok = true;
							}
						}
						if ok {
							println!("  [OK] {}", label);
						} else {
							println!("  [WARN] {} - no ExtrinsicSuccess", label);
						}
						return Ok(ok);
					},
					Err(e) => {
						println!("  [FAIL] {} - dispatch error: {}", label, e);
						return Ok(false);
					},
				}
			},
			Some(Ok(TxStatus::Error { message })) => {
				println!("  [FAIL] {} - TX error: {}", label, message);
				return Ok(false);
			},
			Some(Ok(TxStatus::Invalid { message })) => {
				println!("  [FAIL] {} - TX invalid: {}", label, message);
				return Ok(false);
			},
			Some(Ok(TxStatus::Dropped { message })) => {
				println!("  [FAIL] {} - TX dropped: {}", label, message);
				return Ok(false);
			},
			Some(Err(e)) => {
				println!("  [FAIL] {} - stream error: {}", label, e);
				return Ok(false);
			},
			None => {
				println!("  [FAIL] {} - stream ended", label);
				return Ok(false);
			},
			_ => {},
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("=== INITIALIZE AH STAKING-ASYNC ===\n");

	let rc_url =
		std::env::var("RC_RPC").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
	let ah_url =
		std::env::var("AH_RPC").unwrap_or_else(|_| "ws://127.0.0.1:40944".to_string());

	// Connect to RC
	let rc_api = OnlineClient::<PezkuwiConfig>::from_url(&rc_url).await?;
	println!("RC connected: {}", rc_url);

	// Connect to AH
	let ah_api = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;
	println!("AH connected: {}\n", ah_url);

	// Load sudo key
	let mnemonic_str =
		std::env::var("SUDO_MNEMONIC").expect("SUDO_MNEMONIC environment variable required");
	let mnemonic = Mnemonic::from_str(&mnemonic_str)?;
	let sudo = Keypair::from_phrase(&mnemonic, None)?;

	// Alice and Bob keypairs (dev accounts)
	let alice =
		Keypair::from_uri(&pezkuwi_subxt_signer::SecretUri::from_str("//Alice").unwrap())
			.unwrap();
	let bob = Keypair::from_uri(&pezkuwi_subxt_signer::SecretUri::from_str("//Bob").unwrap())
		.unwrap();

	println!("Sudo: {}", sudo.public_key().to_account_id());
	println!("Alice: {}", alice.public_key().to_account_id());
	println!("Bob: {}\n", bob.public_key().to_account_id());

	// =========================================================
	// STEP 1: Give Bob balance on AH (Alice already has from local_sim_setup)
	// =========================================================
	println!("--- Step 1: Set Bob balance on AH via XCM ---");

	let bob_balance_call = encode_force_set_balance(&BOB_PUBKEY, 100_000 * HEZ);
	let (dest, msg) = build_xcm_transact(AH_PARA_ID, &bob_balance_call);
	sudo_xcm_send(&rc_api, &sudo, dest, msg, "Bob 100K HEZ on AH").await?;

	// Wait for XCM to be processed
	println!("  Waiting 24s for DMP processing...");
	tokio::time::sleep(std::time::Duration::from_secs(24)).await;

	// =========================================================
	// STEP 2: Alice bonds on AH
	// =========================================================
	println!("\n--- Step 2: Alice bonds 10K HEZ on AH ---");

	let bond_amount = 10_000 * HEZ;
	let alice_bond = pezkuwi_subxt::dynamic::tx(
		"Staking",
		"bond",
		vec![
			Value::u128(bond_amount),
			// RewardDestination::Staked = 0
			Value::unnamed_variant("Staked", vec![]),
		],
	);

	let progress = ah_api
		.tx()
		.sign_and_submit_then_watch_default(&alice_bond, &alice)
		.await?;
	wait_ah_tx(progress, "Alice bond").await?;

	// =========================================================
	// STEP 3: Bob bonds on AH
	// =========================================================
	println!("\n--- Step 3: Bob bonds 10K HEZ on AH ---");

	let bob_bond = pezkuwi_subxt::dynamic::tx(
		"Staking",
		"bond",
		vec![
			Value::u128(bond_amount),
			Value::unnamed_variant("Staked", vec![]),
		],
	);

	let progress = ah_api
		.tx()
		.sign_and_submit_then_watch_default(&bob_bond, &bob)
		.await?;
	wait_ah_tx(progress, "Bob bond").await?;

	// =========================================================
	// STEP 4: Alice validates on AH
	// =========================================================
	println!("\n--- Step 4: Alice validates on AH ---");

	let alice_validate = pezkuwi_subxt::dynamic::tx(
		"Staking",
		"validate",
		vec![Value::named_composite([
			// ValidatorPrefs { commission: Perbill(0), blocked: false }
			("commission", Value::u128(0)),
			("blocked", Value::bool(false)),
		])],
	);

	let progress = ah_api
		.tx()
		.sign_and_submit_then_watch_default(&alice_validate, &alice)
		.await?;
	wait_ah_tx(progress, "Alice validate").await?;

	// =========================================================
	// STEP 5: Bob validates on AH
	// =========================================================
	println!("\n--- Step 5: Bob validates on AH ---");

	let bob_validate = pezkuwi_subxt::dynamic::tx(
		"Staking",
		"validate",
		vec![Value::named_composite([
			("commission", Value::u128(0)),
			("blocked", Value::bool(false)),
		])],
	);

	let progress = ah_api
		.tx()
		.sign_and_submit_then_watch_default(&bob_validate, &bob)
		.await?;
	wait_ah_tx(progress, "Bob validate").await?;

	// =========================================================
	// STEP 6: set_validator_count(2) via XCM
	// =========================================================
	println!("\n--- Step 6: set_validator_count(2) via XCM ---");

	let svc_call = encode_set_validator_count(2);
	let (dest, msg) = build_xcm_transact(AH_PARA_ID, &svc_call);
	sudo_xcm_send(&rc_api, &sudo, dest, msg, "ValidatorCount = 2").await?;

	// =========================================================
	// STEP 7: force_new_era via XCM
	// =========================================================
	println!("\n--- Step 7: force_new_era via XCM ---");

	let fne_call = encode_force_new_era();
	let (dest, msg) = build_xcm_transact(AH_PARA_ID, &fne_call);
	sudo_xcm_send(&rc_api, &sudo, dest, msg, "ForceEra = ForceNew").await?;

	// Wait for XCM processing
	println!("\n  Waiting 24s for XCM processing...");
	tokio::time::sleep(std::time::Duration::from_secs(24)).await;

	// =========================================================
	// VERIFY
	// =========================================================
	println!("\n--- Verification ---");

	// Check AH Staking::ValidatorCount
	let vc_key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"ValidatorCount").iter())
		.copied()
		.collect();
	let vc = ah_api
		.storage()
		.at_latest()
		.await?
		.fetch_raw(vc_key)
		.await?;
	if !vc.is_empty() {
		let count = u32::from_le_bytes(vc[..4].try_into().unwrap_or([0; 4]));
		println!("  ValidatorCount: {}", count);
	} else {
		println!("  ValidatorCount: None (NOT SET!)");
	}

	// Check ForceEra
	let fe_key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"ForceEra").iter())
		.copied()
		.collect();
	let fe = ah_api
		.storage()
		.at_latest()
		.await?
		.fetch_raw(fe_key)
		.await?;
	if !fe.is_empty() {
		let modes = ["NotForcing", "ForceNew", "ForceNone", "ForceAlways"];
		let mode = fe[0] as usize;
		println!(
			"  ForceEra: {} ({})",
			modes.get(mode).unwrap_or(&"Unknown"),
			mode
		);
	}

	// Check Bonded (Alice)
	let bonded_prefix: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"Bonded").iter())
		.copied()
		.collect();
	let alice_hash = pezsp_crypto_hashing::blake2_128(&ALICE_PUBKEY);
	let mut alice_bonded_key = bonded_prefix.clone();
	alice_bonded_key.extend_from_slice(&alice_hash);
	alice_bonded_key.extend_from_slice(&ALICE_PUBKEY);
	let alice_bonded = ah_api
		.storage()
		.at_latest()
		.await?
		.fetch_raw(alice_bonded_key)
		.await?;
	println!("  Alice bonded: {}", !alice_bonded.is_empty());

	println!("\n=== INITIALIZATION COMPLETE ===");
	println!("\nNext: Wait for AH to plan a new era and send validator set to RC.");
	println!("Monitor with: AH ActiveEra, RC StakingAhClient::ValidatorSet");

	Ok(())
}
