---
id: the-side-certificate-has-no-row-or-argument-on-a-spline-carrier
kind: issue
title: The deferred one-carrier pairs and their side certificate are argued and pinned on analytic carriers only; a spline or fitted carrier is a blind spot
status: open
opened: 2026-11-01
priority: P2
cost: M
refs: [a-spline-body-against-its-copy-refuses-before-the-on-verdict]
---


The FULL review of PR 6070 named this blind spot as N3. That
review's own words are not on the branch. This row says what the code
shows (`splitdoor/sided-followups`, 2026-11-01).

## What

`ops::OnDeferred` holds a refusing section-pass pair only where the
reduction settled it one carrier (`OnDeferred::hold`), and answers it
from the shells' `On` verdicts or their sides (`OnDeferred::answered`).
The side answer rests on the fallback having no null pair: the vertex
classifier lumps each in-face vertex's on-sectors by Eq. 15.3, so every
way a shell leaves a shared patch lies to one side.

Every input to that is analytic today:

- The carrier ladder settles only the kinds in its inventory. The
  spline and the fitted surface are left out by definition
  (`boolean/carrier_eq.rs`, `the_inventory_named_is_the_inventory`:
  `SurfaceKind::Nurbs | SurfaceKind::Approx`), so no spline pair is
  ever settled, held or deferred.
- `∩` and `∖` refuse `Nurbs` at the door (`reduce::revert_arm_exists`).
  `∪` admits it for the plane × NURBS germ arm.
- The vertex classifier's on-sector lumping needs a declared
  coplanarity (`vtxfac` Delta 2), and refuses on a curved pierced face
  whose carrier algebra needs an origin.
- Every row that pins the side certificate is over cylinders, cones,
  tori, planes and spheres (`two_rims_review_probes`,
  `reach_continuation`, `reach_aligned_half_rods`,
  `strut_cover_on_cylinder_pairs`, `cone_operand_rows`,
  `germ_torus_doors`). So was the review's membership oracle.

So a spline wall lying on a spline wall refuses today, before any of
this runs. The blind spot opens when the inventory or the revert roster
takes a spline kind. Then the deferral takes spline pairs with no row,
and the null-pair argument has to hold for a classifier reading
sectors against a spline face.

## What closes it

When a spline kind enters the carrier inventory or the revert roster,
the same PR does two things. It pins a spline pose of each shape in
`two_rims_review_probes` (two lengths, partial overlap, a step) against
a membership oracle. It also checks that the vertex classifier mints a
null pair or refuses at an in-face vertex of a spline wall whose sectors
read two classes. Until then, the spline-copy refusal
(`a-spline-body-against-its-copy-refuses-before-the-on-verdict`) keeps
the blind spot visible. Lifting that refusal is when this row's work
comes due.
