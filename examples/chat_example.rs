// Here we will use the chat completion endpoint
use openai_rust;

#[tokio::main]
async fn main() {
    let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
    let args = openai_rust::chat::ChatArguments::new(
        "gpt-3.5-turbo",
        vec![openai_rust::chat::Message {
            role: "user".to_owned(),
            content: "Hello GPT!".to_owned(),
        }],
    );
    let res = client.create_chat(args).await.unwrap();
    println!("{}", res);
}
