use crate::model::virtual_machine::{event::CreateVirtualMachine, VirtualMachine};
pub trait VirtualMachineRepository {
    fn insert(&self, event: CreateVirtualMachine) -> VirtualMachine;
}
