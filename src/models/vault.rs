use super::entry::Entry;
use crate::err::AppError;
use serde::{de::Error, Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

//
//
// SecretStore represents the whole vault
//   - salt used for key derivation
//   - collection of entries (kept plaintext in memory)
//
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SecretStore {
    pub salt: String,
    pub entries: Vec<Entry>,
}

impl SecretStore {
    //
    // Create a new empty vault
    //
    pub fn new() -> SecretStore {
        SecretStore { salt: String::new(), entries: Vec::new() }
    }

    //
    // Load and decrypt the vault from disk
    //
    pub fn load(path: &PathBuf, secret: &str) -> Result<SecretStore, AppError> {
        // Read encrypted container from disk
        let data = fs::read(path).map_err(AppError::IOError)?;
        let enc: EncryptedVault = serde_json::from_slice(&data)
            .map_err(AppError::DeserializeError)?;

        // Decode salt, nonce, ciphertext
        let salt = general_purpose::STANDARD
            .decode(&enc.salt)
            .map_err(|e| AppError::DeserializeError(serde_json::Error::custom(e)))?;
        let nonce_bytes = general_purpose::STANDARD
            .decode(&enc.nonce)
            .map_err(|e| AppError::DeserializeError(serde_json::Error::custom(e)))?;
        let ctext = general_purpose::STANDARD
            .decode(&enc.ciphertext)
            .map_err(|e| AppError::DeserializeError(serde_json::Error::custom(e)))?;

        // Derive key
        let iterations = 100_000;
        let key = pbkdf2_hmac_array::<Sha256, 32>(secret.as_bytes(), &salt, iterations);
        let cipher = Aes256Gcm::new(&key.into());

        // Decrypt ciphertext into plaintext
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ptext = cipher.decrypt(nonce, ctext.as_ref())
            .map_err(|e| AppError::PermissionDenied(e.to_string()))?;

        // Deserialize into SecretStore
        let store: SecretStore = serde_json::from_slice(&ptext)
            .map_err(AppError::DeserializeError)?;
        Ok(store)
    }

    //
    // Save and encrypt the vault to disk
    //
    pub fn save(&mut self, path: &PathBuf, secret: &str) -> Result<(), AppError> {
        // Generate salt only once
        let salt_bytes = if self.salt.is_empty() {
            let mut buf = [0u8; 16];
            rand::thread_rng().fill_bytes(&mut buf);
            self.salt = general_purpose::STANDARD.encode(buf);
            buf.to_vec()
        } else {
            general_purpose::STANDARD
                .decode(&self.salt)
                .map_err(|e| AppError::DeserializeError(serde_json::Error::custom(e)))?
        };

        // Derive key
        let iterations = 100_000;
        let key = pbkdf2_hmac_array::<Sha256, 32>(secret.as_bytes(), &salt_bytes, iterations);
        let cipher = Aes256Gcm::new(&key.into());

        // Serialize vault into plaintext JSON
        let ptext = serde_json::to_vec(&self).map_err(AppError::SerializeError)?;

        // Generate random nonce (12 bytes)
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt plaintext into ciphertext
        let ctext = cipher.encrypt(nonce, ptext.as_ref())
            .map_err(|e| AppError::PermissionDenied(e.to_string()))?;

        // Wrap encrypted fields
        let enc = EncryptedVault {
            salt: general_purpose::STANDARD.encode(salt_bytes),
            nonce: general_purpose::STANDARD.encode(nonce_bytes),
            ciphertext: general_purpose::STANDARD.encode(ctext),
        };

        // Serialize encrypted vault
        let json = serde_json::to_vec_pretty(&enc).map_err(AppError::SerializeError)?;

        // Write file
        let mut f = fs::File::create(path).map_err(AppError::IOError)?;
        f.write_all(&json).map_err(AppError::IOError)?;
        Ok(())
    }

    //
    // Add new entry (plaintext in memory until save)
    //
    pub fn add_entry(&mut self, name: String, password: String) {
        self.entries.push(Entry { name, password });
    }

    //
    // Get entry by name (plaintext)
    //
    pub fn get_entry(&self, name: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.name == name)
    }

    //
    // Get all entries (plaintext)
    //
    pub fn get_all(&self) -> Vec<Entry> {
        self.entries.clone()
    }
}

//
// Internal structure of the encrypted vault stored on disk
//
#[derive(Serialize, Deserialize, Debug)]
struct EncryptedVault {
    salt: String,
    nonce: String,
    ciphertext: String,
}
