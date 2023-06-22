#![doc = include_str!("../README.md")]
use anyhow::{anyhow, Result};
use futures_util::stream::Stream;
use futures_util::StreamExt;
use lazy_static::lazy_static;
use reqwest;
#[cfg(feature = "middleware")]
use reqwest_middleware;

pub extern crate futures_util;

/// See <https://platform.openai.com/docs/api-reference/models>.
pub mod models;

/// See <https://platform.openai.com/docs/api-reference/chat>.
pub mod chat;

/// See <https://platform.openai.com/docs/api-reference/completions>.
pub mod completions;

/// See <https://platform.openai.com/docs/api-reference/edits>.
pub mod edits;

/// See <https://platform.openai.com/docs/api-reference/embeddings>.
pub mod embeddings;

lazy_static! {
    static ref BASE_URL: reqwest::Url =
        reqwest::Url::parse("https://api.openai.com/v1/models").unwrap();
}

/// This is the main interface to interact with the api.
pub struct Client {
    req_client: ReqwestClient,
    key: String,
}

#[cfg(feature = "middleware")]
/// This type is is a [reqwest_middleware::ClientWithMiddleware]. To use a normal [reqwest::Client] instead, disable the middleware feature.
pub type ReqwestClient = reqwest_middleware::ClientWithMiddleware;
#[cfg(feature = "middleware")]
type ReqwestClientBuilder = reqwest_middleware::ClientBuilder;


#[cfg(not(feature = "middleware"))]
/// This type is is a [reqwest::Client]. To use a client that supports adding middleware, enable the middleware feature.
pub type ReqwestClient = reqwest::Client;
#[cfg(not(feature = "middleware"))]
type ReqwestClientBuilder = reqwest::ClientBuilder;

impl Client {
    /// Create a new client.
    /// This will automatically build a [ReqwestClient] used internally.
    pub fn new(api_key: &str) -> Client {
        #[cfg(not(feature = "middleware"))]
        let req_client = ReqwestClientBuilder::new().build().unwrap();
        #[cfg(feature = "middleware")]
        let req_client = ReqwestClientBuilder::new(reqwest::Client::new()).build();
        Client {
            req_client,
            key: api_key.to_owned(),
        }
    }

    /// Build a client using your own [ReqwestClient].
    pub fn new_with_client(api_key: &str, req_client: ReqwestClient) -> Client {
        Client {
            req_client,
            key: api_key.to_owned(),
        }
    }

    /// List and describe the various models available in the API. You can refer to the [Models](https://platform.openai.com/docs/models) documentation to understand what models are available and the differences between them.
    ///
    /// ```no_run
    /// # let api_key = "";
    /// # tokio_test::block_on(async {
    /// let client = openai_rust::Client::new(api_key);
    /// let models = client.list_models().await.unwrap();
    /// # })
    /// ```
    ///
    /// See <https://platform.openai.com/docs/api-reference/models/list>.
    pub async fn list_models(&self) -> Result<Vec<models::Model>, anyhow::Error> {
        let mut url = BASE_URL.clone();
        url.set_path("/v1/models");

        let res = self
            .req_client
            .get(url)
            .bearer_auth(&self.key)
            .send()
            .await?;

        if res.status() == 200 {
            Ok(res.json::<models::ListModelsResponse>().await?.data)
        } else {
            Err(anyhow!(res.text().await?))
        }
    }

    /// Given a list of messages comprising a conversation, the model will return a response.
    ///
    /// See <https://platform.openai.com/docs/api-reference/chat>.
    /// ```no_run
    /// # use tokio_test;
    /// # tokio_test::block_on(async {
    /// # use openai_rust;
    /// # let api_key = "";
    /// let client = openai_rust::Client::new(api_key);
    /// let args = openai_rust::chat::ChatArguments::new("gpt-3.5-turbo", vec![
    ///    openai_rust::chat::Message {
    ///        role: "user".to_owned(),
    ///        content: "Hello GPT!".to_owned(),
    ///    }
    /// ]);
    /// let res = client.create_chat(args).await.unwrap();
    /// println!("{}", res.choices[0].message.content);
    /// # })
    /// ```
    pub async fn create_chat(
        &self,
        args: chat::ChatArguments,
    ) -> Result<chat::ChatResponse, anyhow::Error> {
        let mut url = BASE_URL.clone();
        url.set_path("/v1/chat/completions");

        let res = self
            .req_client
            .post(url)
            .bearer_auth(&self.key)
            .json(&args)
            .send()
            .await?;

        if res.status() == 200 {
            Ok(res.json::<chat::ChatResponse>().await?)
        } else {
            Err(anyhow!(res.text().await?))
        }
    }

