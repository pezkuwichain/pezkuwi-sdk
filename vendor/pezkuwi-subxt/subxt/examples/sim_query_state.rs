//! Query AH staking state using subxt dynamic storage API
//!
//! Run:
//!   cargo run --release -p pezkuwi-subxt --example sim_query_state

#![allow(missing_docs)]
use pezkuwi_subxt::dynamic::Value;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let ah_url = std::env::var("AH_RPC").unwrap_or_else(|_| "ws://127.0.0.1:40944".to_string());
	let api = OnlineClient::<PezkuwiConfig>::from_url(&ah_url).await?;

	println!("=== AH STAKING STATE ===\n");

	let storage = api.storage().at_latest().await?;

	let items = [
		("Staking", "CurrentEra"),
		("Staking", "ActiveEra"),
		("Staking", "ForceEra"),
		("Staking", "ValidatorCount"),
		("Staking", "CounterForValidators"),
		("Staking", "CounterForNominators"),
		("Staking", "NextElectionPage"),
		("Staking", "OutgoingValidatorSet"),
		("Staking", "ElectableStashes"),
		("Staking", "BondedEras"),
		("Staking", "MinimumValidatorCount"),
	];

	for (pallet, name) in &items {
		let addr = pezkuwi_subxt::dynamic::storage::<(), Value>(*pallet, *name);
		match storage.entry(addr) {
			Ok(entry) => match entry.try_fetch(()).await {
				Ok(Some(val)) => {
					let decoded = val.decode();
					println!("{}.{} = {:?}", pallet, name, decoded);
				},
				Ok(None) => println!("{}.{} = None", pallet, name),
				Err(e) => println!("{}.{} = <fetch error: {}>", pallet, name, e),
			},
			Err(e) => println!("{}.{} = <entry error: {}>", pallet, name, e),
		}
	}

	println!();

	let mbe_items = [
		("MultiBlockElection", "CurrentPhase"),
		("MultiBlockElection", "Round"),
	];

	for (pallet, name) in &mbe_items {
		let addr = pezkuwi_subxt::dynamic::storage::<(), Value>(*pallet, *name);
		match storage.entry(addr) {
			Ok(entry) => match entry.try_fetch(()).await {
				Ok(Some(val)) => println!("{}.{} = {:?}", pallet, name, val.decode()),
				Ok(None) => println!("{}.{} = None", pallet, name),
				Err(e) => println!("{}.{} = <fetch error: {}>", pallet, name, e),
			},
			Err(e) => println!("{}.{} = <entry error: {}>", pallet, name, e),
		}
	}

	println!();

	let rc_items = [
		("StakingRcClient", "LastSessionReportEndingIndex"),
		("StakingRcClient", "Mode"),
	];

	for (pallet, name) in &rc_items {
		let addr = pezkuwi_subxt::dynamic::storage::<(), Value>(*pallet, *name);
		match storage.entry(addr) {
			Ok(entry) => match entry.try_fetch(()).await {
				Ok(Some(val)) => println!("{}.{} = {:?}", pallet, name, val.decode()),
				Ok(None) => println!("{}.{} = None", pallet, name),
				Err(e) => println!("{}.{} = <fetch error: {}>", pallet, name, e),
			},
			Err(e) => println!("{}.{} = <entry error: {}>", pallet, name, e),
		}
	}

	Ok(())
}
