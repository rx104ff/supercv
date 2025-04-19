use crate::models::message_payload::{ChatRequest, ChatResponse, Message};
use reqwest::blocking::Client;
use std::env;

pub fn send_chat_request(messages: Vec<Message>) -> anyhow::Result<String> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    let request = ChatRequest {
        model: "gpt-4".to_string(),
        messages,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request)
        .send()?
        .json::<ChatResponse>()?;

    let reply = &response.choices[0].message.content;
    Ok(reply.to_string())
}
