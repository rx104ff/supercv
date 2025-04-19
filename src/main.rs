mod models;
mod services;

use services::{message::build_resume_update_prompt, openai::send_chat_request};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let section = "Experience";
    let content = r"\item Developed an OCR system for invoice parsing.";

    let messages = build_resume_update_prompt(section, content);
    let reply = send_chat_request(messages)?;
    println!("Suggested Update:\n{}", reply);

    Ok(())
}
