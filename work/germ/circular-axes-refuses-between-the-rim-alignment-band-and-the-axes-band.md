---
id: circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band
kind: issue
title: Tilts between eps/r and sqrt(2 eps/r) refuse CircularAxes with no lever: pc_rim_alignment and ellipse_axes_distinct read two different quantities
status: closed
opened: 2026-10-12
priority: P3
cost: M
branch: germ/circular-axes-band-gap
closed: 2026-10-19
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

`crates/sweep/examples/circular_axes_gap_probe.rs` (deleted once the
gap closed; its last tree is `5ada12a58`) sweeps
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

## Decided (Ev, PR 5156, 2026-10-19)

- **The pose decides the kind, once.** The table's tilt reading is the
  only decision: square gives the rim `Circle`; definitely off square
  gives the exact `Ellipse`, however near-circular. `Circle` is the
  kind of the pose, not of the curve. Ev: "I am sure that we should
  check only once and not twice". On the one-check comparison below,
  "this makes sense!"
- `Curve3::ellipse`'s `ellipse_axes_distinct` gate and
  `EllipseInvalid::CircularAxes` go. The constructor keeps only its
  positive-minor-axis check.
- That decides all the sibling arms too: plane × cone, the
  equal-cylinder bisector, and `cap_transverse`'s `CapEllipse`.
- Every reader of a near-circular ellipse's major direction takes it
  from the surface's axis (`Curve3::section_frame`), never from the
  semi-axes. The coplanar-ellipse one-curve bound reads loci
  (`coplanar-ellipses-one-curve-bound-compares-frames-not-loci`).
- What remains refused is the pose's own band, with its own lever.
- Text that moves with the implementation:
  - the `Curve3::ellipse` and `EllipseInvalid` docs;
  - the "double gate" sentences on `plane_cylinder_section` and
    `plane_cone_section`;
  - `crates/sweep/README.md`'s cut-off-cap clause;
  - `docs/GERM-CONE-SECTOR-SPEC.md` row 7.

Why one check on the pose, not one check on the curve (a − b, which
would serve a `Circle` in the cutting plane and fold the rim in):

| | refuses (in band) | at r = 1, ε = 1e-9, K = 10 | carrier |
|---|---|---|---|
| pose check | ε/r < θ < Kε/r | 1e-9 – 1e-8 rad | exact (C1) |
| curve check | √(2ε/r) < θ < √(2Kε/r) | 4.5e-5 – 1.4e-4 rad | within ε; C1 moves |

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

## Closed (2026-10-19, PR 5156)

Implemented as decided. `Curve3::ellipse` keeps only
`ellipse_minor_positive`; `EllipseInvalid::CircularAxes` and every arm
that existed for it are gone (`join.rs` `section_refusal` and the
equal-cylinder frame, `sweep` `cap_transverse` → `CapEllipse` and
`FILLET3_CAP_ELLIPSE_RECOURSE`). The plane × cylinder, plane × cone and
equal-cylinder bisector arms serve the exact ellipse at every tilt the
pose reads definite; `crates/sweep/tests/near_circular_sections.rs`
pins the table, the served carrier's miss off both surfaces (measured
at most 2.8 `f64::EPSILON` at its coordinate scale, held to 8, at all
three ε rows and in three frames: as built, turned about a generic
axis, and turned then carried about 1 km off), a split, and a Boolean
at three tilts across the old gap at three scales, each red with the
gate restored, and the split and the Boolean again densely in the
turned frame. The miss holds in a general frame because the ellipse's
frame is built orthonormal (`geom_brep::tilted_section_major`); the
bisector's half-angle is read off `â₁ ∓ â₂` without cancellation, so
its long semi-axis is `r / sin(φ/2)` to a few u at any served `φ`. The pose's own band still refuses
(`pc_rim_alignment`, `pn_axis_normal`, `cc_axes_parallel`), pinned in
the same file and in `topo` `cone_frame_rows.rs`.

Readers of a near-circular ellipse's major direction were audited (PR
5156's body); the one that reads it off the semi-axes as a public
answer is filed as
`edge-pose-reads-a-near-circular-ellipses-u-ref-off-its-semi-axes`.
