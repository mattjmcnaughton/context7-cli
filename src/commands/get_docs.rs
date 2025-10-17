use anyhow::Result;

use crate::clients::Context7ClientTrait;

pub async fn execute<T: Context7ClientTrait>(client: &T, id: String) -> Result<()> {
    let body = client.get_docs(&id).await?;
    println!("{}", body);
    Ok(())
}
