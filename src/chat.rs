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
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature `but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// How many chat completion choices to generate for each input message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Whether to stream back partial progress.
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
///
/// It implements [Display](std::fmt::Display) as a shortcut to easily extract the content.
/// ```
/// # use serde_json;
/// # let json = "{
/// #  \"id\": \"chatcmpl-123\",
/// #  \"object\": \"chat.completion\",
/// #  \"created\": 1677652288,
/// #  \"choices\": [{
/// #    \"index\": 0,
/// #    \"message\": {
/// #     \"role\": \"assistant\",
/// #     \"content\": \"\\n\\nHello there, how may I assist you today?\"
/// #    },
/// #    \"finish_reason\": \"stop\"
/// #  }],
/// #  \"usage\": {
/// #   \"prompt_tokens\": 9,
/// #   \"completion_tokens\": 12,
/// #   \"total_tokens\": 21
/// #  }
/// # }";
/// # let res = serde_json::from_str::<openai_rust::chat::ChatCompletion>(json).unwrap();
/// let msg = &res.choices[0].message.content;
/// // or
/// let msg = res.to_string();
/// ```
#[derive(Deserialize, Debug, Clone)]
pub struct ChatCompletion {
    pub id: String,
    pub created: u32,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

impl std::fmt::Display for ChatCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.choices[0].message.content)?;
        Ok(())
    }
}

/// Structs and deserialization method for the responses
/// when using streaming chat responses.
pub mod stream {
    use bytes::Bytes;
    use futures_util::Stream;
    use serde::Deserialize;
    use std::pin::Pin;
    use std::task::Poll;
    use std::str;

    /// This is the partial chat result received when streaming.
    ///
    /// It implements [Display](std::fmt::Display) as a shortcut to easily extract the content.
    /// ```
    /// # use serde_json;
    /// # let json = "{
    /// # \"id\": \"chatcmpl-6yX67cSCIAm4nrNLQUPOtJu9JUoLG\",
    /// # \"object\": \"chat.completion.chunk\",
    /// # \"created\": 1679884927,
    /// # \"model\": \"gpt-3.5-turbo-0301\",
    /// # \"choices\": [
    /// #   {
    /// #     \"delta\": {
    /// #       \"content\": \" today\"
    /// #     },
    /// #     \"index\": 0,
    /// #     \"finish_reason\": null
    /// #   }
    /// # ]
    /// # }";
    /// # let res = serde_json::from_str::<openai_rust::chat::stream::ChatCompletionChunk>(json).unwrap();
    /// let msg = &res.choices[0].delta.content;
    /// // or
    /// let msg = res.to_string();
    /// ```
    #[derive(Deserialize, Debug, Clone)]
    pub struct ChatCompletionChunk {
        pub id: String,
        pub created: u32,
        pub model: String,
        pub choices: Vec<Choice>,
        pub system_fingerprint: Option<String>,
    }

    impl std::fmt::Display for ChatCompletionChunk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.choices[0].delta.content.as_ref().unwrap_or(&"".into())
            )?;
            Ok(())
        }
    }

    /// Choices for [ChatCompletionEvent].
    #[derive(Deserialize, Debug, Clone)]
    pub struct Choice {
        pub delta: ChoiceDelta,
        pub index: u32,
        pub finish_reason: Option<String>,
    }

    /// Additional data from [Choice].
    #[derive(Deserialize, Debug, Clone)]
    pub struct ChoiceDelta {
        pub content: Option<String>,
    }

    pub struct ChatCompletionChunkStream {
        byte_stream: Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>>>>,
        // internal buffer of incomplete completionchunks
        buf: String,
    }

    impl ChatCompletionChunkStream {

        pub(crate) fn new(stream: Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>>>>) -> Self {
            Self {
                byte_stream: stream,
                buf: String::new(),
            }
        }

        /// If possible, returns a the first deserialized chunk
        /// from the buffer.
        fn deserialize_buf(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Option<anyhow::Result<ChatCompletionChunk>> {
            // let's take the first chunk
            let bufclone = self.buf.clone();
            let mut chunks = bufclone.split("\n\n").peekable();
            let first = chunks.next();
            let second = chunks.peek();

            match first {
                Some(first) => {
                    match first.strip_prefix("data: ") {
                        Some(chunk) => {
                            if !chunk.ends_with("}") {
                                // This guard happens on partial chunks or the
                                // [DONE] marker
                                None
                            } else {
                                // If there's a second chunk, wake
                                if let Some(second) = second {
                                    if second.ends_with("}") {
                                        cx.waker().wake_by_ref();
                                    }
                                }

                                // Save the remainder
                                self.get_mut().buf = chunks.collect::<Vec<_>>().join("\n\n");
                                //self.get_mut().buf = chunks.remainder().unwrap_or("").to_owned();

                                Some(
                                    serde_json::from_str::<ChatCompletionChunk>(&chunk)
                                    .map_err(|e| anyhow::anyhow!(e))
                                )
                            }
                        },
                        None => None,
                    }
                },
                None => None,
            }
        }
    }

    impl Stream for ChatCompletionChunkStream {
        type Item = anyhow::Result<ChatCompletionChunk>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {

            // Possibly fetch a chunk from the buffer
            match self.as_mut().deserialize_buf(cx) {
                Some(chunk) => return Poll::Ready(Some(chunk)),
                None => {},
            };

            match self.byte_stream.as_mut().poll_next(cx) {
                Poll::Ready(bytes_option) => match bytes_option {
                    Some(bytes_result) => match bytes_result {
                        Ok(bytes) => {
                            // Finally actually get some bytes
                            let data = str::from_utf8(&bytes)?.to_owned();
                            self.buf = self.buf.clone() + &data;
                            match self.deserialize_buf(cx) {
                                Some(chunk) => Poll::Ready(Some(chunk)),
                                // Partial
                                None => {
                                    // On a partial, I think the best we can do is just to wake the
                                    // task again. If we don't this task will get stuck.
                                    cx.waker().wake_by_ref();
                                    Poll::Pending
                                },
                            }
                        },
                        Err(e) => Poll::Ready(Some(Err(e.into()))),
                    },
                    // Stream terminated
                    None => Poll::Ready(None),
                },
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

/// Infomration about the tokens used by [ChatCompletion].
#[derive(Deserialize, Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Completion choices from [ChatCompletion].
#[derive(Deserialize, Debug, Clone)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

/// A message.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Role of a [Message].
pub enum Role {
    System,
    Assistant,
    User,
}
