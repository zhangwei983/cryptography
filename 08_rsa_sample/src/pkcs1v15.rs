use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut rng = rand::thread_rng();

    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let public_key = RsaPublicKey::from(&private_key);

    // Encrypt.
    let data = b"hello world";
    let enc_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .unwrap();
    println!("Encrypted data: {:?}", hex::encode(&enc_data));

    // Decrypt.
    let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &enc_data).unwrap();
    assert_eq!(&data[..], &dec_data[..]);

    println!("Decrypted data: {:?}", String::from_utf8(dec_data).unwrap());

    println!("--- End module: {}", module_path!());
}
