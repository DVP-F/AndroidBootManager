package com.carnx.bootmanager

//* Copyright (c) 2026 DVP-F/Carnx00  
//* Licensed under the GNU General Public License v3.0 (GPLv3).  
//* SPDX-License-Identifier: GPLv3

object BootNative {
    // add calls into the packaged Rust lib

    init {
        System.loadLibrary("bootmanager_native")
    }

    external fun nativeGetCurrentSlot(): Int
    external fun nativeSetActiveSlot(slot: Int): Boolean
}
