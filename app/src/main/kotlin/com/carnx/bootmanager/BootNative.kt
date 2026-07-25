package com.carnx.bootmanager

object BootNative {
    // add calls into the packaged Rust lib

    init {
        System.loadLibrary("bootmanager_native")
    }

    external fun nativeGetCurrentSlot(): Int
    external fun nativeSetActiveSlot(slot: Int): Boolean
}
