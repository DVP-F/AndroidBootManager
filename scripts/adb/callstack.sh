#!/usr/bin/env bash

# Copyright (c) 2026 DVP-F/Carnx00  
# Licensed under the GNU General Public License v3.0 (GPLv3).  
# SPDX-License-Identifier: GPL-3.0-only

set -euo pipefail

# run if the app hangs. dumps the entire callstack, threads, and more

C_PID=$(adb shell pidof com.carnx.bootmanager)

## backtrace
adb shell debuggerd -b $C_PID > hang.txt

## this one might get ACL'd
adb shell kill -3 $C_PID
adb logcat -d -v threadtime >> hang.txt

## kernel wait site backup dump
adb shell '
pid=$(pidof com.carnx.bootmanager)
for t in /proc/$pid/task/*; do
    echo "===== TID ${t##*/} ====="
    printf "name: "; cat "$t/comm"
    printf "state: "; awk "{print \$3}" "$t/stat"
    printf "wchan: "; cat "$t/wchan"
done
' > hang.bak.txt

# optionally use:
#* adb shell dumpsys activity processes com.carnx.bootmanager
