use anyhow::Result;

use crate::models::search::SearchResult;

/// Fields that can be used to sort search results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    Stars,
    TotalPages,
    TotalSnippets,
    TotalTokens,
    TrustScore,
}

impl SortField {
    /// Parse a string into a SortField
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "stars" => Ok(SortField::Stars),
            "totalPages" => Ok(SortField::TotalPages),
            "totalSnippets" => Ok(SortField::TotalSnippets),
            "totalTokens" => Ok(SortField::TotalTokens),
            "trustScore" => Ok(SortField::TrustScore),
            _ => anyhow::bail!(
                "Invalid sort field '{}'. Valid options are: stars, totalPages, totalSnippets, totalTokens, trustScore",
                s
            ),
        }
    }
}

/// Sort search results by a specified field in descending order.
///
/// # Arguments
/// * `results` - Vector of search results to sort
/// * `sort_by` - Field to sort by
///
/// # Returns
/// * Sorted results (highest values first)
pub fn sort_search_results(
    mut results: Vec<SearchResult>,
    sort_by: SortField,
) -> Vec<SearchResult> {
    results.sort_by(|a, b| match sort_by {
        SortField::Stars => b.stars.unwrap_or(0).cmp(&a.stars.unwrap_or(0)),
        SortField::TotalPages => b.total_pages.unwrap_or(0).cmp(&a.total_pages.unwrap_or(0)),
        SortField::TotalSnippets => b
            .total_snippets
            .unwrap_or(0)
            .cmp(&a.total_snippets.unwrap_or(0)),
        SortField::TotalTokens => b
            .total_tokens
            .unwrap_or(0)
            .cmp(&a.total_tokens.unwrap_or(0)),
        SortField::TrustScore => b
            .trust_score
            .unwrap_or(0.0)
            .partial_cmp(&a.trust_score.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal),
    });

    results
}

