// src/infrastructure/openai.rs
use crate::application::gpt_api::GptApi;
use crate::domain::message_payload::{ChatRequest, ChatResponse, Message};
use reqwest::Client;


pub struct OpenAiClient {
    api_key: String,
    client: Client,
}

impl OpenAiClient {
    pub fn new() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set");
        let client = Client::new();
        OpenAiClient { api_key, client }
    }
}

#[async_trait::async_trait]
impl GptApi for OpenAiClient {
    async fn send_chat_request(&self, messages: Vec<Message>) -> anyhow::Result<String> {
        let body = serde_json::json!({
            "model": "gpt-4",
            "messages": messages
        });

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let reply = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(reply)
    }
}
