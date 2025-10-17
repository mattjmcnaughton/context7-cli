use anyhow::Result;

use crate::clients::Context7ClientTrait;
use crate::models::SearchResponse;
use crate::models::search::SearchResult;

/// Mock implementation of Context7Client for testing.
/// Returns predefined test data instead of making real API calls.
pub struct MockContext7Client;

impl MockContext7Client {
    pub fn new() -> Self {
        Self
    }
}

impl Context7ClientTrait for MockContext7Client {
    async fn search(&self, _query: &str) -> Result<SearchResponse> {
        // Return realistic test data with varied characteristics
        let results = vec![
            SearchResult {
                id: "/facebook/react".to_string(),
                title: Some("React".to_string()),
                description: Some("A JavaScript library for building user interfaces".to_string()),
                branch: Some("main".to_string()),
                state: Some("active".to_string()),
                stars: Some(220000),
                last_update_date: Some("2025-01-15".to_string()),
                total_pages: Some(150),
                total_snippets: Some(850),
                total_tokens: Some(450000),
                trust_score: Some(9.5),
                versions: Some(vec![]),
            },
            SearchResult {
                id: "/vercel/next.js".to_string(),
                title: Some("Next.js".to_string()),
                description: Some("The React Framework for the Web".to_string()),
                branch: Some("canary".to_string()),
                state: Some("active".to_string()),
                stars: Some(120000),
                last_update_date: Some("2025-01-14".to_string()),
                total_pages: Some(200),
                total_snippets: Some(1200),
                total_tokens: Some(600000),
                trust_score: Some(9.2),
                versions: Some(vec![]),
            },
            SearchResult {
                id: "/vuejs/core".to_string(),
                title: Some("Vue.js".to_string()),
                description: Some("Progressive JavaScript Framework".to_string()),
                branch: Some("main".to_string()),
                state: Some("active".to_string()),
                stars: Some(45000),
                last_update_date: Some("2025-01-10".to_string()),
                total_pages: Some(80),
                total_snippets: Some(400),
                total_tokens: Some(200000),
                trust_score: Some(8.8),
                versions: Some(vec![]),
            },
            SearchResult {
                id: "/sveltejs/svelte".to_string(),
                title: Some("Svelte".to_string()),
                description: Some("Cybernetically enhanced web apps".to_string()),
                branch: Some("master".to_string()),
                state: Some("active".to_string()),
                stars: Some(75000),
                last_update_date: Some("2025-01-12".to_string()),
                total_pages: Some(60),
                total_snippets: Some(300),
                total_tokens: Some(150000),
                trust_score: Some(8.5),
                versions: Some(vec![]),
            },
        ];

        Ok(SearchResponse { results })
    }

    async fn get_docs(&self, id: &str) -> Result<String> {
        // Return sample documentation based on the ID
        let docs = format!(
            r#"# Documentation for {}

## Overview
This is sample documentation for testing purposes.

## Installation
```bash
npm install {}
```

## Usage
```javascript
import {{ something }} from '{}';
```

## API Reference
- `function1()` - Does something useful
- `function2()` - Does something else useful

## Examples
See the examples directory for more information.
"#,
            id, id, id
        );

        Ok(docs)
    }
}

impl Default for MockContext7Client {
    fn default() -> Self {
        Self::new()
    }
}
