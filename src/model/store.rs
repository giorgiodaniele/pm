use std::{collections::HashMap, fmt, fs, io::Write, path::PathBuf};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::model::secret::Secret;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    data: HashMap<String, Secret>,
    #[serde(skip)]
    file: PathBuf,
}


impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for secret in self.data.values() {
            writeln!(f, "{}", secret)?;
        }
        Ok(())
    }
}


impl Store {

    pub fn open(file: PathBuf) -> Store {

        if !file.exists() {
            // Create a empty file
            fs::File::create(&file)
                .expect("file should create");
            // Write an empty array
            fs::write(&file, "[]")
                .expect("file should write");
        }

        // Open the JSON file for reading
        let reader = fs::File::open(&file)
            .expect("file should open read only");
        
        // Deserialize into a Vec<Secret>
        let rdata: Vec<Secret> = serde_json::from_reader(reader)
            .expect("file should be proper JSON");

        // Build the HashMap<String, Secret>
        let data = rdata
            .into_iter()
            .map(|secret| (secret.get_hash(), secret))
            .collect::<HashMap<_, _>>();

        Store { data, file }
    }

    pub fn save(&self) {

        // Open the JSON file for writing
        let mut file = fs::File::open(&self.file)
            .expect("file should open read only");

        // Convert map into a vec
        let data: Vec<&Secret> = self.data.values().collect();

        // Serialized into JSON
        let rdata = serde_json::to_string_pretty(&data)
            .expect("data should be proper JSON");

        // Write on file
        file.write_all(rdata.as_bytes())
            .expect("file should write");

    }

    pub fn get_secret(&self, name: &str) -> Option<&Secret> {
        self.data.values().find(|secret| secret.get_name() == name)
    }

    pub fn add_secret(&mut self, name: String, user: String, text: String) {

        // Get curent timestamp
        let now = chrono::Utc::now().to_rfc3339();

        // Generate hash
        let mut hasher = Sha256::new();
        hasher.update(&name);
        hasher.update(&text);
        hasher.update(&now);
        hasher.update(&user);
        let hash = format!("{:x}", hasher.finalize());

        // Create a new secret
        let secret = Secret::new(hash, name, user, text);

        self.data.insert(secret.get_hash(), secret);
    }
    
    pub fn get_secrets(&self) -> Vec<&Secret> {
        self.data.values().collect()
    }

    pub fn del_secret (&mut self, name: String) {
        // Search the secret
        let secret = self.get_secret(&name);
        // Remove the secret
        self.data.remove(&secret.unwrap().get_hash());
    }

}
