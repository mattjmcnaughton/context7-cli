use anyhow::Result;

use crate::clients::Context7ClientTrait;
use crate::core::sorting::{SortField, sort_search_results};
use crate::core::validation::validate_search_results_not_empty;

pub async fn execute<T: Context7ClientTrait>(client: &T, query: String) -> Result<()> {
    let search_response = client.search(&query).await?;

    validate_search_results_not_empty(&search_response.results, &query)?;

    let sorted_results = sort_search_results(search_response.results, SortField::Stars);

    let first_result = &sorted_results[0];

    let body = client.get_docs(&first_result.id).await?;
    println!("{}", body);

    Ok(())
}
