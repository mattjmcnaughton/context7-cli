use anyhow::Result;

use crate::models::search::SearchResult;

/// Format search results based on the output mode.
///
/// # Arguments
/// * `results` - Search results to format
/// * `id_only` - If true, output only IDs (one per line); otherwise output JSON
///
/// # Returns
/// * Formatted string ready for output
pub fn format_search_results(results: &[SearchResult], id_only: bool) -> Result<String> {
    if id_only {
        Ok(format_ids_only(results))
    } else {
        format_json_pretty(results)
    }
}

/// Format search results as a list of IDs (one per line).
///
/// # Arguments
/// * `results` - Search results to format
///
/// # Returns
/// * String with one ID per line
pub fn format_ids_only(results: &[SearchResult]) -> String {
    results
        .iter()
        .map(|r| r.id.as_str())
        .collect::<Vec<&str>>()
        .join("\n")
}

/// Format search results as pretty-printed JSON.
///
/// # Arguments
/// * `results` - Search results to format
///
/// # Returns
/// * Pretty-printed JSON string
pub fn format_json_pretty(results: &[SearchResult]) -> Result<String> {
    Ok(serde_json::to_string_pretty(results)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_result(id: &str, title: Option<&str>, stars: Option<i64>) -> SearchResult {
        SearchResult {
            id: id.to_string(),
            branch: None,
            description: None,
            last_update_date: None,
            stars,
            state: None,
            title: title.map(|s| s.to_string()),
            total_pages: None,
            total_snippets: None,
            total_tokens: None,
            trust_score: None,
            versions: None,
        }
    }

    #[test]
    fn test_format_ids_only_single_result() {
        let results = vec![create_test_result("/test/lib", None, None)];
        let output = format_ids_only(&results);
        assert_eq!(output, "/test/lib");
    }

    #[test]
    fn test_format_ids_only_multiple_results() {
        let results = vec![
            create_test_result("/first/lib", None, None),
            create_test_result("/second/lib", None, None),
            create_test_result("/third/lib", None, None),
        ];
        let output = format_ids_only(&results);
        assert_eq!(output, "/first/lib\n/second/lib\n/third/lib");
    }

    #[test]
    fn test_format_ids_only_empty_results() {
        let results: Vec<SearchResult> = vec![];
        let output = format_ids_only(&results);
        assert_eq!(output, "");
    }

    #[test]
    fn test_format_json_pretty_single_result() {
        let results = vec![create_test_result(
            "/test/lib",
            Some("Test Library"),
            Some(100),
        )];
        let output = format_json_pretty(&results).unwrap();

        // Verify it's valid JSON
        assert!(output.contains("\"id\": \"/test/lib\""));
        assert!(output.contains("\"title\": \"Test Library\""));
        assert!(output.contains("\"stars\": 100"));

        // Verify it's pretty-printed (contains newlines and indentation)
        assert!(output.contains("\n"));
        assert!(output.contains("  "));
    }

    #[test]
    fn test_format_json_pretty_multiple_results() {
        let results = vec![
            create_test_result("/first/lib", Some("First"), Some(50)),
            create_test_result("/second/lib", Some("Second"), Some(100)),
        ];
        let output = format_json_pretty(&results).unwrap();

        // Verify both results are in the output
        assert!(output.contains("\"/first/lib\""));
        assert!(output.contains("\"/second/lib\""));
        assert!(output.contains("\"First\""));
        assert!(output.contains("\"Second\""));

        // Verify it's an array
        assert!(output.starts_with('['));
        assert!(output.ends_with(']'));
    }

    #[test]
    fn test_format_json_pretty_empty_results() {
        let results: Vec<SearchResult> = vec![];
        let output = format_json_pretty(&results).unwrap();
        assert_eq!(output, "[]");
    }

    #[test]
    fn test_format_json_pretty_is_valid_json() {
        let results = vec![create_test_result("/test/lib", Some("Test"), Some(42))];
        let output = format_json_pretty(&results).unwrap();

        // Verify we can parse it back
        let parsed: Vec<SearchResult> = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].id, "/test/lib");
        assert_eq!(parsed[0].title, Some("Test".to_string()));
        assert_eq!(parsed[0].stars, Some(42));
    }

    #[test]
    fn test_format_search_results_id_only_mode() {
        let results = vec![
            create_test_result("/first/lib", None, None),
            create_test_result("/second/lib", None, None),
        ];
        let output = format_search_results(&results, true).unwrap();
        assert_eq!(output, "/first/lib\n/second/lib");
    }

    #[test]
    fn test_format_search_results_json_mode() {
        let results = vec![create_test_result("/test/lib", Some("Test"), Some(100))];
        let output = format_search_results(&results, false).unwrap();

        assert!(output.contains("\"id\": \"/test/lib\""));
        assert!(output.contains("\"title\": \"Test\""));
        assert!(output.contains("\"stars\": 100"));
    }

    #[test]
    fn test_format_search_results_empty_with_id_only() {
        let results: Vec<SearchResult> = vec![];
        let output = format_search_results(&results, true).unwrap();
        assert_eq!(output, "");
    }

    #[test]
    fn test_format_search_results_empty_with_json() {
        let results: Vec<SearchResult> = vec![];
        let output = format_search_results(&results, false).unwrap();
        assert_eq!(output, "[]");
    }
}
