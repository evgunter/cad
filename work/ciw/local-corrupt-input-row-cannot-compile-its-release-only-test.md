---
id: local-corrupt-input-row-cannot-compile-its-release-only-test
kind: issue
title: The local corrupt-input row builds release with debug assertions on, so its release-only test never compiles and the row is always red; the parity checker cannot see it
status: open
opened: 2026-09-28
priority: P3
cost: E
---


## What

The hosted `corrupt input (release profile)` job (`.github/workflows/ci.yml`)
sets `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS: "false"`. Its local mirror,
`topo_release` in `local-scripts/ci-local.sh`, does not, and the workspace's
`[profile.release]` turns debug assertions on. So
`foreign_parent_loop_garbage_in_garbage_out_release`
(`crates/topo/src/review_m1_pr2/release_corruption.rs`), which is
`#[cfg(not(debug_assertions))]`, never compiles locally, and the row's own
guard fails: `ERROR: garbage_in_garbage_out_release did not run`. Every
full local run is red on this row, whatever the change. Seen on GERM's full
local gate (PR 3265, 2026-09-26).

`scripts/check-ci-mirror-parity.py` does not catch the divergence, because
its env arm treats every `CARGO_PROFILE_*` name as a throughput knob. This one
changes which tests COMPILE, so it is semantic.

## Fix shape

Set the same variable as a prefix on the local row's `cargo test`, and teach
the checker that `CARGO_PROFILE_*_DEBUG_ASSERTIONS` (and `*_OVERFLOW_CHECKS`)
are semantic, with a selftest case for a one-sided one.
