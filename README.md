# BootManager  

Copyright (c) 2026 DVP-F/Carnx00  
Licensed under the GNU General Public License v3.0 (GPLv3).  
SPDX-License-Identifier: GPL-3.0-only

ships with gradle wrapper and gradle info/config  

missing for complete workspace:  

- local.properties (sdk and ndk directories)  

gradle and project setup occurred through android studio. 

## Licenses  

found under [LICENSES](/LICENSES/)  

see [NOTICE.md](/NOTICE.md)  

- BootManager ; GPLv3  
- gradle wrapper ; Apache-2.0  
- Reload icon ; MIT   
- AOSP files ; Apache-2.0  

## Build targets  

Written to be compatible with Android 8+, Android API 28+

## Versions and dependencies  

significant program versions:  

- JDK (OpenJDK) v17.0.15-ea 2025-04-15 (+5-Debian-1)  
- gradle v9.4.1  
- kotlin v2.0.20  
- cargo v1.97.1 (c980f4866 2026-06-30)  
- rustup v1.29.0 (28d1352db 2026-03-05)  
- rustc v1.97.1 (8bab26f4f 2026-07-14)  
- cargo-ndk v4.1.2  
- android ndk (sdkmanager) v26.1.10909125  

rustup targets:  

- aarch64-linux-android  
- armv7-linux-androideabi  
- i686-linux-android  
- x86_64-linux-android  

cargo dependencies:  

- jni 0.22.4  
- thiserror 2  
- log 0.4  

app dependencies:  

- release:  
  - platform ( androidx.compose:compose-bom:2024.09.03 )  
  - androidx.activity:activity-compose:1.9.2  
  - androidx.compose.ui:ui  
  - androidx.compose.ui:ui-tooling-preview  
  - androidx.compose.material3:material3  
- debug:  
    androidx.compose.ui:ui-tooling  

kotlin plugins:  

- com.android.application 8.5.2  
- org.jetbrains.kotlin.android 2.0.20  
- org.jetbrains.kotlin.plugin.compose 2.0.20  

gradle wrapper versions:  

- agp 9.2.1
- coreKtx 1.10.1
- junit 4.13.2
- junitVersion 1.1.5
- espressoCore 3.5.1
- lifecycleRuntimeKtx 2.6.1
- activityCompose 1.8.0
- kotlin 2.0.20
- composeBom 2026.02.01
