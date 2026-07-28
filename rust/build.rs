use std::path::PathBuf;

// Copyright (c) 2026 DVP-F/Carnx00  
// Licensed under the GNU General Public License v3.0 (GPLv3).  
// SPDX-License-Identifier: GPL-3.0-only

fn main() {
    // first ensure generation dir is present
    let _ = std::fs::create_dir_all("src/generated");
    // then build aidl dependency
    rsbinder_aidl::Builder::new()
        .source(PathBuf::from(
            "aidl/android/hardware/boot/IBootControl.aidl",
        ))
        .source(PathBuf::from(
            "aidl/android/hardware/boot/MergeStatus.aidl",
        ))
        .output(PathBuf::from("src/generated/aidl_boot.rs"))
        .generate()
        .unwrap();
}
