use openai_rs;
use std::env::var;

#[tokio::test]
pub async fn list_models() {
    let c = openai_rs::Client::new(var("OPENAI_API_KEY").unwrap());
    c.list_models().await.unwrap();
}
