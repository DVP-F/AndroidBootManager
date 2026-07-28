// HAL access layer that either talks to HIDL/AIDL binder or the legacy boot control module

//* Copyright (c) 2026 DVP-F/Carnx00  
//* Licensed under the GNU General Public License v3.0 (GPLv3).  
//* SPDX-License-Identifier: GPL-3.0-only

use crate::error::{BootError, Result};

pub trait BootControl {
    #![allow(dead_code)]
    fn get_current_slot(&self) -> Result<u32>;
    fn set_active_boot_slot(&self, slot: u32) -> Result<()>;
    fn mark_boot_successful(&self) -> Result<()>;
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

    fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
        Ok(())
    }

    fn mark_boot_successful(&self) -> Result<()> {
        Ok(())
    }
}

pub struct AidlBootControl;

impl AidlBootControl {
    pub fn new() -> Result<Self> {
        Err(BootError::HalUnavailable)
    }
}

impl BootControl for AidlBootControl {
    fn get_current_slot(&self) -> Result<u32> {
        Err(BootError::HalUnavailable)
    }

    fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
        Err(BootError::HalUnavailable)
    }

    fn mark_boot_successful(&self) -> Result<()> {
        Err(BootError::HalUnavailable)
    }
}
