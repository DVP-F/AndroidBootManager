package com.carnx.bootmanager

class BootRepository {
    // rename the jni funcs
    fun currentSlot(): String {
        // success :: "Current boot slot: A|B"
        // failure :: "Failed to load!" and an error message maybe?
        BootNative.nativeGetCurrentSlot() // -1 if failed otherwise 0|1
    }
    fun switchTo(slot: Int): Boolean = BootNative.nativeSetActiveSlot(slot)
}
