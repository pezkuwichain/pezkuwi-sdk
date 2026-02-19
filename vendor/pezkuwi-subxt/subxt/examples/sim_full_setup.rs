//! Full Simulation Setup — Mainnet Simulation Initialization
//!
//! Uses REAL validator stash keys (from wallet file) on the local mainnet simulation.
//!
//! Phase 1: Fund validator stash accounts + test wallets on AH (via XCM)
//! Phase 2: Staking config via XCM (ValidatorCount, min bonds, pool configs)
//! Phase 3: Validators bond + validate on AH (direct tx, real stash keys)
//! Phase 4: Test wallets bond + nominate validators
//! Phase 5: NominationPool — Test06 creates, Test07-10 join
//! Phase 6: Fund People Chain + referrals
//! Phase 7: Force new era
//! Phase 8: Verify state
//!
//! Run:
//!   SUDO_MNEMONIC="..." WALLETS_FILE="/path/to/wallets.json" \
//!     cargo run --release -p pezkuwi-subxt --example sim_full_setup
//!
//! Optional:
//!   RC_RPC="ws://127.0.0.1:9944"
//!   AH_RPC="ws://127.0.0.1:40944"
//!   PEOPLE_RPC="ws://127.0.0.1:41944"

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::tx::TxStatus;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use pezkuwi_subxt_signer::SecretUri;
use std::str::FromStr;

/// Load a keypair from the wallets JSON array by wallet name
fn wallet_keypair(wallets: &[serde_json::Value], name: &str) -> Keypair {
	let wallet = wallets
		.iter()
		.find(|w| w["name"].as_str() == Some(name))
		.unwrap_or_else(|| panic!("Wallet '{}' not found in wallets file", name));
	let seed = wallet["seed_phrase"]
		.as_str()
		.unwrap_or_else(|| panic!("Wallet '{}' has no seed_phrase", name));
	let mnemonic = Mnemonic::from_str(seed).unwrap_or_else(|e| {
		panic!("Invalid mnemonic for '{}': {}", name, e)
	});
	Keypair::from_phrase(&mnemonic, None).unwrap_or_else(|e| {
		panic!("Failed to create keypair for '{}': {}", name, e)
	})
}

const HEZ: u128 = 1_000_000_000_000; // 10^12 — pezkuwichain_runtime_constants::currency::UNITS
const AH_PARA_ID: u32 = 1000;
const PEOPLE_PARA_ID: u32 = 1004;

// AH pallet indices
const AH_BALANCES: u8 = 10;
const AH_STAKING: u8 = 80;
const AH_NOM_POOLS: u8 = 81;

// People pallet indices
const PEOPLE_BALANCES: u8 = 10;
const PEOPLE_REFERRAL: u8 = 52;

/// SCALE Compact<u128>
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

/// SCALE Compact<u32>
fn encode_compact_u32(buf: &mut Vec<u8>, val: u32) {
	encode_compact_u128(buf, val as u128);
}

/// Encode Balances::force_set_balance(who, amount)
fn encode_force_set_balance(pallet: u8, account: &[u8; 32], amount: u128) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(pallet);
	data.push(8); // call_index
	data.push(0x00); // MultiAddress::Id
	data.extend_from_slice(account);
	encode_compact_u128(&mut data, amount);
	data
}

/// Encode Staking::set_validator_count(new)
fn encode_set_validator_count(count: u32) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(AH_STAKING);
	data.push(9);
	encode_compact_u32(&mut data, count);
	data
}

/// Encode Staking::force_new_era()
fn encode_force_new_era() -> Vec<u8> {
	vec![AH_STAKING, 13]
}

