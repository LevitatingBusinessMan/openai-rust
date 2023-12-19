//! See <https://platform.openai.com/docs/api-reference/models>.
//! Use with [Client::list_models](crate::Client::list_models).
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct ListModelsResponse {
    pub data: Vec<Model>,
}

/// Describes an OpenAI model offering that can be used with the API.
#[derive(Deserialize, Debug)]
pub struct Model {
    /// The model identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The organization that owns the model.
    pub owned_by: String,
    /// The Unix timestamp (in seconds) when the model was created.
    pub created: u64,
}

// /// Permissions of a model
// #[derive(Deserialize, Debug)]
// pub struct ModelPermission {
//     pub id: String,
//     pub created: u32,
//     pub allow_create_engine: bool,
//     pub allow_sampling: bool,
//     pub allow_logprobs: bool,
//     pub allow_search_indices: bool,
//     pub allow_view: bool,
//     pub allow_fine_tuning: bool,
//     pub organization: String,
//     pub group: Option<String>,
//     pub is_blocking: bool,
// }
