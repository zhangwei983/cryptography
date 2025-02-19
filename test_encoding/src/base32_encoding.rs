use base32::Alphabet;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let data = b"Hello world";

    // Encode and decode a string with Rfc4648 with no-padding.
    let encoded = base32::encode(Alphabet::Rfc4648 { padding: false }, data);
    println!("Encoded: {}", encoded);
    let decoded = base32::decode(Alphabet::Rfc4648 { padding: false }, &encoded).unwrap();
    let result = String::from_utf8(decoded).unwrap();
    println!("Decoded: {}\n", result);

    // Encode and decode a string in different aplhabets.
    println!(
        "Encoded Rfc4648 with padding: {}",
        base32::encode(Alphabet::Rfc4648 { padding: true }, data)
    );
    println!(
        "Encoded Rfc4648Lower with padding: {}",
        base32::encode(Alphabet::Rfc4648Lower { padding: true }, data)
    );
    println!(
        "Encoded Rfc4648Hex with padding: {}",
        base32::encode(Alphabet::Rfc4648Hex { padding: true }, data)
    );
    println!(
        "Encoded Rfc4648HexLower with padding: {}",
        base32::encode(Alphabet::Rfc4648HexLower { padding: true }, data)
    );
    println!(
        "Encoded Crockford: {}",
        base32::encode(Alphabet::Crockford, data)
    );
    println!("Encoded Z-base-32: {}", base32::encode(Alphabet::Z, data));

    println!("--- End module: {}", module_path!());
}
