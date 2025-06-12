use axum::serve;
use tokio::net::TcpListener;
use crate::interface::routes::create_router;
use std::sync::Arc;

mod domain;
mod application;
mod infrastructure;
mod interface;
mod test_utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let state = Arc::new(infrastructure::openai::OpenAiClient::new());
    let app = create_router(state);

    // Bind to TCP port first
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind port");

    println!("🚀 Server running at http://127.0.0.1:3000");

    serve(listener, app)
        .await
        .expect("Server failed");
}
