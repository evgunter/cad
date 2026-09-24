---
id: sweep-cap-plane-winds-against-a-convex-arc-region
kind: issue
title: extrude and loft mint a cap plane inside out when a big convex arc makes the profile's inscribed polygon wind against the region
status: open
opened: 2026-09-24
priority: P0
cost: D
refs: [sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts]
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
- Tier 3 now refuses both bodies: `LoopRoleInverted` at exactly the two
  caps (check 6's planar arm reaches arc-bearing loops since ATREST-4).
  Before that, the inside-out caps certified.

Pinned in `crates/sweep/tests/m5_s10_face_sense.rs`:
`a_convex_arc_c_shape_cap_is_minted_inside_out_and_check_6_refuses_it`
— red the day the verbs orient the cap correctly, and re-cut then.

## The fix's shape

Orient the cap by the region's arc-exact winding, which `profile`
already decides at validation (`loop_orientation`, the circular-segment
correction). The plane's POSITION can stay Newell over `cap_points`;
only its orientation is wrong. A partial revolve's caps use the same
`cap_points` and are presumed affected (see the pin row's sibling
measurement in ATREST-4's PR).
