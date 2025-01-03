use encrypt::aes256::Aes256Factory;
use encrypt::EncryptAlgorithm;

fn main() {
    let mut aes = Aes256Factory::new();
    let salt = "123456".as_bytes();
    let key = aes.derive_key_from("123456", salt).unwrap();
    println!("{:?}", key);
    let encrypted = aes.encrypt("hello").unwrap();
    println!("{:?}", encrypted);
    let decrypted = aes.decrypt(&encrypted).unwrap();
    println!("{:?}", decrypted);
}
