use std::error::Error;

mod aes256;

pub trait EncryptAlgorithm {
    fn derive_key_from(&self, password: &str, salt: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertext: &str) -> String;
}
