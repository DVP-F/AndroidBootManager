#!/usr/bin/env bash

# Copyright (c) 2026 DVP-F/Carnx00  
# Licensed under the GNU General Public License v3.0 (GPLv3).  
# SPDX-License-Identifier: GPL-3.0-only

set -euxo pipefail

_STARTDIR=$(pwd)

echo "PWD: $(pwd)"
echo "HOME: $HOME"
echo "PATH: $PATH"

CARGO="$HOME/.cargo/bin/cargo"

# fuck fallbacks this is where rustup normally installs cargo iirc so imma force this location
#? consider calling cargo through rustup instead - more likely to resolve correctly and can still use a fallback thru default location and then cargo itself

if [[ ! -x "$CARGO" ]]; then
    echo "Rust cargo not found at $CARGO"
    exit 1
fi

"$CARGO" --version
"$CARGO" ndk --version

# expect the call to be roughly "/absolute/path/here"
# so we move relative to the scripts dir into the rust dir
cd "$(dirname "$0")/../../rust"

# cargo install cargo-ndk
# rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

"$CARGO" ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 -o ../app/src/main/jniLibs build --release

# and go back to start in case this is run in a standalone shell
cd "$_STARTDIR"
