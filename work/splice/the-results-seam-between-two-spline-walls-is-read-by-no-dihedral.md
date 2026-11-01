---
id: the-results-seam-between-two-spline-walls-is-read-by-no-dihedral
kind: issue
title: The result's seam between two spline walls is re-described through classify_dihedral, which reads no arm on a NURBS surface: lily wall 8 stops there
status: closed
opened: 2026-10-27
priority: P0
cost: M
closed: 2026-10-27
refs: [the-side-of-a-spline-face-is-read-by-no-dihedral, a-unions-flush-reading-cannot-follow-a-nurbs-carrier]
branch: splice/spline-seam-dihedral
---

Filed by the SPLICE lane that built the spline span × spline face arm
(`a-spline-span-against-a-spline-face-has-no-interior-arm`).

## What

With that arm built, lily wall 8 (the sheath grafted on its blade) gets
past every face reduction in both operand orders and stops at the
result, `Escalated { decision: LeverArm(Seam), predicate:
"dihedral_arm", margin: Invalid }`. Measured with a probe in
`topo::boolean::ops::describe_edges` at the `seam_reading` call: the
seam is the shared rim, a `Circle` edge, between a NURBS wall of each
body (`Nurbs × Nurbs`, witness `(0, 0, −0.028)`, reach 1.2).

`describe_edges` reads each seam of the result through
`seam_reading` → `geom_brep::classify_dihedral`, whose arm and wedge
read implicit gradients and curvature arms; on a spline surface those
are poison (`classify_dihedral`'s own docs: "an unimplemented `Nurbs`
kind"), so the arm gate refuses with an invalid margin. The side
reading has a spline arm since `the-side-of-a-spline-face-is-read-by-no-dihedral`
(`geom_brep::dihedral_side`), but the seam re-description does not use
it.

## Witnesses

- `crates/sweep/tests/a_sheath_grafted_on_its_blade.rs`
  `a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim` pins this site, in
  both orders, by decision and predicate (the refusal names no edge or
  face).
- `demos/tour/src/lily.rs` wall 8 pins the same.

## The likely shape

The transverse/smooth class of a seam between two spline walls read
from their normals at a foot on each (`S_u × S_v`, as `dihedral_side`
reads a spline face's normal), with the arm folded from the walls'
principal curvatures, as the side reading already does.

## Closed

`describe_edges` reads each face of a seam by its kind through
`geom_brep::classify_seam`: an analytic face off its implicit form at
the witness, as `classify_dihedral` reads it (bit for bit on two
analytic faces); a spline face at the witness's parameters on its own
chart, read off the edge's pcurve on that face
(`topo::pcurves::pcurve_of` at the middle of the edge's interval,
`ops::seam_face`), never a foot. There the normal is the chart's
`S_u × S_v` through `regular_jet` and `chart_normal_at`, so a seam within
the band of a crease, a closed chart's seam or an irregular point
refuses as `EdgeUndescribed { failure: Crease }`, and a spline face with
no image to read, or a null edge, refuses `Unplaced`. A
spline face's arm is bracketed: between a certified lower bound on the
chart's length scale at the witness (`NurbsLane::chart_arm`, from
`patch_bound` and `offset_meters`' meter 2), whose slack has no bound, and
the reach, the fold's ceiling. The seam reads smooth only where the wedge
reads Zero over the upper end, transverse only where it is decided past the
band over the lower end, and escalates at the arm between. It is folded
with the reach and the other face's arm through the
same gate and wedge decision as the analytic path (`dihedral::wedge_of`).

Witnesses: `geom_brep` `dihedral::seam_rows` (transverse and smooth
spline pairs, a rational chart, a crease and a pinch, a crease within
and past the reach, a bend within and past the lower arm, the reviews' two
far-bend and two never-smooth counterexamples, a tangent seam with a short
lower arm, the sheet the parameters name, a spline
carrier read as analytic, analytic parity in both orders), each at ε
1e-6, 1e-12 and the default; `topo` `boolean::ops::spline_seam_rows`
(two flat spline faces, a turning chart read at its witness, a null edge
refused unplaced); the sweep row
`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`, which re-pins lily
wall 8's next stop in both orders: the rim's `Intersection` certification
on the NURBS pair, limb 2's `ImageMiss`
(`work/ssiedge/plane-nurbs-certificate-bound-does-not-refine-with-eps`,
measured there).

Residue, filed: `a-smooth-seam-between-spline-walls-has-no-second-order-reading`,
`the-shared-rim-routing-exempts-a-spline-face-by-kind`,
`a-seam-on-a-creased-spline-chart-reads-no-arm`,
`the-seams-chart-arm-builds-every-cell-of-the-chart-per-read`,
`the-chart-curvature-bound-over-reads-a-flat-chart-at-uneven-speed`.
