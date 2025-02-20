mod basic_usage;
mod bool_random;
mod fill_with;
mod indexed_random;
mod iterator;
mod slice_shuffle;

fn main() {
    basic_usage::test();
    println!("");
    fill_with::test();
    println!("");
    iterator::test();
    println!("");
    bool_random::test();
    println!("");
    slice_shuffle::test();
    println!("");
    indexed_random::test();
}
