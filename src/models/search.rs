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
