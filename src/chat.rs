use serde::{Deserialize, Serialize};

/// Request arguments for chat completion
/// See <https://platform.openai.com/docs/api-reference/chat/create>
#[derive(Serialize, Debug)]
pub struct ChatArguments {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    // logit_bias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ChatArguments {
    pub fn new(model: impl AsRef<str>, messages: Vec<ChatMessage>) -> ChatArguments {
        ChatArguments {
            model: model.as_ref().to_owned(),
            messages,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            user: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub choices: Vec<Choice>,
    pub usage: ChatUsage
}

#[derive(Deserialize, Debug)]
pub struct ChatUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String
}

pub enum Role {
    System,Assistant,User
}
