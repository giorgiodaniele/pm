use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Secret {
    hash: String,
    name: String,
    username: String,
    password: String,
}

impl Secret {

    pub fn new(hash: String, name: String, username: String, password: String) -> Secret {
        Secret { hash, name, password, username }
    }

    pub fn get_hash(&self) -> String { self.hash.clone() }
    pub fn get_name(&self) -> String { self.name.clone() }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name={}, username={}, password={}", 
                self.name, 
                self.username,
                self.password)
    }
}