// Here we will use the chat completion endpoint
use openai_rust;
use openai_rust::futures_util::StreamExt;
use std::io::Write;

#[tokio::main]
async fn main() {
    let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
    let args = openai_rust::chat::ChatArguments::new(
        "gpt-3.5-turbo",
        vec![openai_rust::chat::Message {
            role: openai_rust::chat::Role::User,
            content: "Hello GPT!".to_owned(),
        }],
    );
    let mut res = client.create_chat_stream(args).await.unwrap();
    while let Some(events) = res.next().await {
        for event in events.unwrap() {
            print!("{}", event);
            std::io::stdout().flush().unwrap();
        }
    }
}
