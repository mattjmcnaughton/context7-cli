use anyhow::Result;

use crate::clients::Context7Client;

pub async fn execute(id: String) -> Result<()> {
    let client = Context7Client::new();
    let body = client.get_docs(&id).await?;
    println!("{}", body);
    Ok(())
}
