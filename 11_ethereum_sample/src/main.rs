pub mod call_builder;
pub mod connect_eth_node;
pub mod contract_abigen;
pub mod contract_compile;
pub mod http_provider;
pub mod http_provider_multi_threads;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    connect_eth_node::test().await?;
    println!("");
    http_provider::test().await?;
    println!("");
    http_provider_multi_threads::test().await?;
    println!("");
    call_builder::test().await?;
    println!("");
    contract_abigen::test().await?;
    println!("");
    contract_compile::test().await?;

    Ok(())
}
