use uuid::Uuid;

pub struct CreateVirtualMachine {
    pub user_id: Uuid,
    pub template_id: Uuid,
    pub name: String,
}

pub struct DeleteVirtualMachine {
    pub user_id: Uuid,
    pub virual_machine_id: Uuid,
}
