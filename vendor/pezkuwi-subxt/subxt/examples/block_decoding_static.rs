#![allow(missing_docs)]
use pezkuwi_subxt::{
	OnlineClient, PezkuwiConfig,
	utils::{AccountId32, MultiAddress},
};

use codec::Decode;

#[pezkuwi_subxt::subxt(runtime_metadata_path = "../artifacts/pezkuwi_metadata_small.scale")]
pub mod pezkuwi {}

use pezkuwi::balances::calls::types::TransferKeepAlive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Create a client that subscribes to blocks of the Pezkuwi network.
	let api = OnlineClient::<PezkuwiConfig>::from_url("wss://rpc.pezkuwi.io:443").await?;

	// Subscribe to all finalized blocks:
	let mut blocks_sub = api.blocks().subscribe_finalized().await?;

	// For each block, print details about the `TransferKeepAlive` transactions we are interested
	// in.
	while let Some(block) = blocks_sub.next().await {
		let block = block?;
		let block_number = block.header().number;
		let block_hash = block.hash();
		println!("Block #{block_number}  ({block_hash}):");

		let extrinsics = block.extrinsics().await?;
		for transfer in extrinsics.find::<TransferKeepAlive>() {
			let transfer = transfer?;

			let Some(extensions) = transfer.details.transaction_extensions() else {
				panic!("TransferKeepAlive should be signed")
			};

			let addr_bytes =
				transfer.details.address_bytes().expect("TransferKeepAlive should be signed");
			let sender = MultiAddress::<AccountId32, ()>::decode(&mut &addr_bytes[..])
				.expect("Decoding should work");
			let sender = display_address(&sender);
			let receiver = display_address(&transfer.value.dest);
			let value = transfer.value.value;
			let tip = extensions.tip().expect("Should have tip");
			let nonce = extensions.nonce().expect("Should have nonce");

			println!(
				"    Transfer of {value} DOT:\n        {sender} (Tip: {tip}, Nonce: {nonce}) ---> {receiver}",
			);
		}
	}

	Ok(())
}

fn display_address(addr: &MultiAddress<AccountId32, ()>) -> String {
	if let MultiAddress::Id(id32) = addr { format!("{id32}") } else { "MultiAddress::...".into() }
}
