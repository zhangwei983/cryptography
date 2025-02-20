use ring::agreement;
use ring::rand::SystemRandom;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    // Generate a key pair for Alice
    let rng = SystemRandom::new();

    let alice_private_key =
        agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
    let alice_public_key = alice_private_key.compute_public_key().unwrap();

    // Generate a key pair for Bob
    let bob_private_key =
        agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
    let bob_public_key = bob_private_key.compute_public_key().unwrap();

    // Alice computes the shared secret
    let alice_shared_secret = agreement::agree_ephemeral(
        alice_private_key,
        &agreement::UnparsedPublicKey::new(&agreement::X25519, bob_public_key),
        |key_material| key_material.to_vec(),
    )
    .unwrap();

    // Bob computes the shared secret
    let bob_shared_secret = agreement::agree_ephemeral(
        bob_private_key,
        &agreement::UnparsedPublicKey::new(&agreement::X25519, alice_public_key.as_ref()),
        |key_material| key_material.to_vec(),
    )
    .unwrap();

    // Both shared secrets should be the same
    assert_eq!(alice_shared_secret, bob_shared_secret);
    println!("Shared secret: 0x{}", hex::encode(alice_shared_secret));

    println!("--- End module: {}", module_path!());
}
