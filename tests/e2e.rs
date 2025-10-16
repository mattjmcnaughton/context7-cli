use assert_cmd::Command;
use predicates::prelude::*;

/// Helper function to check if network tests should run
fn should_run_network_tests() -> bool {
    std::env::var("RUN_NETWORK_TESTS").is_ok()
}

/// Test the complete flow: search for fastapi, then get its docs
///
/// This test makes real network requests to the Context7 API.
/// Run with: RUN_NETWORK_TESTS=1 cargo test -- --ignored
#[test]
#[ignore = "network test - set RUN_NETWORK_TESTS=1 and run with: cargo test -- --ignored"]
fn test_search_and_get_docs_flow() -> Result<(), Box<dyn std::error::Error>> {
    if !should_run_network_tests() {
        eprintln!("Skipping network test - set RUN_NETWORK_TESTS=1 to run");
        return Ok(());
    }
    // Test 1: Search for fastapi
    let mut cmd = Command::cargo_bin("context7-cli")?;
    cmd.arg("search").arg("fastapi").arg("--limit").arg("5");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("/fastapi/fastapi"))
        .stdout(predicate::str::contains("\"id\""));

    // Test 2: Get docs for fastapi
    let mut cmd = Command::cargo_bin("context7-cli")?;
    cmd.arg("get-docs").arg("/fastapi/fastapi");

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"(?i)fastapi").unwrap());

    Ok(())
}

/// Test search with --id-only flag
///
/// This test makes real network requests to the Context7 API.
/// Run with: RUN_NETWORK_TESTS=1 cargo test -- --ignored
#[test]
#[ignore = "network test - set RUN_NETWORK_TESTS=1 and run with: cargo test -- --ignored"]
fn test_search_with_id_only_flag() -> Result<(), Box<dyn std::error::Error>> {
    if !should_run_network_tests() {
        eprintln!("Skipping network test - set RUN_NETWORK_TESTS=1 to run");
        return Ok(());
    }
    let mut cmd = Command::cargo_bin("context7-cli")?;
    cmd.arg("search")
        .arg("fastapi")
        .arg("--id-only")
        .arg("--limit")
        .arg("3");

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^/[\w-]+/[\w-]+\n").unwrap());

    Ok(())
}

/// Test search with different sort options
///
/// This test makes real network requests to the Context7 API.
/// Run with: RUN_NETWORK_TESTS=1 cargo test -- --ignored
#[test]
#[ignore = "network test - set RUN_NETWORK_TESTS=1 and run with: cargo test -- --ignored"]
fn test_search_with_sort_by() -> Result<(), Box<dyn std::error::Error>> {
    if !should_run_network_tests() {
        eprintln!("Skipping network test - set RUN_NETWORK_TESTS=1 to run");
        return Ok(());
    }
    let mut cmd = Command::cargo_bin("context7-cli")?;
    cmd.arg("search")
        .arg("rust")
        .arg("--sort-by")
        .arg("trustScore")
        .arg("--limit")
        .arg("5");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"trustScore\""));

    Ok(())
}

/// Test invalid sort field returns error
///
/// This test makes real network requests to the Context7 API.
/// Run with: RUN_NETWORK_TESTS=1 cargo test -- --ignored
#[test]
#[ignore = "network test - set RUN_NETWORK_TESTS=1 and run with: cargo test -- --ignored"]
fn test_search_with_invalid_sort_field() -> Result<(), Box<dyn std::error::Error>> {
    if !should_run_network_tests() {
        eprintln!("Skipping network test - set RUN_NETWORK_TESTS=1 to run");
        return Ok(());
    }
    let mut cmd = Command::cargo_bin("context7-cli")?;
    cmd.arg("search")
        .arg("rust")
        .arg("--sort-by")
        .arg("invalid_field");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid sort field"));

    Ok(())
}
