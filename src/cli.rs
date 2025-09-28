use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pm", version)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Initialize a new vault
    Init,
    /// Add a new password to the vault
    Add {
        #[arg(long)]
        service: String,
        #[arg(long)]
        password: String,
    },
    /// Retrieve a password from the vault
    Get {
        #[arg(long)]
        service: String,
    },
    /// List all the services stored in the vault
    List,
}
