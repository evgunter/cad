---
id: merge-faces-should-panic-rows-on-a-debug-assert-fail-in-release
kind: issue
title: Two merge_faces should_panic rows rest on an ungated debug_assert and fail in a release test run
status: open
opened: 2026-10-04
---


(TOPO implementer, from `stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body`, PR 4029.)

`merge_faces::tests::a_curved_run_told_planar_is_caught_before_any_surgery`
and `a_planar_group_told_curved_is_caught_before_any_surgery`
(`crates/topo/src/merge_faces.rs`, near :4211 and :4224) are
`#[should_panic]` on a `debug_assert!` in `merge_group`, with no
`#[cfg(debug_assertions)]` gate. Under `cargo test --release` with
`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false` they fail: the assert is
compiled out, so nothing panics. They fail the same way on `main` before
PR 4029. No CI job runs them in that profile today (the
`corrupt input (release profile)` job filters to other modules), so the
gap only shows when someone runs the whole lib in release. Gate them on
`debug_assertions`, as the file's other debug-only rows are.
