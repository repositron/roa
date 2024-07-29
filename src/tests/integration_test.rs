use axum::{http, Router};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum_test::TestServer;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::{Service, ServiceExt};
use uuid::Uuid;

use crate::infra::app_state::create_app_state;
use crate::infra::db::create_db_pool;
use crate::models::order_model::OrderModel;
use crate::routes::orders_route::make_orders_route;

async fn create_orders_test_app() -> Router {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    let db_pool = create_db_pool(&database_url).await;
    let app_state = create_app_state(db_pool);
    let app = make_orders_route(app_state.clone())
        .with_state(app_state.clone());
    return app;
}

#[tokio::test]
async fn when_an_order_is_posted_then_it_returns_the_complete_order_model() {

    let uuid = Uuid::new_v4().to_string();
    let order_body = json!({
        "id": uuid,
        "tableId": 55,
        "item": "food",
    });

    let response = create_orders_test_app()
        .await
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/orders")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&order_body).unwrap()
                ))
                .unwrap()
        )
        .await
        .unwrap();
        //.expect("failed to create test server");



    assert_eq!(response.status(), StatusCode::OK);
   // assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    let response_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&response_bytes).unwrap();

    assert_eq!(body["id"].as_str().unwrap(), uuid);
    assert_eq!(body["tableId"].as_u64().unwrap(), 55);
    let created_at_str = body["createdAt"].as_str().expect("createdAt field not found");
    let created_at: NaiveDateTime = NaiveDateTime::parse_from_str(created_at_str, "%Y-%m-%dT%H:%M:%S.%f")
        .expect("Invalid createdAt format");

    let created_at_utc: DateTime<Utc> = created_at.and_utc();

    let now = Utc::now();
    let acceptable_range = Duration::seconds(5);

    assert!(
        created_at_utc > now - acceptable_range && created_at_utc < now + acceptable_range,
        "createdAt is not within the acceptable range"
    );

}

#[tokio::test]
async fn when_an_order_is_reposted_with_same_uuid_then_it_fails() {

    let uuid = Uuid::new_v4().to_string();
    let order_body = json!({
        "id": uuid,
        "tableId": 55,
        "item": "food",
    });

    let app = create_orders_test_app().await;

    let server = TestServer::new(app).expect("testserver not intialized");

    let response = server
        .post("/orders")
        .content_type(mime::APPLICATION_JSON.as_ref())
        .json(&order_body)
        .await;
    assert_eq!(response.status_code(), axum::http::StatusCode::OK);


    let order_body_with_same_uuid = json!({
        "id": uuid,
        "tableId": 61,
        "item": "pasta",
    });


    let response = server
        .post("/orders")
        .content_type(mime::APPLICATION_JSON.as_ref())
        .json(&order_body)
        .await;
    assert_eq!(response.status_code(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
   // assert_eq!(response.status_code(), axum::http::StatusCode::CONFLICT);

}