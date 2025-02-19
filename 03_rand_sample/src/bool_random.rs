use rand::Rng;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut rng = rand::rng();
    let numerator: u32 = 1;
    let denominator: u32 = 3;

    // Random_bool.
    let p = f64::from(numerator) / f64::from(denominator);
    let mut count = 0;
    for _i in 0..100 {
        if rng.random_bool(p) {
            count += 1;
        }
    }
    println!("Random_bool with true in 100 times: {}", count);

    // Random_ratio.
    count = 0;
    for _i in 0..100 {
        if rng.random_ratio(numerator, denominator) {
            count += 1;
        }
    }
    println!("Random_ratio with true in 100 times: {}", count);

    println!("--- End module: {}", module_path!());
}
