use anyhow::anyhow;
use std::convert::TryFrom;
use uuid::Uuid;

use crate::error::{ValidationError, ValidationResult};

#[derive(Debug)]
pub struct CreateVirtualMachine {
    pub template_id: Uuid,
    pub name: String,
}

pub struct DeleteVirtualMachine {
    pub virual_machine_id: Uuid,
}

pub struct FindVirtualMachine {
    pub virtual_machine_id: Uuid,
}

pub struct UpdateVirtualMachine {
    pub virtual_machine_id: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct UnvalidatedCreateVirtualMachine {
    pub template_id: Uuid,
    pub name: String,
}

pub struct UnvalidatedUpdateVirtualMachine {
    pub virtual_machine_id: Uuid,
    pub name: String,
}

impl TryFrom<UnvalidatedCreateVirtualMachine> for CreateVirtualMachine {
    type Error = ValidationError;

    fn try_from(value: UnvalidatedCreateVirtualMachine) -> ValidationResult<Self> {
        if value.template_id.is_nil() {
            return Err(ValidationError(
                anyhow!("テンプレートIDが無効です").context("template_id is nil"),
            ));
        }
        if value.name.trim().is_empty() {
            return Err(ValidationError(
                anyhow!("名前が1文字以上である必要があります").context("name is empty"),
            ));
        }
        Ok(CreateVirtualMachine {
            template_id: value.template_id,
            name: value.name,
        })
    }
}

impl TryFrom<UnvalidatedUpdateVirtualMachine> for UpdateVirtualMachine {
    type Error = ValidationError;

    fn try_from(value: UnvalidatedUpdateVirtualMachine) -> ValidationResult<Self> {
        if value.virtual_machine_id.is_nil() {
            return Err(ValidationError(
                anyhow!("テンプレートIDが無効です").context("template_id is nil"),
            ));
        }
        if value.name.trim().is_empty() {
            return Err(ValidationError(
                anyhow!("名前が1文字以上である必要があります").context("name is empty"),
            ));
        }
        Ok(UpdateVirtualMachine {
            virtual_machine_id: value.virtual_machine_id,
            name: value.name,
        })
    }
}
