use crate::model::virtual_machine::{
    event::{CreateVirtualMachine, DeleteVirtualMachine, FindVirtualMachine, UpdateVirtualMachine},
    VirtualMachine,
};
use crate::repository::{
    proxmox_api::ProxmoxApiRepository, virtual_machine::VirtualMachineRepository,
};
use std::sync::Arc;

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

    pub async fn create_virtual_machine(
        &self,
        create_virtual_machine: CreateVirtualMachine,
    ) -> VirtualMachine {
        let virtual_machine = self.db.insert(create_virtual_machine).await;
        self.proxmox_api.create_vm(virtual_machine.clone());
        virtual_machine
    }

    pub async fn delete_virtual_machine(
        &self,
        delete_virtual_machine: DeleteVirtualMachine,
    ) -> VirtualMachine {
        let virtual_machine = self.db.delete(delete_virtual_machine).await;
        self.proxmox_api.delete_vm(virtual_machine.clone());
        virtual_machine
    }

    pub async fn find_virtual_machine(
        &self,
        find_virtual_machine: FindVirtualMachine,
    ) -> VirtualMachine {
        let virtual_machine = self.db.find(find_virtual_machine).await;
        self.proxmox_api.fetch_vm(virtual_machine.clone()).await;
        virtual_machine
    }

    pub async fn update_virtual_machine(
        &self,
        update_virtual_machine: UpdateVirtualMachine,
    ) -> VirtualMachine {
        let virtual_machine = self.db.update(update_virtual_machine).await;
        self.proxmox_api.update_vm(virtual_machine.clone()).await;
        virtual_machine
    }
}
