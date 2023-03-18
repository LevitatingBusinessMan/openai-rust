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
