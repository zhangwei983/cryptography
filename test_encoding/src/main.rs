mod base64_encoding;
mod pem_encoding;

fn main() {
    base64_encoding::test();
    println!("");
    pem_encoding::test();
}
