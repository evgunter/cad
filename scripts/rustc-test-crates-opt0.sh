#!/usr/bin/env bash
# rustc-test-crates-opt0.sh — the PR test job's RUSTC_WORKSPACE_WRAPPER,
# and local-scripts/test-fast.sh's.
#
# A workspace member's integration-test binary (`tests/all.rs`, which
# `scripts/gates/test-aggregation.sh` holds to one per member) builds at
# opt-level 0 when cargo asked for 1. Every library, every dependency, each
# lib's own unit-test harness, and a package whose profile override asks
# for more (`mesh` at 2) keep the level cargo chose.
#
# WHY. An integration binary's build is mostly codegen of the kernel's
# generics at the scalars the suites drive (`Interval`, `Sym<_>`, `Dual<_>`),
# which no library instantiates and so none can share. The `f64` code the
# suites mostly run is shared from the libraries, which stay at opt 1. A
# lib's unit-test harness is left alone: it recompiles the crate's own code,
# and at opt 0 its tests ran that code unoptimized. The measurement is in
# work/green/kernel-shard-long-pole-is-one-test-binary.md.
#
# CAD_OPT0_LOG, when set, gets one line per integration binary:
# `rewrote <src>` or `kept <src> <cargo's opt-level>`. ci.yml's test job
# checks it against the binaries the build wrote, so a wrapper that stops
# applying (cargo no longer spelling `-C opt-level=1`, the profile moving)
# goes red instead of silently costing the build again.
#
# rustc takes the last `-C opt-level`, so appending one overrides cargo's.
set -euo pipefail
rustc=$1
shift
test_crate=false
src=
level=none
prev=
for arg in "$@"; do
  case $arg in
    --test) test_crate=true ;;
    */tests/all.rs) src=$arg ;;
  esac
  case $prev/$arg in
    -C/opt-level=*) level=${arg#opt-level=} ;;
  esac
  prev=$arg
done
if $test_crate && [ -n "$src" ]; then
  if [ "$level" = 1 ]; then
    [ -z "${CAD_OPT0_LOG:-}" ] || echo "rewrote $src" >> "$CAD_OPT0_LOG"
    exec "$rustc" "$@" -C opt-level=0
  fi
  [ -z "${CAD_OPT0_LOG:-}" ] || echo "kept $src $level" >> "$CAD_OPT0_LOG"
fi
exec "$rustc" "$@"
