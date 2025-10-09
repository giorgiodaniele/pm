use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Secret {
    hash: String,
    name: String,
    user: String,
    text: String,
}

impl Secret {

    pub fn new(hash: String, name: String, user: String, text: String) -> Secret {
        Secret { hash, name, text, user }
    }

    pub fn get_user(&self) -> String { self.user.clone() }
    pub fn get_hash(&self) -> String { self.hash.clone() }
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_text(&self) -> String { self.text.clone() }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name={}, user={}, text={}", self.name, self.user,self.text
        )
    }
}