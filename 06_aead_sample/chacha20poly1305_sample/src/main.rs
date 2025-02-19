use chacha20poly1305::{
    aead::{heapless::Vec, Aead, AeadCore, AeadInPlace, OsRng},
    ChaCha20Poly1305, KeyInit,
};

fn main() {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let message = b"Hello, world!";

    // Normal encryption/decryption.
    let ciphertext = cipher.encrypt(&nonce, message.as_ref()).unwrap();
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    assert_eq!(&plaintext, message);

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
    assert_eq!(&buffer, message);

    println!(
        "Decrypted message: {:?}",
        String::from_utf8(plaintext.to_vec()).unwrap()
    );
}
