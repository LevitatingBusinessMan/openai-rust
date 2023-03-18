//! test
#![doc = include_str!("../README.md")]
use reqwest;
use lazy_static::lazy_static;
use anyhow::{anyhow, Result};


lazy_static! {
    static ref BASE_URL: reqwest::Url = reqwest::Url::parse("https://api.openai.com/v1/models").unwrap();
}


/// This is the main interface to interact with the api.
pub struct Client {
    api_key: String,
    req_client: reqwest::Client,
}


/// See <https://platform.openai.com/docs/api-reference/models>
pub mod models;

/// See <https://platform.openai.com/docs/api-reference/chat>
pub mod chat;

impl Client {

    /// Create a new client.
    /// This will automatically build a [reqwest::Client] used internally.
    pub fn new(api_key: String) -> Client {
        use reqwest::header;

        // Create the header map
        let mut headers = header::HeaderMap::new();
        let mut key_headervalue = header::HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap();
        key_headervalue.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, key_headervalue);
        let req_client = reqwest::ClientBuilder::new().default_headers(headers).build().unwrap();

        Client {
            api_key,
            req_client,
        }
    }

    /// Lists the currently available models, and provides basic information about each one such as the owner and availability.
    /// See <https://platform.openai.com/docs/api-reference/models/list>
    pub async fn list_models(&self) -> Result<models::ListModelsResponse, anyhow::Error> {
        let mut url = BASE_URL.clone();
        url.set_path("/v1/models");

        let res = self.req_client.get(url).send().await?;

        if res.status() == 200 {
            Ok(res.json::<models::ListModelsResponse>().await?)
        } else {
            Err(anyhow!(res.text().await?))
        }
    }

    /// Given a chat conversation, the model will return a chat completion response.
    /// See <https://platform.openai.com/docs/api-reference/chat>
    pub async fn create_chat(&self, args: chat::ChatArguments) -> Result<chat::ChatResponse, anyhow::Error>  {
        let mut url = BASE_URL.clone();
        url.set_path("/v1/chat/completions");

        let res = self.req_client.post(url).json(&args).send().await?;

        if res.status() == 200 {
            Ok(res.json::<chat::ChatResponse>().await?)
        } else {
            Err(anyhow!(res.text().await?))
        }  
    }
}
