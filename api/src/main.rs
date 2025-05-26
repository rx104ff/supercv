use axum::serve;
use tokio::net::TcpListener;
use crate::interface::routes::create_router;

mod domain;
mod application;
mod infrastructure;
mod interface;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = create_router();

    // Bind to TCP port first
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind port");

    println!("🚀 Server running at http://127.0.0.1:3000");

    serve(listener, app)
        .await
        .expect("Server failed");
}