/// Encode Staking::set_staking_configs(...)
/// All params are ConfigOp<T> where Noop=0, Set=1(value), Remove=2
/// Balance values are plain u128 LE (NOT compact)
fn encode_set_staking_configs(
	min_nominator_bond: Option<u128>,
	min_validator_bond: Option<u128>,
) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(AH_STAKING);
	data.push(22); // call_index for set_staking_configs

	// min_nominator_bond: ConfigOp<Balance>
	match min_nominator_bond {
		Some(v) => {
			data.push(1); // Set
			data.extend_from_slice(&v.to_le_bytes());
		},
		None => data.push(0), // Noop
	}

	// min_validator_bond: ConfigOp<Balance>
	match min_validator_bond {
		Some(v) => {
			data.push(1);
			data.extend_from_slice(&v.to_le_bytes());
		},
		None => data.push(0),
	}

	// max_nominator_count: Noop
	data.push(0);
	// max_validator_count: Noop
	data.push(0);
	// chill_threshold: Noop
	data.push(0);
	// min_commission: Noop
	data.push(0);
	// max_staked_rewards: Noop
	data.push(0);

	data
}

/// Encode NominationPools::set_configs(min_join, min_create, ...)
/// Balance values are plain u128 LE (NOT compact)
fn encode_pool_set_configs(min_join: u128, min_create: u128) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(AH_NOM_POOLS);
	data.push(11); // call_index for set_configs

	// min_join_bond: Set(value)
	data.push(1);
	data.extend_from_slice(&min_join.to_le_bytes());

	// min_create_bond: Set(value)
	data.push(1);
	data.extend_from_slice(&min_create.to_le_bytes());

	// max_pools: Noop
	data.push(0);
	// max_members: Noop
	data.push(0);
	// max_members_per_pool: Noop
	data.push(0);
	// global_max_commission: Noop
	data.push(0);

	data
}

/// Encode Referral::force_confirm_referral(referrer, referred)
fn encode_force_confirm_referral(referrer: &[u8; 32], referred: &[u8; 32]) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(PEOPLE_REFERRAL);
	data.push(1); // call_index for force_confirm_referral
	data.extend_from_slice(referrer);
	data.extend_from_slice(referred);
	data
}

/// Build XCM V3 transact
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

/// Send sudo XCM
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
	}
	Ok(())
}

/// Submit AH tx with retry on priority/nonce/invalid errors
async fn submit_ah_tx(
	api: &OnlineClient<PezkuwiConfig>,
	tx: &pezkuwi_subxt::tx::DynamicPayload,
	signer: &Keypair,
	label: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
	for attempt in 0..5 {
		if attempt > 0 {
			// Wait one block before retry (nonce race condition)
			tokio::time::sleep(std::time::Duration::from_secs(12)).await;
		}
		match api.tx().sign_and_submit_then_watch_default(tx, signer).await {
			Ok(progress) => return wait_tx(progress, label).await,
			Err(e) => {
				let msg = format!("{}", e);
				if msg.contains("Priority") || msg.contains("priority") ||
					msg.contains("Invalid Transaction") || msg.contains("1010")
				{
					println!(
						"  [RETRY] {} — {} (attempt {}/5)",
						label,
						if msg.contains("1010") { "nonce race" } else { "priority" },
						attempt + 1
					);
					continue;
				}
				println!("  [FAIL] {} — {}", label, e);
				return Ok(false);
			},
		}
	}
	println!("  [FAIL] {} — max retries exceeded", label);
	Ok(false)
}

/// Wait for AH tx result
async fn wait_tx(
	mut progress: pezkuwi_subxt::tx::TxProgress<PezkuwiConfig, OnlineClient<PezkuwiConfig>>,
	label: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
	loop {
		match progress.next().await {
			Some(Ok(TxStatus::InBestBlock(details))) => match details.wait_for_success().await {
				Ok(_) => {
					println!("  [OK] {}", label);
					return Ok(true);
				},
				Err(e) => {
					let err_str = format!("{:?}", e);
					println!("  [FAIL] {} — {}", label, e);
					if err_str.contains("DispatchError") {
						println!("  [DEBUG] Raw error: {}", err_str);
					}
					return Ok(false);
				},
			},
			Some(Ok(TxStatus::Error { message })) |
			Some(Ok(TxStatus::Invalid { message })) |
			Some(Ok(TxStatus::Dropped { message })) => {
				println!("  [FAIL] {} — {}", label, message);
				return Ok(false);
			},
			Some(Err(e)) => {
				println!("  [FAIL] {} — {}", label, e);
				return Ok(false);
			},
			None => {
				println!("  [FAIL] {} — stream ended", label);
				return Ok(false);
			},
			_ => {},
		}
	}
}

