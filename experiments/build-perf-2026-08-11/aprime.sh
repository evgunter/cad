#!/usr/bin/env bash
# aprime.sh — rerun the BASELINE config under current machine conditions.
#
# WHY: the A baseline (4189 s) and the C1 run (186 s) were taken ~2 h
# apart. A 22x gap is too large to attribute to mold + line-tables-only on
# CI's evidence (-38%), so the gap needs a control taken NOW, against the
# same load, before any of it is reported as the knobs' doing.
#
# Env overrides beat ~/.cargo/config.toml, so this restores the old world
# without touching the machine config the other lanes are now using:
#   RUSTFLAGS=""            (an empty-but-SET env var overrides config rustflags)
#   RUSTC_WRAPPER=""        (no sccache)
#   CARGO_INCREMENTAL=1     (incremental back on)
#   CARGO_PROFILE_*_DEBUG=2 (full DWARF)
set -uo pipefail
cd "$(dirname "$0")/cad"

LOG=../aprime.log
: > "$LOG"
say() { echo "$*" | tee -a "$LOG"; }

export RUSTFLAGS=""
export RUSTC_WRAPPER=""
export CARGO_INCREMENTAL=1
export CARGO_PROFILE_DEV_DEBUG=2
export CARGO_PROFILE_TEST_DEBUG=2

say "A' baseline control (full DWARF, incremental, no sccache, GNU ld)"
say "load at start: $(cut -d' ' -f1-3 /proc/loadavg)"
say "MemAvailable:  $(awk '/MemAvailable/{printf "%.1f GB", $2/1048576}' /proc/meminfo)"

cargo clean >/dev/null 2>&1
t0=$(date +%s)
timeout 1500 cargo build --workspace --all-targets >>"$LOG.raw" 2>&1
rc=$?
t1=$(date +%s)
say "$(printf "A%s cold, baseline config %5ds (rc=%d; 124=hit 25min cap)" "'" "$((t1 - t0))" "$rc")"
say "    target/: $(du -sh target 2>/dev/null | cut -f1)"
say "    load at end: $(cut -d' ' -f1-3 /proc/loadavg)"
say "    peak swap used: $(free -m | awk '/Swap/{print $3" MB"}')"
say "done"
