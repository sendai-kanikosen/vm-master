use crate::model::virtual_machine::{event::CreateVirtualMachine, VirtualMachine};
use crate::repository::{
    proxmox_api::ProxmoxApiRepository, virtual_machine::VirtualMachineRepository,
};
use std::sync::Arc;

use super::event::DeleteVirtualMachine;

pub struct VirtualMachineWorkflow {
    db: Arc<dyn VirtualMachineRepository>,
    proxmox_api: Arc<dyn ProxmoxApiRepository>,
}

impl VirtualMachineWorkflow {
    pub fn new(
        db: Arc<dyn VirtualMachineRepository>,
        proxmox_api: Arc<dyn ProxmoxApiRepository>,
    ) -> Self {
        Self { db, proxmox_api }
    }

    pub fn create_virtual_machine(
        &self,
        create_virtual_machine: CreateVirtualMachine,
    ) -> VirtualMachine {
        let virtual_machine = self.db.insert(create_virtual_machine);
        self.proxmox_api.create_vm(virtual_machine);
        virtual_machine
    }

    pub fn delete_virtual_machine(
        &self,
        delete_virtual_machine: DeleteVirtualMachine,
    ) -> VirtualMachine {
        let virtual_machine = self.db.delete(delete_virtual_machine);
        self.proxmox_api.delete_vm(virtual_machine);
        virtual_machine
    }
}
