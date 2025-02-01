mod sha2_hashing;
mod sha3_hashing;

fn main() {
    sha2_hashing::test();
    println!("");
    sha3_hashing::test();
}
