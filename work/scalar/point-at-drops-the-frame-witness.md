---
id: point-at-drops-the-frame-witness
kind: issue
title: point_at decides a frame witness and returns only its affine; a caller wanting the axis as a UnitVec3 re-asks the aim decision
status: closed
opened: 2026-09-19
closed: 2026-09-20
pr: 2896
---



## Where this came from

MSOLVE-8 (`docs/MSOLVE-8-SPEC.md` §3), which wanted the mate solve's
directions read as `UnitVec3` witnesses at the mate frame read, through
"the `OrthoFrame` ladder that matches `point_at`'s construction". No
such public ladder exists, measured against the tree:

- `geom_core::linalg::frame::point_at` (`crates/geom-core/src/linalg/frame.rs`)
  mints the aim as `UnitVec3::new(aim, "frame_point_at_aim", band)`,
  forms `perp = roll_reference × aim`, and builds the frame through
  `OrthoFrame::from_aim(eye, unit, perp, …)` — which is
  `pub(in crate::linalg)` — then returns `.to_affine()`. The witness
  the ladder held is dropped at the return.
- The public aim door, `OrthoFrame::from_aim_and_reference`, decides
  the Gram–Schmidt residual `reference − aim·(reference·aim)` and puts
  the reference along `u`; `point_at` puts the reference in the local
  `+Y` half-plane and normalizes the cross product without a residual
  subtraction. The two are a quarter turn apart in roll, and passing
  `ref × aim` as the reference reproduces `point_at` only up to the
  residual's rounding (`perp·aim` is not exactly zero for an oblique
  aim), so it is not the same affine bit for bit in general.

MSOLVE-8 therefore took the re-mint road the spec sanctions when a
`geom-core` door is missing: `editor_core::MateFrame::axis(tol)`
(`crates/editor-core/src/mate.rs`) re-asks the ONE aim decision through
`UnitVec3::new` under the same funnel name on the same vector, which is
the placement's third column bit for bit (pinned by
`crates/editor-core/tests/msolve8_levered_clash.rs`,
`the_axis_witness_is_the_placements_third_column_bit_for_bit`), and
projects the direction door's refusal onto `FrameError` exactly as
`point_at`'s private `refused_direction` does — an eight-line restating
of a private map, pinned equal by
`the_axis_door_refuses_exactly_as_the_placement_does`.

## The door this wants

A `point_at` sibling in `geom-core` that returns the `OrthoFrame<T>`
(or the affine and the witness together), so the caller that holds the
frame at the read holds its witness without a second decision and
without restating the refusal projection. `MateFrame::placement` and
`MateFrame::axis` then become one read of that door, and the
`frame_point_at_aim` funnel stops recording the mate frame's aim twice
per solve. SCALAR's ground (`crates/geom-core/src/linalg/frame.rs`,
`ortho_frame.rs`); the consumer is MSOLVE's `mate.rs`.

## Closed (2026-09-20, PR 2896)

Done by MSOLVE-8 itself, on the ruling at its fix pass that the fence
widens by one door: `geom_core::linalg::frame::point_at_frame`
(`crates/geom-core/src/linalg/frame.rs`) returns the `OrthoFrame`
`point_at` builds, and `point_at` is that door's `to_affine()` — one
construction, bit-identical by construction and pinned over a grid by
`point_at_is_point_at_frame_to_affine_bit_for_bit`. `MateFrame::frame`
reads it; `placement` is its affine and `axis` its `w`; the re-mint,
the restated refusal projection and the third aim decision per mate
are gone (`msolve8_levered_clash::kstats_aim_decided_twice_per_mate`).
