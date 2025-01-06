use std::error::Error;

pub mod sha256;
pub trait PasswordHash {
    fn get_pass_hash(&self, input: &str) -> Result<String, Box<dyn Error>>;
}
