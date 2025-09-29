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
// External structure of the encrypted vault stored on disk
//
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Vault {
    pub salt:    String,
    pub entries: Vec<Entry>,
}

//
// Internal structure of the encrypted vault stored on disk
//
#[derive(Serialize, Deserialize, Debug)]
struct EncryptedVault {
    salt:       String,
    nonce:      String,
    ciphertext: String,
}

impl Vault {

    /// Generate a new random salt (16 bytes).
    fn generate_salt() -> Vec<u8> {
        let mut buf = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut buf);
        buf.to_vec()
    }

    /// Derive a 256-bit AES key from the provided secret and salt using PBKDF2.
    fn derive_key(secret: &str, salt: &[u8]) -> Aes256Gcm {

        //
        // KDF = PBKDF2-HMAC-SHA256 is used to derive a 256-bit AES key
        // It takes 100,000 iterations, and uses a 16-byte salt from a
        // a secret
        //

        let key = pbkdf2_hmac_array::<Sha256, 32>(secret.as_bytes(), salt, 100_000);
        Aes256Gcm::new(&key.into())
    }

    /// Generate a random nonce (12 bytes).
    fn generate_nonce() -> [u8; 12] {
        let mut buf = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut buf);
        buf
    }


    /// Create a new empty vault.
    pub fn new() -> Vault {
        Vault { salt: String::new(), entries: Vec::new() }
    }


    pub fn load(path: &PathBuf, secret: &str) -> Result<Vault, AppError> {
        // Read encrypted vault from disk into memory
        let data = fs::read(path).map_err(AppError::IOError)?;

        // Deserialize encrypted vault into internal structure
        let enc: EncryptedVault = serde_json::from_slice(&data)
            .map_err(AppError::DeserializeError)?;

        // Decode salt/nonce/ciphertext
        let salt = general_purpose::STANDARD
            .decode(&enc.salt)
            .map_err(|e| AppError::DeserializeError(serde_json::Error::custom(e)))?;
        let nonce = general_purpose::STANDARD
            .decode(&enc.nonce)
            .map_err(|e| AppError::DeserializeError(serde_json::Error::custom(e)))?;
        let ctext = general_purpose::STANDARD
            .decode(&enc.ciphertext)
            .map_err(|e| AppError::DeserializeError(serde_json::Error::custom(e)))?;

        // Derive key
        let cipher = Vault::derive_key(secret, &salt);

        // Decrypt
        let nonce = Nonce::from_slice(&nonce);
        let ptext = cipher.decrypt(nonce, ctext.as_ref())
            .map_err(|e| AppError::PermissionDenied(e.to_string()))?;

        // Deserialize decrypted vault
        let store: Vault = serde_json::from_slice(&ptext)
            .map_err(AppError::DeserializeError)?;
        Ok(store)
        
    }

    pub fn save(&mut self, path: &PathBuf, secret: &str) -> Result<(), AppError> {
        // Generate or decode salt
        let salt = if self.salt.is_empty() {
            let salt_bytes = Vault::generate_salt();
            self.salt = general_purpose::STANDARD.encode(&salt_bytes);
            salt_bytes
        } else {
            general_purpose::STANDARD
                .decode(&self.salt)
                .map_err(|e| AppError::DeserializeError(serde_json::Error::custom(e)))?
        };

        // Derive key
        let cipher = Vault::derive_key(secret, &salt);

        // Serialize vault into plaintext JSON
        let ptext = serde_json::to_vec(&self).map_err(AppError::SerializeError)?;

        // Generate random nonce
        let nonce_bytes = Vault::generate_nonce();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt plaintext into ciphertext
        let ctext = cipher.encrypt(nonce, ptext.as_ref())
            .map_err(|e| AppError::PermissionDenied(e.to_string()))?;

        // Wrap encrypted fields
        let enc = EncryptedVault {
            salt:       general_purpose::STANDARD.encode(salt),
            nonce:      general_purpose::STANDARD.encode(nonce_bytes),
            ciphertext: general_purpose::STANDARD.encode(ctext),
        };

        // Serialize encrypted vault
        let json = serde_json::to_vec_pretty(&enc).map_err(AppError::SerializeError)?;

        // Write file
        let mut f = fs::File::create(path).map_err(AppError::IOError)?;
        f.write_all(&json).map_err(AppError::IOError)?;
        Ok(())
    }

    /// Add new entry (plaintext in memory until save).
    pub fn add_entry(&mut self, name: String, pass: String, desc: String) {
        self.entries.push(Entry::new(name, pass, desc));
    }

    /// Get entry by name (plaintext).
    pub fn get_entry(&self, name: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.name == name)
    }

    /// Get all entries (plaintext).
    pub fn get_all(&self) -> Vec<Entry> {
        self.entries.clone()
    }
}
