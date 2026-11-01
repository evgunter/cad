---
id: a-frame-turned-by-a-radian-angle-is-spelled-as-its-columns
kind: issue
title: A frame that is a world frame turned by an authored angle is spelled as its columns and decided, where a rotation mint needs no decision
status: open
opened: 2026-10-21
---

## What

Found by the class sweep of `frame/exact-frames-are-turns`. Several
callers hand the Gram–Schmidt mint the columns of a single rotation
of a world frame by an angle they authored, so they pay a decision
(and, in the tour, `tour_frame_axis`) for a frame
`RigidMotion::rotation` / `rotation_by` builds with none:

- `demos/tour/src/curvedcut.rs` `walls`: `sketch_frame` over
  `(cos φ, 0, −sin φ)` and `ŷ` — the xy frame turned `φ` (`PHI`,
  0.3 rad) about `ŷ`. The tour's split plane in `cut` is the same
  frame's normal.
- `crates/sweep/tests/reach_volume_backstop.rs` (two sites) and
  `review_blend1_r1_probes.rs`: `(x̂, (0, cos θ, sin θ))`, a turn `θ`
  about `x̂`.
- `crates/sweep/tests/continuation_pose.rs` and
  `a_keel_ring_in_an_elliptic_section.rs`: `(c, 0, −s)` / `(0, c, s)`
  pairs of the same shape.
- `crates/sweep/tests/witness_ladder.rs` and `review_m2_pr5.rs`:
  `(x̂, (0, s, s))` and `((s, 0, s), ŷ)` with `s = 1/√2`, an eighth
  of a turn — a written fraction, so `RigidMotion::turned(o, x̂, Turns::new(1, 8))`.

These are not the axis-aligned class the unit moved (their entries
are not `0` and `±1`), so they stayed on `sketch_frame` /
`sketch_from_axes`. Some fixtures may want the decided door on
purpose (to exercise a mint's rounding); each site says which before
it moves.

## The volume backstop left the operation path (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the backstop: `BooleanError::{ResultVolumeImplausible, VolumeUnmeasured, VolumeCorrupt, VolumeUndecided}`, `BooleanDecision::VolumeBackstop`, `AtRestPolicy::gate_volume_backstop` and the `volume_backstop*` predicates are gone. `reach_volume_backstop.rs` is now `curved_trims_build_at_their_volumes.rs`; its `tilted_about_x` frame is unchanged.
