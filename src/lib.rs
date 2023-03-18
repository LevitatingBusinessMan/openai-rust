use reqwest;
use lazy_static::lazy_static;

lazy_static! {
    static ref BASE_URL: reqwest::Url = reqwest::Url::parse("https://api.openai.com/v1/models").unwrap();
}

pub struct Client {
    api_key: String,
    req_client: reqwest::Client,
}


/// See <https://platform.openai.com/docs/api-reference/models>
pub mod models {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct ListModelsResponse {
        pub data: Vec<Model>,
        pub object: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Model {
        pub id: String,
        pub object: String,
        pub owned_by: String,
        pub permission: Vec<ModelPermission>
    }

    #[derive(Deserialize, Debug)]
    pub struct ModelPermission {
        pub id: String,
        pub object: String,
        pub created: u32,
        pub allow_create_engine: bool,
        pub allow_sampling: bool,
        pub allow_logprobs: bool,
        pub allow_search_indices: bool,
        pub allow_view: bool,
        pub allow_fine_tuning: bool,
        pub organization: String,
        pub group: Option<String>,
        pub is_blocking: bool,
    }
}

impl Client {

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
    pub async fn list_models(&self) -> Result<models::ListModelsResponse, reqwest::Error> {
        let mut url = BASE_URL.clone();
        url.set_path("/v1/models");

        match self.req_client.get(url).send().await {
            Ok(res) => {
                res.json::<models::ListModelsResponse>().await
            },
            Err(err) => {
                Err(err)
            }
        }
    }
}
