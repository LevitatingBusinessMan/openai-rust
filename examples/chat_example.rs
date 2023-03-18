// Here we will use the chat completion endpoint
use openai_rs;

#[tokio::main]
async fn main() {
    let client = openai_rs::Client::new(std::env::var("OPENAI_API_KEY").unwrap());
    let args = openai_rs::chat::ChatArguments::new("gpt-3.5-turbo", vec![
        openai_rs::chat::ChatMessage {
            role: "user".to_owned(),
            content: "Hello GPT!".to_owned(),
        }
    ]);
    let res = client.create_chat(args).await.unwrap();
    println!("{}", res.choices[0].message.content);
}
