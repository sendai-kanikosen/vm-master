use crate::model::virtual_machine::{
    event::{CreateVirtualMachine, DeleteVirtualMachine},
    VirtualMachine,
};
use async_trait::async_trait;

#[async_trait]
pub trait VirtualMachineRepository: Send + Sync {
    async fn insert(&self, event: CreateVirtualMachine) -> VirtualMachine;
    async fn delete(&self, event: DeleteVirtualMachine) -> VirtualMachine;
}
