package com.carnx.bootmanager

class BootRepository {
    // rename the jni funcs
    fun currentSlot(): Int = BootNative.nativeGetCurrentSlot()
    fun switchTo(slot: Int): Boolean = BootNative.nativeSetActiveSlot(slot)
}