use std::error::Error;

pub mod aes256;

pub trait EncryptAlgorithm {
    fn derive_key_from(&mut self, password: &str, salt: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
    fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, Box<dyn Error>>;
    fn decrypt(&self, ciphertext: &[u8]) -> Result<String, Box<dyn Error>>;
}
