use aes_gcm::{
    aead::{Aead, AeadCore, OsRng},
    Aes256Gcm, KeyInit,
};

fn main() {
    let key = Aes256Gcm::generate_key(OsRng);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let message = b"plaintext message";
    let ciphertext = cipher
        .encrypt(&nonce, message.as_ref())
        .expect("Failed to encrypt.");

    let plaintext = cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .expect("Failed to decrypt.");
    assert_eq!(plaintext, message);

    println!(
        "Decrypted message: {:?}",
        String::from_utf8(plaintext.to_vec()).unwrap()
    );
}
