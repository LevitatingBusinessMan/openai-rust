use futures_util::StreamExt;
use openai_rust;
use std::env::var;
use lazy_static::lazy_static;

lazy_static! {
    static ref KEY: String = var("OPENAI_API_KEY").unwrap();
}

#[tokio::test]
pub async fn list_models() {
    let c = openai_rust::Client::new(&KEY);
    c.list_models().await.unwrap();
}

#[tokio::test]
pub async fn create_chat() {
    let c = openai_rust::Client::new(&KEY);
    let args = openai_rust::chat::ChatArguments::new("gpt-3.5-turbo", vec![
        openai_rust::chat::Message {
            role: "user".to_owned(),
            content: "Hello GPT!".to_owned(),
        }
    ]);
    c.create_chat(args).await.unwrap();
}

#[tokio::test]
pub async fn create_chat_stream() {
    let c = openai_rust::Client::new(&KEY);
    let args = openai_rust::chat::ChatArguments::new("gpt-3.5-turbo", vec![
        openai_rust::chat::Message {
            role: "user".to_owned(),
            content: "Hello GPT!".to_owned(),
        }
    ]);
    c.create_chat_stream(args).await.unwrap().collect::<Vec<_>>().await;
}

#[tokio::test]
pub async fn create_completion() {
    let c = openai_rust::Client::new(&KEY);
    let args = openai_rust::completions::CompletionArguments::new("text-davinci-003", "The quick brown fox".to_owned());
    c.create_completion(args).await.unwrap();
}

#[tokio::test]
pub async fn create_completion_logprobs() {
    let c = openai_rust::Client::new(&KEY);
    let mut args = openai_rust::completions::CompletionArguments::new("text-davinci-003", "The quick brown fox".to_owned());
    args.logprobs = Some(1);
    c.create_completion(args).await.unwrap();
}

#[tokio::test]
pub async fn create_edit() {
    let c = openai_rust::Client::new(&KEY);
    let args = openai_rust::edits::EditArguments::new("text-davinci-edit-001", "The quick brown fox".to_owned(), "Complete this sentence.".to_owned());
    c.create_edit(args).await.unwrap();
}

#[tokio::test]
pub async fn create_embeddings() {
    let c = openai_rust::Client::new(&KEY);
    let args = openai_rust::embeddings::EmbeddingsArguments::new("text-embedding-ada-002", "The food was delicious and the waiter...".to_owned());
    c.create_embeddings(args).await.unwrap();
}
