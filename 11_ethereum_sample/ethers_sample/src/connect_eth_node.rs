use ethers::prelude::*;

const RPC_URL: &str = "https://eth.llamarpc.com";

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    let provider = Provider::<Http>::try_from(RPC_URL)?;

    let chain_id = provider.get_chainid().await?;
    println!("Chain ID: {:?}", chain_id);

    let block_number = provider.get_block_number().await?;
    println!("Block number: {:?}", block_number);

    let tx_pool_content = provider.txpool_content().await;
    match tx_pool_content {
        Ok(content) => println!("Transaction pool content: {:?}", content),
        Err(e) => println!("Error fetching transaction pool content: {:?}", e),
    }

    println!("--- End module: {}", module_path!());

    Ok(())
}
