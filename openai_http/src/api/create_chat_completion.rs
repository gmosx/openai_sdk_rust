//! <https://platform.openai.com/docs/guides/chat>

use serde::{Deserialize, Serialize};

use crate::{
    client::{Request, DEFAULT_MODEL},
    types::Message,
};

// #TODO extract model field?

#[derive(Default, Clone, Serialize)]
pub struct CreateChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
}

impl Request for CreateChatCompletionRequest {
    type Response = CreateChatCompletionResponse;

    fn path(&self) -> String {
        "/v1/chat/completions".to_string()
    }
}

impl CreateChatCompletionRequest {
    pub fn new(messages: &[Message]) -> Self {
        Self {
            model: DEFAULT_MODEL.to_owned(),
            messages: messages.to_vec(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: i32,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: i64,
    pub prompt_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}
