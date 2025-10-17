use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod core;

// Use the modules from the library
use context7_cli::clients;
use context7_cli::models;

use cli::{Cli, Commands};
use clients::Context7Client;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = Context7Client::new();

    match cli.command {
        Commands::Search {
            query,
            sort_by,
            limit,
            id_only,
        } => {
            commands::search::execute(&client, query, sort_by, limit, id_only).await?;
        }
        Commands::GetDocs { id } => {
            commands::get_docs::execute(&client, id).await?;
        }
        Commands::Lucky { query } => {
            commands::lucky::execute(&client, query).await?;
        }
    }

    Ok(())
}
