---
id: the-boolean-recovers-no-structural-selector-for-a-rim-known-in-advance
kind: issue
title: The boolean's describe_edges and the split's finish map every Transverse seam to a witness Intersection; a rim known in advance (a boundary row of both walls) should recover its structural selector (D2), owing no limb 3
status: open
opened: 2026-10-28
priority: P0
cost: H
refs: [a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary, onpair-has-no-incidence-lane-on-a-spline-adjacent-pair]
---


Filed by SPLICE, from the limb-3 design fork that Ev approved on PR 5933
(fork-log row 208). It is SPLICE's lily wall 8 blocker, and a unit
SELECT's list lacks.

## What

D2 picks a two-surface edge's selector by one question: is the curve
known in advance? The boolean never asks it. Two places mint a witness
`Intersection` for every seam whose dihedral reads `Transverse`:
- `topo/src/boolean/ops.rs` `describe_edges`;
- the split's `topo/src/splitting/finish.rs` (`into_intersection`).

So a rim that is a boundary row of both walls reaches limb 3. That is
the result of a loft or sweep unioned onto another at a shared section:
lily wall 8's sheath on its blade. Limb 3's one-arc claim is ill-posed
there (the exact locus of two trimmed charts at a shared row is decided
by one rounding), and it refuses
(`work/splice/a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary.md`).

## What would close it

Both doors recover the structural selector for a rim known in advance,
sharing the reader import uses (this program's unit 6, boundary isos
first).
- The operands' rim edges already carry the row as their stated image.
- The seam's two faces each have it as a boundary row.
- The certificate is per surface, through each wall's chart-image
  (`IsoArc`) certificate (C2 as re-worded on PR 5933), with no limb 3.
- The dihedral class stays a certificate reading. It no longer picks
  the selector.

It needs unit 1, the selector type. It also makes reachable the two
rows owed by `nurbs_pair_limbs`' exact arm: its limbs zeroed, and the
tube's feet off the exact row.

## What it leaves open

A structural edge's certificate does not exclude a second piece of the
pair just inside the rim. That is the boolean's interior-loop guard's
obligation (`interior_loop_verdict`). The guard has no NURBS × NURBS
arm today (`pair_section` routes `(Nurbs, Nurbs)` to `Elsewhere`), and
SPLICE files that separately.
