use std::time::Duration;

use axum::{
    Router, routing::{get, post}
};
use axum::body::Bytes;
use axum::extract::MatchedPath;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::response::Response;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

use crate::infra::app_state::AppState;
use super::super::handlers;

pub fn make_orders_route(state: AppState) -> Router<AppState> {
    let router = Router::new()
        .route("/orders/:id", get(handlers::orders_handlers::get_list_by_table_id).delete(handlers::orders_handlers::delete_order))
        .route("/orders", post(handlers::orders_handlers::create_order))
        //.route("/health", get(health_check()))
        .with_state(state)
        .fallback((StatusCode::NOT_FOUND, "The requested resource was not found"))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    },
                ),
    );
    router
}
