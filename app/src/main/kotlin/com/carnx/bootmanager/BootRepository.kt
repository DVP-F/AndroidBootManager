package com.carnx.bootmanager

class BootRepository {
    fun currentSlot(): Int = BootNative.nativeGetCurrentSlot()
    fun switchTo(slot: Int): Boolean = BootNative.nativeSetActiveSlot(slot)
}