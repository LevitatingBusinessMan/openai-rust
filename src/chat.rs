use serde::{Deserialize, Serialize};

/// Request arguments for chat completion.
/// 
/// See <https://platform.openai.com/docs/api-reference/chat/create>.
/// 
/// ```
/// let args = openai_rust::chat::ChatArguments::new("gpt-3.5-turbo", vec![
///     openai_rust::chat::Message {
///         role: "user".to_owned(),
///         content: "Hello GPT!".to_owned(),
///     }
/// ]);
/// ```
/// 
#[derive(Serialize, Debug, Clone)]
pub struct ChatArguments {
    pub model: String,
    pub messages: Vec<Message>,
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
    pub fn new(model: impl AsRef<str>, messages: Vec<Message>) -> ChatArguments {
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

/// This is the response of a chat.
/// ```ignore
/// let msg = res.choices[0].content;
/// ```
#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    pub created: u32,
    pub choices: Vec<Choice>,
    pub usage: Usage
}

/// Structs and deserialization method for the responses
/// when using streaming chat responses.
pub mod stream {
    use serde::Deserialize;
    use bytes::Bytes;
    use anyhow::Context;
    use std::str;

    /// This is the partial chat result received when streaming.
    /// 
    /// ```ignore
    /// msg += res.choices[0].delta.content;
    /// ```
    #[derive(Deserialize, Debug)]
    pub struct ChatResponseEvent {
        pub id: String,
        pub created: u32,
        pub model: String,
        pub choices: Vec<Choice>,
    }
    
    /// Choices for [ChatResponseEvent].
    #[derive(Deserialize, Debug)]
    pub struct Choice {
        pub delta: ChoiceDelta,
        pub index: u32,
        pub finish_reason: Option<String>,
    }
    
    /// Additional data from [Choice].
    #[derive(Deserialize, Debug)]
    pub struct ChoiceDelta {
        pub content: Option<String>,
    }

    /// Used for deserializing the event stream
    pub(crate) fn deserialize_chat_events(body: Result<Bytes, reqwest::Error>)-> Result<Vec<ChatResponseEvent>, anyhow::Error>{
        let body = body?;
        let data = str::from_utf8(&body)?.to_owned();
        
        // All events are in the form of data: {...}
        // Except the last event which is always in the form of data: [DONE]

        let events = data.split("\n\n");

        let mut responses = vec![];

        for event in events {
            if event.is_empty() {break};

            // Remove the 'data: ' to make it valid JSON
            let str = event.strip_prefix("data: ").context("Unexpected response format")?;

            if str == "[DONE]" {
                break
            }

            responses.push(serde_json::from_str::<ChatResponseEvent>(&str)?);
        }

        Ok(responses)
    }
}

/// Infomration about the tokens used by [ChatResponse].
#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Completion choices from [ChatResponse].
#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String
}

/// A message.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String
}

/// Role of a [Message].
pub enum Role {
    System,Assistant,User
}
