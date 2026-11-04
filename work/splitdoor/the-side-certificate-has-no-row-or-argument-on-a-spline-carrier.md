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
- Every op admits `Nurbs` on one face-kind roster
  (`reduce::boolean_arm_exists`; ∩ and ∖ since SPLICE's PR 6476, note
  below).
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

## The revert roster took the spline kind: the obligation stays on the inventory (SPLICE PR 6476, 2026-11-03)

PR 6476 deleted the ∖/∩ revert roster, so ∩ and ∖ now admit `Nurbs`
wherever ∪ does. That is half of this row's trigger ("the inventory or
the revert roster takes a spline kind"). The PR does not discharge the
obligation. It loses no protection, because the deferral this row is
about still takes no spline pair:

- `OnDeferred::hold` holds only a pair the reduction settled to one
  carrier (`ops.rs`, `settled.iter().any(..)`).
- Settling runs the carrier ladder.
- The ladder's inventory still leaves out `SurfaceKind::Nurbs |
  SurfaceKind::Approx` (`boolean/carrier_eq.rs`,
  `the_inventory_named_is_the_inventory`).

So no spline pair is settled, held or answered from its sides under any
op, as before. A spline wall lying on a spline wall still refuses
before the side certificate runs. PR 6476's census holds that at
shared-cap and stacked placements: every op refuses typed.

The obligation now comes due with the inventory alone. The PR that
puts a spline kind into the carrier ladder's inventory pins the spline
poses in `two_rims_review_probes` and checks the vertex classifier, as
"What closes it" says.
