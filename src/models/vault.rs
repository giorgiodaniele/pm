use super::entry::Entry;
use crate::err::AppError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SecretStore {
    pub hash: [u8; 32],
    pub vals: Vec<Entry>,
}

impl SecretStore {

    pub fn new(hashed: [u8; 32]) -> SecretStore {
        SecretStore { hash: hashed, vals: Vec::new() }
    }

    pub fn search(&self, name: &str) -> Option<&Entry> {
        self.vals.iter().find(|entry| entry.name == name)
    }

    pub fn read(path: &PathBuf, pass: String) -> Result<SecretStore, AppError> {

        let data = fs::read_to_string(path)?;
        let vault: SecretStore = serde_json::from_str(&data)?;

        if vault.validate(pass) == false {
            println!("pm> wrong password");
            return Err(AppError::PermissionDenied)
        } 

        Ok(vault)
    }

    pub fn insert(&mut self, name: String, pass: String) -> &mut Self {
        // aggiorna self.entries
        self.vals.push(Entry { name, pass });
        self
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), AppError> {
        let data = serde_json::to_string_pretty(self)
            .map_err(|_e| AppError::SerializeError)?;
        fs::write(path, data)?;
        Ok(())
    }


    fn validate(&self, pass: String) -> bool {
        let hash = Sha256::digest(pass.as_bytes());
        let hash_bits: [u8; 32] = hash.into();
        if self.hash == hash_bits {
            true
        } else {
            false
        }
    }

}
