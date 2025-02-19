use rand::Rng;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut rng = rand::rng();

    // Random number generation with different types.
    println!("Random u8: {}", rng.random::<u8>());
    println!("Random f64: {}", rng.random::<f64>());
    println!("Random bool: {}", rng.random::<bool>());
    println!("Random u128: 0x{:x}", rng.random::<u128>());

    // Random number generation with ranges.
    println!("Random dice roll: {}", rng.random_range(1..=6));
    println!("Random u8 in range: {}", rng.random_range(1..=255));
    println!("Random f64 in range: {}", rng.random_range(1.0..=100.0));

    println!("--- End module: {}", module_path!());
}
