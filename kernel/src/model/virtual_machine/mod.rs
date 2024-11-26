use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod event;
pub mod workflow;

#[derive(Clone)]
pub struct VirtualMachine {
    pub id: Uuid,
    pub name: String,
    pub status: VirtualMachineStatus,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
    pub template_id: Uuid,
}
#[derive(Clone)]
pub enum VirtualMachineStatus {}
