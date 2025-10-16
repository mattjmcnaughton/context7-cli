use anyhow::Result;

use crate::clients::Context7Client;

pub async fn execute(
    query: String,
    sort_by: String,
    limit: Option<usize>,
    id_only: bool,
) -> Result<()> {
    let client = Context7Client::new();
    let mut search_response = client.search(&query).await?;

    // Validate sort_by field
    let valid_fields = [
        "stars",
        "totalPages",
        "totalSnippets",
        "totalTokens",
        "trustScore",
    ];
    if !valid_fields.contains(&sort_by.as_str()) {
        anyhow::bail!(
            "Invalid sort field '{}'. Valid options are: {}",
            sort_by,
            valid_fields.join(", ")
        );
    }

    // Sort the results
    search_response
        .results
        .sort_by(|a, b| match sort_by.as_str() {
            "stars" => b.stars.unwrap_or(0).cmp(&a.stars.unwrap_or(0)),
            "totalPages" => b.total_pages.unwrap_or(0).cmp(&a.total_pages.unwrap_or(0)),
            "totalSnippets" => b
                .total_snippets
                .unwrap_or(0)
                .cmp(&a.total_snippets.unwrap_or(0)),
            "totalTokens" => b
                .total_tokens
                .unwrap_or(0)
                .cmp(&a.total_tokens.unwrap_or(0)),
            "trustScore" => b
                .trust_score
                .unwrap_or(0.0)
                .partial_cmp(&a.trust_score.unwrap_or(0.0))
                .unwrap_or(std::cmp::Ordering::Equal),
            _ => unreachable!("Already validated sort_by field"),
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

    Ok(())
}
