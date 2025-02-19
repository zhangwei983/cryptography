mod base16_encoding;
mod base32_encoding;
mod base64_encoding;
mod pem_encoding;

fn main() {
    base64_encoding::test();
    println!("");
    base32_encoding::test();
    println!("");
    base16_encoding::test();
    println!("");
    pem_encoding::test();
}
