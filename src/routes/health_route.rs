use axum::{routing::get, Json, Router};
use serde::Serialize;
use crate::infra::app_state::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub async fn health_check() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "OK",
    };
    Json(response)
}

pub fn create_health_check(state: AppState) -> Router<AppState>{
    Router::new().route("/health", get(health_check))
}