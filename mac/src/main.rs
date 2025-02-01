use hmac::{Hmac, Mac};
use rand::{rng, TryRngCore};
use sha2::Sha256;

fn send_message(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new(key.into());
    mac.update(message);

    mac.finalize().into_bytes().to_vec()
}

fn receive_message(key: &[u8], message: &[u8], authentication_tag: &[u8]) -> bool {
    let mut mac = Hmac::<Sha256>::new(key.into());
    mac.update(message);

    mac.verify(authentication_tag.into()).is_ok()
}

fn generate_key() -> Vec<u8> {
    let mut key = vec![0u8; 64];
    rng()
        .try_fill_bytes(&mut key[..])
        .expect("Failed to generate key");
    key
}

fn main() {
    let key = generate_key();
    println!("Key is: {}", hex::encode(&key));

    let message = b"Hello, world!";
    let authentication_tag = send_message(&key, message);
    println!(
        "Authentication tag is: {}",
        hex::encode(&authentication_tag)
    );

    let is_valid = receive_message(&key, message, &authentication_tag);
    println!("Is valid: {}", is_valid);
}
