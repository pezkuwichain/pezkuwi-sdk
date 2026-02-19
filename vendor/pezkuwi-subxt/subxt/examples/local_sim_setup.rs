//! Local Mainnet Simulation Setup
//!
//! Replicates mainnet sudo operations on the local simulation:
//! 1. RC: Set balances for test accounts (Founder, Alice, Bob, TestWallet)
//! 2. AH (via XCM): Set NominationPools config (MinJoinBond, MinCreateBond)
//! 3. AH (via XCM): Set Founder and Alice balances
//! 4. People (via XCM): Set Founder balance + create Nfts Collection 0 for Tiki
//!
//! Run:
//!   SUDO_MNEMONIC="..." cargo run --release -p pezkuwi-subxt --example local_sim_setup

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};
use pezkuwi_subxt_signer::bip39::Mnemonic;
use pezkuwi_subxt_signer::sr25519::Keypair;
use std::str::FromStr;

const HEZ: u128 = 1_000_000_000_000_000_000; // 10^18

const AH_PARA_ID: u32 = 1000;
const PEOPLE_PARA_ID: u32 = 1004;

// Known public keys
const FOUNDER_PUBKEY: [u8; 32] = [
	0x28, 0x92, 0x5e, 0xd8, 0xb4, 0xc0, 0xc9, 0x54, 0x02, 0xb3, 0x15, 0x63, 0x25, 0x1f, 0xd3,
	0x18, 0x41, 0x43, 0x51, 0x11, 0x4b, 0x1c, 0x77, 0x97, 0xee, 0x78, 0x86, 0x66, 0xd2, 0x7d,
	0x63, 0x05,
];
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
const TEST_WALLET_PUBKEY: [u8; 32] = [
	0x3e, 0x2e, 0xb6, 0x2a, 0x8a, 0x77, 0xf5, 0xfc, 0x15, 0xfd, 0x3d, 0x4c, 0x6d, 0xa5, 0x3f,
	0xa6, 0xdb, 0xf9, 0x0c, 0x15, 0xd3, 0xa0, 0xd1, 0xc8, 0x8c, 0x3b, 0x1d, 0xfc, 0xe2, 0xd7,
	0x1e, 0x63,
];

/// Pallet indices (from construct_runtime!)
const AH_BALANCES: u8 = 10;
const AH_NOMINATION_POOLS: u8 = 81;
const PEOPLE_BALANCES: u8 = 10;
const PEOPLE_NFTS: u8 = 60;

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
) -> Result<bool, Box<dyn std::error::Error>> {
	let xcm_send = pezkuwi_subxt::dynamic::tx("XcmPallet", "send", vec![dest, message]);
	let sudo_tx =
		pezkuwi_subxt::dynamic::tx("Sudo", "sudo", vec![xcm_send.into_value()]);

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
	Ok(sent)
}

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
		// BigInteger mode
		let bytes = val.to_le_bytes();
		let len = 16 - val.leading_zeros() as usize / 8;
		let len = if len == 0 { 1 } else { len };
		buf.push(((len as u8 - 4) << 2) | 0x03);
		buf.extend_from_slice(&bytes[..len]);
	}
}

/// Encode Balances::force_set_balance(who, #[compact] new_free)
/// call_index = 8, amount is compact-encoded
fn encode_force_set_balance(pallet: u8, account: &[u8; 32], amount: u128) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(pallet);
	data.push(8); // force_set_balance call_index
	data.push(0x00); // MultiAddress::Id
	data.extend_from_slice(account);
	encode_compact_u128(&mut data, amount);
	data
}

/// Encode NominationPools::set_configs (call_index=11)
/// ConfigOp: Noop=0, Set(val)=1, Remove=2
/// Balance values are plain u128 LE (NOT compact)
fn encode_set_configs(pallet: u8, min_join: u128, min_create: u128) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(pallet);
	data.push(11); // set_configs call_index

	// min_join_bond: ConfigOp::Set(Balance)
	data.push(1); // Set variant
	data.extend_from_slice(&min_join.to_le_bytes());

	// min_create_bond: ConfigOp::Set(Balance)
	data.push(1); // Set variant
	data.extend_from_slice(&min_create.to_le_bytes());

	// max_pools: ConfigOp<u32>::Noop
	data.push(0);
	// max_members: ConfigOp<u32>::Noop
	data.push(0);
	// max_members_per_pool: ConfigOp<u32>::Noop
	data.push(0);
	// global_max_commission: ConfigOp<Perbill>::Noop
	data.push(0);

	data
}

