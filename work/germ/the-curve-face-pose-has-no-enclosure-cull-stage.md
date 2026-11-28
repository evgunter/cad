---
id: the-curve-face-pose-has-no-enclosure-cull-stage
kind: issue
title: The curve × face pose has no enclosure cull stage ahead of its closed rungs
status: closed
opened: 2026-11-20
closed: 2026-11-25
---


Found by U3a (PR 7226), which built `curve_face::pose` but did not move
the cull. `docs/GERM-CROSSING-U3-SPEC.md` §a.2 ("Apart's cull stage")
asks for `reduce::conic_clearance` (`crates/topo/src/boolean/reduce.rs`,
`bool_conic_curved_clearance`) to move into the pose as `Apart`'s cull
stage, ahead of the closed rungs, one match on surface kind replacing
`implicit::conic_residual_extremes`' harmonic-else-sampled path; in band
it hands on and never refuses alone.

Today the sweep still reads the clearance itself before the door
(`reduce.rs`, the `conic_clearance` call in the curved lane), and
`curve_face::pose` reads no enclosure: a conic definitely clear of its
surface is decided `Apart` by the closed rungs alone, where they can.

## What would close it

The cull as the pose's first stage for the conic arms (`pose.rs`,
`conic_quadric` and `conic_torus`), its row `curve_face_pose_clearance`,
and the sweep reading the pose's `Apart` in place of its own call; a row
pinning that a definite clearance answers `Apart` where the closed rungs
would escalate.

## Closed

GERM U4b: the cull is the pose's first stage on the conic arms
(`curve_face::pose::culled`, `curve_face_pose_clearance`), re-posed as a
ball's: the whole conic in the ball of its larger semi-axis about its
centre, each kind's 1-Lipschitz `g` decided past `L·ℓ`. `conic_clearance`
and `bool_conic_curved_clearance` are gone; the sweep reads the pose's
`Apart`. The row the issue asked for is
`cull_rows::a_definite_clearance_answers_apart_where_the_closed_rungs_refuse`.
