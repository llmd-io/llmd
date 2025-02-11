mod commands;
mod proto;

use clap::{Parser, Subcommand};
use commands::model::{ModelCommands, handle_model_command};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage models (pull, push, list, copy, remove)
    Model {
        #[command(subcommand)]
        command: ModelCommands,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Model { command } => handle_model_command(command).await,
    }
    Ok(())
}
