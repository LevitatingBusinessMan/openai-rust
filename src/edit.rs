use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct EditArguments {
    /// ID of the model to use. You can use the `text-davinci-edit-001` or `code-davinci-edit-001` model with this endpoint.
    pub model: String,

    /// The input text to use as a starting point for the edit.
    pub input: Option<String>,

    /// The instruction that tells the model how to edit the prompt.
    pub instruction: String,

    //How many edits to generate for the input and instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    
    /// What sampling temperature to use, between 0 and 2.
    /// Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    /// 
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    
    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    /// 
    /// We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

impl EditArguments {
    pub fn new(model: impl AsRef<str>, input: String, instruction: String) -> EditArguments {
        EditArguments {
            model: model.as_ref().to_owned(),
            input: Some(input),
            instruction: instruction,
            n: None,
            temperature: None,
            top_p: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct EditResponse {
    pub created: u32,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Choice {
    pub text: String,
    pub index: u32,
}

/// Infomration about the tokens used by [EditResponse].
#[derive(Deserialize, Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
