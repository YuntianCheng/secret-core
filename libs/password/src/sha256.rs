use sha256::digest;
use crate::PasswordHash;
use std::error::Error;
pub struct Sha256Hash;

impl Sha256Hash {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHash for Sha256Hash {
    fn get_pass_hash(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let val = digest(input);
        Ok(val)
    }
}