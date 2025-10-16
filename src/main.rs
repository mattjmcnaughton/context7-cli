use anyhow::Result;
use clap::Parser;

mod cli;
mod clients;
mod commands;
mod models;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            query,
            sort_by,
            limit,
            id_only,
        } => {
            commands::search::execute(query, sort_by, limit, id_only).await?;
        }
        Commands::GetDocs { id } => {
            commands::get_docs::execute(id).await?;
        }
        Commands::Lucky { query } => {
            commands::lucky::execute(query).await?;
        }
    }

    Ok(())
}
