use encrypt::aes256::Aes256Factory;
use encrypt::EncryptAlgorithm;
use password::sha256::Sha256Hash;
use password::PasswordHash;
use base64::prelude::*;
fn main() {
    let sha256 = Sha256Hash::new();
    let passkey = sha256.get_pass_hash("zheshiyigemiyao").unwrap();
    println!("{:?}", passkey);
    let mut aes = Aes256Factory::new();
    let salt = "hahaha".as_bytes();
    let key = aes.derive_key_from(&passkey, salt).unwrap();
    println!("{:?}", BASE64_STANDARD.encode(key));
    let encrypted = aes.encrypt("hello").unwrap();
    println!("{:?}", BASE64_STANDARD.encode(&encrypted));
    let decrypted = aes.decrypt(&encrypted).unwrap();
    println!("{:?}", decrypted); 
}
