mod oaep;
mod pkcs1v15;
mod pkcs1v15_sign;

fn main() {
    pkcs1v15::test();
    println!("");
    oaep::test();
    println!("");
    pkcs1v15_sign::test();
}