/// Poll AH balance for an account until nonzero or timeout (seconds)
async fn poll_ah_balance(
	api: &OnlineClient<PezkuwiConfig>,
	account: &[u8; 32],
	timeout_secs: u64,
) -> Result<u128, Box<dyn std::error::Error>> {
	let start = std::time::Instant::now();
	loop {
		let storage = api.storage().at_latest().await?;
		let mut key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"System")
			.iter()
			.chain(pezsp_crypto_hashing::twox_128(b"Account").iter())
			.copied()
			.collect();
		key.extend_from_slice(&pezsp_crypto_hashing::blake2_128(account));
		key.extend_from_slice(account);
		let data = storage.fetch_raw(key).await?;
		// AccountInfo layout: nonce(4) + consumers(4) + providers(4) + sufficients(4)
		// + AccountData { free(16), reserved(16), frozen(16), flags(16) }
		if data.len() >= 32 {
			let offset = 16; // skip nonce+consumers+providers+sufficients
			let free = u128::from_le_bytes(data[offset..offset + 16].try_into().unwrap());
			if free > 0 {
				return Ok(free);
			}
		}
		if start.elapsed().as_secs() >= timeout_secs {
			return Ok(0);
		}
		tokio::time::sleep(std::time::Duration::from_secs(6)).await;
	}
}

/// Check if an account is already bonded on AH
/// Note: Bonded storage uses Twox64Concat hasher (NOT Blake2_128Concat)
async fn is_bonded(
	api: &OnlineClient<PezkuwiConfig>,
	account: &[u8; 32],
) -> Result<bool, Box<dyn std::error::Error>> {
	let storage = api.storage().at_latest().await?;
	let mut key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"Bonded").iter())
		.copied()
		.collect();
	// Twox64Concat: twox_64(account) ++ account
	let hash = pezsp_crypto_hashing::twox_64(account);
	key.extend_from_slice(&hash);
	key.extend_from_slice(account);
	// fetch_raw returns NoValueFound when key doesn't exist — that means "not bonded"
	match storage.fetch_raw(key).await {
		Ok(data) => Ok(!data.is_empty()),
		Err(_) => Ok(false),
	}
}

/// Generate test keypair from derivation path
fn test_keypair(n: u32) -> Keypair {
	let uri = format!("//Test{}", n);
	Keypair::from_uri(&SecretUri::from_str(&uri).unwrap()).unwrap()
}

