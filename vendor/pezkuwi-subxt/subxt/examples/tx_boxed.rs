#![allow(missing_docs)]
use pezkuwi_subxt_signer::sr25519::dev;
use pezkuwi_subxt::{OnlineClient, PezkuwiConfig};

#[pezkuwi_subxt::subxt(runtime_metadata_path = "../artifacts/pezkuwi_metadata_small.scale")]
pub mod pezkuwi {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let api = OnlineClient::<PezkuwiConfig>::new().await?;

	// Prepare some extrinsics. These are boxed so that they can live alongside each other.
	let txs = [dynamic_remark(), balance_transfer(), remark()];

	for tx in txs {
		let from = dev::alice();
		api.tx()
			.sign_and_submit_then_watch_default(&tx, &from)
			.await?
			.wait_for_finalized_success()
			.await?;

		println!("Submitted tx");
	}

	Ok(())
}

fn balance_transfer() -> Box<dyn pezkuwi_subxt::tx::Payload> {
	let dest = dev::bob().public_key().into();
	Box::new(pezkuwi::tx().balances().transfer_allow_death(dest, 10_000))
}

fn remark() -> Box<dyn pezkuwi_subxt::tx::Payload> {
	Box::new(pezkuwi::tx().system().remark(vec![1, 2, 3, 4, 5]))
}

fn dynamic_remark() -> Box<dyn pezkuwi_subxt::tx::Payload> {
	use pezkuwi_subxt::dynamic::{Value, tx};
	let tx_payload = tx("System", "remark", vec![Value::from_bytes("Hello")]);

	Box::new(tx_payload)
}
