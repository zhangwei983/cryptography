pub fn test() {
    println!("--- Start module: {}", module_path!());

    let vec: Vec<i32> = rand::random_iter().take(5).collect();
    println!("Random i32 vec: {:?}", vec);

    println!("--- End module: {}", module_path!());
}
