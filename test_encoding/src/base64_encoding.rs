use base64::{decode, decode_config, encode, encode_config};

pub fn test() {
    println!("--- Start module: {}", module_path!());

    // Encode and decode a string with the default base64::STANDARD config.
    let data = "Hello world";
    let encoded = encode(data);
    println!("Encoded: {}", encoded);

    let bytes = decode(&encoded).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("Decoded: {}", result);

    // Encode and decode a string with the base64::STANDARD_NO_PAD config.
    let data = "Hello world";
    let encoded_no_pad = encode_config(data, base64::STANDARD_NO_PAD);
    println!("Encoded no padding: {}", encoded_no_pad);

    let bytes = decode_config(encoded_no_pad, base64::STANDARD_NO_PAD).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("Decoded no padding: {}", result);

    // Encode and decode a string with the base64::URL_SAFE config.
    let data = "Hello world?"; // To generate a string encoded with '/' ending.
    println!("Encoded standard: {}", encode(data));
    let encoded = encode_config(data, base64::URL_SAFE);
    println!("Encoded URL safe: {}", encoded);
    
    let bytes = decode_config(encoded, base64::URL_SAFE).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("Decoded URL safe: {}", result);

    println!("--- End module: {}", module_path!());
}
