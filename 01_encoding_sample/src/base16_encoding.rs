pub fn test() {
    println!("--- Start module: {}", module_path!());

    let data = "Hello world";

    // Encode and decode a string with hex.
    let encoded = hex::encode(data);
    println!("Encoded: {}", encoded);
    let decoded = hex::decode(&encoded).unwrap();
    let result = String::from_utf8(decoded).unwrap();
    println!("Decoded Hex: {}", result);

    assert_eq!(base16::encode_lower(data), hex::encode(data));
    assert_eq!(base16::encode_upper(data), hex::encode_upper(data));

    println!("--- End module: {}", module_path!());
}
