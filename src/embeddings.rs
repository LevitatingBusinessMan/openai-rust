//! See <https://platform.openai.com/docs/api-reference/embeddings>.
//! Use with [Client::create_embeddings](crate::Client::create_embeddings).

use serde::{Deserialize, Serialize};
/// Request arguments for embeddings.
///
/// See <https://platform.openai.com/docs/api-reference/embeddings/create>.
///
/// ```
/// openai_rust::embeddings::EmbeddingsArguments::new(
///     "text-embedding-ada-002",
///     "The food was delicious and the waiter...".to_owned(),
/// );
/// ```
#[derive(Serialize, Debug, Clone)]
pub struct EmbeddingsArguments {
    /// ID of the model to use. You can use the [List models](crate::Client::list_models) API to see all of your available models, or see our [Model overview](https://platform.openai.com/docs/models/overview) for descriptions of them.
    pub model: String,
    /// Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. Each input must not exceed the max input tokens for the model (8191 tokens for `text-embedding-ada-002`). [Example Python code](https://github.com/openai/openai-cookbook/blob/main/examples/How_to_count_tokens_with_tiktoken.ipynb) for counting tokens.
    pub input: String,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl EmbeddingsArguments {
    pub fn new(model: impl AsRef<str>, input: String) -> EmbeddingsArguments {
        EmbeddingsArguments {
            model: model.as_ref().to_owned(),
            input,
            user: None,
        }
    }
}

/// The response of an embeddings request.
#[derive(Deserialize, Debug, Clone)]
pub struct EmbeddingsResponse {
    pub data: Vec<EmbeddingsData>,
    pub model: String,
    pub usage: Usage,
}

/// The data from an embeddings request.
#[derive(Deserialize, Debug, Clone)]
pub struct EmbeddingsData {
    pub embedding: Vec<f32>,
    pub index: usize,
}

/// Token usage information for an [EmbeddingsResponse].
#[derive(Deserialize, Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
