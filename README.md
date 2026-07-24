# BootManager  

ships without gradle wrapper but with gradlew and gradle info/config  

licenses:  
found under [LICENSES](/LICENSES/)

- gradlew ; Apache-2.0
- BootManager ; GPL-3.0

missing for complete workspace:  

- .gradle/*
- .idea/*
- gradle/
  - wrapper/
    - gradle-wrapper.jar
    - gradle-wrapper.properties
- local.properties (sdk and ndk directories)

gradle and project setup occurred through android studio.  

significant versions:  

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
