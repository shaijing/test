use clap::{Parser, Subcommand};
use commands::{IdCommand, FileCommand};

mod commands;

#[derive(Parser)]
#[command(name = "test-bin")]
#[command(about = "A multi-purpose CLI toolkit", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// ID generation tools (ULID, UUID)
    Id {
        #[command(subcommand)]
        command: IdCommand,
    },
    /// File processing tools
    File {
        #[command(subcommand)]
        command: FileCommand,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Id { command } => command.execute(),
        Commands::File { command } => command.execute(),
    }
}