package com.example.bootslot

object BootNative {
    init {
        System.loadLibrary("bootslot_native")
    }

    external fun nativeGetCurrentSlot(): Int
    external fun nativeSetActiveSlot(slot: Int): Boolean
}