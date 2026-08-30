// boot slot control logic

//* Copyright (c) 2026 DVP-F/Carnx00  
//* Licensed under the GNU General Public License v3.0 (GPLv3).  
//* SPDX-License-Identifier: GPL-3.0-only

use crate::error::{BootError, Result};
use crate::hal::{
    BootControl,
    fake_bc::FakeBootControl,
    aidl_bc::AidlBootControl,
    hidl_bc::HidlBootControl,
    ffi_bc::FFIBootControl,
};

//? Fallback chain:
//? ```
//? AIDL if on; then
//? HIDL if on; then
//? FFI if on; then
//? Fake
//? ```

pub struct BootManager {
    backend: Box<dyn BootControl + Send + Sync>,
}

impl Default for BootManager {
    fn default() -> Self {
        Self {
            backend: match AidlBootControl::new() { // Returns Err if feature `aidl` is disabled
                Ok(hal) => Box::new(hal),
                Err(_) => match HidlBootControl::new() { // Returns Err if feature `hidl` is disabled
                    Ok(hal) => Box::new(hal),
                    Err(_) => match FFIBootControl::new() { // Returns Err if feature `legacy_ffi` is disabled
                        Ok(hal) => Box::new(hal),
                        Err(_) => Box::new(FakeBootControl::new())
                    }
                }
            }
        }
    }
}

impl BootManager {
    pub fn get_current_slot(&self) -> Result<u32> {
        self.backend.get_current_slot()
    }

    pub fn set_active_slot(&self, slot: u32) -> Result<()> {
        let slots = 2u32;
        if slot >= slots {
            return Err(BootError::InvalidSlot { slot, slots });
        }
        self.backend.set_active_boot_slot(slot)
    }
}
