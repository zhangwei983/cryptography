use rsa::pkcs1v15::SigningKey;
use rsa::sha2::Sha256;
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::RsaPrivateKey;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut rng = rand::thread_rng();

    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let verifying_key = signing_key.verifying_key();

    // Sign.
    let data = b"hello world";
    let signature = signing_key.sign_with_rng(&mut rng, data);
    println!(
        "Signature: {:?}",
        hex::encode(signature.to_bytes().as_ref())
    );

    // Verify.
    let result = verifying_key.verify(data, &signature);
    println!("Verification result: {:?}", result.is_ok());

    println!("--- End module: {}", module_path!());
}
