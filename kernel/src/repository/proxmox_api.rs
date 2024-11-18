use crate::model::virtual_machine::VirtualMachine;

pub trait ProxmoxApiRepository {
    fn create_vm(&self, virtual_machine: VirtualMachine) -> ();
}
