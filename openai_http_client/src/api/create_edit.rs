//! <https://platform.openai.com/docs/api-reference/edits/create>

use serde::{Deserialize, Serialize};

use crate::client::Request;

pub const DEFAULT_MODEL: &str = "text-davinci-edit-001";

// #TODO extract model field?
// #TODO have different default model per request type.

#[derive(Default, Clone, Serialize)]
pub struct CreateEditRequest {
    pub model: String,
    pub input: String,
    pub instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
}

impl Request for CreateEditRequest {
    type Response = CreateEditResponse;

    fn path(&self) -> String {
        "/v1/edits".to_string()
    }
}

impl CreateEditRequest {
    pub fn new(input: &str, instruction: &str) -> Self {
        Self {
            model: DEFAULT_MODEL.to_owned(),
            input: input.to_owned(),
            instruction: instruction.to_owned(),
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

    pub fn n(self, n: i32) -> Self {
        Self { n: Some(n), ..self }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
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
pub struct CreateEditResponse {
    pub object: String,
    pub created: i64,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}
