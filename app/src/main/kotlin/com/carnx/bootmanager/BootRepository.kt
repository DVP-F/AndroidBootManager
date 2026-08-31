package com.carnx.bootmanager

//* Copyright (c) 2026 DVP-F/Carnx00  
//* Licensed under the GNU General Public License v3.0 (GPLv3).  
//* SPDX-License-Identifier: GPL-3.0-only

data class Result(
    val bResult: Boolean,
    val msg: String
)

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
        return Result(true,"Current boot slot: ${if (slot == 0) "A (" + slot + ")" else "B (" + slot + ")"}")
    }
    fun switchTo(slot: Int): Result {
        require(slot in 0..1) { "Slot must be either 0 or 1" }
        val status = BootNative.nativeSetActiveSlot(slot)
        val currentslot = BootNative.nativeGetCurrentSlot()
        return Result(
            status,
            if (status)
                "Set slot to: ${if (slot == 0) "A (" + slot + ")" else "B (" + slot + ")"}"
            else
                "Failed! Current slot: ${if (currentslot == 0) "A (" + slot + ")" else "B (" + slot + ")"}!"
        )
    }
}
