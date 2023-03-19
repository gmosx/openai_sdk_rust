//! <https://platform.openai.com/docs/api-reference>

use reqwest::header;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::error::Error;

pub const DEFAULT_BASE_URL: &str = "https://api.openai.com";
pub const DEFAULT_MODEL: &str = "gpt-3.5-turbo";
pub const DEFAULT_USER_AGENT: &str = "rust-openai-sdk";

pub type Result<T> = std::result::Result<T, Error>;

pub trait Request {
    type Response;
    fn path(&self) -> String;
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

// #TODO add support for timeout.
// #TODO add tracing

#[derive(Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    user_agent: Option<String>,
    api_secret: Option<String>,
    http_client: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_string());
        self
    }

    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    pub fn api_key(mut self, api_secret: &str) -> Self {
        self.api_secret = Some(api_secret.to_string());
        self
    }

    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn build(self) -> Client {
        // #TODO handle the unwrap
        Client {
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            user_agent: self
                .user_agent
                .unwrap_or_else(|| DEFAULT_USER_AGENT.to_string()),
            api_key: self.api_secret.expect("missing API secret key"),
            http_client: self
                .http_client
                .unwrap_or_else(|| reqwest::Client::builder().build().unwrap()),
        }
    }
}

/// A client for the OpenAI HTTP API.
pub struct Client {
    base_url: String,
    user_agent: String,
    api_key: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        ClientBuilder::default().api_key(api_key).build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn call<Req>(&self, request: Req) -> Result<Req::Response>
    where
        Req: Request + Serialize,
        Req::Response: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, request.path());

        let json = serde_json::to_string(&request)?;

        let response = self
            .http_client
            .post(&url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, &self.user_agent)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(json.into_bytes())
            .send()
            .await?;

        self.parse_response(response).await
    }

    async fn parse_response<Resp>(&self, response: reqwest::Response) -> Result<Resp>
    where
        Resp: DeserializeOwned,
    {
        let status = response.status();

        if status == 200 {
            let response: Resp = response.json().await?;
            Ok(response)
        } else {
            let response: ApiErrorResponse = response.json().await?;
            Err(Error::Api(format!(
                "Status {}: {}",
                status, response.error.message,
            )))
        }
    }
}
