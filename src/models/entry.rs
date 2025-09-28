use serde::{Deserialize, Serialize};

//
// Entry represents a single credential in the vault
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub password: String,
}

impl Entry {
    //
    // Create a new entry (just plaintext inside memory)
    //
    pub fn new(name: String, password: String) -> Entry {
        Entry { name, password }
    }
}
