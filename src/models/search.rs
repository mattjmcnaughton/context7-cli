use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchResult {
    pub branch: Option<String>,
    pub description: Option<String>,
    pub id: String,
    #[serde(rename = "lastUpdateDate")]
    pub last_update_date: Option<String>,
    pub stars: Option<i64>,
    pub state: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "totalPages")]
    pub total_pages: Option<i64>,
    #[serde(rename = "totalSnippets")]
    pub total_snippets: Option<i64>,
    #[serde(rename = "totalTokens")]
    pub total_tokens: Option<i64>,
    #[serde(rename = "trustScore")]
    pub trust_score: Option<f64>,
    pub versions: Option<Vec<serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_deserialization_full() {
        let mock_json = r#"{
            "id": "/test/library",
            "title": "Test Library",
            "description": "A test library for testing",
            "branch": "main",
            "state": "active",
            "stars": 1000,
            "lastUpdateDate": "2025-01-15",
            "totalPages": 50,
            "totalSnippets": 200,
            "totalTokens": 100000,
            "trustScore": 8.5,
            "versions": ["1.0.0", "1.1.0"]
        }"#;

        let result: SearchResult = serde_json::from_str(mock_json).unwrap();

        assert_eq!(result.id, "/test/library");
        assert_eq!(result.title, Some("Test Library".to_string()));
        assert_eq!(
            result.description,
            Some("A test library for testing".to_string())
        );
        assert_eq!(result.branch, Some("main".to_string()));
        assert_eq!(result.state, Some("active".to_string()));
        assert_eq!(result.stars, Some(1000));
        assert_eq!(result.last_update_date, Some("2025-01-15".to_string()));
        assert_eq!(result.total_pages, Some(50));
        assert_eq!(result.total_snippets, Some(200));
        assert_eq!(result.total_tokens, Some(100000));
        assert_eq!(result.trust_score, Some(8.5));
        assert!(result.versions.is_some());
    }

    #[test]
    fn test_search_result_deserialization_minimal() {
        let mock_json = r#"{
            "id": "/minimal/lib"
        }"#;

        let result: SearchResult = serde_json::from_str(mock_json).unwrap();

        assert_eq!(result.id, "/minimal/lib");
        assert_eq!(result.title, None);
        assert_eq!(result.description, None);
        assert_eq!(result.branch, None);
        assert_eq!(result.state, None);
        assert_eq!(result.stars, None);
        assert_eq!(result.last_update_date, None);
        assert_eq!(result.total_pages, None);
        assert_eq!(result.total_snippets, None);
        assert_eq!(result.total_tokens, None);
        assert_eq!(result.trust_score, None);
        assert_eq!(result.versions, None);
    }

    #[test]
    fn test_search_result_field_name_mappings() {
        // Test that camelCase API fields map to snake_case Rust fields
        let mock_json = r#"{
            "id": "/test/lib",
            "lastUpdateDate": "2025-01-01",
            "totalPages": 10,
            "totalSnippets": 50,
            "totalTokens": 5000,
            "trustScore": 7.5
        }"#;

        let result: SearchResult = serde_json::from_str(mock_json).unwrap();

        assert_eq!(result.last_update_date, Some("2025-01-01".to_string()));
        assert_eq!(result.total_pages, Some(10));
        assert_eq!(result.total_snippets, Some(50));
        assert_eq!(result.total_tokens, Some(5000));
        assert_eq!(result.trust_score, Some(7.5));
    }

    #[test]
    fn test_search_result_serialization_roundtrip() {
        let original = SearchResult {
            id: "/roundtrip/test".to_string(),
            title: Some("Roundtrip Test".to_string()),
            description: Some("Testing serialization roundtrip".to_string()),
            branch: Some("develop".to_string()),
            state: Some("active".to_string()),
            stars: Some(500),
            last_update_date: Some("2025-01-10".to_string()),
            total_pages: Some(25),
            total_snippets: Some(100),
            total_tokens: Some(50000),
            trust_score: Some(9.0),
            versions: Some(vec![serde_json::json!("2.0.0")]),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SearchResult = serde_json::from_str(&json).unwrap();

        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.title, deserialized.title);
        assert_eq!(original.description, deserialized.description);
        assert_eq!(original.stars, deserialized.stars);
        assert_eq!(original.trust_score, deserialized.trust_score);
    }

    #[test]
    fn test_search_result_serialization_uses_camel_case() {
        let result = SearchResult {
            id: "/test/lib".to_string(),
            branch: None,
            description: None,
            last_update_date: Some("2025-01-01".to_string()),
            stars: None,
            state: None,
            title: None,
            total_pages: Some(10),
            total_snippets: None,
            total_tokens: None,
            trust_score: Some(8.0),
            versions: None,
        };

        let json = serde_json::to_string(&result).unwrap();

        // Verify that serialization uses camelCase for field names
        assert!(json.contains("\"lastUpdateDate\""));
        assert!(json.contains("\"totalPages\""));
        assert!(json.contains("\"trustScore\""));

        // Verify it doesn't use snake_case
        assert!(!json.contains("last_update_date"));
        assert!(!json.contains("total_pages"));
        assert!(!json.contains("trust_score"));
    }

    #[test]
    fn test_search_response_deserialization() {
        let mock_json = r#"{
            "results": [
                {"id": "/first/lib", "stars": 100},
                {"id": "/second/lib", "stars": 50}
            ]
        }"#;

        let response: SearchResponse = serde_json::from_str(mock_json).unwrap();

        assert_eq!(response.results.len(), 2);
        assert_eq!(response.results[0].id, "/first/lib");
        assert_eq!(response.results[0].stars, Some(100));
        assert_eq!(response.results[1].id, "/second/lib");
        assert_eq!(response.results[1].stars, Some(50));
    }

    #[test]
    fn test_search_response_empty_results() {
        let mock_json = r#"{"results": []}"#;
        let response: SearchResponse = serde_json::from_str(mock_json).unwrap();
        assert_eq!(response.results.len(), 0);
    }

    #[test]
    fn test_search_result_clone() {
        let original = SearchResult {
            id: "/clone/test".to_string(),
            title: Some("Clone Test".to_string()),
            description: None,
            branch: None,
            state: None,
            stars: Some(42),
            last_update_date: None,
            total_pages: None,
            total_snippets: None,
            total_tokens: None,
            trust_score: Some(7.5),
            versions: None,
        };

        let cloned = original.clone();

        assert_eq!(original.id, cloned.id);
        assert_eq!(original.title, cloned.title);
        assert_eq!(original.stars, cloned.stars);
        assert_eq!(original.trust_score, cloned.trust_score);
    }
}
