use ethers::{contract::Abigen, solc::Solc};

// This code compiles a Solidity contract and generates Rust bindings.
pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    let contract_name = "SimpleStorage";
    let contract_path = "./contracts/contract.sol";

    // Compile the Solidity contract.
    // It requires the `solc` binary to be installed and available in the PATH.
    // https://docs.soliditylang.org/en/latest/installing-solidity.html
    let compiled_contract = Solc::default().compile_source(&contract_path)?;
    //println!("Compiled contract: {:?}", compiled_contract.sources.keys());

    // Get the ABI of the contract.
    let abi = compiled_contract
        .get(&contract_path, &contract_name)
        .ok_or(format!("Contract {} not found", contract_name))?
        .abi
        .ok_or("ABI not found")?;

    let abi_json = serde_json::to_string(abi)?;

    // Generate Rust bindings for the contract and write to the stdout.
    let bindings = Abigen::new(&contract_name, abi_json)?.generate()?;
    bindings.write(&mut std::io::stdout())?;

    println!("--- End module: {}", module_path!());

    Ok(())
}
