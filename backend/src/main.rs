mod modules;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/open_ai_api", post(modules::open_ai_api::handler));

    let addr = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", addr.local_addr().unwrap());

    axum::serve(addr, app).await.unwrap();
}

async fn hello() -> String {
    String::from("hello!")
}
