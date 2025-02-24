use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network};
use std::str::FromStr;

fn generate_keys_and_address(secret_key: SecretKey, is_compressed: bool) {
    // Create a uncompressed private key from the secret key.
    let private_key = if is_compressed {
        let key = bitcoin::PrivateKey::new(secret_key, Network::Bitcoin);
        println!("Compressed WIF: {}", key.to_wif());
        key
    } else {
        let key = bitcoin::PrivateKey::new_uncompressed(secret_key, Network::Bitcoin);
        println!("Uncompressed WIF: {}", key.to_wif());
        key
    };

    // Generate a public key from the private key.
    let secp = Secp256k1::new();
    let public_key = private_key.public_key(&secp);

    // Generate a pay-to-pubkey-hash address.
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    if is_compressed {
        println!("Compressed address: {}", address);
    } else {
        println!("Uncompressed address: {}", address);
    }
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    // Create a secret key from a hex string.
    let secret_key =
        SecretKey::from_str("1e99423a4ed27608a15a2616a2b0e9e52ced330ac530edcc32c8ffc6a526aedd")
            .unwrap();
    generate_keys_and_address(secret_key, false);
    generate_keys_and_address(secret_key, true);

    // Another way to create a compressed private key.
    let bytes =
        hex::decode(b"1e99423a4ed27608a15a2616a2b0e9e52ced330ac530edcc32c8ffc6a526aedd").unwrap();
    let _private_key = bitcoin::PrivateKey::from_slice(&bytes, Network::Bitcoin).unwrap();

    println!("--- End module: {}", module_path!());
}
