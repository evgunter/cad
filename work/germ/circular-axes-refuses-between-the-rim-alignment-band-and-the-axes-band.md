---
id: circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band
kind: issue
title: Tilts between eps/r and sqrt(2 eps/r) refuse CircularAxes with no lever: pc_rim_alignment and ellipse_axes_distinct read two different quantities
status: open
opened: 2026-10-12
priority: P3
cost: M
design: true
needs_ev: true
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

## The question

In the gap, the plane is decided off square to the axis, and the
section is decided to be within ε of a circle. Which kind does the
kernel mint there? That turns on what `Circle` is: the kind of a POSE
(the plane decided square to the axis), or the kind of a CURVE (a locus
within ε of a circle). Refusing is not a candidate. Both designers, and
the measurements above, find a sound carrier at every gap pose, and no
reader downstream needs the refusal.

- **Ellipse: `Circle` is the kind of the pose.** The pose reading alone
  decides: square gives the rim `Circle`, off square gives the exact
  `Ellipse`, however near-circular.
  - `ellipse_axes_distinct`, `EllipseInvalid::CircularAxes` and the
    "double gate" sentences on `plane_cylinder_section` and
    `plane_cone_section` go.
  - The carrier keeps C1's zero residual by construction.
  - What remains refused is the pose's own band, with its own lever.
  - Reading a near-circular ellipse's major direction off its semi-axes
    is ill-conditioned (about u/sin θ). Every reader must take it from
    the surface's axis (`Curve3::section_frame`, which the cone's
    `FocalSection` already uses).
  - Measured with the gate deleted: every gap pose splits at three ε,
    and a tilted cylinder minus a box builds at every gap pose (volume
    within 3.7e-14 relative).
  - Text that moves:
    - the `Curve3::ellipse` and `EllipseInvalid` docs;
    - `crates/sweep/README.md`'s cut-off-cap clause ("the ellipse door's
      own verdict on its axes is the second decision … `CapEllipse`");
    - `docs/GERM-CONE-SECTOR-SPEC.md` row 7.
- **Circle: `Circle` is the kind of the curve.** `ellipse_axes_distinct`
  stays the one reading of a conic's kind, and on Zero the door mints
  the `Circle` it decided (in the cutting plane, radius b).
  - A circular locus then has one kind whatever cut it, compared by
    centre, axis and radius.
  - The carrier misses the section by a − b ≤ ε, the margin that
    decided it.
  - It moves C1 (`crates/geom-brep/README.md`: a tilted section is a
    rung-2 `Ellipse` with zero residual by construction).
  - The section variants' payload contracts change as well.
  - The cone's chart has no image for a tilted circle
    (`Grazer::ConeCircle` refuses it). That image would have to be
    built.
  - The in-band span a − b ∈ (ε, Kε) still escalates: 9–15 of 150
    probe poses per ε.

Either way, `curve_pair::pose`'s `Coplanar::Ellipses` one-curve bound
(`frame_gap`) compares frames point by parameter, not loci. It already
refuses two derivations of one ordinary tilted ellipse whose frames
differ by rounding (r = 100, tilt 1e-4, ε = 1e-12, on main). That is
filed as its own item and fixed independently of this decision.

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
