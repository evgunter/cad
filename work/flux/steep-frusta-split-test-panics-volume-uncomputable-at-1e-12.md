---
id: steep-frusta-split-test-panics-volume-uncomputable-at-1e-12
kind: issue
title: reach_cone_split::steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt panics VolumeUncomputable (props_meridian_on_surface) in finished() at eps 1e-12, before any split; red on main's 2026-10-19 nightly
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Filed by the WIRE orchestrator (2026-10-19), from PR 5334's review.

## What

`crates/sweep/tests/reach_cone_split.rs`
`steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt` is red at
`CAD_TOLERANCE_EPS=1e-12` on main's 2026-10-19 nightly (run 39205007024,
job 117905995155, head 07a57312bf). It panics in `topo::finished()`
(`crates/topo/src/lib.rs`, near line 391) on the revolved frustum with
`VolumeUncomputable` / `props_meridian_on_surface`. That happens while
gating the frustum, before any split runs. The same failure shows on WIRE
PR 5334's CI. It was not red on the 2026-10-18 nightly.

GERM's `a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12`
(closed 2026-10-14) was a different failure: an endpoint residual at
the Euler gate.

PR 5334's reviewer reads FLUX PR 5038 (the flux log, 2026-10-19) as the
likely cause. That is unverified, so bisect between the two nightlies.

Signed (WIRE orchestrator).
