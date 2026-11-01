---
id: the-boolean-recovers-no-structural-selector-for-a-rim-known-in-advance
kind: issue
title: The boolean's describe_edges and the split's finish map every Transverse seam to a witness Intersection; a rim known in advance (a boundary row of both walls) should recover its structural selector (D2), owing no limb 3
status: closed
opened: 2026-10-28
closed: 2026-10-28
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

## Closed

Landed on `splice/structural-rim` (PR 5959).

- **The selector.** A two-surface edge's description is
  `Intersection { pair, selector }`, the selector
  `CurveSelector::Witness(point)` or `CurveSelector::Structural(..)`, whose
  one form is `Structural::SharedRow`: each wall's row image, held in the
  pair's key order, admitted only as an iso row (`IsoLine` along either
  coordinate, `IsoArc` along `u`). A key remap goes through
  `CurveSelector::remapped`, a moved interval through
  `CurveSelector::repinned`. `TangentIntersection` keeps its witness.
- **Per-surface certification.** Each wall's row must lie on a side of its
  chart's domain (`description::on_domain_side`, else
  `CertifyError::Row { refusal: OffSide }`) and certify there
  (`PcurveCache::certify`, its limbs read through
  `PcurveCertificate::limbs`; else `Row { refusal: Image(check) }`, or
  `Row { refusal: ImageEscalated { .. } }`, routed through
  `arc_rim_check`). The walls' crossing is read at the certification
  samples, and over a cover of the row images as evaluated, ends included
  (`ssi::row_crossing`, limb 3's crossing reading without its one-arc
  count): zero-free pieces of opposite signs with no crease between them
  are a tangency, refused definitely; otherwise the least zero-free graph
  margin passes clear of the band and escalates in it. No limb 3, no
  witness pin.
- **Recovery.** `describe_edges` and the split's finish ask D2's question
  through `pcurves::seam_intersection`, built for boundary rows: both
  faces spline walls, each side's image a row on its chart's domain side
  that certifies over the kept carrier (`IntersectionDraft::shared_row`).
  Otherwise the witness, as before.

Measured first: the rim's existing description at the describer is the
boolean's `Scaffold`, not a stated image; each side's `pcurve_of` derives
the exact `IsoArc` on the `v = 0` row.

Lily wall 8's rims are recovered as shared rows, and on one of them the
walls are tangent at two points, symmetric about its middle: their
crossing reading changes sign across each
(`lily-wall-8-at-1e-6-reads-the-rims-crossing-angle-in-band`, which now
holds that blocker). Her sweep row stops there at every ε
(`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`).

Rows: `geom-brep` `a_shared_row_is_certified_per_wall`, seventeen rows at ε
1e-9, 1e-6 and 1e-12, each guarded by a mutant of the gate it pins.
