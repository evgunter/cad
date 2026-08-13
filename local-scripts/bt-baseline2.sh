#!/usr/bin/env bash
# Temporary investigation script (NOT for merge): battery 2.
# `touch` only bumps mtime; rustc's incremental cache then finds every
# dep-graph input unchanged and reuses everything. That measures cargo's
# freshness path, not a real edit. These rows make a REAL semantic change
# to the generic core so downstream incremental caches genuinely invalidate.
set -u
cd "$(dirname "$0")/.."
CORE=crates/geom-core/src/real.rs
MARK='pub(crate) const __BT_MARKER: u64 ='

t() { local label="$1"; shift; local s e
  s=$(date +%s.%N); "$@" >/dev/null 2>&1; local rc=$?; e=$(date +%s.%N)
  printf '%-52s %8.1fs  (rc=%d)\n' "$label" "$(echo "$e - $s" | bc)" "$rc"; }

edit() { # append/replace a real item so the source genuinely differs
  sed -i "/$MARK/d" "$CORE"
  printf '%s %s;\n' "$MARK" "$1" >> "$CORE"
}
restore() { sed -i "/$MARK/d" "$CORE"; }
trap restore EXIT

echo "### REAL edit to $CORE (semantic change, invalidates downstream)"
cargo build --workspace --all-targets >/dev/null 2>&1
edit 1; t "5a. cargo check --all-targets (real edit)"   cargo check --workspace --all-targets
edit 2; t "5b. cargo check --all-targets (real edit)"   cargo check --workspace --all-targets
edit 3; t "5c. cargo build --all-targets (real edit)"   cargo build --workspace --all-targets
edit 4; t "5d. cargo build --all-targets (real edit)"   cargo build --workspace --all-targets
restore

echo
echo "### contrast: REAL edit to a LEAF crate (stl, no Real generics)"
LEAF=crates/stl/src/lib.rs
sedl() { sed -i "/$MARK/d" "$LEAF"; printf '%s %s;\n' "$MARK" "$1" >> "$LEAF"; }
cargo build --workspace --all-targets >/dev/null 2>&1
sedl 1; t "5e. cargo build --all-targets (leaf edit)"   cargo build --workspace --all-targets
sed -i "/$MARK/d" "$LEAF"
cargo build --workspace --all-targets >/dev/null 2>&1

echo
echo "### clean builds"
cargo clean >/dev/null 2>&1
t "1. clean cargo build (libs, dev)"                    cargo build
cargo clean >/dev/null 2>&1
t "1b. clean cargo build --all-targets (dev)"           cargo build --workspace --all-targets
cargo clean >/dev/null 2>&1
t "2. clean cargo build --release (libs)"               cargo build --release
cargo clean >/dev/null 2>&1
t "2b. clean cargo build --release --all-targets"       cargo build --release --workspace --all-targets

echo
echo "### the SECOND feature lane: --features interval (separate resolution)"
cargo clean >/dev/null 2>&1
t "6a. clean build --all-targets, default features"     cargo build --workspace --all-targets
t "6b. THEN build --all-targets --features interval"    cargo build --workspace --all-targets --features interval
t "6c. THEN build --all-targets, default (back-swap)"   cargo build --workspace --all-targets
