#!/usr/bin/env bash
# make-stl.sh <checkout> <mixed|knife|chain> <size> <out-dir>
#
# Builds scenes.rs against <checkout>'s `crates/pncad` in a throwaway
# crate (kept out of the repo so it is no cargo root of this tree) and
# writes the group's STLs and refusals into <out-dir>. SCENES_CRATE names
# the throwaway crate's directory; reusing one keeps its build warm.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
checkout=$(cd "$1" && pwd)
crate=${SCENES_CRATE:-$(mktemp -d)}
mkdir -p "$crate/src"
cp "$here/scenes.rs" "$crate/src/main.rs"
[ -f "$crate/Cargo.lock" ] || cp "$checkout/demos/tour/Cargo.lock" "$crate/Cargo.lock"
cat >"$crate/Cargo.toml" <<TOML
[package]
name = "band-render-scenes"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
pncad = { path = "$checkout/crates/pncad" }

[workspace]

[profile.release]
opt-level = 1
debug-assertions = true
overflow-checks = true
TOML
cargo run --release --manifest-path "$crate/Cargo.toml" -- "$2" "$3" "$4"
