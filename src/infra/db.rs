use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;

//pub type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub async fn create_db_pool(database_url: &String) -> Pool<AsyncPgConnection> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(manager).build().expect("Failed to create pool.");
    pool
}