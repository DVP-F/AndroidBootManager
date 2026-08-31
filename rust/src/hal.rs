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
use crate::ffi_helper::BootControlHal;

pub trait BootControl {
    #![allow(dead_code)]
    fn get_current_slot(&self) -> Result<u32>;
    fn set_active_boot_slot(&self, slot: u32) -> Result<()>;
}

pub mod fake_bc {
    use super::{BootControl, Result, BootError};
    pub struct FakeBootControl;

    impl FakeBootControl {
        pub fn new() -> Self {
            Self
        }
    }

    impl BootControl for FakeBootControl {
        fn get_current_slot(&self) -> Result<u32> {
            Err(BootError::FakeControl)
        }

        fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
            Err(BootError::FakeControl)
        }
    }
}

//? AIDL is on by default, unless legacy-ffi is enabled without aidl.
#[cfg(not(all(feature = "legacy-ffi", not(feature = "aidl"))))]
pub mod aidl_bc {
    use super::{BootControl, BootError, Result, Strong, check_interface, IBootControl, mpsc, thread, Duration};
    pub struct AidlBootControl {
        boot: Strong<dyn IBootControl>,
    }

    fn get_current_slot_with_timeout(boot: Strong<dyn IBootControl>) -> Result<u32> {
        // might not get a return if binder cant talk to firm/hardware
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
    }

    impl BootControl for AidlBootControl {
        fn get_current_slot(&self) -> Result<u32> {
            get_current_slot_with_timeout(self.boot.clone())
        }

        fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
            Err(BootError::HalUnavailable)
        }
    }
}

//? If AIDL feature is not on:
#[cfg(all(feature = "legacy-ffi", not(feature = "aidl")))]
pub mod aidl_bc {
    use super::{BootControl, BootError, Result};
    pub struct AidlBootControl {
    }

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
    }
}


//? HIDL is off by default, only on when the "hidl" feature is enabled.
#[cfg(feature = "hidl")]
pub mod hidl_bc {
}

//? If HIDL is not on
#[cfg(not(feature = "hidl"))]
pub mod hidl_bc {
    use super::{BootControl, BootError, Result};
    pub struct HidlBootControl {
    }

    impl HidlBootControl {
        pub fn new() -> Result<Self> {
            Err(BootError::HalUnavailable)
        }
    }

    impl BootControl for HidlBootControl {
        fn get_current_slot(&self) -> Result<u32> {
            Err(BootError::HalUnavailable)
        }

        fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
            Err(BootError::HalUnavailable)
        }
    }
}


//? FFI is off by default, only on when the "legacy-ffi" feature is enabled.
// #[cfg(feature = "legacy-ffi")]
pub mod ffi_bc {
    use super::{BootControl, BootError, Result, Strong, IBootControl, BootControlHal};
    pub struct FFIBootControl {
        boot: Strong<dyn IBootControl>,
    }

    impl FFIBootControl {
        pub fn new() {
            let hal = BootControlHal::load()?;
            #[allow(unsafe_code)]
            unsafe {
                let boot: Strong<dyn IBootControl> = hal.module();
            }

            unsafe {
                // Call init if available
                if let Some(init) = self.boot.init {
                    init(module as *mut _);
                }

                // // Get number of slots
                // if let Some(get_num) = self.boot.getNumberSlots {
                //     let num = get_num(self.boot as *mut _);
                //     println!("Number of slots: {}", num);
                // }

                // Get current slot
                if let Some(get_cur) = self.boot.getCurrentSlot {
                    let cur = get_cur(self.boot as *mut _);
                    println!("Current slot: {}", cur);
                }

                // Get suffix
                if let Some(get_suffix) = self.boot.getSuffix {
                    let suffix_ptr = get_suffix(self.boot as *mut _, 0);
                    if !suffix_ptr.is_null() {
                        let suffix = CStr::from_ptr(suffix_ptr).to_string_lossy();
                        println!("Slot 0 suffix: {}", suffix);
                    }
                }
            }
        }
    }
}

//? If FFI is off
#[cfg(not(feature = "legacy-ffi"))]
pub mod ffi_bc {
    use super::{BootControl, BootError, Result};
    pub struct FFIBootControl {
    }

    impl FFIBootControl {
        pub fn new() -> Result<Self> {
            return Err(BootError::HalUnavailable)
        }
    }

    impl BootControl for FFIBootControl {
        fn get_current_slot(&self) -> Result<u32> {
            Err(BootError::HalUnavailable)
        }

        fn set_active_boot_slot(&self, _slot: u32) -> Result<()> {
            Err(BootError::HalUnavailable)
        }
    }
}
