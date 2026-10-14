---
id: boolean-in-span-readings-of-grazing-roots-are-levered-by-arc-length
kind: issue
title: boolean: bool_wall_root_in_span (line × cylinder or sphere) and bool_carrier_cross_in_span read a near-graze root's span by arc length, with no root-slack bound
status: open
opened: 2026-10-08
---


(TANG implementer, the in-span class sweep of PR 4246's fourth review.
The files are CLEAVE's and HONE's.)

## What

A root found near a graze is ill-conditioned. A carrier moved by δ
moves it by δ over the slope it crosses at, and near a graze that slope
is small. A span reading levered by arc length alone can therefore be
decided while the root lies within its own error of the span's end.
PR 4246 found and fixed this shape in the ring lane
(`split_ring_path_in_span`). The sweep for it here:

- **`bool_wall_root_in_span`** (`crates/topo/src/boolean/reduce.rs:3431`)
  reads `gap · speed_at(end)`, which is arc length.
  - **The conic doors are covered.** Circle and ellipse against a
    sphere, wall or cone, circle and ellipse against a torus, and line
    against a cone each certify a root slack: the root's error over the
    slope, decided inside the zero band (`circle_roots.rs:943`,
    `circle_torus.rs:497`, `ellipse_torus.rs:100`, `reduce.rs:3705`).
    A root that passes it is within the zero band of its true place, so
    the arc-length margin is sound there.
  - **Line against a cylinder or a sphere is not covered.**
    `solid_contain::line_wall_roots` (`solid_contain.rs:4107`) and
    `line_sphere_roots` (`:4245`) decide only the discriminant, as the
    sagitta `disc/|d⊥|²/(2r)`. A decided graze `g ≥ Kε` leaves the root
    error at `≈ δ·√(r/(2g))`, far past the band, with no slack row.
- **`bool_carrier_cross_in_span`** (`carrier_cross.rs:186`, called at
  `:161`) reads `gap · metres_per_param`, again arc length. The meeting
  comes from `meetings` (`carrier_cross.rs:228`): a line through a
  circle's or an ellipse's plane, or `circle_meets_plane` (`:381`).
  These are decided `transverse` by a sine levered at the reach or the
  radius. A decided sine `≥ Kε/arm` still leaves the meeting's error at
  `δ·arm/(Kε)`.

## What it costs

A root or meeting within its own error of an end of the swept span,
read as strictly inside or outside. Strictly inside splits the edge a
hair from its vertex, a sliver. Outside skips a crossing that the
endpoint arms do not see either. Nothing is known to reach it. The
corpora that pass today pass with the zero band absorbing the error.

## Fix

Either add a root-slack rung to the two line doors and to `meetings`,
as the conic doors have, or lever each span reading by the crossing's
slope (`gap · speed · |n̂·τ̂|`), as PR 4246 does. Port PR 4246's
vertex-graze rows
(`topo::ring_path::graze_rows::a_graze_at_a_smooth_vertex_never_decides_a_wrong_parity*`)
as the guard.

## The transverse case over-reads (TANG, 2026-10-08)

The arc-length reading also over-reads a transverse crossing near a
span's end, by `1/sin θ`. The tube of
`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs` at radius
`R ± 0.9·zero`, unioned with `dome_on_the_cap()`, discs `Rest`: the
dome's rim stands `0.9·zero` off the tube's wall, which is inside the
zero band. The dome's meridian meets the wall at 45°, `√2·0.9·zero` along
its arc from the rim. Both member orders escalate
`bool_wall_root_in_span` at margin `±1.2727919974285932e-9` (ε 1e-9; the
same `√2·0.9` multiple at 1e-6 and 1e-12). Levered by the crossing's
slope, as the Fix above proposes, the margin is the rim's own deviation,
`0.9·zero`, and the offset reads Zero. Pinned at its current reading by
`a_rim_offset_inside_the_zero_band_answers_alike_in_both_member_orders`;
that row's `±0.9` arm moves when this lands.
