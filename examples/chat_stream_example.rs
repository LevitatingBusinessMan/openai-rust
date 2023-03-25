// Here we will use the chat completion endpoint
use openai_rust;
use openai_rust::futures_util::StreamExt;
use std::io::Write;

#[tokio::main]
async fn main() {
    let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
    let args = openai_rust::chat::ChatArguments::new("gpt-3.5-turbo", vec![
        openai_rust::chat::Message {
            role: "user".to_owned(),
            content: "Hello GPT!".to_owned(),
        }
    ]);
    let mut res = client.create_chat_stream(args).await.unwrap();
    while let Some(item) = res.next().await {
        for event in item.unwrap() {
            print!("{}", event.choices[0].delta.content.as_ref().unwrap_or(&"".to_owned()));
            std::io::stdout().flush().unwrap();
        }
    }
}
