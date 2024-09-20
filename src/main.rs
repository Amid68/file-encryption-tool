mod cli;
mod encryption;
mod decryption;
mod key_management;
mod file_io;
mod operations;

use anyhow::Result;
use cli::{Cli, Commands};
use clap::Parser;
use operations::{decrypt_file, encrypt_file};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt(args) => {
            encrypt_file(args)?;
        }
        Commands::Decrypt(args) => {
            decrypt_file(args)?;
        }
    }

    Ok(())
}

