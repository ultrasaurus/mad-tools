use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Account {
    pub id: String,
    pub username: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://quakers.social/api/v1/directory?local=true")
        .await?
        .json::<Vec<Account>>()
        .await?;
    println!("{:#?}", resp);
    println!("count: {:#?}", resp.len());
    Ok(())
}
