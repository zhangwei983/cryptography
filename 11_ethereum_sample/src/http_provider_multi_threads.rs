use ethers::providers::{Middleware, Provider};
use std::sync::Arc;

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    let rpc_url = "https://eth.llamarpc.com";
    let provider = Arc::new(Provider::try_from(rpc_url)?);

    let current_block = provider.get_block_number().await?;
    let prev_block = current_block - 1;

    // Clone the Arc<Provider>.
    let provider_1 = provider.clone();

    // Spin up a new thread to get the uncle count of the current block.
    let task_0 = tokio::spawn(async move { provider.get_uncle_count(current_block).await });

    // Spin up a new thread to get the uncle count of the previous block.
    let task_1 = tokio::spawn(async move { provider_1.get_uncle_count(prev_block).await });

    // Wait for the tasks to finish.
    for task in [task_0, task_1] {
        if let Ok(uncle_count) = task.await? {
            println!("Uncle count: {:?}", uncle_count);
        }
    }

    println!("--- End module: {}", module_path!());

    Ok(())
}
