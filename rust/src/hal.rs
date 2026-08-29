// HAL access layer that either talks to HIDL/AIDL binder or the legacy boot control module

//* Copyright (c) 2026 DVP-F/Carnx00  
//* Licensed under the GNU General Public License v3.0 (GPLv3).  
//* SPDX-License-Identifier: GPL-3.0-only

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::error::{Result, BootError};
use rsbinder::{hub::check_interface, Strong};
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
    use super::{BootControl, BootError, Result, Strong, check_interface, IBootControl, mpsc, thread, Duration};
    pub struct AidlBootControl {
        boot: Strong<dyn IBootControl>,
    }


    fn get_current_slot_with_timeout(boot: Strong<dyn IBootControl>) -> Result<u32> {
        let (tx, rx) = mpsc::sync_channel(1);

        thread::spawn(move || {
            let result = boot
                .r#getCurrentSlot()
                .map(|slot| slot as u32)
                .map_err(|_| BootError::HalUnavailable);
            let _ = tx.send(result);
        });

        rx.recv_timeout(Duration::from_millis(500))
            .map_err(|_| BootError::HalUnavailable)?
    }

    impl AidlBootControl {
        pub fn new() -> Result<Self> {
            let boot: Strong<dyn IBootControl> = 
                check_interface("android.hardware.boot.IBootControl/default")
                .map_err(|_| BootError::HalUnavailable)?;

            // Probe it once. If this hangs, timeout and let the caller select the fallback backend.
            get_current_slot_with_timeout(boot.clone())?;

            Ok(Self { boot })
        }
        const __FAKE :bool = false;
    }

    impl BootControl for AidlBootControl {
        fn get_current_slot(&self) -> Result<u32> { // might not get a return if binder cant talk to firm/hardware
            get_current_slot_with_timeout(self.boot.clone())
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
