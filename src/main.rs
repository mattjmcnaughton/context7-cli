use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://context7.com/api/v1/search?query=fastapi";

    let response = reqwest::get(url).await?;
    let search_response: SearchResponse = response.json().await?;

    let pretty_json = serde_json::to_string_pretty(&search_response.results)?;
    println!("{}", pretty_json);

    Ok(())
}
