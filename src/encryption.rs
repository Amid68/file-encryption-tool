use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{anyhow, Result};
use rand::rngs::OsRng;
use rand::RngCore;

const NONCE_SIZE: usize = 12;

/// encrypts the given plaintext using AES-GCM with the provided key
/// returns the ciphertext and the nonce used during encryption
pub fn encrypt_data(key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, [u8; NONCE_SIZE])> {
    // initialize the cipher
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| anyhow!("Failed to initialize AES-256GCM cipher"))?;

    // generate a random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);    // 96 bits unique per msg

    // encrypt plaintext
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| anyhow!("encryption failed"))?;

    Ok((ciphertext, nonce_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key_management::generate_key;

    #[test]
    fn test_encrypt_data() {
        let key = generate_key();
        let plaintext = b"Test plaintext data";
        let (ciphertext, nonce) = encrypt_data(&key, plaintext).expect("Encryption failed");

        assert!(!ciphertext.is_empty(), "Ciphertext should not be empty");
        assert_eq!(nonce.len(), NONCE_SIZE, "Nonce size should be correct");
    }
}