/// Encode Nfts::force_create(owner, config) - call_index=1
/// CollectionConfig { settings: u64, max_supply: Option<u32>, mint_settings: MintSettings }
/// MintSettings { mint_type: Issuer, price: None, start_block: None, end_block: None,
///                default_item_settings: u64 }
fn encode_nfts_force_create(pallet: u8, owner: &[u8; 32]) -> Vec<u8> {
	let mut data = Vec::new();
	data.push(pallet);
	data.push(1); // force_create call_index

	// owner: MultiAddress::Id
	data.push(0x00);
	data.extend_from_slice(owner);

	// CollectionConfig:
	// settings: CollectionSettings = BitFlags<CollectionSetting> as u64 = 0 (all_enabled)
	data.extend_from_slice(&0u64.to_le_bytes());
	// max_supply: Option<u32> = None
	data.push(0x00);
	// mint_settings.mint_type: MintType::Issuer = variant 0
	data.push(0x00);
	// mint_settings.price: Option<Balance> = None
	data.push(0x00);
	// mint_settings.start_block: Option<BlockNumber> = None
	data.push(0x00);
	// mint_settings.end_block: Option<BlockNumber> = None
	data.push(0x00);
	// mint_settings.default_item_settings: ItemSettings = BitFlags<ItemSetting> as u64 = 0
	data.extend_from_slice(&0u64.to_le_bytes());

	data
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("╔═══════════════════════════════════════════════════════╗");
	println!("║  LOCAL SIM SETUP - Mainnet Duplication                ║");
	println!("╚═══════════════════════════════════════════════════════╝\n");

	let rc_url =
		std::env::var("RC_RPC").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
	let api = OnlineClient::<PezkuwiConfig>::from_url(&rc_url).await?;
	println!("Connected to RC: {}", rc_url);

	let mnemonic_str =
		std::env::var("SUDO_MNEMONIC").expect("SUDO_MNEMONIC environment variable required");
	let mnemonic = Mnemonic::from_str(&mnemonic_str)?;
	let sudo_keypair = Keypair::from_phrase(&mnemonic, None)?;
	println!("Sudo: {}\n", sudo_keypair.public_key().to_account_id());

	// ─── STEP 1: Set balances on RC ───
	println!("=== STEP 1: Set balances on RC ===");

	let accounts: &[(&str, &[u8; 32], u128)] = &[
		("Founder", &FOUNDER_PUBKEY, 1_000_000 * HEZ),
		("Alice", &ALICE_PUBKEY, 100_000 * HEZ),
		("Bob", &BOB_PUBKEY, 10_000 * HEZ),
		("TestWallet", &TEST_WALLET_PUBKEY, 10_000 * HEZ),
	];

	for (name, pubkey, amount) in accounts {
		let sudo_tx = pezkuwi_subxt::dynamic::tx(
			"Sudo",
			"sudo",
			vec![pezkuwi_subxt::dynamic::tx(
				"Balances",
				"force_set_balance",
				vec![
					Value::unnamed_variant("Id", vec![Value::from_bytes(*pubkey)]),
					Value::u128(*amount),
				],
			)
			.into_value()],
		);

		let progress = api
			.tx()
			.sign_and_submit_then_watch_default(&sudo_tx, &sudo_keypair)
			.await?;
		let _events = progress.wait_for_finalized_success().await?;
		println!("  {}: {} HEZ  [OK]", name, amount / HEZ);
	}

	// ─── STEP 2: AH NominationPools config via XCM ───
	println!("\n=== STEP 2: AH NominationPools config ===");

	let set_configs_data = encode_set_configs(
		AH_NOMINATION_POOLS,
		10 * HEZ,  // MinJoinBond = 10 HEZ
		100 * HEZ, // MinCreateBond = 100 HEZ
	);
	println!("  set_configs encoded: {} bytes", set_configs_data.len());

	let (dest, msg) = build_xcm_transact(AH_PARA_ID, &set_configs_data);
	let sent = sudo_xcm_send(&api, &sudo_keypair, dest, msg).await?;
	println!(
		"  MinJoinBond=10 HEZ, MinCreateBond=100 HEZ: {}",
		if sent { "XCM Sent!" } else { "WARN: no Sent event" }
	);

	// ─── STEP 3: AH balances via XCM ───
	println!("\n=== STEP 3: AH balances via XCM ===");

	let ah_balances: &[(&str, &[u8; 32], u128)] = &[
		("Founder", &FOUNDER_PUBKEY, 1_000_000 * HEZ),
		("Alice", &ALICE_PUBKEY, 100_000 * HEZ),
	];

	for (name, pubkey, amount) in ah_balances {
		let data = encode_force_set_balance(AH_BALANCES, pubkey, *amount);
		let (dest, msg) = build_xcm_transact(AH_PARA_ID, &data);
		let sent = sudo_xcm_send(&api, &sudo_keypair, dest, msg).await?;
		println!(
			"  AH {}: {} HEZ  [{}]",
			name,
			amount / HEZ,
			if sent { "OK" } else { "WARN" }
		);
	}

	// ─── STEP 4: People Chain setup via XCM ───
	println!("\n=== STEP 4: People Chain setup ===");

	// Founder balance
	let people_balance_data =
		encode_force_set_balance(PEOPLE_BALANCES, &FOUNDER_PUBKEY, 100_000 * HEZ);
	let (dest, msg) = build_xcm_transact(PEOPLE_PARA_ID, &people_balance_data);
	let sent = sudo_xcm_send(&api, &sudo_keypair, dest, msg).await?;
	println!(
		"  People Founder: 100,000 HEZ  [{}]",
		if sent { "OK" } else { "WARN" }
	);

	// Nfts Collection 0 (for Tiki)
	let nfts_data = encode_nfts_force_create(PEOPLE_NFTS, &FOUNDER_PUBKEY);
	println!("  Nfts.force_create encoded: {} bytes", nfts_data.len());
	let (dest, msg) = build_xcm_transact(PEOPLE_PARA_ID, &nfts_data);
	let sent = sudo_xcm_send(&api, &sudo_keypair, dest, msg).await?;
	println!(
		"  Nfts Collection 0: [{}]",
		if sent { "OK" } else { "WARN" }
	);

	// ─── SUMMARY ───
	println!("\n╔═══════════════════════════════════════════════════════╗");
	println!("║  SETUP COMPLETE                                       ║");
	println!("╠═══════════════════════════════════════════════════════╣");
	println!("║  RC:  Founder=1M, Alice=100K, Bob=10K, Test=10K HEZ  ║");
	println!("║  AH:  MinJoin=10, MinCreate=100 HEZ                  ║");
	println!("║       Founder=1M, Alice=100K HEZ                     ║");
	println!("║  People: Founder=100K HEZ, Nfts Collection 0         ║");
	println!("╚═══════════════════════════════════════════════════════╝");

	Ok(())
}
