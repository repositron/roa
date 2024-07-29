use std::time::Duration;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tokio::{spawn, time::interval};

use crate::infra::app_state::AppState;
use crate::schema::orders::dsl::*;
use crate::schema::orders::expire_at;

pub async fn start_cleanup_thread(state: AppState) {
    let mut interval = interval(Duration::from_secs(60));

    spawn(async move {
        loop {
            interval.tick().await;

            let mut conn = state
                .db_pool
                .get()
                .await
                .expect("couldn't get db");

           // let mut conn = state.db_pool.get().await.map_err(internal_error)?;

/*            let delete_result = diesel::delete(orders::table)
                .execute(&mut conn)
                .await
                .map_err(internal_error);*/

            let delete_result= diesel::delete(orders.filter(expire_at.lt(diesel::dsl::now)))
                    .execute(&mut conn)
                    .await;

            match delete_result {
                Ok(deleted_orders) => {
                    tracing::info!("Deleted expired {} orders", deleted_orders);
                }
                Err(err) => {
                    tracing::error!("Error deleting expired orders: {}", err);
                }
            }
        }
    });
}