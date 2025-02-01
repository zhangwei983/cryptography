use aes_gcm::{
    aead::{heapless::Vec, Aead, AeadCore, AeadInPlace, OsRng},
    Aes256Gcm, KeyInit,
};

fn main() {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let message = b"plaintext message";

    // Normal encryption/decryption.
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

    // In-place encryption/decryption.
    let mut buffer: Vec<u8, 128> = Vec::new();
    buffer.extend_from_slice(message).ok();

    cipher.encrypt_in_place(&nonce, b"", &mut buffer).unwrap();
    assert_ne!(&buffer, message);

    cipher.decrypt_in_place(&nonce, b"", &mut buffer).unwrap();
    assert_eq!(buffer, message);

    println!(
        "Decrypted message: {:?}",
        String::from_utf8(buffer.to_vec()).unwrap()
    );
}
