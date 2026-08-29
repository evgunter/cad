#!/usr/bin/env bash
# serve-wasm.sh — build the viewer's browser spike and serve it to a phone.
#
#   local-scripts/serve-wasm.sh [port]        # default port 8080
#
# Produces the three files a wasm-bindgen `--target web` page needs
# (index.html, viewer.js, viewer_bg.wasm) in one self-contained
# directory and serves it on every interface, then prints the LAN URL
# to type into the phone. That URL is the whole point of the script.
#
# WHY THE RUSTFLAGS ARE HERE AND NOT EXPORTED. `getrandom` refuses to
# build for wasm32 until a backend is named, and naming one takes both
# halves: the `getrandom/wasm_js` feature (already in the viewer's
# `cfg(target_arch = "wasm32")` stanza) AND this cfg flag. Setting only
# the flag is the error docs/GQ6-RESURVEY.md §4 records. It is passed as
# a per-command prefix rather than an `export` because RUSTFLAGS
# silently REPLACES any .cargo/config.toml rustflags — see gate.sh's
# hazard list. The repo sets none today, so scoping it costs nothing and
# means a future config.toml entry is not quietly dropped by this script.
#
# WHY ONLY THE BUILD TAKES A BUILD SLOT. test-fast.sh re-execs itself
# under with-build-slot.sh, but this script's second half is an HTTP
# server that runs for as long as someone is holding the phone. Holding
# the machine-wide build mutex for that would block every other lane for
# the length of a demo, so the slot wraps the cargo invocation alone.
set -euo pipefail
cd "$(dirname "$0")/.."

PORT="${1:-8080}"
# Honours an overridden target dir; `target/` is the repo's ignored
# default (.gitignore `/target`), so the served tree is never committable.
TARGET_DIR="${CARGO_TARGET_DIR:-target}"
OUT_DIR="$TARGET_DIR/wasm-spike"
WASM_IN="$TARGET_DIR/wasm32-unknown-unknown/release/viewer.wasm"

# --- preflight: fail loud, with the command that fixes it -------------
#
# Both of these produce failures that read as something else if they are
# not caught here: a missing target surfaces as a wall of "can't find
# crate for `core`", and a wasm-bindgen CLI whose version differs from
# the wasm-bindgen crate in Cargo.lock produces a schema-mismatch error
# at IMPORT time in the browser, i.e. on the phone, where there is no
# console to read it in.

if ! rustup target list --installed | grep -qx wasm32-unknown-unknown; then
  echo "serve-wasm: the wasm32-unknown-unknown target is not installed." >&2
  echo "            fix: rustup target add wasm32-unknown-unknown" >&2
  exit 1
fi

# The required version is DERIVED from the lockfile rather than written
# down here, so it cannot drift from what eframe actually pins the day
# the dependency moves.
WB_WANT=$(awk '/^name = "wasm-bindgen"$/ { getline; gsub(/[",]/, ""); print $3; exit }' Cargo.lock)
[ -n "$WB_WANT" ] || { echo "serve-wasm: no wasm-bindgen entry in Cargo.lock" >&2; exit 1; }

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "serve-wasm: the wasm-bindgen CLI is not installed." >&2
  echo "            fix: cargo install wasm-bindgen-cli --version $WB_WANT" >&2
  exit 1
fi

WB_HAVE=$(wasm-bindgen --version | awk '{print $2}')
if [ "$WB_HAVE" != "$WB_WANT" ]; then
  echo "serve-wasm: wasm-bindgen CLI is $WB_HAVE but Cargo.lock pins the" >&2
  echo "            wasm-bindgen crate at $WB_WANT. A mismatch does not fail" >&2
  echo "            here — it fails in the browser with an opaque schema" >&2
  echo "            error, which is undebuggable on a phone." >&2
  echo "            fix: cargo install wasm-bindgen-cli --version $WB_WANT --force" >&2
  exit 1
fi

# --- build ------------------------------------------------------------
BUILD=(env RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
       cargo build -p viewer --features app --bin viewer
       --target wasm32-unknown-unknown --release)
if [ -n "${BUILD_SLOT_HELD:-}" ]; then
  "${BUILD[@]}"
else
  local-scripts/with-build-slot.sh -- "${BUILD[@]}"
fi

rm -rf "$OUT_DIR"
wasm-bindgen --target web --no-typescript --out-dir "$OUT_DIR" "$WASM_IN"

# The page ships beside the crate it belongs to and is COPIED, not
# symlinked, so the served directory stands alone: it can be zipped,
# scp'd, or handed to any other static server unchanged.
cp crates/viewer/web/index.html "$OUT_DIR/index.html"

# --- serve ------------------------------------------------------------
#
# `hostname -I` lists every address on the box; the first is the one a
# phone on the same LAN can route to. If there is none (no network,
# containers), say so rather than printing a URL that cannot work.
LAN_IP=$(hostname -I 2>/dev/null | awk '{print $1}')

echo
echo "serve-wasm: serving $OUT_DIR ($(du -sh "$OUT_DIR" | cut -f1))"
if [ -n "$LAN_IP" ]; then
  echo "serve-wasm: OPEN THIS ON THE PHONE ->  http://$LAN_IP:$PORT/"
else
  echo "serve-wasm: no LAN address found (hostname -I is empty); try http://localhost:$PORT/" >&2
fi
# Plain http over a LAN address is NOT a secure context, so the phone's
# navigator.gpu is absent and wgpu falls back to WebGL2. That is fine —
# egui-wgpu's default features enable wgpu/webgl — but it means the
# spike does not exercise the WebGPU path. index.html prints which one
# the browser actually offered.
echo "serve-wasm: (http, so no WebGPU on the phone — wgpu will use WebGL2)"
echo "serve-wasm: Ctrl-C to stop"
echo
# --bind 0.0.0.0 is what makes it reachable from the phone at all; the
# default binds loopback only. python3 maps .wasm to application/wasm,
# which streaming instantiation requires.
exec python3 -m http.server --bind 0.0.0.0 --directory "$OUT_DIR" "$PORT"
