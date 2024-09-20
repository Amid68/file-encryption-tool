use clap::{Args, Parser, Subcommand};

/// command-line interface definition
#[derive(Debug, Parser)]
#[command(name = "File Encryption Tool")]
#[command(about = "Encrypt and decrypt files using AES-GCM encryption", long_about = None)]
pub struct Cli {
    /// subcommands for encryption and decryption
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// encrypt a file
    Encrypt(EncryptArgs),

    /// decrypt a file
    Decrypt(DecryptArgs),
}

#[derive(Debug, Args)]
pub struct EncryptArgs {
    /// input file to encrypt
    #[arg(short, long)]
    pub input: String,

    /// output file for the encrypted data
    #[arg(short, long)]
    pub output: Option<String>,

    /// key file to use for encryption
    #[arg(short = 'k', long)]
    pub key: Option<String>,
}

#[derive(Debug, Args)]
pub struct DecryptArgs {
    /// input file to decrypt
    #[arg(short, long)]
    pub input: String,
    
    /// output file for the decrypted data
    #[arg(short, long)]
    pub output: Option<String>,

    /// key file to use for decryption
    #[arg(short = 'k', long)]
    pub key: Option<String>,
}















