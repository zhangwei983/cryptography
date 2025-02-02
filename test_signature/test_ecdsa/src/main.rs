use k256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;

fn main() {
    // Prepare the message to sign.
    let message = String::from("Hello world");
    let msg = message.as_bytes();

    // Generate the signing key.
    let signing_key = SigningKey::random(&mut OsRng);
    let sk = signing_key.to_bytes();
    println!("Signing key: {:x?}", hex::encode(sk));

    // Sign with the signing key.
    let signature: Signature = signing_key.sign(msg);
    println!("Signature: {:x?}", hex::encode(signature.to_bytes()));

    // Generate the verifying key.
    let verify_key = VerifyingKey::from(&signing_key);
    let vk = verify_key.to_sec1_bytes();
    println!("Verifying key: {:x?}", hex::encode(vk));

    // Verify with the verifying key.
    let verified = verify_key.verify(msg, &signature).is_ok();

    if verified {
        println!("'{}' signature matches", message);
    } else {
        println!("'{}' signature mismatches", message);
    }

    // Prepare the wrong message.
    let message = String::from("Hello");

    let msg = message.as_bytes();

    let verified = verify_key.verify(msg, &signature).is_ok();

    if verified {
        println!("'{}' signature matches", message);
    } else {
        println!("'{}' signature mismatches", message);
    }
}
