#!/usr/bin/env bash
set -euo pipefail

base="$( cd "$( dirname "${BASH_SOURCE[0]}" )"/.. && pwd )"
pushd "$base"

mkdir -p "build"

# busybox -- I couldn't get this to build.  I ended up restoring to a third-party static binary which isn't ideal but it get's things running.
if [ ! -f "build/busybox-aarch64-linux-gnu" ]; then
    pushd "build"
    wget https://github.com/shutingrz/busybox-static-binaries-fat/raw/refs/heads/main/busybox-aarch64-linux-gnu
    chmod +x busybox-aarch64-linux-gnu
    popd
fi
popd