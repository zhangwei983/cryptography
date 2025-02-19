use rand::Rng;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut rng = rand::rng();
    let mut vec = vec![0u8; 16];
    rng.fill(&mut vec[..]);
    println!("Random u8 vec: {:?}", hex::encode(vec));

    println!("--- End module: {}", module_path!());
}
