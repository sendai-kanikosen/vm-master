use chrono::{DateTime, Utc};
use kernel::model::User;
use uuid::Uuid;

pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub passwrd_hash: String,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}
