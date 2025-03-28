use ethers::providers::{Middleware, Provider};
use std::sync::Arc;

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    let rpc_url = "https://eth.llamarpc.com";
    let provider = Arc::new(Provider::try_from(rpc_url)?);

    let current_block = provider.get_block_number().await?;
    let prev_block = current_block - 1;

    let provider_1 = provider.clone();

    let task_0 = tokio::spawn(async move { provider.get_uncle_count(current_block).await });

    let task_1 = tokio::spawn(async move { provider_1.get_uncle_count(prev_block).await });

    for task in [task_0, task_1] {
        if let Ok(uncle_count) = task.await? {
            println!("Uncle count: {:?}", uncle_count);
        }
    }

    println!("--- End module: {}", module_path!());

    Ok(())
}
