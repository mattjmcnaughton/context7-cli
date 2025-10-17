use anyhow::Result;

use crate::models::search::SearchResult;

/// Validate that search results are not empty.
///
/// # Arguments
/// * `results` - The search results to validate
/// * `query` - The query string (used in error message)
///
/// # Returns
/// * `Ok(())` - If results are not empty
/// * `Err` - If results are empty, with an error message including the query
pub fn validate_search_results_not_empty(results: &[SearchResult], query: &str) -> Result<()> {
    if results.is_empty() {
        anyhow::bail!("No results found for query: '{}'", query);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_search_results_not_empty_with_results() {
        let results = vec![SearchResult {
            id: "/test/lib".to_string(),
            branch: None,
            description: None,
            last_update_date: None,
            stars: Some(100),
            state: None,
            title: None,
            total_pages: None,
            total_snippets: None,
            total_tokens: None,
            trust_score: None,
            versions: None,
        }];

        assert!(validate_search_results_not_empty(&results, "test query").is_ok());
    }

    #[test]
    fn test_validate_search_results_not_empty_with_empty_results() {
        let results: Vec<SearchResult> = vec![];
        let result = validate_search_results_not_empty(&results, "test query");

        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("No results found"));
        assert!(error_msg.contains("test query"));
    }

    #[test]
    fn test_validate_search_results_error_includes_query() {
        let results: Vec<SearchResult> = vec![];
        let result = validate_search_results_not_empty(&results, "my special search");

        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("my special search"));
    }

    #[test]
    fn test_validate_search_results_with_multiple_results() {
        let results = vec![
            SearchResult {
                id: "/test/lib1".to_string(),
                branch: None,
                description: None,
                last_update_date: None,
                stars: Some(100),
                state: None,
                title: None,
                total_pages: None,
                total_snippets: None,
                total_tokens: None,
                trust_score: None,
                versions: None,
            },
            SearchResult {
                id: "/test/lib2".to_string(),
                branch: None,
                description: None,
                last_update_date: None,
                stars: Some(50),
                state: None,
                title: None,
                total_pages: None,
                total_snippets: None,
                total_tokens: None,
                trust_score: None,
                versions: None,
            },
        ];

        assert!(validate_search_results_not_empty(&results, "test").is_ok());
    }
}
