use ethers::{prelude::abigen, providers::Provider, types::Address};
use std::sync::Arc;

// Use abigen to generate the contract bindings for the Uniswap V2 Pair contract.
abigen!(
    IUniswapV2Pair,
    "[function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)]"
);

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    let rpc_url = "https://eth.llamarpc.com";
    let provider = Arc::new(Provider::try_from(rpc_url)?);

    // Initialize a new instance of the Weth/Dai Uniswap V2 pair contract.
    let pair_address: Address = "0xA478c2975Ab1Ea89e8196811F51A7B7Ade33eB11".parse()?;
    let uniswap_v2_pair = IUniswapV2Pair::new(pair_address, provider);

    // Use the get_reserves() function to fetch the pool reserves.
    let (reserve0, reverse_1, block_timestamp_last) = uniswap_v2_pair.get_reserves().call().await?;
    println!("Reserve0: {:?}", reserve0);
    println!("Reserve1: {:?}", reverse_1);
    println!("Block Timestamp Last: {:?}", block_timestamp_last);

    println!("--- End module: {}", module_path!());

    Ok(())
}
