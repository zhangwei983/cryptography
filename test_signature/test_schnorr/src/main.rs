use k256::schnorr::{
    signature::{Signer, Verifier},
    SigningKey,
};
use rand_core::OsRng;

fn main() {
    let signing_key = SigningKey::random(&mut OsRng);

    let message = b"Hello, world!";
    let signature = signing_key.sign(message);
    println!("Signature: {:?}", hex::encode(signature.to_bytes()));

    let verifying_key = signing_key.verifying_key();
    let result = verifying_key.verify(message, &signature);
    println!("Signature valid: {}", result.is_ok());
}
