use ring::agreement;
use ring::error::Unspecified;
use ring::rand::SystemRandom;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let server_address = "127.0.0.1:8080";

    // Start a tcp server in antoher thread.
    let server = thread::spawn(move || {
        let listener = TcpListener::bind(server_address).unwrap();
        let mut stream = listener.accept().unwrap().0;

        ecdh_x25519("Server", &mut stream).unwrap();
    });

    // Connect to the server.
    let mut stream = TcpStream::connect(server_address).unwrap();
    ecdh_x25519("Client", &mut stream).unwrap();

    // Block the main thread until the server thread finishes.
    server.join().unwrap();

    println!("--- End module: {}", module_path!());
}

fn ecdh_x25519(actor: &str, stream: &mut TcpStream) -> Result<(), Unspecified> {
    let rng = SystemRandom::new();

    let algorithm = &agreement::X25519;

    // Generate a key pair.
    let private_key = agreement::EphemeralPrivateKey::generate(algorithm, &rng).unwrap();
    let public_key = private_key.compute_public_key().unwrap();
    println!(
        "{} public key: 0x{}",
        actor,
        hex::encode(public_key.as_ref())
    );

    // Send the public key.
    stream.write_all(public_key.as_ref()).unwrap();

    // Receive the peer's public key.
    let mut peer_public_key = [0u8; 32];
    stream.read_exact(&mut peer_public_key).unwrap();
    println!(
        "{} peer public key: 0x{}",
        actor,
        hex::encode(&peer_public_key.as_ref())
    );

    // Compute the shared secret
    agreement::agree_ephemeral(
        private_key,
        &agreement::UnparsedPublicKey::new(algorithm, peer_public_key),
        |key_material| {
            println!(
                "{} shared secret: 0x{}",
                actor,
                hex::encode(key_material.as_ref())
            );
        },
    )
}
