use rand::seq::IndexedRandom;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut rng = rand::rng();

    let choices = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Select one element randomly.
    let selected = choices.choose(&mut rng).unwrap();
    println!("Random choice: {}", selected);

    // Select multiples elements randomly.
    let selected: Vec<i32> = choices.choose_multiple(&mut rng, 3).cloned().collect();
    println!("Random choices: {:?}", selected);

    // Select a fixed-size array randomly.
    let selected: [i32; 3] = choices.choose_multiple_array(&mut rng).unwrap();
    println!("Random choices: {:?}", selected);

    println!("--- End module: {}", module_path!());
}
