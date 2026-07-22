// HAL access layer that either talks to HIDL/AIDL binder or the legacy boot control module

use crate::error::{BootError, Result};

pub trait BootControl {
    fn get_current_slot(&self) -> Result<u32>;
    fn get_number_slots(&self) -> Result<u32>;
    fn set_active_boot_slot(&self, slot: u32) -> Result<()>;
}

pub struct FakeBootControl;

impl FakeBootControl {
    pub fn new() -> Self {
        Self
    }
}

impl BootControl for FakeBootControl {
    fn get_current_slot(&self) -> Result<u32> {
        Ok(0)
    }

    fn get_number_slots(&self) -> Result<u32> {
        Ok(2)
    }

    fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
        Ok(())
    }
}

pub struct HalBootControl;

impl HalBootControl {
    pub fn new() -> Result<Self> {
        Err(BootError::HalUnavailable)
    }
}

impl BootControl for HalBootControl {
    fn get_current_slot(&self) -> Result<u32> {
        Err(BootError::HalUnavailable)
    }

    fn get_number_slots(&self) -> Result<u32> {
        Err(BootError::HalUnavailable)
    }

    fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
        Err(BootError::HalUnavailable)
    }
}
