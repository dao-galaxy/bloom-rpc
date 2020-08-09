
/*
#[tokio::main]
async fn main() -> web3::Result<()> {
	let _ = env_logger::try_init();
	let transport = web3::transports::Http::new("http://localhost:3030")?;
	let web3 = web3::Web3::new(transport);

	println!("Calling accounts.");
	let mut accounts = web3.eth().accounts().await?;
	println!("Accounts: {:?}", accounts);
	accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

	println!("Calling balance.");
	for account in accounts {
		let balance = web3.eth().balance(account, None).await?;
		println!("Balance of {:?}: {}", account, balance);
	}

	Ok(())
}
*/


#[tokio::main]
async fn main() -> web3::Result {
	let _ = env_logger::try_init();
	let http = web3::transports::Http::new("http://localhost:3030")?;
	let web3 = web3::Web3::new(web3::transports::Batch::new(http));

	let accounts = web3.eth().accounts();
	let block = web3.eth().block_number();

	let result = web3.transport().submit_batch().await?;
	println!("Result: {:?}", result);

	let accounts = accounts.await?;
	println!("Accounts: {:?}", accounts);

	let block = block.await?;
	println!("Block: {:?}", block);

	println!("\n==== ==== ====\n");

	let _ = env_logger::try_init();
	let transport = web3::transports::Http::new("http://localhost:3030")?;
	let web3 = web3::Web3::new(transport);

	println!("Calling accounts.");
	let mut accounts = web3.eth().accounts().await?;
	println!("Accounts: {:?}", accounts);
	accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

	println!("Calling balance.");
	for account in accounts {
		let balance = web3.eth().balance(account, None).await?;
		println!("Balance of {:?}: {}", account, balance);
	}

	Ok(())
}