/// Apply a limit to the number of results.
///
/// # Arguments
/// * `results` - Vector of search results
/// * `limit` - Maximum number of results to return
///
/// # Returns
/// Truncated vector with at most `limit` elements
pub fn apply_limit(mut results: Vec<SearchResult>, limit: usize) -> Vec<SearchResult> {
    results.truncate(limit);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_result(
        id: &str,
        stars: Option<i64>,
        total_pages: Option<i64>,
        total_snippets: Option<i64>,
        total_tokens: Option<i64>,
        trust_score: Option<f64>,
    ) -> SearchResult {
        SearchResult {
            id: id.to_string(),
            branch: None,
            description: None,
            last_update_date: None,
            stars,
            state: None,
            title: None,
            total_pages,
            total_snippets,
            total_tokens,
            trust_score,
            versions: None,
        }
    }

    #[test]
    fn test_sort_field_from_str_valid() {
        assert_eq!(SortField::from_str("stars").unwrap(), SortField::Stars);
        assert_eq!(
            SortField::from_str("totalPages").unwrap(),
            SortField::TotalPages
        );
        assert_eq!(
            SortField::from_str("totalSnippets").unwrap(),
            SortField::TotalSnippets
        );
        assert_eq!(
            SortField::from_str("totalTokens").unwrap(),
            SortField::TotalTokens
        );
        assert_eq!(
            SortField::from_str("trustScore").unwrap(),
            SortField::TrustScore
        );
    }

    #[test]
    fn test_sort_field_from_str_invalid() {
        let result = SortField::from_str("invalid");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid sort field")
        );
    }

    #[test]
    fn test_sort_by_stars_descending() {
        let results = vec![
            create_test_result("low", Some(10), None, None, None, None),
            create_test_result("high", Some(100), None, None, None, None),
            create_test_result("medium", Some(50), None, None, None, None),
        ];

        let sorted = sort_search_results(results, SortField::Stars);

        assert_eq!(sorted[0].id, "high");
        assert_eq!(sorted[1].id, "medium");
        assert_eq!(sorted[2].id, "low");
    }

    #[test]
    fn test_sort_by_total_pages_descending() {
        let results = vec![
            create_test_result("low", None, Some(5), None, None, None),
            create_test_result("high", None, Some(100), None, None, None),
            create_test_result("medium", None, Some(50), None, None, None),
        ];

        let sorted = sort_search_results(results, SortField::TotalPages);

        assert_eq!(sorted[0].id, "high");
        assert_eq!(sorted[1].id, "medium");
        assert_eq!(sorted[2].id, "low");
    }

    #[test]
    fn test_sort_by_total_snippets_descending() {
        let results = vec![
            create_test_result("low", None, None, Some(10), None, None),
            create_test_result("high", None, None, Some(200), None, None),
            create_test_result("medium", None, None, Some(75), None, None),
        ];

        let sorted = sort_search_results(results, SortField::TotalSnippets);

        assert_eq!(sorted[0].id, "high");
        assert_eq!(sorted[1].id, "medium");
        assert_eq!(sorted[2].id, "low");
    }

    #[test]
    fn test_sort_by_total_tokens_descending() {
        let results = vec![
            create_test_result("low", None, None, None, Some(1000), None),
            create_test_result("high", None, None, None, Some(50000), None),
            create_test_result("medium", None, None, None, Some(25000), None),
        ];

        let sorted = sort_search_results(results, SortField::TotalTokens);

        assert_eq!(sorted[0].id, "high");
        assert_eq!(sorted[1].id, "medium");
        assert_eq!(sorted[2].id, "low");
    }

    #[test]
    fn test_sort_by_trust_score_descending() {
        let results = vec![
            create_test_result("low", None, None, None, None, Some(5.0)),
            create_test_result("high", None, None, None, None, Some(9.5)),
            create_test_result("medium", None, None, None, None, Some(7.0)),
        ];

        let sorted = sort_search_results(results, SortField::TrustScore);

        assert_eq!(sorted[0].id, "high");
        assert_eq!(sorted[1].id, "medium");
        assert_eq!(sorted[2].id, "low");
    }

    #[test]
    fn test_sort_handles_none_values_for_stars() {
        let results = vec![
            create_test_result("none", None, None, None, None, None),
            create_test_result("some", Some(50), None, None, None, None),
            create_test_result("also_none", None, None, None, None, None),
        ];

        let sorted = sort_search_results(results, SortField::Stars);

        // The one with a value should be first
        assert_eq!(sorted[0].id, "some");
        // The Nones should maintain their relative order (stable sort)
        assert!(sorted[1].id == "none" || sorted[1].id == "also_none");
    }

    #[test]
    fn test_sort_handles_none_values_for_trust_score() {
        let results = vec![
            create_test_result("none", None, None, None, None, None),
            create_test_result("some", None, None, None, None, Some(7.5)),
        ];

        let sorted = sort_search_results(results, SortField::TrustScore);

        assert_eq!(sorted[0].id, "some");
        assert_eq!(sorted[1].id, "none");
    }

    #[test]
    fn test_sort_empty_list() {
        let results: Vec<SearchResult> = vec![];
        let sorted = sort_search_results(results, SortField::Stars);
        assert_eq!(sorted.len(), 0);
    }

    #[test]
    fn test_sort_single_item() {
        let results = vec![create_test_result("only", Some(42), None, None, None, None)];
        let sorted = sort_search_results(results, SortField::Stars);
        assert_eq!(sorted.len(), 1);
        assert_eq!(sorted[0].id, "only");
    }

    #[test]
    fn test_apply_limit_truncates_results() {
        let results = vec![
            create_test_result("first", Some(100), None, None, None, None),
            create_test_result("second", Some(90), None, None, None, None),
            create_test_result("third", Some(80), None, None, None, None),
            create_test_result("fourth", Some(70), None, None, None, None),
        ];

        let limited = apply_limit(results, 2);

        assert_eq!(limited.len(), 2);
        assert_eq!(limited[0].id, "first");
        assert_eq!(limited[1].id, "second");
    }

    #[test]
    fn test_apply_limit_larger_than_list() {
        let results = vec![
            create_test_result("first", Some(100), None, None, None, None),
            create_test_result("second", Some(90), None, None, None, None),
        ];

        let limited = apply_limit(results.clone(), 10);

        assert_eq!(limited.len(), 2);
        assert_eq!(limited[0].id, "first");
        assert_eq!(limited[1].id, "second");
    }

    #[test]
    fn test_apply_limit_zero() {
        let results = vec![create_test_result(
            "first",
            Some(100),
            None,
            None,
            None,
            None,
        )];

        let limited = apply_limit(results, 0);

        assert_eq!(limited.len(), 0);
    }
}
