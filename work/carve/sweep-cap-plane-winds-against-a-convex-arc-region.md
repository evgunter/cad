---
id: sweep-cap-plane-winds-against-a-convex-arc-region
kind: issue
title: extrude and loft mint a cap plane inside out when a big convex arc makes the profile's inscribed polygon wind against the region
status: dispatched
opened: 2026-09-24
priority: P0
cost: M
refs: [3190]
branch: carve/cap-winds-with-the-region
---


## The defect

Filed by ATREST-4, outside its fence. `sweep`'s cap planes are
oriented by Newell over the profile's vertices plus ONE apex per arc
segment — `cap_points` in `crates/sweep/src/swept.rs`, consumed by
`newell_plane(..)` for the bottom and top caps in
`crates/sweep/src/extrude.rs` and `crates/sweep/src/loft.rs`, and for
the start and end caps in `crates/sweep/src/revolve/partial.rs`. That
polygon is inscribed in the region, not the region. A CONCAVE arc only
makes it larger than a positive region, but a CONVEX arc makes it
smaller, and a large one flips its sign.

**Repro (the PR 3190 reviewer's C-shape).** A counterclockwise
profile: one convex 350° arc of radius 1 from 1∠5° to 1∠355° (bulge
tan 87.5°), a radial line in to 0.9∠355°, a clockwise polyline
through 0.9∠270°, 0.9∠180°, 0.9∠90° to 0.9∠5°, and a radial line out.
Signed area: true region +1.437; vertex polygon −1.704; vertices plus
the arc's apex −1.530.

Measured through the public doors (2026-09-24):

- `Profile::validate` accepts it (its own orientation pass is
  arc-exact).
- `extrude(.., Distance(1.0))` builds it; the bottom cap (z = 0)
  carries a `+z` plane normal and the top cap (z = 1) a `−z` one, both
  `sense: true` — each outward normal points INTO the material.
- `loft_body` over two copies does the same.
- A partial revolve (`Revolution::Partial(π/2)` about the sketch y
  axis, the C-shape moved to x = 3) mints both wedge caps inside out.
- Tier 3 now refuses both bodies: `LoopRoleInverted` at exactly the two
  caps (check 6's planar arm reaches arc-bearing loops since ATREST-4).
  Before that, the inside-out caps certified.

Pinned in `crates/sweep/tests/m5_s10_face_sense.rs`:
`a_convex_arc_c_shape_extrude_and_loft_caps_point_out_and_certify`.

## The fix's shape

Orient the cap by the region's arc-exact winding, which `profile`
already decides at validation (`loop_orientation`, the circular-segment
correction). The plane's POSITION can stay Newell over `cap_points`;
only its orientation is wrong. A partial revolve's caps use the same
`cap_points` and are affected too, measured: the C-shape moved three
units off the axis and revolved a quarter turn mints both wedge caps
inside out, and tier 3 refuses exactly those two
(`a_convex_arc_c_shape_partial_revolve_caps_point_out_and_certify`,
same file).

## Built (2026-10-06)

`sweep::swept::cap_plane` is the one home of a cap's plane for
`extrude`, `loft_body` and the partial `revolve`. The plane is Newell's
over `cap_points`, unchanged. The decision `cap_plane_orientation`
compares its normal with the cap's expected outward normal: the placed
sketch normal, signed by the profile's validated winding (an outer
loop is canonical counterclockwise), the verb's traversal reversal and
the cap's end. Where the two disagree, the plane is negated. A cap
whose Newell normal already agrees keeps its bits. The escalation is
typed as `sweep::CapPlaneError` inside each verb's `CapPlane`. The
decision presumes a rigid, right-handed placement, which the public
`SketchPlane::new` does not enforce
(`work/paths/sketch-plane-holds-the-affine-and-the-witness-dies-at-the-read-boundary.md`).
Pinned in `crates/sweep/tests/m5_s10_face_sense.rs`: the two rows
above, `a_convex_arc_c_shape_caps_point_out_against_the_normal_and_reversed`
and `every_new_cap_plane_refusal_states_one_ending_and_no_declaration`.
