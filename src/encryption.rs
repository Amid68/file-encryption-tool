use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{Context, Result};
use rand::RngCore;

const NONCE_SIZE: usize = 12;

/// encrypts the given plaintext using AES-GCM with the provided key
/// returns the ciphertext and the nonce used during encryption
pub fn encrypt_data(key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, [u8; NONCE_SIZE])> {
    // initialize the cipher
    let cipher = Aes256Gcm::new_from_slice(key)
        .with_context(|| "Failed to initialize AES-256GCM cipher")?;

    // generate a random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);    // 96 bits unique per msg

    // encrypt plaintext
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .with_context(|| "encryption failed")?;

    Ok((ciphertext, nonce_bytes))
}
