use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize, sqlx::FromRow)]
pub struct Template {
    pub vmid: i32,
    pub node: String,
}
