/// Integration tests for Context7 API client
///
/// These tests verify that our client correctly interacts with the real Context7 API.
/// They are marked as ignored by default and require network access.
///
/// Run with: RUN_NETWORK_TESTS=1 cargo test -- --ignored
use context7_cli::clients::Context7ClientTrait;

/// Helper function to check if network tests should run
fn should_run_network_tests() -> bool {
    std::env::var("RUN_NETWORK_TESTS").is_ok()
}

#[tokio::test]
#[ignore = "network test - set RUN_NETWORK_TESTS=1 and run with: cargo test -- --ignored"]
async fn test_context7_search_returns_results() {
    if !should_run_network_tests() {
        eprintln!("Skipping network test - set RUN_NETWORK_TESTS=1 to run");
        return;
    }

    use context7_cli::clients::Context7Client;

    let client = Context7Client::new();
    let response = client
        .search("fastapi")
        .await
        .expect("Search should succeed");

    // Verify we got results
    assert!(!response.results.is_empty(), "Should return search results");

    // Verify results have expected structure
    let first_result = &response.results[0];
    assert!(!first_result.id.is_empty(), "Result should have an ID");
}

#[tokio::test]
#[ignore = "network test - set RUN_NETWORK_TESTS=1 and run with: cargo test -- --ignored"]
async fn test_context7_get_docs_returns_content() {
    if !should_run_network_tests() {
        eprintln!("Skipping network test - set RUN_NETWORK_TESTS=1 to run");
        return;
    }

    use context7_cli::clients::Context7Client;

    let client = Context7Client::new();
    let docs = client
        .get_docs("/fastapi/fastapi")
        .await
        .expect("Get docs should succeed");

    // Verify we got documentation content
    assert!(!docs.is_empty(), "Should return documentation");
    assert!(
        docs.to_lowercase().contains("fastapi"),
        "Documentation should mention fastapi"
    );
}

#[tokio::test]
#[ignore = "network test - set RUN_NETWORK_TESTS=1 and run with: cargo test -- --ignored"]
async fn test_context7_get_docs_with_leading_slash() {
    if !should_run_network_tests() {
        eprintln!("Skipping network test - set RUN_NETWORK_TESTS=1 to run");
        return;
    }

    use context7_cli::clients::Context7Client;

    let client = Context7Client::new();
    let docs = client
        .get_docs("/fastapi/fastapi")
        .await
        .expect("Should handle leading slash");

    assert!(!docs.is_empty(), "Should return documentation");
}

#[tokio::test]
#[ignore = "network test - set RUN_NETWORK_TESTS=1 and run with: cargo test -- --ignored"]
async fn test_context7_get_docs_without_leading_slash() {
    if !should_run_network_tests() {
        eprintln!("Skipping network test - set RUN_NETWORK_TESTS=1 to run");
        return;
    }

    use context7_cli::clients::Context7Client;

    let client = Context7Client::new();
    let docs = client
        .get_docs("fastapi/fastapi")
        .await
        .expect("Should handle ID without leading slash");

    assert!(!docs.is_empty(), "Should return documentation");
}
