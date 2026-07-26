package com.carnx.bootmanager

data class Result(
    val bResult: Boolean,
    val msg: String
)

//! remapping to more useful labels. 0 = A ; 1 = B
class BootRepository {
    // boilerplate the jni funcs
    fun currentSlot(): Result {
        // success :: "Current boot slot: A|B"
        // failure :: "Failed to load!" and an error message maybe?
        val slot :Int = BootNative.nativeGetCurrentSlot() // -1 if failed otherwise 0|1
        // failure & unknown
        if (slot !in 0..1) {
            return Result(false, "Failed to load!")
        }
        // success
        return Result(true,"Current boot slot: ${if (slot == 0) "A" else "B"}")
    }
    fun switchTo(slot: Int): Result {
        require(slot in 0..1) { "Slot must be either 0 or 1" }
        val status = BootNative.nativeSetActiveSlot(slot)
        return Result(
            status,
            if (status)
                "Set slot to: ${if (slot == 0) "A" else "B"}"
            else
                "Failed! Current slot: ${if (BootNative.nativeGetCurrentSlot() == 0) "A" else "B"}!"
        )
    }
}
