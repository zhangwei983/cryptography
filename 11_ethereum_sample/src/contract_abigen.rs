use ethers::{prelude::abigen, providers::Provider, types::Address};
use std::sync::Arc;

// Use abigen to generate the contract bindings for the Uniswap V2 Pair contract.
abigen!(
    IERC20,
    r#"[
            function totalSupply() external view returns (uint256)
            function balanceOf(address account) external view returns (uint256)
            function transfer(address recipient, uint256 amount) external returns (bool)
            function allowance(address owner, address spender) external view returns (uint256)
            function approve(address spender, uint256 amount) external returns (bool)
            function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
            event Approval(address indexed owner, address indexed spender, uint256 value)
        ]"#,
);

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    let rpc_url = "https://eth.llamarpc.com";
    let weth_address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

    let provider = Arc::new(Provider::try_from(rpc_url)?);

    // Initialize a new instance of the Weth contract.
    let address: Address = weth_address.parse()?;
    let contract = IERC20::new(address, provider);

    // Fetch the total supply of WETH.
    if let Ok(total_supply) = contract.total_supply().call().await {
        println!("WETH total Supply: {:?}", total_supply);
    }

    println!("--- End module: {}", module_path!());

    Ok(())
}
