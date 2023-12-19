//! See <https://platform.openai.com/docs/api-reference/images>.
//! Use with [Client::create_image](crate::Client::create_image).

use serde::{Deserialize, Serialize};

/// The format in which the generated images are returned.
#[derive(Serialize, Debug, Clone)]
pub enum ResponseFormat {
    Url,
    Base64JSON,
}

#[derive(Serialize, Debug, Clone)]
pub struct ImageArguments {
    /// A text description of the desired image(s). The maximum length is 1000 characters.
    pub prompt: String,
    /// The number of images to generate. Must be between 1 and 10. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// The format in which the generated images are returned Defaults to `url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`. Defaults to `1024x1024`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ImageArguments {
    pub fn new(prompt: impl AsRef<str>) -> Self {
        Self {
            prompt: prompt.as_ref().to_owned(),
            n: None,
            response_format: None,
            size: None,
            user: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) enum ImageObject {
    #[serde(alias = "url")]
    Url(String),
    #[serde(alias = "b64_json")]
    Base64JSON(String), 
}

#[derive(Deserialize, Debug)]
pub(crate) struct ImageResponse {
    #[allow(dead_code)]
    created: u32,
    pub data: Vec<ImageObject>,
}
