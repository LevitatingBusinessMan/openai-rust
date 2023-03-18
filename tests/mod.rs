use openai_rs;
use std::env::var;

#[tokio::test]
pub async fn list_models() {
    let c = openai_rs::Client::new(var("OPENAI_API_KEY").unwrap());
    c.list_models().await.unwrap();
}

#[tokio::test]
pub async fn create_chat() {
    let c = openai_rs::Client::new(var("OPENAI_API_KEY").unwrap());
    let args = openai_rs::chat::ChatArguments::new("gpt-3.5-turbo", vec![
        openai_rs::chat::ChatMessage {
            role: "user".to_owned(),
            content: "Hello GPT!".to_owned(),
        }
    ]);
    c.create_chat(args).await.unwrap();
}
