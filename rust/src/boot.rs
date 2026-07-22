// boot slot control logic

use crate::error::{BootError, Result};
use crate::hal::{BootControl, FakeBootControl, HalBootControl};

pub struct BootManager {
    backend: Box<dyn BootControl + Send + Sync>,
}

impl Default for BootManager {
    fn default() -> Self {
        Self {
            backend: match HalBootControl::new() {
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
        let slots = self.backend.get_number_slots()?;
        if slot >= slots {
            return Err(BootError::InvalidSlot { slot, slots });
        }
        self.backend.set_active_boot_slot(slot)
    }
}
