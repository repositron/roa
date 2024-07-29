use diesel::Insertable;
use serde::Deserialize;
use uuid::Uuid;

use crate::schema::orders;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = orders)]
#[serde(rename_all = "camelCase")]
pub struct NewOrder {
    pub id: Uuid,
    pub table_id: i32,
    pub item: String,
    pub duration: Option<i32>,
}
