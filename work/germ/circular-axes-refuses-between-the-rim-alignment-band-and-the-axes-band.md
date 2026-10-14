---
id: circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band
kind: issue
title: Tilts between eps/r and sqrt(2 eps/r) refuse CircularAxes with no lever: pc_rim_alignment and ellipse_axes_distinct read two different quantities
status: open
opened: 2026-10-12
priority: P3
cost: M
design: true
branch: germ/circular-axes-band-gap
---

Found by the CLEAVE section-end design fork (PR 4810).

## What

`pc_rim_alignment` decides sin θ·r against the band, where θ is the
plane's tilt off square to a cylinder's axis. Past it, the table builds
an ellipse through `Curve3::ellipse`, whose `ellipse_axes_distinct`
decides (a − b) ≈ r·θ²/2. Tilts between about ε/r and √(2ε/r)
therefore pass the first check and refuse `CircularAxes` at the
second, with no lever. PR 4810 measured this: "1e-4 is not clear
enough off square" at ε = 1e-9 or so.

## Measured (2026-10-14, main at the branch's base)

`crates/sweep/examples/circular_axes_gap_probe.rs` sweeps
sin θ ∈ [ε/(4r), 4·√(2Kε/r)] (25 tilts, log-spaced) at r ∈ {1e-2, 1, 1e2}
and ε ∈ {1e-6, 1e-9, 1e-12}, reading the section table
(`geom_brep::intersect::plane_cylinder_section`, `plane_cone_section`)
and a split of a real body (`topo::splitting::split`) at each tilt.
Every scale and ε shows the same five bands, in order:

| r·sin θ | table | split |
|---|---|---|
| ≤ ε | `Rim` | splits |
| (ε, Kε) | `Escalated[pc_rim_alignment]` | refuses, in band |
| ≥ Kε, a − b ≤ ε | `Carrier(CircularAxes)` | refuses "construct the Circle carrier" |
| a − b ∈ (ε, Kε) | `Carrier(Escalated[ellipse_axes_distinct])` | refuses, in band |
| a − b ≥ Kε | `TiltedEllipse` | splits |

The third band is the gap: 8–17 of the 25 tilts per scale, widest at
1e-12 (r·sin θ from 10ε to about 4000ε at r = 1, ε = 1e-9). The
split's refusal ends "construct the Circle carrier, or …" through
`SplitJoinError::Section`'s fallback Display (`chord_join.rs`), a lever
no caller of a split or a Boolean holds.

## Which circle could stand in (derived, then measured)

Both curves below are centred where the axis pierces the plane.

- **The rim circle** (axis the cylinder's, radius r — what `Rim`
  serves). A section point at azimuth φ is on the wall at height
  r·cos φ·tan θ above the rim circle's plane, straight above the rim
  point at the same azimuth, so the miss is **r·tan θ**. That is
  `pc_rim_alignment`'s own margin up to 1/cos θ, so `Rim` is sound
  exactly where that row reads Zero. **In the gap the rim circle is
  NOT sound**: its measured miss is Kε up to thousands of ε.
- **The flat circle** (in the cutting plane, axis the plane normal,
  radius r = b). It lies exactly on the plane; the true section is the
  ellipse (a, b) about the same centre, so the miss is **a − b =
  r·(sec θ − 1)**, which is `ellipse_axes_distinct`'s own margin.
  Where that row reads Zero, the flat circle is within ε of the true
  section along its whole length. It stands off the wall by at most
  r·(1 − cos θ) ≤ a − b. Measured flat miss in the gap: < ε at every
  tilt, at every scale and ε (the largest was 0.94ε, at r = 1e-2,
  ε = 1e-9).

So "stand the circle in" is sound only as the flat circle. That is a
carrier `PlaneCylinderSection::Rim`'s public contract excludes ("carrier
axis is the CYLINDER axis, u_ref the cylinder's seam",
`intersect.rs` `PlaneCylinderSection`).

## The fork (needs a decision, not picked)

- **A. Serve the flat circle where `ellipse_axes_distinct` reads Zero.**
  This is sound (above) and is what D3's one kind per configuration and
  `EllipseInvalid::CircularAxes`'s own text prescribe ("this
  configuration is a Circle"). It moves a public contract: either
  `Rim`'s axis may be the plane normal, or a new variant is added
  (likewise `PlaneConeSection::AxisNormalCircle` for the cone sibling
  below). Every consumer would have to take a circle that is exact on
  the plane and within ε of the wall: the germ frame
  (`join.rs` `conic_section_frame_at`'s match on `Rim`), the chord lanes
  (`chord_join.rs` `agreed_section` and `section_case`), `offset_derive.rs`
  `section_closed`,
  and the cylinder face's pcurve, which is no longer an iso-line. The
  in-band sub-band (a − b ∈ (ε, Kε)) still escalates, and its
  recourse still names no tilt.
- **B. Refuse the whole span between the two Zero bands as one
  decision, typed with the tilt lever.** The table reads the tilt once
  (`pc_rim_alignment`). On a definite tilt it reads the semi-axis
  difference r·(sec θ − 1) itself, before the constructor, and refuses
  its Zero and in-band arms as one decision. The recourse: "square the
  plane to the axis (r·sin θ under ε), or tilt it until the ellipse's
  semi-axes differ by more than Kε (θ past about √(2Kε/r))". There is
  precedent in the tree: the fillet end cap already does this
  (`sweep/src/blend/battery.rs` `cap_transverse` maps both
  `CircularAxes` and `Escalated` to `BlendDecision::CapEllipse`, whose
  `FILLET3_CAP_ELLIPSE_RECOURSE` names exactly this tilt). No existing
  error changes meaning, but the table's public `SectionError` returns
  a different, typed refusal at these tilts. A configuration within ε
  of a circle is refused, and the user must move the plane by about
  √(2Kεr) of length to clear it.

## Siblings (same shape, swept 2026-10-14)

The shape: one predicate passes, then a second predicate on a derived
quantity refuses. The sweep grepped every non-test `Curve3::ellipse(`
caller.

- `plane_cone_section` (`pn_axis_normal` → `Curve3::ellipse`): the
  same gap, measured by the same probe. `CircularAxes` holds 10–16
  tilts per scale at ε = 1e-9 and 1e-12. A split of the frustum
  refuses either `CircularAxes` or, at the first few tilts, the rim
  edge's own plane-parallel escalation, which is levered at the rim
  radius rather than at `max(rim radius, extent)`. The decision on
  this item decides this arm too.
- `cylinder_cylinder_section`, equal radii, meeting axes (the bisector
  ellipse across both axes, semi-axes r/cos(φ/2) and r): after
  `cc_axes_parallel` reads φ definite, the constructor refuses at
  r·φ²/8 ≤ ε. This is the same shape. `join.rs` `section_refusal` turns
  it into an escalation (pinned by `cone_frame_rows.rs`
  `declared_equal_walls_meeting_at_a_sliver_escalate_the_circle_question`),
  so a Boolean never reaches the "construct the Circle" text. The
  decided-Zero arm still reads as in-band. This decision decides it too.
- `sweep` `cap_transverse`: already option B (above).

Second pass, aimed at what the grep could not see: other constructor
doors that a classifier hands off to (`Curve3::spiric`, from
`plane_torus_section`). The spiric door's `spiric_off_axis` and
`spiric_two_ovals` read the same quantities the table decided just
before (the offset, and (R − r) − |d|, which the table reads with the
tilt's swing beside it, so more strictly). No derived quantity sits
between them, so there is no gap.
