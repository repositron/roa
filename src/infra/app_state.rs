use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<AsyncPgConnection>,
}

pub fn create_app_state(db_pool: Pool<AsyncPgConnection>) -> AppState {
    AppState {
        db_pool,
    }
}