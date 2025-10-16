use anyhow::Result;

use crate::models::SearchResponse;

const BASE_URL: &str = "https://context7.com/api/v1";

pub struct Context7Client {
    client: reqwest::Client,
}

impl Context7Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn search(&self, query: &str) -> Result<SearchResponse> {
        let url = format!("{}/search?query={}", BASE_URL, query);
        let response = self.client.get(&url).send().await?;
        let search_response = response.json().await?;
        Ok(search_response)
    }

    pub async fn get_docs(&self, id: &str) -> Result<String> {
        // Strip leading slash if present for URL construction
        let id_path = id.strip_prefix('/').unwrap_or(id);
        let url = format!("{}/{}", BASE_URL, id_path);

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }
}

impl Default for Context7Client {
    fn default() -> Self {
        Self::new()
    }
}
