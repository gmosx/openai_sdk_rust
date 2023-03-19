//! <https://platform.openai.com/docs/api-reference/completions/create>

use serde::{Deserialize, Serialize};

use crate::client::{Request, DEFAULT_MODEL};

// #TODO extract model field?
// #TODO have different default model per request type.

#[derive(Default, Clone, Serialize)]
pub struct CreateCompletionRequest {
    pub model: String,
    // #TODO also support array-of-strings prompt.
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
}

impl Request for CreateCompletionRequest {
    type Response = CreateCompletionResponse;

    fn path(&self) -> String {
        "/v1/completions".to_string()
    }
}

impl CreateCompletionRequest {
    pub fn new(prompt: &str) -> Self {
        Self {
            model: DEFAULT_MODEL.to_owned(),
            prompt: prompt.to_owned(),
            ..Default::default()
        }
    }

    pub fn model(self, model: &str) -> Self {
        Self {
            model: model.to_owned(),
            ..self
        }
    }

    pub fn suffix(self, suffix: &str) -> Self {
        Self {
            suffix: Some(suffix.to_owned()),
            ..self
        }
    }

    pub fn temperature(self, temperature: f64) -> Self {
        Self {
            temperature: Some(temperature),
            ..self
        }
    }

    pub fn max_tokens(self, max_tokens: i32) -> Self {
        Self {
            max_tokens: Some(max_tokens),
            ..self
        }
    }

    pub fn n(self, n: i32) -> Self {
        Self { n: Some(n), ..self }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: i32,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: i64,
    pub prompt_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}
