package com.carnx.bootmanager

object BootNative {
    init {
        System.loadLibrary("bootslot_native")
    }

    external fun nativeGetCurrentSlot(): Int
    external fun nativeSetActiveSlot(slot: Int): Boolean
}