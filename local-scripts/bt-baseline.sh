#!/usr/bin/env bash
# Temporary investigation script (NOT for merge): build-time baseline battery.
set -u
cd "$(dirname "$0")/.."
CORE=crates/geom-core/src/real.rs

t() { # t <label> <cmd...>
  local label="$1"; shift
  local s e
  s=$(date +%s.%N)
  "$@" >/dev/null 2>&1
  local rc=$?
  e=$(date +%s.%N)
  printf '%-46s %8.1fs  (rc=%d)\n' "$label" "$(echo "$e - $s" | bc)" "$rc"
}

echo "### incremental (warm dev tree), touch $CORE"
touch $CORE; t "3a. cargo build (lib only)"            cargo build
touch $CORE; t "3a'. cargo build (lib only, repeat)"   cargo build
touch $CORE; t "4a. cargo check (lib only)"            cargo check
touch $CORE; t "4a'. cargo check (lib only, repeat)"   cargo check

echo
echo "### incremental --all-targets (the real dev loop)"
cargo build --workspace --all-targets >/dev/null 2>&1   # warm it
touch $CORE; t "3b. cargo build --all-targets"          cargo build --workspace --all-targets
touch $CORE; t "4b. cargo check --all-targets"          cargo check --workspace --all-targets

echo
echo "### touch a leaf crate for contrast (topo)"
touch crates/topo/src/lib.rs; t "3c. cargo build --all-targets (topo)" cargo build --workspace --all-targets
