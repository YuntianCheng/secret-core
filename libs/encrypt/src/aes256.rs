use crate::EncryptAlgorithm;
use aes_gcm::aead::{Aead, AeadCore, OsRng};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use std::error::Error;

pub struct Aes256Factory {
    key: [u8; 32],
}

impl Aes256Factory {
    pub fn new() -> Self {
        Aes256Factory { key: [0u8; 32] }
    }
}

impl EncryptAlgorithm for Aes256Factory {
    fn encrypt(&self, data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let cipher = Aes256Gcm::new(self.key.as_slice().into());
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, data.as_bytes());
        if ciphertext.is_err() {
            return Err("encrypt failed".into());
        }
        let ciphertext = ciphertext.unwrap();
        // 将 nonce 和密文连接
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }
    fn decrypt(&self, ciphertext: &[u8]) -> Result<String, Box<dyn Error>> {
        let nonce = &ciphertext[..12];
        let encrypted_data = &ciphertext[12..];
        let cipher = Aes256Gcm::new(self.key.as_slice().into());
        let decrypted_data = cipher.decrypt(Nonce::from_slice(nonce), encrypted_data);
        if decrypted_data.is_err() {
            return Err("decrypt failed".into());
        }
        let decrypted_data = decrypted_data.unwrap();
        let decrypted_data = String::from_utf8(decrypted_data);
        if decrypted_data.is_err() {
            return Err("decrypt failed".into());
        }
        let decrypted_data = decrypted_data.unwrap();
        Ok(decrypted_data)
    }
    //根据密码创建加密密钥
    fn derive_key_from(&mut self, password: &str, salt: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        self.key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(
            password.as_bytes(),
            salt,
            100_000, // 迭代次数
            &mut self.key,
        );
        Ok(self.key.to_vec())
    }
}
