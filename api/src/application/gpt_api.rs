// File: src/application/gpt_api.rs
use crate::domain::models::message_payload::Message;

#[async_trait::async_trait]
pub trait GptApi: Send + Sync {
    async fn send_chat_request(&self, messages: Vec<Message>) -> anyhow::Result<String>;
}
