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
/// To use streaming, use [crate::Client::create_chat_stream].
/// 
#[derive(Serialize, Debug, Clone)]
pub struct ChatArguments {
    /// ID of the model to use. See the model [endpoint compatibility table](https://platform.openai.com/docs/models/model-endpoint-compatibility) for details on which models work with the Chat API.
    pub model: String,
    /// The [Message]s to generate chat completions for
    pub messages: Vec<Message>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    /// 
    /// We generally recommend altering this or top_p but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    /// 
    /// We generally recommend altering this or temperature but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// How many chat completion choices to generate for each input message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stream: Option<bool>,
    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
    /// The maximum number of [tokens](https://platform.openai.com/tokenizer) to generate in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is limited by the model's context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    // logit_bias
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
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
