mod address_sample;
mod segwit_sample;
mod taproot_sample;

fn main() {
    address_sample::test();
    println!("");
    segwit_sample::test();
    println!("");
    taproot_sample::test();
}
