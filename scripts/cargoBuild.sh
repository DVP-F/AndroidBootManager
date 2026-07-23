#!/usr/bin/env bash

set -euo pipefail
# optional debug
#*set -x

_STARTDIR=$(pwd)

# okay so this shouldnt be necessary outside of debugging but android studio uses a protected non-userspace env for execution THUS:
CARGO="$(command -v cargo 2>/dev/null || printf '%s' "$HOME/.cargo/bin/cargo")"

# expect the call to be roughly "/bin/sh /path/to/this/script.sh"
# so we move relative to the scripts dir into the rust dir
cd "$(dirname "$0")/../rust"

# cargo install cargo-ndk
# rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

"$CARGO" ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 -o ../app/src/main/jniLibs build --release

# and go back to start in case this is run in a standalone shell
cd "$_STARTDIR"
