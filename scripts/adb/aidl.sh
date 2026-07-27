#!/usr/bin/env bash
# check binder services
adb shell service list | grep "aidl::android::hardware::boot::IBootControl"
# optional inspect
#* adb shell dumpsys <service>
# also for various properties relating to boot
#* adb shell getprop | grep -i boot
