use axum::{Router, routing::get};

// https://docs.rs/axum/latest/axum/#example
#[tokio::main]
async fn main() {
    // 単一ルートでアプリケーションを構築します
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // hyper でアプリケーションを起動し、port 3000 で待機します
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
