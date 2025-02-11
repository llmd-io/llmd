use clap::Subcommand;
use crate::commands::{pull, push, list, cp, rm};
use tonic::transport::Channel;

#[derive(Subcommand)]
pub enum ModelCommands {
    /// Pull a model from remote repository
    Pull {
        /// Model name or path
        model: String,
    },
    /// Push a model to remote repository
    Push {
        /// Model name or path
        model: String,
    },
    /// List all available models
    List,
    /// Copy a model
    Cp {
        /// Source model
        source: String,
        /// Destination path
        destination: String,
    },
    /// Remove a model
    Rm {
        /// Model name or path
        model: String,
    },
}

pub async fn handle_model_command(command: &ModelCommands) {
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await
        .unwrap();

    match command {
        ModelCommands::Pull { model } => pull::execute(model, &channel).await,
        ModelCommands::Push { model } => push::execute(model, &channel).await,
        ModelCommands::List => list::execute(&channel).await,
        ModelCommands::Cp { source, destination } => cp::execute(source, destination, &channel).await,
        ModelCommands::Rm { model } => rm::execute(model, &channel).await,
    }
}
