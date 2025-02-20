use keyring::Entry;

fn main() {
    let entry = Entry::new("my-service", "my-name").unwrap();
    entry.set_password("test-password").unwrap();

    let password = entry.get_password().unwrap();
    println!("Password: {}", password);

    entry.delete_credential().unwrap();
}
