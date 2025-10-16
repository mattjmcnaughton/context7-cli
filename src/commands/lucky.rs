use anyhow::Result;

use crate::clients::Context7Client;

pub async fn execute(query: String) -> Result<()> {
    let client = Context7Client::new();
    let mut search_response = client.search(&query).await?;

    // Sort the results by stars (descending) to get the most popular one first
    search_response
        .results
        .sort_by(|a, b| b.stars.unwrap_or(0).cmp(&a.stars.unwrap_or(0)));

    // Hard fail if no search results returned
    if search_response.results.is_empty() {
        anyhow::bail!("No results found for query: '{}'", query);
    }

    // Get the first (most starred) result
    let first_result = &search_response.results[0];

    // Fetch and output the documentation
    let body = client.get_docs(&first_result.id).await?;
    println!("{}", body);

    Ok(())
}
