#!/usr/bin/env bash
set -euo pipefail

base="$( cd "$( dirname "${BASH_SOURCE[0]}" )"/.. && pwd )"
pushd "$base" &>/dev/null || exit 1

mkdir -p etc/generated

read -p "Press enter to start. Then wait until bash bash appears and press ctrl+a x to exit"

cargo run --release | tee /dev/stderr | grep "SYSCALL" > etc/generated/implemented_syscalls_aarch64.txt