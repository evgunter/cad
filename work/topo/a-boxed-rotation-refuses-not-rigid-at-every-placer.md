---
id: a-boxed-rotation-refuses-not-rigid-at-every-placer
kind: issue
title: transform_rigid at Interval refuses a widened rotation NotRigid, so every boxed rotation angle on a placer or a tilted mate frame refuses in a box run
status: open
opened: 2026-10-01
priority: P3
cost: M
design: true
---

Found by a designer weighing MSOLVE's plan item 19 (the mate solve on
analysis lanes). It is filed here because the door is TOPO's. It was
read, not probed.

## What

`topo::transform::check_rigid` decides the orthogonality residuals of
an `Affine3<T>` under `Decide`. Inside `transform_rigid` this means:
- At `Interval`, a map whose rotation part is widened (a boxed angle)
  has residuals that straddle zero beyond `band.zero()`.
- So it is `Indeterminate`, and the door refuses `NotRigid`: "a
  maybe-rigid map is not a rigid map".

Every boxed rotation therefore refuses in a box run. That covers a
`Transform` or `Pattern` angle bound by the box today, and a tilted
mate frame once the solve runs at the box's scalar. A boxed
translation passes, because it needs only finiteness.

## The question

Should an enclosure of a family of rigid maps pass the rigidity door?
For example: certify rigidity per member of the family (a rotation
parametrised by an angle is rigid at every angle) rather than of the
widened matrix.

If not, the refusal should name the box's angle as the cause rather
than read as a non-rigid map.


## Measured: a box-wide translation refuses too (MSOLVE-14, PR 3986)

"A boxed translation passes, because it needs only finiteness" holds
for the rigidity check and not for the door as a whole. Probed on a
`Transform` node over an instance, translation `x = gap`, box
`gap ∈ nominal + [−0.25, 0.25]`, at `Interval`: the node refuses
`NodeErrorKind::Transform(Certify { .. EndpointStart ..
carrier_endpoint_start, margin [0.0, 0.5] })`. The placed edge's
endpoint and its placed carrier are each widened by the box
independently, so the on-carrier check encloses `[0, 2·width]` and
escalates; a box narrower than the band places. A mate solved at the
box's scalar meets the same refusal at the mated instance, pinned by
`msolve14_run_scalar::a2_over_a_wide_box_the_bolts_refusal_is_the_placement_doors`.
