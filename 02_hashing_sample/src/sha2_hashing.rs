use sha2::{Digest, Sha256, Sha512, Sha512_224, Sha512_256};

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut hasher = Sha256::new();
    hasher.update(b"hello");
    let result = hasher.finalize();
    println!("{:x}", result);

    let mut hasher = Sha512::new();
    hasher.update(b"hello");
    let result = hasher.finalize();
    println!("{:x}", result);

    let mut hasher = Sha512_224::new();
    hasher.update(b"hello");
    let result = hasher.finalize();
    println!("{:x}", result);

    let mut hasher = Sha512_256::new();
    hasher.update(b"hello");
    let result = hasher.finalize();
    println!("{:x}", result);

    println!("--- End module: {}", module_path!());
}
