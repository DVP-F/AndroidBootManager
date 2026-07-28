#!/usr/bin/env bash

# Copyright (c) 2026 DVP-F/Carnx00  
# Licensed under the GNU General Public License v3.0 (GPLv3).  
# SPDX-License-Identifier: GPL-3.0-only

# check binder services
adb shell service list | grep "aidl::android::hardware::boot::IBootControl"
# optional inspect
#* adb shell dumpsys <service>
# also for various properties relating to boot
#* adb shell getprop | grep -i boot
