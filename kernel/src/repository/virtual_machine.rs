use crate::model::virtual_machine::{
    event::{CreateVirtualMachine, DeleteVirtualMachine},
    VirtualMachine,
};
pub trait VirtualMachineRepository {
    fn insert(&self, event: CreateVirtualMachine) -> VirtualMachine;
    fn delete(&self, event: DeleteVirtualMachine) -> VirtualMachine;
}
