use crate::domain::message_payload::{ChatRequest, ChatResponse, Message};
use reqwest::Client;
use std::env;

pub async fn send_chat_request(messages: Vec<Message>) -> anyhow::Result<String> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    let body = serde_json::json!({
        "model": "gpt-4",
        "messages": messages
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
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
