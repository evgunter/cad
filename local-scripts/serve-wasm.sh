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

# The address a phone can route to is the source address this box would
# use for outbound traffic — NOT whatever `hostname -I` lists first,
# which on a machine running docker, a VPN, or a container runtime is
# routinely an interface nothing else on the network can reach.
detect_lan_ip() {
  if command -v ip >/dev/null 2>&1; then
    ip -4 route get 1.1.1.1 2>/dev/null | sed -n 's/.* src \([0-9.]*\).*/\1/p' | head -1
  elif command -v ipconfig >/dev/null 2>&1; then
    # macOS: ask for the default route's interface, then that
    # interface's address. `hostname -I` is GNU-only and absent here.
    local dev
    dev=$(route -n get default 2>/dev/null | sed -n 's/.*interface: //p' | head -1)
    for candidate in "$dev" en0 en1; do
      [ -n "$candidate" ] || continue
      if ipconfig getifaddr "$candidate" 2>/dev/null; then return; fi
    done
  fi
}

# The same two markers WSL itself sets that `frame::running_under_wsl`
# reads (see crates/viewer/src/frame.rs) — one detection, two languages.
running_under_wsl() { [ -n "${WSL_DISTRO_NAME:-}" ] || [ -n "${WSL_INTEROP:-}" ]; }

LAN_IP=$(detect_lan_ip || true)

echo
echo "serve-wasm: serving $OUT_DIR ($(du -sh "$OUT_DIR" | cut -f1))"

if running_under_wsl; then
  # THE ONE THAT WOULD OTHERWISE WASTE AN EVENING. WSL2 puts the distro
  # behind its own NAT, so the address detected above is the VM's and is
  # unreachable from anything else on the LAN — binding 0.0.0.0 in here
  # does not change that. Printing it as "open this on the phone" would
  # be a URL that simply times out, with nothing saying why.
  WIN_IP=$(powershell.exe -NoProfile -Command \
    '(Get-NetIPConfiguration | Where-Object { $_.IPv4DefaultGateway -ne $null }).IPv4Address.IPAddress' \
    2>/dev/null | tr -d "\r" | head -1 || true)
  echo "serve-wasm: WSL DETECTED — the phone cannot reach this address directly." >&2
  echo "serve-wasm:   WSL-internal: http://${LAN_IP:-unknown}:$PORT/  (works in the Windows browser)" >&2
  echo >&2
  echo "serve-wasm:   Two ways to reach it from the phone:" >&2
  echo >&2
  echo "serve-wasm:   1. Mirrored networking (Windows 11 22H2+, simplest):" >&2
  echo "serve-wasm:      put this in %USERPROFILE%\\.wslconfig, then \`wsl --shutdown\`:" >&2
  echo "serve-wasm:          [wsl2]" >&2
  echo "serve-wasm:          networkingMode=mirrored" >&2
  echo "serve-wasm:      then the Windows LAN address below just works." >&2
  echo >&2
  echo "serve-wasm:   2. Port-forward from Windows (any version), in an" >&2
  echo "serve-wasm:      ADMINISTRATOR PowerShell:" >&2
  echo "serve-wasm:          netsh interface portproxy add v4tov4 listenport=$PORT \\" >&2
  echo "serve-wasm:              listenaddress=0.0.0.0 connectport=$PORT connectaddress=${LAN_IP:-<wsl-ip>}" >&2
  echo "serve-wasm:          New-NetFirewallRule -DisplayName 'wasm spike' -Direction Inbound \\" >&2
  echo "serve-wasm:              -LocalPort $PORT -Protocol TCP -Action Allow" >&2
  echo >&2
  if [ -n "$WIN_IP" ]; then
    echo "serve-wasm:   Either way, the phone opens ->  http://$WIN_IP:$PORT/" >&2
  else
    echo "serve-wasm:   Then open http://<the-windows-LAN-ip>:$PORT/ on the phone" >&2
    echo "serve-wasm:   (\`ipconfig\` in a Windows shell; the Wi-Fi adapter's IPv4)." >&2
  fi
elif [ -n "$LAN_IP" ]; then
  echo "serve-wasm: OPEN THIS ON THE PHONE ->  http://$LAN_IP:$PORT/"
  echo "serve-wasm: (phone must be on the same Wi-Fi; a guest network or"
  echo "serve-wasm:  AP client-isolation will block it, as may a host firewall)"
else
  echo "serve-wasm: no routable address found; try http://localhost:$PORT/ locally." >&2
  echo "serve-wasm: (a container with no LAN interface cannot serve a phone at all)" >&2
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
