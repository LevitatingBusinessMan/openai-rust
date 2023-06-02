use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Request arguments for completions.
/// 
/// See <https://platform.openai.com/docs/api-reference/completions/create>.
/// 
/// ```
/// let args = openai_rust::completions::CompletionArguments::new(
///     "text-davinci-003",
///     "The quick brown fox".to_owned()
/// );
/// ```
#[derive(Serialize, Debug, Clone)]
pub struct CompletionArguments {
    /// ID of the model to use.
    /// You can use the [List models](crate::Client::list_models) API to see all of your available models,
    /// or see our [Model overview](https://platform.openai.com/docs/models/overview) for descriptions of them.
    pub model: String,
    
    /// The prompt(s) to generate completions for,
    /// encoded as a string, array of strings, array of tokens,
    /// or array of token arrays.
    /// 
    /// Defaults to <|endoftext|>.
    /// 
    /// Note that <|endoftext|> is the document separator that the model
    /// sees during training, so if a prompt is not specified the model
    /// will generate as if from the beginning of a new document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
  
    /// The suffix that comes after a completion of inserted text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    
    /// The maximum number of [tokens](https://platform.openai.com/tokenizer) to generate in the chat completion.
    /// 
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    /// Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    
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
    
    /// How many completions to generate for each prompt.
    /// 
    /// *Note:* Because this parameter generates many completions,it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stream: Option<bool>,
    
    /// Include the log probabilities on the `logprobs` most likely tokens,
    /// as well the chosen tokens. For example, if `logprobs` is 5,
    /// the API will return a list of the 5 most likely tokens.
    /// The API will always return the `logprob` of the sampled token,
    /// so there may be up to `logprobs+1` elements in the response.
    /// 
    /// The maximum value for `logprobs` is 5.
    /// If you need more than this, please contact us through our [Help center](https://help.openai.com/) and describe your use case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u8>,
    
    /// Echo back the prompt in addition to the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    
    /// Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
    
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    /// 
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    /// 
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    
    /// Generates `best_of` completions server-side and returns the "best" (the one with the highest log probability per token).
    /// Results cannot be streamed.
    /// 
    /// When used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return – `best_of` must be greater than `n`.
    /// 
    /// *Note:* Because this parameter generates many completions,it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for max_tokens` and `stop`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,
    
    //logit_bias
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl CompletionArguments {
    pub fn new(model: impl AsRef<str>, prompt: String) -> CompletionArguments {
        CompletionArguments {
            model: model.as_ref().to_owned(),
            prompt: Some(prompt),
            suffix: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            user: None,
            
        }
    }
}

/// The repsonse of a completion request.
/// 
/// It implements [Display](std::fmt::Display) as a shortcut to easily extract the content.
/// ```
/// # use serde_json;
/// # let json = "{
/// #  \"id\": \"cmpl-uqkvlQyYK7bGYrRHQ0eXlWi7\",
/// #  \"object\": \"text_completion\",
/// #  \"created\": 1589478378,
/// #  \"model\": \"text-davinci-003\",
/// #  \"choices\": [
/// #    {
/// #      \"text\": \"\\n\\nThis is indeed a test\",
/// #      \"index\": 0,
/// #      \"logprobs\": null,
/// #      \"finish_reason\": \"length\"
/// #    }
/// #  ],
/// #  \"usage\": {
/// #    \"prompt_tokens\": 5,
/// #    \"completion_tokens\": 7,
/// #    \"total_tokens\": 12
/// #  }
/// # }";
/// # let res = serde_json::from_str::<openai_rust::completions::CompletionResponse>(json).unwrap();
/// let text = &res.choices[0].text;
/// // or
/// let text = res.to_string();
/// ```
#[derive(Deserialize, Debug, Clone)]
pub struct CompletionResponse {
    pub id: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choice>,
}

impl std::fmt::Display for CompletionResponse {
  /// Automatically grab the first choice
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.choices[0].text)?;
        Ok(())
    }
}

/// The completion choices of a completion response. 
#[derive(Deserialize, Debug, Clone)]
pub struct Choice {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<LogProbs>,
    pub finish_reason: String,
}

/// The log probabilities of a completion response.
#[derive(Deserialize, Debug, Clone)]
pub struct LogProbs {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<f32>,
    pub top_logprobs: Vec<HashMap<String, f32>>,
    pub text_offset: Vec<u32>,
}

/*
{
  "logprobs": {
    "tokens": [
      "\"",
      "\n",
      "\n",
      "The",
      " quick",
      " brown",
      " fox",
      " jumped",
      " over",
      " the",
      " lazy",
      " dog",
      "."
    ],
    "token_logprobs": [
      -3.4888523,
      -0.081398554,
      -0.27080205,
      -0.010607235,
      -0.03842781,
      -0.00033003604,
      -0.00006468596,
      -0.8200931,
      -0.0002035838,
      -0.00010665305,
      -0.0003372524,
      -0.002368947,
      -0.0031320814
    ],
    "top_logprobs": [
      {
        "\n": -1.016303
      },
      {
        "\n": -0.081398554
      },
      {
        "\n": -0.27080205
      },
      {
        "The": -0.010607235
      },
      {
        " quick": -0.03842781
      },
      {
        " brown": -0.00033003604
      },
      {
        " fox": -0.00006468596
      },
      {
        " jumps": -0.58238596
      },
      {
        " over": -0.0002035838
      },
      {
        " the": -0.00010665305
      },
      {
        " lazy": -0.0003372524
      },
      {
        " dog": -0.002368947
      },
      {
        ".": -0.0031320814
      }
    ],
    "text_offset": [
      13,
      14,
      15,
      16,
      19,
      25,
      31,
      35,
      42,
      47,
      51,
      56,
      60
    ]
  }
}
*/
