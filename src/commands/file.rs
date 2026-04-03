use clap::Subcommand;
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum FileCommand {
    /// Calculate file hash
    Hash {
        /// File path to hash
        #[arg(required = true)]
        path: PathBuf,
        /// Hash algorithm: sha256, sha512
        #[arg(short, long, default_value = "sha256")]
        algo: String,
    },
}

impl FileCommand {
    pub fn execute(&self) {
        match self {
            FileCommand::Hash { path, algo } => calculate_hash(path, algo),
        }
    }
}

fn calculate_hash(path: &PathBuf, algo: &str) {
    if !path.exists() {
        eprintln!("Error: File not found: {}", path.display());
        return;
    }

    let file = File::open(path).unwrap_or_else(|e| {
        eprintln!("Error: Cannot open file: {}", e);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    let hash = match algo.to_lowercase().as_str() {
        "sha256" => compute_hash::<Sha256>(reader),
        "sha512" => compute_hash::<Sha512>(reader),
        _ => {
            eprintln!("Error: Unknown algorithm: {}. Supported: sha256, sha512", algo);
            std::process::exit(1);
        }
    };

    println!("{}  {}", hash, path.display());
}

fn compute_hash<D: Digest + Default>(mut reader: BufReader<File>) -> String {
    let mut hasher = D::default();
    let mut buffer = [0u8; 8192];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    hex::encode(hasher.finalize())
}