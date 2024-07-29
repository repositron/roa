use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::handlers::handler_errors::InfraError;

#[derive(Default,Debug,Queryable,Identifiable,Insertable,Serialize,Deserialize, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::orders)]
#[serde(rename_all = "camelCase")]
pub struct OrderModel {
    pub id: Uuid,
    pub table_id: i32,
    pub item: String,
    pub duration: i32,
    pub expire_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub enum OrderError {
    InternalServerError,
    UniqueViolation,
    NotFound(Uuid),
    InfraError(InfraError),
}

impl IntoResponse for OrderError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::UniqueViolation => (
                StatusCode::CONFLICT,
                format!("OrderModel UniqueViolation"),
            ),
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("PostModel with id {} has not been found", id),
            ),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"PostModel", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}