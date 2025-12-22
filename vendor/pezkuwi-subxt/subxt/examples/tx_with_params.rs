#![allow(missing_docs)]
use pezkuwi_subxt_signer::sr25519::dev;
use pezkuwi_subxt::{
	OnlineClient, PezkuwiConfig, config::pezkuwi::PezkuwiExtrinsicParamsBuilder as Params,
};

#[pezkuwi_subxt::subxt(runtime_metadata_path = "../artifacts/pezkuwi_metadata_small.scale")]
pub mod pezkuwi {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Create a new API client, configured to talk to Pezkuwi nodes.
	let api = OnlineClient::<PezkuwiConfig>::new().await?;

	// Build a balance transfer extrinsic.
	let dest = dev::bob().public_key().into();
	let tx = pezkuwi::tx().balances().transfer_allow_death(dest, 10_000);

	// Configure the transaction parameters; we give a small tip and set the
	// transaction to live for 32 blocks from the `latest_block` above.
	let tx_params = Params::new().tip(1_000).mortal(32).build();

	// submit the transaction:
	let from = dev::alice();
	let hash = api.tx().sign_and_submit(&tx, &from, tx_params).await?;
	println!("Balance transfer extrinsic submitted with hash : {hash}");

	Ok(())
}
