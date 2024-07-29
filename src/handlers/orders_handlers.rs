use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use diesel::dsl::IntervalDsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::handlers::requests::NewOrder;
use crate::infra::app_state::AppState;
use crate::infra::random_duration_gen::random_duration;
use crate::models::order_model::OrderModel;
use crate::schema::orders;
use crate::schema::orders::dsl::*;

pub async fn get_list_by_table_id(
    State(state): State<AppState>,
    Path(table_id_from_path): Path<i32>
) -> Result<Json<Vec<OrderModel>>, (StatusCode, String)> {
    let mut conn = state.db_pool.get().await.unwrap();
    let res = orders::table
        .select(OrderModel::as_select())
        .filter(table_id.eq(table_id_from_path))
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    return Ok(Json(res));
}
pub async fn create_order(State(state): State<AppState>,
                          Json(order_request): Json<NewOrder>
) -> Result<Json<OrderModel>, (StatusCode, String)> {
    let mut conn = state.db_pool.get().await.map_err(internal_error)?;
    let new_duration = order_request.duration.unwrap_or_else(|| random_duration(5*60, 15* 60));
    let insert_result = diesel::insert_into(orders::table)
        .values((
            id.eq(order_request.id),
            table_id.eq(order_request.table_id),
            item.eq(order_request.item),
            duration.eq(new_duration),
            expire_at.eq(diesel::dsl::now + new_duration.seconds()),
        ))
        .returning(OrderModel::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error);
    Ok(Json(insert_result?))
}

pub async fn delete_order(
    State(state): State<AppState>,
    Path(id_to_delete): Path<Uuid>
) -> Result<StatusCode, (StatusCode, String)> {

    let mut conn = state.db_pool.get().await.map_err(internal_error)?;

    let delete_result = diesel::delete(orders::table)
        .filter(id.eq(id_to_delete))
        .execute(&mut conn)
        .await
        .map_err(internal_error);

    match delete_result {
        Ok(0) => Err((StatusCode::NOT_FOUND, "Order not found".to_string())),
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        _ => Err((StatusCode::NOT_FOUND, "Order not found".to_string()))
    }
}
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
