// boot slot control logic

//* Copyright (c) 2026 DVP-F/Carnx00  
//* Licensed under the GNU General Public License v3.0 (GPLv3).  
//* SPDX-License-Identifier: GPL-3.0-only

use crate::error::{BootError, Result};
use crate::hal::{BootControl, FakeBootControl, AidlBootControl};

// AIDL is on by default, unless legacy-ffi is enabled without aidl.
#[cfg(not(all(feature = "legacy-ffi", not(feature = "aidl"))))]
const BACKEND_AIDL_ENABLED: bool = true;

#[cfg(all(feature = "legacy-ffi", not(feature = "aidl")))]
const BACKEND_AIDL_ENABLED: bool = false;

#[cfg(feature = "legacy-ffi")]
const BACKEND_FFI_ENABLED: bool = true;

#[cfg(not(feature = "legacy-ffi"))]
const BACKEND_FFI_ENABLED: bool = false;

pub struct BootManager {
    backend: Box<dyn BootControl + Send + Sync>,
}

impl Default for BootManager {
    fn default() -> Self {
        Self {
            backend: match AidlBootControl::new() {
                Ok(hal) => Box::new(hal),
                Err(_) => Box::new(FakeBootControl::new()),
            },
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
