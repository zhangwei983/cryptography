use k256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;

fn main() {
    // Generate the signing key.
    let signing_key = SigningKey::random(&mut OsRng);

    // Sign.
    let message = b"Hello world";
    let signature: Signature = signing_key.sign(message);
    println!("Signature: {:x?}", hex::encode(signature.to_bytes()));

    // Verify
    let verify_key = VerifyingKey::from(&signing_key);
    let result = verify_key.verify(message, &signature);
    println!("Result: {:?}", result.is_ok());

    // Prepare the wrong message.
    let message = b"Hello";
    let result = verify_key.verify(message, &signature);
    println!("Result: {:?}", result.is_ok());
}