fn pubkey_bytes(kp: &Keypair) -> [u8; 32] {
	let id = kp.public_key().to_account_id();
	let bytes = id.0;
	bytes
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("=== FULL SIMULATION SETUP ===\n");

	let rc_url = std::env::var("RC_RPC").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
	let ah_url =
		std::env::var("AH_RPC").unwrap_or_else(|_| "ws://127.0.0.1:40944".to_string());
	let _people_url =
		std::env::var("PEOPLE_RPC").unwrap_or_else(|_| "ws://127.0.0.1:41944".to_string());

	let rc_api = OnlineClient::<PezkuwiConfig>::from_url(&rc_url).await?;
	let ah_api = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;

	println!("RC: {} (spec {})", rc_url, rc_api.runtime_version().spec_version);
	println!("AH: {} (spec {})\n", ah_url, ah_api.runtime_version().spec_version);

	let mnemonic_str =
		std::env::var("SUDO_MNEMONIC").expect("SUDO_MNEMONIC environment variable required");
	let sudo = Keypair::from_phrase(&Mnemonic::from_str(&mnemonic_str)?, None)?;

	// Load real validator stash keys from wallet file
	let wallets_path = std::env::var("WALLETS_FILE")
		.unwrap_or_else(|_| "/home/mamostehp/res/MAINNET_WALLETS_20260128_235407.json".to_string());
	let wallets_json: serde_json::Value =
		serde_json::from_str(&std::fs::read_to_string(&wallets_path)?)?;
	let wallets_arr = wallets_json["wallets"]
		.as_array()
		.expect("wallets.json must contain a 'wallets' array");

	let val1 = wallet_keypair(wallets_arr, "Validator_01_Stash");
	let val2 = wallet_keypair(wallets_arr, "Validator_02_Stash");

	let val1_pub = pubkey_bytes(&val1);
	let val2_pub = pubkey_bytes(&val2);

	// Generate 10 test wallets
	let test_wallets: Vec<(Keypair, [u8; 32])> = (1..=10)
		.map(|i| {
			let kp = test_keypair(i);
			let pub_bytes = pubkey_bytes(&kp);
			(kp, pub_bytes)
		})
		.collect();

	println!("Val01:  {}", val1.public_key().to_account_id());
	println!("Val02:  {}", val2.public_key().to_account_id());
	for (i, (kp, _)) in test_wallets.iter().enumerate() {
		println!("Test{:02}: {}", i + 1, kp.public_key().to_account_id());
	}

	// =========================================================
	// PHASE 1: Fund all accounts on AH via XCM
	// =========================================================
	println!("\n========== PHASE 1: Fund AH Accounts ==========");

	// Validator stash accounts: 100K HEZ each
	sudo_xcm(
		&rc_api,
		&sudo,
		AH_PARA_ID,
		&encode_force_set_balance(AH_BALANCES, &val1_pub, 100_000 * HEZ),
		"Val01 100K HEZ on AH",
	)
	.await?;

	sudo_xcm(
		&rc_api,
		&sudo,
		AH_PARA_ID,
		&encode_force_set_balance(AH_BALANCES, &val2_pub, 100_000 * HEZ),
		"Val02 100K HEZ on AH",
	)
	.await?;

	// 10 test wallets: 20K HEZ each
	for (i, (_, pub_bytes)) in test_wallets.iter().enumerate() {
		sudo_xcm(
			&rc_api,
			&sudo,
			AH_PARA_ID,
			&encode_force_set_balance(AH_BALANCES, pub_bytes, 20_000 * HEZ),
			&format!("Test{:02} 20K HEZ on AH", i + 1),
		)
		.await?;
	}

	// Wait for DMP processing, verify Val01 balance on AH
	println!("\n  Waiting for DMP processing (polling Val01 balance on AH)...");
	let val1_balance = poll_ah_balance(&ah_api, &val1_pub, 120).await?;
	if val1_balance == 0 {
		println!("  [FATAL] Val01 has 0 balance on AH after 120s — DMP not processed. Aborting.");
		return Ok(());
	}
	println!("  [OK] Val01 AH balance: {} HEZ", val1_balance / HEZ);

	// =========================================================
	// PHASE 2: Staking config via XCM (root operations)
	// =========================================================
	println!("\n========== PHASE 2: Staking Config ==========");

	// Set staking configs: min_nominator_bond=100 HEZ, min_validator_bond=1000 HEZ
	sudo_xcm(
		&rc_api,
		&sudo,
		AH_PARA_ID,
		&encode_set_staking_configs(Some(100 * HEZ), Some(1_000 * HEZ)),
		"Staking configs (min_nom=100, min_val=1000 HEZ)",
	)
	.await?;

	// Set validator count = 2
	sudo_xcm(
		&rc_api,
		&sudo,
		AH_PARA_ID,
		&encode_set_validator_count(2),
		"ValidatorCount = 2",
	)
	.await?;

	// NominationPools config: MinJoin=10 HEZ, MinCreate=100 HEZ
	sudo_xcm(
		&rc_api,
		&sudo,
		AH_PARA_ID,
		&encode_pool_set_configs(10 * HEZ, 100 * HEZ),
		"NominationPools config (MinJoin=10, MinCreate=100 HEZ)",
	)
	.await?;

	println!("\n  Waiting 30s for DMP processing...");
	tokio::time::sleep(std::time::Duration::from_secs(30)).await;

	// =========================================================
	// PHASE 3: Bond + Validate (Val01 & Val02 directly on AH)
	// =========================================================
	println!("\n========== PHASE 3: Validators Bond + Validate ==========");

	let bond_amount = 50_000 * HEZ;

	// Check if already bonded (idempotency for re-runs)
	let val1_already_bonded = is_bonded(&ah_api, &val1_pub).await?;
	let val2_already_bonded = is_bonded(&ah_api, &val2_pub).await?;

	if val1_already_bonded {
		println!("  [SKIP] Val01 already bonded — skipping bond");
	} else {
		let tx = pezkuwi_subxt::dynamic::tx(
			"Staking",
			"bond",
			vec![Value::u128(bond_amount), Value::unnamed_variant("Staked", vec![])],
		);
		submit_ah_tx(&ah_api, &tx, &val1, "Val01 bond 50K HEZ").await?;
	}
	// Always try validate (idempotent — will fail harmlessly if already validating)
	let tx = pezkuwi_subxt::dynamic::tx(
		"Staking",
		"validate",
		vec![Value::named_composite([
			("commission", Value::u128(0)),
			("blocked", Value::bool(false)),
		])],
	);
	submit_ah_tx(&ah_api, &tx, &val1, "Val01 validate").await?;

	if val2_already_bonded {
		println!("  [SKIP] Val02 already bonded — skipping bond");
	} else {
		let tx = pezkuwi_subxt::dynamic::tx(
			"Staking",
			"bond",
			vec![Value::u128(bond_amount), Value::unnamed_variant("Staked", vec![])],
		);
		submit_ah_tx(&ah_api, &tx, &val2, "Val02 bond 50K HEZ").await?;
	}
	// Always try validate
	let tx = pezkuwi_subxt::dynamic::tx(
		"Staking",
		"validate",
		vec![Value::named_composite([
			("commission", Value::u128(0)),
			("blocked", Value::bool(false)),
		])],
	);
	submit_ah_tx(&ah_api, &tx, &val2, "Val02 validate").await?;

	// =========================================================
	// PHASE 4: Test wallets nominate validators
	// =========================================================
	println!("\n========== PHASE 4: Test Wallets Nominate ==========");

	// Test1-5: bond + nominate both validators
	for i in 0..5 {
		let (kp, pub_bytes) = &test_wallets[i];
		let nom_amount = 5_000 * HEZ;

		if is_bonded(&ah_api, pub_bytes).await? {
			println!("  [SKIP] Test{:02} already bonded — skipping bond", i + 1);
		} else {
			// Bond
			let tx = pezkuwi_subxt::dynamic::tx(
				"Staking",
				"bond",
				vec![Value::u128(nom_amount), Value::unnamed_variant("Staked", vec![])],
			);
			submit_ah_tx(&ah_api, &tx, kp, &format!("Test{:02} bond 5K HEZ", i + 1)).await?;
		}

		// Always try nominate (idempotent — will update if already nominating)
		let tx = pezkuwi_subxt::dynamic::tx(
			"Staking",
			"nominate",
			vec![Value::unnamed_composite(vec![
				Value::unnamed_variant(
					"Id",
					vec![Value::from_bytes(&val1_pub)],
				),
				Value::unnamed_variant(
					"Id",
					vec![Value::from_bytes(&val2_pub)],
				),
			])],
		);
		submit_ah_tx(&ah_api, &tx, kp, &format!("Test{:02} nominate Val01+Val02", i + 1)).await?;
	}

	// =========================================================
	// PHASE 5: NominationPool — Test6 creates, Test7-10 join
	// =========================================================
	println!("\n========== PHASE 5: Nomination Pool ==========");

	let (pool_creator, pool_creator_pub) = &test_wallets[5]; // Test06

	// Check if pool already exists (LastPoolId storage)
	let pool_key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"NominationPools")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"LastPoolId").iter())
		.copied()
		.collect();
	let pool_exists = match ah_api.storage().at_latest().await?.fetch_raw(pool_key).await {
		Ok(d) if !d.is_empty() && d.len() >= 4 => {
			let id = u32::from_le_bytes(d[..4].try_into().unwrap());
			id >= 1
		},
		_ => false,
	};

	if pool_exists {
		println!("  [SKIP] Pool already exists — skipping create+join");
	} else {
		// Test06 creates pool with 1000 HEZ
		let tx = pezkuwi_subxt::dynamic::tx(
			"NominationPools",
			"create",
			vec![
				Value::u128(1_000 * HEZ), // amount
				Value::unnamed_variant("Id", vec![Value::from_bytes(pool_creator_pub)]), // root
				Value::unnamed_variant("Id", vec![Value::from_bytes(pool_creator_pub)]), // nominator
				Value::unnamed_variant("Id", vec![Value::from_bytes(pool_creator_pub)]), // bouncer
			],
		);
		submit_ah_tx(&ah_api, &tx, pool_creator, "Test06 create pool (1000 HEZ)").await?;

		// Pool nominate Val01
		let tx = pezkuwi_subxt::dynamic::tx(
			"NominationPools",
			"nominate",
			vec![
				Value::u128(1), // pool_id
				Value::unnamed_composite(vec![Value::from_bytes(&val1_pub)]),
			],
		);
		submit_ah_tx(&ah_api, &tx, pool_creator, "Pool 1 nominate Val01").await?;

		// Test07-10 join pool
		for i in 6..10 {
			let (kp, _) = &test_wallets[i];
			let tx = pezkuwi_subxt::dynamic::tx(
				"NominationPools",
				"join",
				vec![
					Value::u128(500 * HEZ), // amount
					Value::u128(1),          // pool_id
				],
			);
			submit_ah_tx(&ah_api, &tx, kp, &format!("Test{:02} join pool 1 (500 HEZ)", i + 1))
				.await?;
		}
	}

	// =========================================================
	// PHASE 6: Fund People Chain + Referrals
	// =========================================================
	println!("\n========== PHASE 6: People Chain + Referrals ==========");

	// Fund test wallets on People Chain
	for (i, (_, pub_bytes)) in test_wallets.iter().enumerate() {
		sudo_xcm(
			&rc_api,
			&sudo,
			PEOPLE_PARA_ID,
			&encode_force_set_balance(PEOPLE_BALANCES, pub_bytes, 1_000 * HEZ),
			&format!("Test{:02} 1K HEZ on People", i + 1),
		)
		.await?;
	}

	println!("\n  Waiting 30s for DMP processing...");
	tokio::time::sleep(std::time::Duration::from_secs(30)).await;

	// Referral chain: Test01 refers Test02, Test02 refers Test03, etc.
	// Using force_confirm_referral via XCM (root) since KYC not setup in sim
	for i in 0..9 {
		let referrer_pub = &test_wallets[i].1;
		let referred_pub = &test_wallets[i + 1].1;
		sudo_xcm(
			&rc_api,
			&sudo,
			PEOPLE_PARA_ID,
			&encode_force_confirm_referral(referrer_pub, referred_pub),
			&format!("Test{:02} refers Test{:02}", i + 1, i + 2),
		)
		.await?;
	}

	// =========================================================
	// PHASE 7: Force new era to trigger validator set flow
	// =========================================================
	println!("\n========== PHASE 7: Trigger Era Rotation ==========");

	sudo_xcm(
		&rc_api,
		&sudo,
		AH_PARA_ID,
		&encode_force_new_era(),
		"ForceEra = ForceNew on AH",
	)
	.await?;

	println!("\n  Waiting 30s for era planning...");
	tokio::time::sleep(std::time::Duration::from_secs(30)).await;

	// =========================================================
	// PHASE 8: Verify
	// =========================================================
	println!("\n========== VERIFICATION ==========");

	let storage = ah_api.storage().at_latest().await?;

	// ValidatorCount
	let key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"ValidatorCount").iter())
		.copied()
		.collect();
	let data = storage.fetch_raw(key).await?;
	if data.len() >= 4 {
		let n = u32::from_le_bytes(data[..4].try_into().unwrap());
		println!("  AH ValidatorCount: {}", n);
	} else {
		println!("  AH ValidatorCount: NOT SET");
	}

	// ForceEra
	let key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"ForceEra").iter())
		.copied()
		.collect();
	let data = storage.fetch_raw(key).await?;
	if !data.is_empty() {
		let modes = ["NotForcing", "ForceNew", "ForceNone", "ForceAlways"];
		println!(
			"  AH ForceEra: {}",
			modes.get(data[0] as usize).unwrap_or(&"?")
		);
	}

	// ActiveEra
	let key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"ActiveEra").iter())
		.copied()
		.collect();
	let data = storage.fetch_raw(key).await?;
	if data.len() >= 4 {
		let era = u32::from_le_bytes(data[..4].try_into().unwrap());
		println!("  AH ActiveEra: {}", era);
	}

	// CurrentEra
	let key: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"CurrentEra").iter())
		.copied()
		.collect();
	let data = storage.fetch_raw(key).await?;
	if data.len() >= 4 {
		let era = u32::from_le_bytes(data[..4].try_into().unwrap());
		println!("  AH CurrentEra: {}", era);
	}

	// Bonded check (Twox64Concat hasher)
	let bonded_prefix: Vec<u8> = pezsp_crypto_hashing::twox_128(b"Staking")
		.iter()
		.chain(pezsp_crypto_hashing::twox_128(b"Bonded").iter())
		.copied()
		.collect();
	let mut val1_key = bonded_prefix.clone();
	val1_key.extend_from_slice(&pezsp_crypto_hashing::twox_64(&val1_pub));
	val1_key.extend_from_slice(&val1_pub);
	let val1_bonded = match storage.fetch_raw(val1_key).await {
		Ok(d) => !d.is_empty(),
		Err(_) => false,
	};
	println!("  Val01 bonded: {}", val1_bonded);

	let mut val2_key = bonded_prefix;
	val2_key.extend_from_slice(&pezsp_crypto_hashing::twox_64(&val2_pub));
	val2_key.extend_from_slice(&val2_pub);
	let val2_bonded = match storage.fetch_raw(val2_key).await {
		Ok(d) => !d.is_empty(),
		Err(_) => false,
	};
	println!("  Val02 bonded: {}", val2_bonded);

	println!("\n=== SETUP COMPLETE ===");
	println!("\nExpected flow:");
	println!("  1. AH staking plans new era → sends ValidatorSet to RC via XCM");
	println!("  2. RC receives ValidatorSet → stores it");
	println!("  3. RC session end → sends SessionReport with activation_timestamp to AH");
	println!("  4. AH ActiveEra advances");
	println!("\nMonitor: watch AH ActiveEra, RC StakingAhClient::ValidatorSet");
	println!("If era doesn't advance after 5+ sessions, investigate AH staking election.");

	Ok(())
}
