use std::error::Error;

pub trait SecretItem<T> {
    fn get_item(&self) -> Result<&T, Box<dyn Error>>;
}

pub struct LoginItem {
    pub username: String,
    pub password: String,
    pub websites: Vec<String>,
}

impl SecretItem<LoginItem> for LoginItem {
    fn get_item(&self) -> Result<&LoginItem, Box<dyn Error>> {
        Ok(self)
    }
}

impl LoginItem {
    pub fn new(username: String, password: String, websites: Vec<String>) -> Self {
        Self { username, password, websites }
    }
}
