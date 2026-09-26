---
id: cylinder-wall-trim-overcovers-a-tilted-section
kind: issue
title: point_in_solid's cylinder wall arm reads a wall bounded by a tilted planar section as its vertex rectangle and answers In across the cut (the cut cylinder)
status: open
opened: 2026-09-25
priority: P0
cost: H
refs: [ATREST-9]
---

Found by ATREST-9's fix pass (PR #3204), measuring its new ellipse arm
on the corpus's `cut_cylinder` shape. A false ANSWER from the certified
walk, not a refusal, on the cylinder arm of
`crates/topo/src/boolean/solid_contain.rs` — CONTACT's ground.

## What was measured

The fixture is a unit cylinder of height 2.5 (an extruded two-arc
disc), split by the plane through `(0, 0, 1.25)` with normal
`(sin 0.3, 0, cos 0.3)`; the corpus's `cut_cylinder` is the upper half.
The halves' volumes are exactly `π·1.25`, and their cut face is bounded
by two `Ellipse` arcs. Probed on a 5³ grid of points clear of the
boundary, at six rigid poses (identity included), through
`point_in_solid`:

- **62 false `In`, no false `Out`**, spread over both halves and every
  pose. For example, upper half at identity: `(−0.4, 0.852, 1.0584)`,
  `(0, 0.852, 1.0584)` and `(0.4, 0.852, 1.0584)` read `In`, but all
  three lie BELOW the cut plane (elevations −0.30, −0.18, −0.06).
- 282 typed `VolumeUncertified` refusals: rays that crossed nothing,
  whose at-infinity side needs the closed-form props lane, which does
  not take the tilted-section wall. That is honest, and not this row.

Pinned `#[ignore]`d as
`crates/sweep/tests/pis_arc_capped_poses.rs::the_cut_cylinder_reads_its_truth`.
It goes green when this row is fixed.

## Cause (by reading; the ray through the example point is traced
below)

Take the upper half at identity, `q = (0, 0.852, 1.0584)`, and the
schedule's first ray `+x`:

- it meets the cut plane at `x = 0.619`, where `x² + y² = 1.109 > 1`,
  so outside the cut face — correctly missed;
- it meets the cylinder at `x = 0.523`, at `z = 1.0584`, whose
  elevation is −0.028. That point is BELOW the cut, so it is not on
  the upper half's wall face.

`cylinder_chart_trim` serves the wall face the rectangle
`[azimuth] × [height]` read off its boundary VERTICES. That is exact
only for the ISO-BOUNDED class (rims and meridians). Its own premise
paragraph says so: "a wall bounded by a tilted section takes its
height extreme in an edge's interior, and this rectangle then
under-covers it". Here it over-covers too: the section's end vertices
sit at heights `1.25 ∓ 0.309`, so the rectangle reaches down to
`0.941` on the side where the section is high. The ray lane
"premises it from construction" (the same doc), and a split or boolean
constructs exactly the non-iso wall.

The face-level door (`contain::curved_face_containment`) checks the
iso-bounded class before reading the trim (`iso_bounded_wall`); the ray
lane does not.

## What a fix has to decide

Two routes:
- exact membership for the non-iso wall: parity in the chart, where a
  planar section's image is `h(az) = h₀ − k·cos(az − φ)`;
- a typed refusal in the ray lane when the wall is not iso-bounded,
  confined like ATREST-9's `EdgeCarrierUnsupported`: refuse only a hit
  the rectangle and the section could disagree on.

Either way, the rectangle must stop answering for a class it
misstates. The cone and sphere chart trims fold boundary images the
same way (`torus_chart_windows`' "this is a class" paragraph). They
are unmeasured under tilted sections.

