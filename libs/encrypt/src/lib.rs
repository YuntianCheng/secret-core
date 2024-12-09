mod aes256;

pub trait EncryptAlgorithm {
    fn derive_key_from(&self, password: &str) -> Vec<u8>;
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertext: &str) -> String;
}
