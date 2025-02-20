use rand::seq::SliceRandom;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut rng = rand::rng();

    // Shuffle the full string in place.
    let mut bytes: Vec<u8> = b"Hello, world!".into();
    bytes.shuffle(&mut rng);

    let result = String::from_utf8(bytes).unwrap();
    println!("Shuffled string: '{}'", result);

    // Shuffle the string in two parts.
    let mut bytes: Vec<u8> = b"Hello, world!".into();
    let (result1, result2) = bytes.partial_shuffle(&mut rng, 5);
    let result1 = String::from_utf8(result1.to_vec()).unwrap();
    let result2 = String::from_utf8(result2.to_vec()).unwrap();
    println!("Partial shuffled string: '{}'; '{}'", result1, result2);

    let result = String::from_utf8(bytes).unwrap();
    println!("Shuffled string: '{}'", result);

    println!("--- End module: {}", module_path!());
}
