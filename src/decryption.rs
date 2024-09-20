use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{Context, Result};

const NONCE_SIZE: usize = 12;   // 96 bit nonce size for AES-GCM

/// decrypts the given ciphertext using AES-GCM with the provided key and nonce
/// returns the plaintext
pub fn decrypt_data(key: &[u8], nonce: &[u8; NONCE_SIZE], ciphertext: &[u8]) -> Result<Vec<u8>> {
    // initialize the cipher
    let cipher = Aes256Gcm::new_from_slice(key)
        .with_context(|| "Failed to initialize AES256-GCM cipher")?;

    // create nonce from the provided nonce bytes
    let nonce = Nonce::from_slice(nonce);

    // decrypt the cipher
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .with_context(|| "Decryption failed")?;

    Ok(plaintext)
}


