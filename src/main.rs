use axum::handler::{Handler, HandlerWithoutStateExt};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::infra::app_state::create_app_state;
use crate::infra::db::create_db_pool;
use crate::infra::orders_cleanup::start_cleanup_thread;
use crate::routes::health_route::create_health_check;
use crate::routes::orders_route::make_orders_route;

mod routes;
mod handlers;
mod models;
mod schema;
mod infra;
mod tests;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rest_ordering_app=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");
    let db_pool = create_db_pool(&database_url).await;
    let app_state = create_app_state(db_pool);

/*    {
        let conn = app_state.db_pool.get().await.unwrap();
        conn.interact(move |conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }*/

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create tcp listener");
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let app = make_orders_route(app_state.clone());

    start_cleanup_thread(app_state.clone()).await;

    axum::serve(listener,
        app
            .merge(create_health_check(app_state.clone()))
            .with_state(app_state.clone())
            .into_make_service()
    )
    .await
    .expect("Error serving application");
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

/*async fn run_migrations(pool: &DbPool) {
    let conn = pool.get().await.unwrap();

    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}*/
