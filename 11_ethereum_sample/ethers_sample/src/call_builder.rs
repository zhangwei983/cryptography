use ethers::{
    providers::{Middleware, Provider, RawCall},
    types::{BlockId, H160, TransactionRequest, U64, U256, spoof::State},
    utils::parse_ether,
};
use std::sync::Arc;

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    let rpc_url: &str = "https://eth.llamarpc.com";
    let provider = Arc::new(Provider::try_from(rpc_url)?);

    let from_adress: H160 = "0x6fC21092DA55B392b045eD78F4732bff3C580e2c".parse()?;
    let to_adress: H160 = "0x000000000000000000000000000000000000dead".parse()?;
    let val = parse_ether(1u64)?;

    let tx = TransactionRequest::default()
        .from(from_adress)
        .to(to_adress)
        .value(val)
        .into();

    let previous_block: BlockId = (provider.get_block_number().await? - 1).into();

    // Override the state to simulate complicated transactions without undergoing any actual state changes.
    // Set the account balance to max u256 value and nonce to zero.
    let mut state = State::default();
    state.account(from_adress).balance(U256::MAX);
    state.account(from_adress).nonce(U64::zero());

    //let result = provider.call(&tx, Some(previous_block)).await;
    let result = provider
        .call_raw(&tx)
        .block(previous_block)
        .state(&state)
        .await;
    match result {
        Ok(_) => println!("Transaction sent successfully with: {:?}", result),
        Err(e) => println!("Error sending transaction: {:?}", e),
    }

    println!("--- End module: {}", module_path!());

    Ok(())
}
