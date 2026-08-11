#!/usr/bin/env bash
# setup-build-env.sh — install mold and apply this machine's local build
# configuration (mold linker + line-tables-only debuginfo).
#
#   scripts/setup-build-env.sh [--print]
#
# WHY THIS EXISTS: the local box is a 4-core/8-thread i7-1065G7 (a 15W
# laptop part, sustaining ~1.5 GHz) with 10 GB of WSL RAM, serving up to
# ~10 concurrent agent lanes through the width-1 `with-build-slot.sh`
# mutex. Every lane's build is therefore on one queue, and the two knobs
# below are the measured way to make that queue drain faster. #174 landed
# them on HOSTED CI and deliberately did not mirror them locally; that
# call is superseded here by the agent-concurrency the box now carries.
#
# WHY ~/.cargo/config.toml AND NOT THE REPO'S .cargo/config.toml:
#   * The repo file is checked out on CI runners too. mold lives at a
#     machine-specific path here (~/.local, since Ubuntu 20.04 has no
#     `mold` package), and CI's non-build jobs install no mold at all — a
#     committed linker flag would fail them.
#   * The flags MUST apply to every cargo invocation on this machine.
#     RUSTFLAGS is part of cargo's fingerprint, so if only slot-wrapped
#     builds carried them, every unwrapped `cargo check` an agent ran in
#     its own shell would invalidate the whole target/ dir and the next
#     wrapped build would invalidate it back. That thrash would cost far
#     more than the knobs save, which is why this is machine-wide config
#     rather than an export inside with-build-slot.sh.
#   * `scripts/gate.sh` unsets RUSTFLAGS to protect its warm target dir.
#     Config-file rustflags are not env, so the gate sees them CONSISTENTLY
#     on every run — no re-fingerprinting, which is what that unset is for.
#
# WHY gcc's -B AND NOT -fuse-ld=mold: this machine's gcc is 9.4 (Ubuntu
# 20.04); `-fuse-ld=mold` needs gcc 12.1+. mold ships a `libexec/mold/ld`
# shim exactly for this, and `-B<dir>` makes gcc resolve `ld` to it. That
# gcc-version gap is the most likely reason #174 read a local mold as a
# heavier lift than it is.
#
# DETERMINISM (D9): `-C link-arg=-B...` is a LINK argument. It cannot move
# rounding or instruction selection — the same reasoning #174 recorded for
# CI. `debug=` controls DWARF emission only; `debug-assertions` and
# `overflow-checks` are untouched, so fail-loud postconditions keep their
# teeth and backtraces keep file:line. What is given up is variable
# inspection under a debugger.
#
# COST: changing these knobs re-fingerprints every lane, so each lane pays
# ONE cold rebuild the next time it builds. After that it is warm forever.
set -euo pipefail

MOLD_VERSION=2.41.0
PREFIX="$HOME/.local"
CONFIG="$HOME/.cargo/config.toml"
MOLD_LIBEXEC="$PREFIX/libexec/mold"

want_config() {
  cat <<EOF
# Machine-local build configuration — written by scripts/setup-build-env.sh
# in the cad repo. See that script for the full rationale; the short of it:
# this box serves ~10 agent lanes through one build mutex, and these are the
# measured knobs that make the queue drain. NOT repo-committed on purpose —
# CI runners must not see a machine-specific linker path.

[target.x86_64-unknown-linux-gnu]
# mold via gcc's -B shim (this gcc is 9.4; -fuse-ld=mold needs 12.1+).
# A link argument: cannot affect codegen or rounding (D9).
rustflags = ["-C", "link-arg=-B$MOLD_LIBEXEC"]

[profile.dev]
# Full DWARF across the workspace's test binaries is the bulk of both build
# time and target/ size. line-tables-only keeps backtraces with file:line
# and loses only debugger variable inspection. debug-assertions and
# overflow-checks are NOT affected.
debug = "line-tables-only"

[profile.test]
debug = "line-tables-only"
EOF
}

if [ "${1:-}" = "--print" ]; then
  want_config
  exit 0
fi

# --- mold ------------------------------------------------------------
if [ -x "$MOLD_LIBEXEC/ld" ]; then
  echo "mold: already installed ($("$PREFIX/bin/mold" --version | head -1))"
else
  echo "mold: installing $MOLD_VERSION to $PREFIX (no Ubuntu 20.04 package exists)"
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT
  tarball="mold-$MOLD_VERSION-x86_64-linux.tar.gz"
  if command -v gh >/dev/null 2>&1; then
    gh release download "v$MOLD_VERSION" --repo rui314/mold --pattern "$tarball" --dir "$tmp"
  else
    curl -fsSL -o "$tmp/$tarball" \
      "https://github.com/rui314/mold/releases/download/v$MOLD_VERSION/$tarball"
  fi
  tar xzf "$tmp/$tarball" -C "$tmp"
  mkdir -p "$PREFIX/bin" "$PREFIX/libexec" "$PREFIX/lib"
  cp -r "$tmp/mold-$MOLD_VERSION-x86_64-linux/bin/." "$PREFIX/bin/"
  cp -r "$tmp/mold-$MOLD_VERSION-x86_64-linux/libexec/." "$PREFIX/libexec/"
  cp -r "$tmp/mold-$MOLD_VERSION-x86_64-linux/lib/." "$PREFIX/lib/" 2>/dev/null || true
  echo "mold: installed ($("$PREFIX/bin/mold" --version | head -1))"
fi

# Prove gcc actually resolves `ld` to mold under -B before writing config
# that depends on it — a silent fallback to GNU ld would look like the
# knob simply not working.
resolved=$(readlink -f "$(gcc -B"$MOLD_LIBEXEC" -print-prog-name=ld)")
case "$resolved" in
  *mold) echo "linker: gcc -B resolves ld -> $resolved" ;;
  *) echo "ERROR: gcc -B$MOLD_LIBEXEC resolves ld to $resolved, not mold" >&2; exit 1 ;;
esac

# --- cargo config ----------------------------------------------------
mkdir -p "$(dirname "$CONFIG")"
if [ -e "$CONFIG" ] && ! grep -q 'written by scripts/setup-build-env.sh' "$CONFIG"; then
  echo "ERROR: $CONFIG exists and was not written by this script — refusing to" >&2
  echo "       clobber it. Merge the block from \`$0 --print\` by hand." >&2
  exit 1
fi
want_config > "$CONFIG"
echo "config: wrote $CONFIG"
echo
echo "Every lane pays ONE cold rebuild on its next build, then stays warm."
