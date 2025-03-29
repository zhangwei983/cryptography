use alloy::providers::{Provider, ProviderBuilder};

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    // Create a provider using the given HTTP URL.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get the latest block number.
    let block_number = provider.get_block_number().await?;
    println!("Block number: {}", block_number);

    println!("--- End module: {}", module_path!());

    Ok(())
}
