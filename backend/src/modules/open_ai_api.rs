use serde::{Deserialize, Serialize};

use axum::Json;

#[derive(Deserialize)]
pub struct Input {
    name: String,
    age: u32,
}

#[derive(Serialize)]
pub struct Output {
    message: String,
}

// POST handler
pub async fn handler(Json(payload): Json<Input>) -> Json<Output> {
    let message = format!("Hello {}, age {}", payload.name, payload.age);

    Json(Output { message })
}
