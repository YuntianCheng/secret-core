mod aes256 {
    use std::error::Error;
    use aes_gcm::aead::{Aead, OsRng};
    use aes_gcm::aead::rand_core::RngCore;
    use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
    use pbkdf2::pbkdf2_hmac;
    use sha2::Sha256;
    use crate::EncryptAlgorithm;

    struct Aes256 {
        key: [u8; 32],
    }

    impl EncryptAlgorithm for Aes256 {
        fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
            let mut nonce = [0u8; 12];
            OsRng.fill_bytes(&mut nonce);
            let cipher = Aes256Gcm::new(self.key.as_slice().into());
            // 加密数据
            Ok(cipher.encrypt(Nonce::from_slice(&nonce), data)?)
        }
        fn decrypt(&self, ciphertext: &str) -> Result<String, Box<dyn Error>> {
            let r:Vec<&str> = ciphertext.split('-').collect();
            if r.len() != 2 {
                return Err("invalid ciphertext".into());
            }
            let nonce = r[0].as_bytes();
            if nonce.len() != 12 {
                return Err("invalid ciphertext".into());
            }
            let cipher = Aes256Gcm::new(self.key.as_slice().into());
            let decrypted_data = cipher.decrypt(Nonce::from_slice(nonce), r[1].as_bytes());
            Ok(String::from_utf8(decrypted_data?)?)
        }
        //根据密码创建加密密钥
        fn derive_key_from(&mut self, password: &str, salt: &[u8]) -> Vec<u8> {
            self.key = [0u8; 32];
            pbkdf2_hmac::<Sha256>(
                password.as_bytes(),
                salt,
                100_000, // 迭代次数
                &mut self.key,
            );
            self.key.to_vec()
        }
    }
}