#!/usr/bin/env bash
# check HIDL HAL registrations
adb shell lshal 2>/dev/null | grep -i android.hardware.boot
    # " android.hardware.boot@1.0::IBootControl "
# or list all boot related entries
#* adb shell lshal | grep -i boot
