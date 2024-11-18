use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

pub struct VirtualMachine {
    pub id: Uuid,
    pub name: String,
    pub status: VirtualMachineStatus,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
    pub template_id: Uuid,
}

pub enum VirtualMachineStatus {}

pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub iso: String,
    pub core: usize,
    pub memory: usize,
    pub storage: usize,
}
