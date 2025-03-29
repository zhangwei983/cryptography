use ethers::{contract::Abigen, solc::Solc};

// This code compiles a Solidity contract and generates Rust bindings.
pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    let contract_name = "SimpleStorage";
    let contract_path = "./contracts/contract.sol";

    let compiled_contract = Solc::default().compile(&contract_path)?;
    let abi = compiled_contract
        .get(&contract_path, &contract_name)
        .ok_or(format!("Contract {} not found", contract_name))?
        .abi
        .ok_or("ABI not found")?;

    let abi_json = serde_json::to_string(abi)?;

    let bindings = Abigen::new(&contract_name, abi_json)?.generate()?;
    bindings.write(&mut std::io::stdout())?;

    println!("--- End module: {}", module_path!());

    Ok(())
}
