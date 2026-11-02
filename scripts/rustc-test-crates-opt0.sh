#!/usr/bin/env bash
# rustc-test-crates-opt0.sh — the PR test job's RUSTC_WORKSPACE_WRAPPER.
#
# A workspace member's integration-test binary (its `tests/all.rs`, which
# `scripts/gates/test-aggregation.sh` holds to one per member) builds at
# opt-level 0 when cargo asked for 1. Every library, every dependency, each
# lib's own unit-test harness, and any package whose profile override asks
# for more (`mesh` at 2) keep the level cargo chose.
#
# WHY. An integration binary's build is mostly codegen of generics
# instantiated in it: the kernel's `T: Real` code at the scalars the suites
# drive (`Interval`, `Sym<_>`, `Dual<_>`), which no library instantiates and
# so none can share. At opt 1 the LLVM passes over that code are most of the
# test job's build. The `f64` instantiations the suites lean on are shared
# from the libraries, which stay at opt 1, so the suites run nearly as fast.
#
# A lib's unit-test harness is left alone because it compiles the crate's
# own code a second time: at opt 0 its tests run the crate unoptimized, and
# on a slow runner 14 of them crossed the per-PR 1 s bar where 0 did before.
#
# rustc takes the last `-C opt-level`, so appending one overrides cargo's.
# The paired measurement is in `.github/workflows/ci.yml`'s test job.
set -euo pipefail
rustc=$1
shift
test_crate=false
integration=false
at_opt1=false
prev=
for arg in "$@"; do
  case $arg in
    --test) test_crate=true ;;
    */tests/*.rs) integration=true ;;
  esac
  [ "$prev" = -C ] && [ "$arg" = opt-level=1 ] && at_opt1=true
  prev=$arg
done
if $test_crate && $integration && $at_opt1; then
  exec "$rustc" "$@" -C opt-level=0
fi
exec "$rustc" "$@"
