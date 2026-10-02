---
id: swept-operand-merge-repairs-left-in-tests
kind: issue
title: Test repairs of swept operands that a run-wall sweep made no-ops
status: open
opened: 2026-10-01
priority: P3
cost: E
parent: swept-continuation-walls-reach-the-boolean-unmerged
---

Left behind by `swept-continuation-walls-reach-the-boolean-unmerged`.
Extrude and revolve now build one wall per run of collinear pieces,
and a full revolve sweeps every planar wall whole, so a
`merge_coplanar_faces` call (or a `repaired()` helper wrapping one)
applied to a SWEPT operand before a verb is a no-op: it finds no
same-key pair. The calls still compile and pass, and each one now
reads as if the operand needed a repair it does not.

The run-wall PR retired the ones its own rows tripped on (the lily
lantern's repair in `demos/tour/src/lily.rs`, the torus vessel's coda
in `demos/tour/src/torusvessel.rs`, `test_support.rs`'s docs). The
rest were not audited one by one; candidates are the sweep test files
that call `merge_coplanar_faces` on a body straight out of `extrude` /
`revolve` rather than out of a boolean, e.g.
`crates/sweep/tests/m5_pr9_cosurface_merge.rs`,
`crates/sweep/tests/review_m5_pr9_boss_probe.rs`,
`crates/sweep/tests/germ_torus_doors.rs`,
`crates/sweep/tests/pole_slit_window.rs`,
`crates/sweep/tests/ring_clearance_forms.rs` and the
`review_ring_clearance_*` probes.

## The row

For each call on a swept operand: assert the merge's `groups` is empty
(the operand arrives maximal) or delete the call and say so in the
row's doc. A call whose `groups` is NOT empty on a line-only profile
is a run the sweep failed to join, which is a bug in
`crates/sweep/src/swept.rs::wall_runs`, not a test to keep.
