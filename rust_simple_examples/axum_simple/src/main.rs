use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/plain_text", get(plain_text))
        .route("/json", get(json));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn plain_text() -> &'static str {
    "test"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
