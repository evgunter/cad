---
id: main-red-a-fan-mev-plane-x-nurbs-row-escalates-its-general-lane-at-1e-12
kind: issue
title: Main red at ε 1e-12: the fan-mev plane × NURBS row's pcurve check escalates the General lane instead of reading the face unminted
status: open
opened: 2026-11-11
priority: P2
cost: E
---



## Finding

At ε = 1e-12, `topo`'s
`euler::tests::a_fan_mev_rebases_the_plane_x_nurbs_class_through_the_policy_lane_and_mev_null_splits_it`
fails its final `validate_geometric` assertion (`crates/topo/src/euler.rs:5791`).
It passes at the default ε.

- **Expected** (the row's fixture): `[ScaffoldAtRest(1v1), ScaffoldAtRest(3v1), PlanarBoundaryResidual(2v1, 3v1), Pcurve { Unminted { face: 1v1 } }]`.
- **Got:** the same first three, then
  `Pcurve { Certify { half_edge: 5v1, error: Escalated { check: GeneralLane, sample: 0, cause: Indeterminate { margin: 1.1992030236494026e-12, band: { zero: 1e-12, escalate: 1e-11 }, predicate: "ssi_hull_value_chart" } } } }`.

So at 1e-12 the pcurve pass reaches the split circle's `General` image
before the pillow's rowless spline face, and that image's hull margin
(1.2e-12) is in band, between zero and escalate. The row's expected list
pins the order its findings arrive in at the default ε only.

It is red at merge base `c43415ae04` at 1e-12 too, so it is main's, not
PR 6835's. This was found by a PR 6835 reviewer's 1e-12 run (topo,
nine sweep families).

## The fix

Decide which is meant:

- the row is ε-specific, so it should state its expected pcurve finding
  per band, or pin the default ε; or
- the pcurve pass should report the face's `Unminted` finding ahead of an
  edge's in-band escalation, whatever the ε.

## The merge, and a pin (GREEN, 2026-11-14)

A first-parent bisect of main at 1e-12 over `50cfcf499..c43415ae0`
names `dc1e4c7d4`, the merge of PR 6532 (`pcert/curved-general-image`);
its first parent `c58a72e39` passes. The row is pinned at 1e-12 on main's
order (the `GeneralLane` escalation on `ssi_hull_value_chart` where the
fixture expects `Unminted { face: 1v1 }`); the pin goes red the day the
row's findings change, which is this row's fix either way it decides.

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p topo --lib \
      -E 'test(a_fan_mev_rebases_the_plane_x_nurbs_class_through_the_policy_lane_and_mev_null_splits_it)'
