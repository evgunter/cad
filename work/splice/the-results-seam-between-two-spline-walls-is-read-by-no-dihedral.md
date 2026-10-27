---
id: the-results-seam-between-two-spline-walls-is-read-by-no-dihedral
kind: issue
title: The result's seam between two spline walls is re-described through classify_dihedral, which reads no arm on a NURBS surface: lily wall 8 stops there
status: open
opened: 2026-10-27
priority: P1
cost: M
refs: [the-side-of-a-spline-face-is-read-by-no-dihedral, a-unions-flush-reading-cannot-follow-a-nurbs-carrier]
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
  `a_sheath_on_its_blade_unions_past_the_shared_rim` pins this site, in
  both orders, by decision and predicate (the refusal names no edge or
  face).
- `demos/tour/src/lily.rs` wall 8 pins the same.

## The likely shape

The transverse/smooth class of a seam between two spline walls read
from their normals at a foot on each (`S_u × S_v`, as `dihedral_side`
reads a spline face's normal), with the arm folded from the walls'
principal curvatures, as the side reading already does.
