use crate::{error::ProxmoxApiResult, model::virtual_machine::VirtualMachine};
use async_trait::async_trait;

#[async_trait]
pub trait ProxmoxApiRepository: Send + Sync {
    async fn create_vm(&self, virtual_machine: VirtualMachine) -> ProxmoxApiResult<VirtualMachine>;
    async fn delete_vm(&self, virtual_machine: VirtualMachine) -> ProxmoxApiResult<VirtualMachine>;
    async fn fetch_vm(&self, virtual_machine: VirtualMachine) -> ProxmoxApiResult<VirtualMachine>;
    async fn update_vm(&self, virtual_machine: VirtualMachine) -> ProxmoxApiResult<VirtualMachine>;
}
