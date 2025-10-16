use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "context7-cli")]
#[command(about = "CLI for Context7 API", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SearchResult {
    branch: Option<String>,
    description: Option<String>,
    id: String,
    #[serde(rename = "lastUpdateDate")]
    last_update_date: Option<String>,
    stars: Option<i64>,
    state: Option<String>,
    title: Option<String>,
    #[serde(rename = "totalPages")]
    total_pages: Option<i64>,
    #[serde(rename = "totalSnippets")]
    total_snippets: Option<i64>,
    #[serde(rename = "totalTokens")]
    total_tokens: Option<i64>,
    #[serde(rename = "trustScore")]
    trust_score: Option<f64>,
    versions: Option<Vec<serde_json::Value>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { query, sort_by, limit, id_only } => {
            let url = format!("https://context7.com/api/v1/search?query={}", query);

            let response = reqwest::get(&url).await?;
            let mut search_response: SearchResponse = response.json().await?;

            // Validate sort_by field
            let valid_fields = ["stars", "totalPages", "totalSnippets", "totalTokens", "trustScore"];
            if !valid_fields.contains(&sort_by.as_str()) {
                anyhow::bail!(
                    "Invalid sort field '{}'. Valid options are: {}",
                    sort_by,
                    valid_fields.join(", ")
                );
            }

            // Sort the results
            search_response.results.sort_by(|a, b| {
                match sort_by.as_str() {
                    "stars" => b.stars.unwrap_or(0).cmp(&a.stars.unwrap_or(0)),
                    "totalPages" => b.total_pages.unwrap_or(0).cmp(&a.total_pages.unwrap_or(0)),
                    "totalSnippets" => b.total_snippets.unwrap_or(0).cmp(&a.total_snippets.unwrap_or(0)),
                    "totalTokens" => b.total_tokens.unwrap_or(0).cmp(&a.total_tokens.unwrap_or(0)),
                    "trustScore" => b.trust_score.unwrap_or(0.0).partial_cmp(&a.trust_score.unwrap_or(0.0)).unwrap_or(std::cmp::Ordering::Equal),
                    _ => unreachable!("Already validated sort_by field"),
                }
            });

            // Apply limit if specified
            if let Some(limit) = limit {
                search_response.results.truncate(limit);
            }

            // Output based on format
            if id_only {
                for result in &search_response.results {
                    println!("{}", result.id);
                }
            } else {
                let pretty_json = serde_json::to_string_pretty(&search_response.results)?;
                println!("{}", pretty_json);
            }
        }
        Commands::GetDocs { id } => {
            // Strip leading slash if present for URL construction
            let id_path = id.strip_prefix('/').unwrap_or(&id);
            let url = format!("https://context7.com/api/v1/{}", id_path);

            let response = reqwest::get(&url).await?;
            let body = response.text().await?;
            println!("{}", body);
        }
    }

    Ok(())
}
