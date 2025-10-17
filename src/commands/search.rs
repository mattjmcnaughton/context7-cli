use anyhow::Result;

use crate::clients::Context7ClientTrait;
use crate::core::formatting;
use crate::core::sorting::{SortField, apply_limit, sort_search_results};

pub async fn execute<T: Context7ClientTrait>(
    client: &T,
    query: String,
    sort_by: String,
    limit: Option<usize>,
    id_only: bool,
) -> Result<()> {
    let sort_field = SortField::from_str(&sort_by)?;

    let search_response = client.search(&query).await?;

    let mut results = sort_search_results(search_response.results, sort_field);

    if let Some(limit) = limit {
        results = apply_limit(results, limit);
    }

    let output = formatting::format_search_results(&results, id_only)?;
    println!("{}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::MockContext7Client;

    #[tokio::test]
    async fn test_search_with_mock_client() {
        let mock_client = MockContext7Client::new();

        // Test that search executes without errors using mock data
        let result = execute(
            &mock_client,
            "react".to_string(),
            "stars".to_string(),
            None,
            false,
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_with_limit() {
        let mock_client = MockContext7Client::new();

        // Test that search with limit executes successfully
        let result = execute(
            &mock_client,
            "javascript".to_string(),
            "stars".to_string(),
            Some(2),
            false,
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_id_only_mode() {
        let mock_client = MockContext7Client::new();

        // Test that id-only mode works
        let result = execute(
            &mock_client,
            "vue".to_string(),
            "trustScore".to_string(),
            None,
            true,
        )
        .await;

        assert!(result.is_ok());
    }
}
