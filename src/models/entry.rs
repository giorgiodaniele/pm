use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub name: String,
    pub pass: String,
}

impl Entry {
    pub fn new(name: String, pass: String) -> Entry {
        Entry { name, pass }
    }
}
