use crate::model::virtual_machine::{
    event::{CreateVirtualMachine, DeleteVirtualMachine, FindVirtualMachine, UpdateVirtualMachine},
    VirtualMachine,
};
use async_trait::async_trait;

#[async_trait]
pub trait VirtualMachineRepository: Send + Sync {
    async fn insert(&self, event: CreateVirtualMachine) -> VirtualMachine;
    async fn delete(&self, event: DeleteVirtualMachine) -> VirtualMachine;
    async fn find(&self, event: FindVirtualMachine) -> VirtualMachine;
    async fn update(&self, event: UpdateVirtualMachine) -> VirtualMachine;
}
