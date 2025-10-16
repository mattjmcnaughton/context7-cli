use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "context7-cli")]
#[command(about = "CLI for Context7 API", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search the Context7 API
    Search {
        /// Query string to search for
        query: String,

        /// Field to sort results by
        #[arg(long, default_value = "stars")]
        sort_by: String,

        /// Limit the number of results returned
        #[arg(long)]
        limit: Option<usize>,

        /// Output only the ID field (one per line)
        #[arg(long)]
        id_only: bool,
    },
    /// Get documentation for a library by ID
    GetDocs {
        /// Library ID (e.g., "/fastapi/fastapi")
        id: String,
    },
}
