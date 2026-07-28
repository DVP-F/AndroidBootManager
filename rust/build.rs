use std::path::PathBuf;

// Copyright (c) 2026 DVP-F/Carnx00  
// Licensed under the GNU General Public License v3.0 (GPLv3).  
// SPDX-License-Identifier: GPL-3.0-only

fn main() {
    rsbinder_aidl::Builder::new()
        .source(PathBuf::from(
            "aidl/android/hardware/boot/IBootControl.aidl",
        ))
        .source(PathBuf::from(
            "aidl/android/hardware/boot/CommandResult.aidl",
        ))
        .source(PathBuf::from(
            "aidl/android/hardware/boot/MergeStatus.aidl",
        ))
        .output(PathBuf::from("src/generated/boot.rs"))
        .generate()
        .unwrap();
}
