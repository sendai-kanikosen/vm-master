use crate::{
    error::DatabaseResult,
    model::virtual_machine::{
        event::{
            CreateVirtualMachine, DeleteVirtualMachine, FindVirtualMachine, UpdateVirtualMachine,
        },
        VirtualMachine,
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait VirtualMachineRepository: Send + Sync {
    async fn insert(&self, event: CreateVirtualMachine) -> DatabaseResult<VirtualMachine>;
    async fn delete(&self, event: DeleteVirtualMachine) -> DatabaseResult<VirtualMachine>;
    async fn find(&self, event: FindVirtualMachine) -> DatabaseResult<VirtualMachine>;
    async fn update(&self, event: UpdateVirtualMachine) -> DatabaseResult<VirtualMachine>;
}