    /// Like [Client::create_chat] but with streaming.
    ///
    /// See <https://platform.openai.com/docs/api-reference/chat>.
    ///
    /// This method will return a stream. Calling [next](StreamExt::next) on it will return a vector of [chat::stream::ChatResponseEvent]s.
    ///
    /// ```no_run
    /// # use tokio_test;
    /// # tokio_test::block_on(async {
    /// # use openai_rust;
    /// # use std::io::Write;
    /// # let client = openai_rust::Client::new("");
    /// # let args = openai_rust::chat::ChatArguments::new("gpt-3.5-turbo", vec![
    /// #    openai_rust::chat::Message {
    /// #        role: "user".to_owned(),
    /// #        content: "Hello GPT!".to_owned(),
    /// #    }
    /// # ]);
    /// use openai_rust::futures_util::StreamExt;
    /// let mut res = client.create_chat_stream(args).await.unwrap();
    /// while let Some(events) = res.next().await {
    ///     for event in events.unwrap() {
    ///         print!("{}", event.choices[0].delta.content.as_ref().unwrap_or(&"".to_owned()));
    ///         std::io::stdout().flush().unwrap();
    ///     }
    /// }
    /// # })
    /// ```
    ///
    pub async fn create_chat_stream(
        &self,
        args: chat::ChatArguments,
    ) -> Result<impl Stream<Item = Result<Vec<chat::stream::ChatResponseEvent>>>> {
        let mut url = BASE_URL.clone();
        url.set_path("/v1/chat/completions");

        // Enable streaming
        let mut args = args;
        args.stream = Some(true);

        let res = self
            .req_client
            .post(url)
            .bearer_auth(&self.key)
            .json(&args)
            .send()
            .await?;

        if res.status() == 200 {
            let stream = res.bytes_stream();
            let stream = stream.map(chat::stream::deserialize_chat_events);
            Ok(stream)
        } else {
            Err(anyhow!(res.text().await?))
        }
    }

    /// Given a prompt, the model will return one or more predicted completions, and can also return the probabilities of alternative tokens at each position.
    ///
    /// See <https://platform.openai.com/docs/api-reference/completions>
    ///
    /// ```no_run
    /// # use openai_rust::*;
    /// # use tokio_test;
    /// # tokio_test::block_on(async {
    /// # let api_key = "";
    /// let c = openai_rust::Client::new(api_key);
    /// let args = openai_rust::completions::CompletionArguments::new("text-davinci-003", "The quick brown fox".to_owned());
    /// println!("{}", c.create_completion(args).await.unwrap().choices[0].text);
    /// # })
    /// ```
    pub async fn create_completion(
        &self,
        args: completions::CompletionArguments,
    ) -> Result<completions::CompletionResponse> {
        let mut url = BASE_URL.clone();
        url.set_path("/v1/completions");

        let res = self
            .req_client
            .post(url)
            .bearer_auth(&self.key)
            .json(&args)
            .send()
            .await?;

        if res.status() == 200 {
            Ok(res.json::<completions::CompletionResponse>().await?)
        } else {
            Err(anyhow!(res.text().await?))
        }
    }

    /// Given a prompt and an instruction, the model will return an edited version of the prompt.
    ///
    /// See <https://platform.openai.com/docs/api-reference/edits>
    ///
    /// ```no_run
    /// # use openai_rust;
    /// # use tokio_test;
    /// # tokio_test::block_on(async {
    /// # let api_key = "";
    /// let c = openai_rust::Client::new(api_key);
    /// let args = openai_rust::edits::EditArguments::new("text-davinci-edit-001", "The quick brown fox".to_owned(), "Complete this sentence.".to_owned());
    /// println!("{}", c.create_edit(args).await.unwrap().to_string());
    /// # })
    /// ```
    ///
    pub async fn create_edit(&self, args: edits::EditArguments) -> Result<edits::EditResponse> {
        let mut url = BASE_URL.clone();
        url.set_path("/v1/edits");

        let res = self
            .req_client
            .post(url)
            .bearer_auth(&self.key)
            .json(&args)
            .send()
            .await?;

        if res.status() == 200 {
            Ok(res.json::<edits::EditResponse>().await?)
        } else {
            Err(anyhow!(res.text().await?))
        }
    }

    /// Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
    ///
    /// See <https://platform.openai.com/docs/api-reference/embeddings>
    ///
    /// ```no_run
    /// # use openai_rust;
    /// # use tokio_test;
    /// # tokio_test::block_on(async {
    /// # let api_key = "";
    /// let c = openai_rust::Client::new(api_key);
    /// let args = openai_rust::embeddings::EmbeddingsArguments::new("text-embedding-ada-002", "The food was delicious and the waiter...".to_owned());
    /// println!("{:?}", c.create_embeddings(args).await.unwrap().data);
    /// # })
    /// ```
    ///
    pub async fn create_embeddings(
        &self,
        args: embeddings::EmbeddingsArguments,
    ) -> Result<embeddings::EmbeddingsResponse> {
        let mut url = BASE_URL.clone();
        url.set_path("/v1/embeddings");

        let res = self
            .req_client
            .post(url)
            .bearer_auth(&self.key)
            .json(&args)
            .send()
            .await?;

        if res.status() == 200 {
            Ok(res.json::<embeddings::EmbeddingsResponse>().await?)
        } else {
            Err(anyhow!(res.text().await?))
        }
    }
}
