use crate::{
    error::{DatabaseError, ProxmoxApiError, ValidationError},
    model::virtual_machine::{
        event::{
            CreateVirtualMachine, DeleteVirtualMachine, FindVirtualMachine, UpdateVirtualMachine,
        },
        VirtualMachine,
    },
    repository::{proxmox_api::ProxmoxApiRepository, virtual_machine::VirtualMachineRepository},
};
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum VirtualMachineWorkflowError {
    #[error("バリデーションに失敗しました: {0}")]
    Validation(#[from] ValidationError),
    #[error("DB処理実行時にエラーが発生しました: {0}")]
    Database(#[from] DatabaseError),
    #[error("Proxmox API呼び出し時にエラーが発生しました: {0}")]
    ProxmoxApi(#[from] ProxmoxApiError),
}

pub type VirtualMachineWorkflowResult<T> = Result<T, VirtualMachineWorkflowError>;

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
        event: CreateVirtualMachine,
    ) -> VirtualMachineWorkflowResult<VirtualMachine> {
        let vm = self.db.insert(event).await?;
        let vm = self.proxmox_api.create_vm(vm).await?;
        Ok(vm)
    }

    pub async fn delete_virtual_machine(
        &self,
        event: DeleteVirtualMachine,
    ) -> VirtualMachineWorkflowResult<VirtualMachine> {
        let vm = self.db.delete(event).await?;
        let vm = self.proxmox_api.delete_vm(vm).await?;
        Ok(vm)
    }

    pub async fn find_virtual_machine(
        &self,
        event: FindVirtualMachine,
    ) -> VirtualMachineWorkflowResult<VirtualMachine> {
        let vm = self.db.find(event).await?;
        let vm = self.proxmox_api.fetch_vm(vm).await?;
        Ok(vm)
    }

    pub async fn update_virtual_machine(
        &self,
        event: UpdateVirtualMachine,
    ) -> VirtualMachineWorkflowResult<VirtualMachine> {
        let vm = self.db.update(event).await?;
        let vm = self.proxmox_api.update_vm(vm).await?;
        Ok(vm)
    }
}
