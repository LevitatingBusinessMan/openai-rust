use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct CompletionArguments {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,
    //logit_bias
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

#[derive(Deserialize, Debug)]
pub struct CompletionResponse {
    pub id: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<LogProbs>,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
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
