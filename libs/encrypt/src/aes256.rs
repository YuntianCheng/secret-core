use crate::EncryptAlgorithm;
mod aes256 {
    use crate::EncryptAlgorithm;

    struct Aes256 {

    }

    impl EncryptAlgorithm for Aes256 {
        fn encrypt(&self, data: &[u8]) -> Vec<u8> {
            todo!()
        }
        fn decrypt(&self, ciphertext: &str) -> String {
            todo!()
        }
        fn derive_key_from(&self, password: &str) -> Vec<u8> {
            todo!()
        }
    }
}