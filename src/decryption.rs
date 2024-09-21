use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{anyhow, Result};

const NONCE_SIZE: usize = 12;   // 96 bit nonce size for AES-GCM

/// decrypts the given ciphertext using AES-GCM with the provided key and nonce
/// returns the plaintext
pub fn decrypt_data(key: &[u8], nonce: &[u8; NONCE_SIZE], ciphertext: &[u8]) -> Result<Vec<u8>> {
    // initialize the cipher
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| anyhow!("Failed to initialize AES256-GCM cipher"))?;

    // create nonce from the provided nonce bytes
    let nonce = Nonce::from_slice(nonce);

    // decrypt the cipher
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("Decryption failed"))?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encryption::encrypt_data;
    use crate::key_management::generate_key;

    #[test]
    fn test_decrypt_data() {
        let key = generate_key();
        let plaintext = b"Test plaintext data";

        // encrypt the data
        let (ciphertext, nonce) = encrypt_data(&key, plaintext).expect("Encryption failed");

        assert_eq!(
            plaintext.to_vec(),
            decrypted_plaintext,
            "Decrypted plaintext should match original"
        );
    }

    #[test]
    fn test_decrypt_with_wrong_key() {
        let key = generate_key();
        let wrong_key = generate_key();
        let plaintext = b"Test plaintext data";

        // encrypt the data with the correct key
        let (ciphertext, nonce) = encrypt_data(&key, plaintext).expect("Encryption failed");

        // attempt to decrypt with the wrong key
        let result = decrypt_data(&wrong_key, &nonce, &ciphertext);

        assert!(
            result.is_err(),
            "Decryption should fail with the wrong key"
        );
    }
}




