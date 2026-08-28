// HAL access layer that either talks to HIDL/AIDL binder or the legacy boot control module

//* Copyright (c) 2026 DVP-F/Carnx00  
//* Licensed under the GNU General Public License v3.0 (GPLv3).  
//* SPDX-License-Identifier: GPL-3.0-only

use crate::error::{Result, BootError};
use rsbinder::{get_interface, Strong};
use crate::generated::aidl_boot::android::hardware::boot::IBootControl::IBootControl;

pub trait BootControl {
    #![allow(dead_code)]
    fn get_current_slot(&self) -> Result<u32>;
    fn set_active_boot_slot(&self, slot: u32) -> Result<()>;
}

pub mod fake_bc {
    use super::{BootControl, Result};
    pub struct FakeBootControl;

    impl FakeBootControl {
        pub fn new() -> Self {
            Self
        }
        const __FAKE :bool = true;
    }

    impl BootControl for FakeBootControl {
        fn get_current_slot(&self) -> Result<u32> {
            Ok(0)
        }

        fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
            Ok(())
        }
    }
}

pub mod aidl_bc {
    use super::{BootControl, BootError, Result, Strong, get_interface, IBootControl};
    pub struct AidlBootControl {
        boot: Strong<dyn IBootControl>,
    }

    impl AidlBootControl {
        pub fn new() -> Result<Self> {
            let boot: Strong<dyn IBootControl> = 
                get_interface("android.hardware.boot.IBootControl/default")
                .map_err(|_| BootError::HalUnavailable)?;
            Ok(Self { boot })
        }
        const __FAKE :bool = false;
    }

    impl BootControl for AidlBootControl {
        fn get_current_slot(&self) -> Result<u32> {
            Err(BootError::HalUnavailable)
        }

        fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
            Err(BootError::HalUnavailable)
        }
    }
}

pub mod hidl_bc {
}

pub mod ffi_bc {
}
