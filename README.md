# openai-rust

[![Test Status](https://github.com/LevitatingBusinessMan/openai-rust/workflows/Build/badge.svg)](https://github.com/LevitatingBusinessMan/openai-rust/actions)
[![Crates.io](https://img.shields.io/crates/v/openai-rust)](https://crates.io/crates/openai-rust)
[![docs.rs](https://img.shields.io/docsrs/openai-rust)](https://docs.rs/openai-rust/latest/openai_rust/)


This is an unofficial library to interact with the [Openai-API](https://platform.openai.com/docs/api-reference). The goal of this crate is to support the entire api.

#### Current features:
- [x] [Listing models](https://platform.openai.com/docs/api-reference/models/list)
- [x] [Completions](https://platform.openai.com/docs/api-reference/completions/create)
- [x] [Chat](https://platform.openai.com/docs/api-reference/chat/create)
- [x] [Streaming Chat](https://platform.openai.com/docs/api-reference/chat/create#chat/create-stream)
- [x] [Edit](https://platform.openai.com/docs/api-reference/edits/create)
- [x] [Embeddings](https://platform.openai.com/docs/api-reference/embeddings/create)
- [ ] Images
- [ ] Audio
- [ ] Files
- [ ] Moderations
- [ ] Engines
- [ ] Fine-tunes

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
println!("{}", res);
```

You can run this code as an example with `OPENAI_API_KEY=(your key) cargo run --example chat`.

Checkout the examples directory for more usage examples. You can find documentation on [docs.rs](https://docs.rs/openai-rust/latest/openai_rust/).

### Projects using openai-rust
* [openai-cli](https://github.com/LevitatingBusinessMan/openai-cli): a CLI for interacting with GPT.
* [gpt-cli-rust](https://github.com/memochou1993/gpt-cli-rust): Another CLI.
* [electocracy](https://github.com/marioloko/electocracy): A digital voting system.
* [awsgpt](https://github.com/fizlip/awsgpt): Interact with the aws-cli via GPT.
