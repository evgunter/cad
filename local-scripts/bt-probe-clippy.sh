#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge).
# The check that was MISSING and let a broken commit reach CI: plain
# `cargo check -p geom-core` does not compile `#[cfg(test)] mod tests`
# inside the lib, so an ungated `Probe` use in geom-core's own unit tests
# survived local verification. `--workspace --all-targets` is what CI runs.
set -u
cd "$(dirname "$0")/.."
echo "=== default features, --workspace --all-targets (-D warnings) ==="
cargo clippy --workspace --all-targets -- -D warnings 2>&1 | tail -6
echo
echo "=== --features probe, --workspace --all-targets (-D warnings) ==="
cargo clippy --workspace --all-targets --features probe -- -D warnings 2>&1 | tail -6
echo
echo "=== --features interval (probe and interval are independent axes) ==="
cargo clippy --workspace --all-targets --features interval -- -D warnings 2>&1 | tail -4
echo
echo "=== --features probe,interval (the combination) ==="
cargo clippy --workspace --all-targets --features probe,interval -- -D warnings 2>&1 | tail -4
