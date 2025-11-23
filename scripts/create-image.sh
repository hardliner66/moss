#!/usr/bin/env bash
set -euo pipefail

base="$( cd "$( dirname "${BASH_SOURCE[0]}" )"/.. && pwd )"
pushd "$base"

img="$base/moss.img"
mount="$base/build/mount"

mkdir -p "$mount"

dd if=/dev/zero of="$img" bs=1M count=128
mkfs.vfat -F 32 "$img"

if ! mountpoint -q "$mount"; then
    mount -o loop "$img" "$mount"
fi

mkdir -p "$mount/bin"

if [ ! -f "$mount/bin/busybox-aarch64-linux-gnu" ]; then
    cp "$base/build/busybox-aarch64-linux-gnu" "$mount/bin/sh"
fi
popd

mkdir -p "$mount/dev"

if mountpoint -q "$mount"; then
    umount "$mount"
fi
