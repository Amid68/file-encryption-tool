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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_read_file_success() -> Result<()> {
        // Create a temporary directory
        let dir = tempdir()?;
        let file_path = dir.path().join("test_read.txt");
        let data = b"Hello, world!";
        fs::write(&file_path, data)?;

        // Read the file using read_file
        let read_data = read_file(file_path.to_str().unwrap())?;
        assert_eq!(read_data, data);

        // Temporary directory and its contents are deleted when `dir` goes out of scope
        Ok(())
    }

    #[test]
    fn test_read_file_not_found() {
        let result = read_file("non_existent_file.txt");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Failed to read file"));
        }
    }

    #[test]
    fn test_write_file_success() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_write.txt");
        let data = b"Write this data.";

        // Write data to the file using write_file
        write_file(file_path.to_str().unwrap(), data)?;

        // Read back the data to verify
        let read_data = fs::read(&file_path)?;
        assert_eq!(read_data, data);

        Ok(())
    }

    #[test]
    fn test_create_and_write_file_success() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_create_and_write.txt");
        let data = b"Creating and writing data.";

        // Create and write data to the file
        create_and_write_file(file_path.to_str().unwrap(), data)?;

        // Verify the file exists and contains the correct data
        assert!(file_path.exists());
        let read_data = fs::read(&file_path)?;
        assert_eq!(read_data, data);

        Ok(())
    }

    #[test]
    fn test_append_to_file_existing() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_append_existing.txt");
        let initial_data = b"Initial data.\n";
        let append_data = b"Appended data.";

        // Create the file with initial data
        fs::write(&file_path, initial_data)?;

        // Append data to the file
        append_to_file(file_path.to_str().unwrap(), append_data)?;

        // Read back the data to verify
        let read_data = fs::read(&file_path)?;
        let expected_data = [initial_data.as_ref(), append_data.as_ref()].concat();
        assert_eq!(read_data, expected_data);

        Ok(())
    }

    #[test]
    fn test_append_to_file_non_existing() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_append_non_existing.txt");
        let append_data = b"Appended data to new file.";

        // Append data to a non-existing file (should create it)
        append_to_file(file_path.to_str().unwrap(), append_data)?;

        // Verify the file exists and contains the appended data
        assert!(file_path.exists());
        let read_data = fs::read(&file_path)?;
        assert_eq!(read_data, append_data);

        Ok(())
    }

    #[test]
    fn test_ensure_directory_exists_new() -> Result<()> {
        let dir = tempdir()?;
        let nested_dir = dir.path().join("nested/directory/structure");
        let file_path = nested_dir.join("test_file.txt");
        let data = b"Data in nested directory.";

        // Ensure the directory exists
        ensure_directory_exists(file_path.to_str().unwrap())?;

        // Verify that the nested directory was created
        assert!(nested_dir.exists());

        // Optionally, create a file in the newly created directory
        fs::write(&file_path, data)?;

        // Verify the file was created with correct data
        let read_data = fs::read(&file_path)?;
        assert_eq!(read_data, data);

        Ok(())
    }

    #[test]
    fn test_ensure_directory_exists_existing() -> Result<()> {
        let dir = tempdir()?;
        let existing_dir = dir.path().join("existing_dir");
        fs::create_dir(&existing_dir)?;

        let file_path = existing_dir.join("test_file.txt");
        let data = b"Data in existing directory.";

        // Ensure the existing directory is handled correctly
        ensure_directory_exists(file_path.to_str().unwrap())?;

        // Verify that the existing directory still exists
        assert!(existing_dir.exists());

        // Create a file in the existing directory
        fs::write(&file_path, data)?;

        // Verify the file was created with correct data
        let read_data = fs::read(&file_path)?;
        assert_eq!(read_data, data);

        Ok(())
    }
}