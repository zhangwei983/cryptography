use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut hasher = Sha3_224::new();
    hasher.update(b"hello");
    let result = hasher.finalize();
    println!("{:x}", result);

    let mut hasher = Sha3_256::new();
    hasher.update(b"hello");
    let result = hasher.finalize();
    println!("{:x}", result);

    let mut hasher = Sha3_384::new();
    hasher.update(b"hello");
    let result = hasher.finalize();
    println!("{:x}", result);

    let mut hasher = Sha3_512::new();
    hasher.update(b"hello");
    let result = hasher.finalize();
    println!("{:x}", result);

    println!("--- End module: {}", module_path!());
}