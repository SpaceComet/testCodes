use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/plain_text", get(plain_text))
        .route("/json", get(json))
        .route("/say_hello_to", get(say_hello_to))
        .route("/:version/*path", get(extract_path))
        .route("/test/test/test", get(plain_text));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ---- Responses ----

// Plain text response
async fn plain_text() -> &'static str {
    "test"
}

// JSON response
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

// ---- Extractors ----

// Plain text response
async fn say_hello_to(Query(params): Query<HashMap<String, String>>) -> String {
    if let Some(name) = params.get("name") {
        format!("Hello {}", name)
    } else {
        "Please type a name".to_string()
    }
}

// Path extractor. Plain text response.
async fn extract_path(Path((version, api_path)): Path<(String, String)>) -> String {
    format!("version: {}, \npath: {}", version, api_path)
}
