// error handling

//* Copyright (c) 2026 DVP-F/Carnx00  
//* Licensed under the GNU General Public License v3.0 (GPLv3).  
//* SPDX-License-Identifier: GPLv3

use thiserror::Error;

pub type Result<T> = std::result::Result<T, BootError>;

#[derive(Debug, Error)]
pub enum BootError {
    #[error("invalid slot {slot}, device has {slots} slots")]
    InvalidSlot { slot: u32, slots: u32 },
    #[error("hal unavailable")]
    HalUnavailable,
    #[error("permission denied")]
    PermissionDenied,
    #[error("unknown error")]
    Unknown,
}
