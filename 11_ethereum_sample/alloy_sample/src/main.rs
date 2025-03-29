pub mod basic_usage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    basic_usage::test().await?;

    Ok(())
}
