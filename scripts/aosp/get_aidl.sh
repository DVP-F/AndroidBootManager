#!/usr/bin/env bash

# Copyright (c) 2026 DVP-F/Carnx00  
# Licensed under the GNU General Public License v3.0 (GPLv3).  
# SPDX-License-Identifier: GPL-3.0-only

mkdir -p "$(dirname "$0")/../../rust/aidl/android/hardware/boot"
cd "$(dirname "$0")/../../rust/aidl/android/hardware/boot"

curl 'https://android.googlesource.com/platform/hardware/interfaces/+/refs/heads/main/boot/aidl/android/hardware/boot/IBootControl.aidl?format=TEXT' | base64 -d > IBootControl.aidl
curl 'https://android.googlesource.com/platform/hardware/interfaces/+/refs/heads/main/boot/aidl/android/hardware/boot/MergeStatus.aidl?format=TEXT' | base64 -d > MergeStatus.aidl
