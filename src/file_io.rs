use anyhow::{Context, Result};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// reads all bytes from the specified file path
pub fn read_file(path: &str) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("Failed to read file: {}", path))
}

/// writes bytes to the specified file path
pub fn write_file(path: &str, data: &[u8]) -> Result<()> {
    fs::write(path, data)
        .with_context(|| format!("Failed to write to file: {}", path))
}

/// creates a new file and writes data to it
pub fn create_and_write_file(path: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(path)
        .with_context(|| format!("Failed to create file: {}", path))?;
    file.write_all(data)
        .with_context(|| format!("Failed to write data to file: {}", path))?;
    Ok(())
}

/// appends data to an existing file or creates it if it doesn't exist
pub fn append_to_file(path: &str, data: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .with_context(|| format!("Failed to open file for appending: {}", path))?;
    file.write_all(data)
        .with_context(|| format!("Failed to append data to file: {}", path))?;
    Ok(())
}

/// ensures that the directory for the given file path exists, creating it if necessary
pub fn ensure_directory_exists(file_path: &str) -> Result<()> {
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }
    Ok(())
}


