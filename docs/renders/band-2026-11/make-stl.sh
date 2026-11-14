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

[features]
older-kernel = []

[profile.release]
opt-level = 1
debug-assertions = true
overflow-checks = true
TOML
# A kernel whose `extrude` still hands back construction state has no
# `topo::finish` door; scenes.rs finishes through the at-rest gate there.
features=()
grep -q "pub fn finish<" "$checkout/crates/topo/src/finish.rs" 2>/dev/null || features=(--features older-kernel)
cargo run --release --manifest-path "$crate/Cargo.toml" "${features[@]}" -- "$2" "$3" "$4"
