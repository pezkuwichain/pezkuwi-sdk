#![allow(missing_docs)]
use pezkuwi_subxt_signer::sr25519::dev;
use pezkuwi_subxt::{
	OnlineClient, PezkuwiConfig,
	backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
	config::DefaultExtrinsicParamsBuilder as Params,
};

#[pezkuwi_subxt::subxt(runtime_metadata_path = "../artifacts/pezkuwi_metadata_small.scale")]
pub mod pezkuwi {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// First, create a raw RPC client:
	let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944").await?;

	// Use this to construct our RPC methods:
	let rpc = LegacyRpcMethods::<PezkuwiConfig>::new(rpc_client.clone());

	// We can use the same client to drive our full Subxt interface too:
	let api = OnlineClient::<PezkuwiConfig>::from_rpc_client(rpc_client.clone()).await?;

	// Now, we can make some RPC calls using some legacy RPC methods.
	println!(
		"📛 System Name: {:?}\n🩺 Health: {:?}\n🖫 Properties: {:?}\n🔗 Chain: {:?}\n",
		rpc.system_name().await?,
		rpc.system_health().await?,
		rpc.system_properties().await?,
		rpc.system_chain().await?
	);

	// We can also interleave RPC calls and using the full Subxt client, here to submit multiple
	// transactions using the legacy `system_account_next_index` RPC call, which returns a nonce
	// that is adjusted for any transactions already in the pool:

	let alice = dev::alice();
	let bob = dev::bob();

	loop {
		let current_nonce = rpc.system_account_next_index(&alice.public_key().into()).await?;

		let ext_params = Params::new().mortal(8).nonce(current_nonce).build();

		let balance_transfer = pezkuwi::tx()
			.balances()
			.transfer_allow_death(bob.public_key().into(), 1_000_000);

		let ext_hash = api
			.tx()
			.create_partial_offline(&balance_transfer, ext_params)?
			.sign(&alice)
			.submit()
			.await?;

		println!("Submitted ext {ext_hash} with nonce {current_nonce}");

		// Sleep less than block time, but long enough to ensure
		// not all transactions end up in the same block.
		tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
	}
}
