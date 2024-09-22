use crate::cli::{DecryptArgs, EncryptArgs};
use crate::decryption::decrypt_data;
use crate::encryption::encrypt_data;
use crate::file_io::{create_and_write_file, append_to_file, ensure_directory_exists, read_file, write_file};
use crate::key_management::{generate_key, load_key_from_file, save_key_to_file, KEY_SIZE};
use anyhow::{anyhow, Context, Result};
use std::path::Path;

const NONCE_SIZE: usize = 12; // 96 bit nonce size for AES-GCM

/// handles the file encryption process
pub fn encrypt_file(args: EncryptArgs) -> Result<()> {
    // step 1: load/generate the encryption key
    let key = if let Some(key_path) = &args.key {
        // load key from specified file
        load_key_from_file(key_path)
            .with_context(|| format!("Failed to load key from file: {}", key_path))?
    } else {
        // generate new key
        let key = generate_key();
        // save key to a default location
        let default_key_path = "keys/default.key";
        save_key_to_file(&key, default_key_path)
            .with_context(|| format!("Failed to save key to file: {}", default_key_path))?;
        println!("Generated new key and saved to {}", default_key_path);
        key.to_vec();
    };

    // step 2: read the plaintext data from the input file
    let plaintext = read_file(&args.input)
        .with_context(|| format!("Failed to read input file: {}", args.input))?;

    // step 3: encrypt the data
    let (ciphertext, nonce) = encrypt_data(&key, &plaintext)?;

    // step 4: prepare the output file path
    let output_path = args.output.clone().unwrap_or_else(|| format!("{}.enc", args.input));

    // step 5: write the nonce and ciphertext to the output file
    ensure_directory_exists(&output_path)?;
    create_and_write_file(&output_path, &nonce)?;
    append_to_file(&output_path, &ciphertext)?;

    println!("File encrypted successfully: {}", output_path);
    Ok(())
}

/// handles the file decryption process
pub fn decrypt_file(args: &DecryptArgs) -> Result<()> {
    // step 1: load the decryption key
    let key_path = args
        .key
        .as_ref()
        .context("Key file must be specified with --key")?;
    let key = load_key_from_file(key_path)
        .with_context(|| format!("Failed to load key from file: {}", key_path))?;

    // step 2: read the encrypted data from the input file
    let data = read_file(&args.input)
        .with_context(|| format!("Failed to read input file: {}", args.input))?;

    if data.len() < NONCE_SIZE {
        anyhow::bail!("Invalid encrypted file: data is too short");
    }

    // step 3: extract the nonce and ciphertext
    let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
    let mut nonce = [0u8; NONCE_SIZE];
    nonce.copy_from_slice(nonce_bytes);

    // step 4: decrypt the data
    let plaintext = decrypt_data(&key, &nonce, ciphertext)?;

    // step 5: prepare the output file
    let output_path = args.output.clone().unwrap_or_else(|| {
        Path::new(&args.input)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| format!("{}_decrypted", s))
            .unwrap_or_else(|| "decrypted_output".to_string())
    });

    // step 6: write the plaintext to the output file
    write_file(&output_path, &plaintext)
        .with_context(|| format!("Failed to write decrypted data to: {}", output_path))?;

    println!("File decrypted successfully: {}", output_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key_management::generate_key;
    use tempfile::tempdir;

    #[test]
    fn test_encrypt_and_decrypt() {
        let dir = tempdir().expect("Failed to create temp dir");
        let input_file = dir.path().join("input.txt");
        let encrypted_file = dir.path().join("input.txt.enc");
        let decrypted_file = dir.path().join("input_decrypted.txt");
        let key_file = dir.path().join("test.key");

        let input_data = b"Secret message to encrypt";

        // write input data to file
        write_file(input_file.to_str().unwrap(), input_data)
            .expect("Failed to write input file");

        // create EncryptArgs
        let encrypt_args = EncryptArgs {
            input: input_file.to_str().unwrap().to_string(),
            output: Some(encrypted_file.to_str().unwrap().to_string()),
            key: Some(key_file.to_str().unwrap().to_string()),
        };

        // encrypt the file
        encrypt_file(&encrypt_args).expect("Encryption failed");

        // create DecryptArgs
        let decrypt_args = DecryptArgs {
            input: encrypted_file.to_str().unwrap().to_string(),
            output: Some(decrypted_file.to_str().unwrap().to_string()),
            key: Some(key_file.to_str().unwrap().to_string()),
        };

        // decrypt the file
        decrypt_file(&decrypt_args).expect("Decryption failed");
        
        // verify the decrypted data matches the original
        let decrypted_data = read_file(decrypted_file.to_str().unwrap())
            .expect("Failed to read decrypted file");
        assert_eq!(
            input_data.to_vec(),
            decrypted_data,
            "Decrypted data should match original input data"
        );
    }
}


