use crate::model::virtual_machine::VirtualMachine;
use async_trait::async_trait;

#[async_trait]
pub trait ProxmoxApiRepository: Send + Sync {
    async fn create_vm(&self, virtual_machine: VirtualMachine) -> VirtualMachine;
    async fn delete_vm(&self, virtual_machine: VirtualMachine) -> VirtualMachine;
}
