mod basic_usage;
mod bool_random;
mod fill_with;
mod iterator;

fn main() {
    basic_usage::test();
    println!("");
    fill_with::test();
    println!("");
    iterator::test();
    println!("");
    bool_random::test();
}
