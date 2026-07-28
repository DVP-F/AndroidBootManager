#!/usr/bin/env bash

# Copyright (c) 2026 DVP-F/Carnx00  
# Licensed under the GNU General Public License v3.0 (GPLv3).  
# SPDX-License-Identifier: GPL-3.0-only

# check HIDL HAL registrations
adb shell lshal 2>/dev/null | grep -i android.hardware.boot
    # " android.hardware.boot@1.0::IBootControl "
# or list all boot related entries
#* adb shell lshal | grep -i boot
