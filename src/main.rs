mod models;
mod services;
use services::prompt::generate_one_shot_prompt;

use services::{message::build_resume_update_prompt, openai::send_chat_request};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let resume = std::fs::read_to_string("resume.txt")?;
    let job_description = std::fs::read_to_string("job.txt")?;
    let note = Some("Focus on emphasizing open source and Rust experience.");

    let messages = generate_one_shot_prompt(&resume, &job_description, note);
    let response = send_chat_request(messages)?;

    std::fs::write("resume_updated.tex", response)?;
    println!("Updated resume written to resume_updated.tex");

    Ok(())
}
