#!/usr/bin/env bash

# Copyright (c) 2026 DVP-F/Carnx00  
# Licensed under the GNU General Public License v3.0 (GPLv3).  
# SPDX-License-Identifier: GPL-3.0-only

# check device log for errors relating to this package
adb logcat | grep -E "AndroidRuntime|Rust|panic|com.carnx.bootmanager"
