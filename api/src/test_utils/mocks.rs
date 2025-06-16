// File: src/test_utils/mocks.rs
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::application::gpt_api::GptApi;
use crate::domain::message_payload::Message;
use async_trait::async_trait;

pub struct MockGptApi {
    response: Arc<Mutex<String>>,
}

impl MockGptApi {
    #[allow(dead_code)]
    pub fn new_with_response(response: String) -> Self {
        Self {
            response: Arc::new(Mutex::new(response)),
        }
    }
}

#[async_trait]
impl GptApi for MockGptApi {
    async fn send_chat_request(&self, _messages: Vec<Message>) -> anyhow::Result<String> {
        let resp = self.response.lock().await.clone();
        Ok(resp)
    }
}
