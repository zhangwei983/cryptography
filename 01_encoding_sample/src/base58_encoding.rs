pub fn test() {
    println!("--- Start module: {}", module_path!());

    let hash_256 = b"hello world";
    let encoded = base58ck::encode(hash_256);
    println!("Encoded: {}", encoded);
    let decoded = base58ck::decode(&encoded).unwrap();
    let decoded = String::from_utf8(decoded).unwrap();
    println!("Decoded: {:?}", decoded);

    let encoded = base58ck::encode_check(hash_256);
    println!("Encoded (with checksum): {}", encoded);
    let decoded = base58ck::decode_check(&encoded).unwrap();
    let decoded = String::from_utf8(decoded).unwrap();
    println!("Decoded (with checksum): {:?}", decoded);

    println!("--- End module: {}", module_path!());
}
