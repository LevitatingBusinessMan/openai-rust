# openai-rust
This is a library to interact with the [Openai-API](https://platform.openai.com/docs/api-reference). The goal of this crate is to support the entire api.

### Example usage
```rust ignore
// Here we will use the chat completion endpoint
let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
let args = openai_rust::chat::ChatArguments::new("gpt-3.5-turbo", vec![
    openai_rust::chat::Message {
        role: "user".to_owned(),
        content: "Hello GPT!".to_owned(),
    }
]);
let res = client.create_chat(args).await.unwrap();
println!("{}", res.choices[0].message.content);
```

You can run this code as an example with `OPENAI_API_KEY=(your key) cargo run --example chat`.

Checkout the examples directory for more usage examples.

### Projects using openai-rust
* [openai-cli](https://github.com/LevitatingBusinessMan/openai-cli), a CLI for interacting with GPT.
