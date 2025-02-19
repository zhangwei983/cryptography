use rsa::{sha2::Sha256, Oaep, RsaPrivateKey, RsaPublicKey};

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut rng = rand::thread_rng();

    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let public_key = RsaPublicKey::from(&private_key);

    // Encrypt.
    let data = b"hello world";
    let padding = Oaep::new::<Sha256>();
    let enc_data = public_key.encrypt(&mut rng, padding, &data[..]).unwrap();

    println!("Encrypted data: {:?}", hex::encode(&enc_data));

    // Decrypt.
    let padding = Oaep::new::<Sha256>();
    let dec_data = private_key.decrypt(padding, &enc_data).unwrap();
    assert_eq!(&data[..], &dec_data[..]);

    println!("Decrypted data: {:?}", String::from_utf8(dec_data).unwrap());

    println!("--- End module: {}", module_path!());
}
