use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

pub struct CryptoService {}

impl CryptoService {
    pub fn get_random_string(len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    pub fn hash_string(value: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        base64::encode(&hasher.finalize()[..])
    }

    pub fn hash_string_with_salt(value: &str, salt: &str) -> String {
        let hash_base = format!("{}{}{}", salt, value, salt);
        Self::hash_string(&hash_base)
    }

    pub fn hash_and_salt_string(value: &str) -> (String, String) {
        let salt = Self::get_random_string(32);
        let hash = Self::hash_string_with_salt(value, &salt);

        (hash, salt)
    }
}
