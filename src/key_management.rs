use anyhow::{Context, Result};
use rand::rngs::OsRng;
use rand::RngCore;
use std::path::Path;

use crate::file_io::{create_and_write_file, ensure_directory_exists, read_file};

// key size in bytes (32 bytes for AES-256)
pub const KEY_SIZE: usize = 32;

/// generates a new random key of size `KEY_SIZE`
pub fn generate_key() -> [u8; KEY_SIZE] {
    let mut key = [0u8; KEY_SIZE];
    OsRng.fill_bytes(&mut key);
    key
}

/// saves the key to a file at the specified path
pub fn save_key_to_file(key: &[u8], path: &str) -> Result<()> {
    // ensure the keys directory exists
    ensure_directory_exists(path)?;
    // write the key to the file
    create_and_write_file(path, key)
}

/// loads a key from a file at the specified path
pub fn load_key_from_file(path: &str) -> Result<Vec<u8>> {
    let key = read_file(path)?;

    if key.len() != KEY_SIZE {
        anyhow::bail!(
            "Invalid key size: expected {} bytes, found {} bytes",
            KEY_SIZE,
            key.len()
        );
    }

    Ok(key)
}



