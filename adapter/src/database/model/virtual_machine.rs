use chrono::{DateTime, Utc};
use kernel::model::vertual_machine::{VirtualMachine, VirtualMachineStatus};
use uuid::Uuid;

pub struct VirtualMachineRow {
    pub id: Uuid,
    pub name: String,
    pub status: VirtualMachineStatus,
    pub user_id: Uuid,
    pub template_id: Uuid,
}

impl VirtualMachineStatusRow {}
