use core::time;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

//
// Entry represents a single credential in the vault
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub pass: String,
    pub desc: String,
    pub when: u128,
}

impl Entry {

    pub fn new(name: String, pass: String, desc: String) -> Entry {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        Entry { name, pass, desc, when: timestamp }
    }
}
