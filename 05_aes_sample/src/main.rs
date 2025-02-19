use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit, generic_array::GenericArray
};

fn main() {
    // Generate a random 128-bit key.
    let key = GenericArray::from([0u8; 16]);
    println!("{}", hex::encode(key.as_slice()));

    // Initialize 128bits block to all 42s.
    let mut block = GenericArray::from([42u8; 16]);
    println!("{}", hex::encode(block.as_slice()));
    
    // Create an AES128 cipher with the key.
    let cipher = Aes128::new(&key);
    let block_copy = block.clone();

    // Encrypt block in-place.
    cipher.encrypt_block(&mut block);
    println!("{}", hex::encode(block.as_slice()));

    // Decrypt block in-place.
    cipher.decrypt_block(&mut block);

    assert_eq!(block, block_copy);
}
