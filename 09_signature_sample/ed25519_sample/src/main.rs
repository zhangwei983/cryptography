use ed25519_dalek::{Signer, SigningKey, Verifier};
use rand::rngs::OsRng;

fn main() {
    // Generate a signing key.
    let signing_key = SigningKey::generate(&mut OsRng);

    // Sign.
    let message = b"Hello, World!";
    let signature = signing_key.sign(message);
    println!("Signature: {:?}", hex::encode(signature.to_bytes()));

    assert!(signing_key.verify(message, &signature).is_ok());

    // Verify.
    let verifying_key = signing_key.verifying_key();
    let result = verifying_key.verify(message, &signature);
    println!("Result: {:?}", result.is_ok());
}
