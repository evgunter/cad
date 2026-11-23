---
id: the-spline-lanes-lever-an-edges-transversality-at-the-edge-not-its-faces
kind: issue
title: The plane × NURBS and NURBS × NURBS lanes lever an Intersection edge's transversality at the edge's own extent, not its faces'
status: open
opened: 2026-10-15
priority: P2
cost: M
---


Filed by TANG U-lever (`u-lever-the-wedge-is-levered-at-the-faces-extent`).

Every reader of the wedge between an edge's two faces now levers it at
the farther face's extent (`topo::wedge_reach`, passed to
certification as `reach`). The two spline lanes in `geom_brep::certify`'s
`run_checks` do not: they still compute
`edge_extent(&spec.carrier, t0, t1, start.distance(end))` and hand it to
`NurbsLane::limbs` / `NurbsLane::pair_limbs`, where
`edge_nurbs::plane_nurbs_limbs` levers `plane_nurbs_transversality` at
`ssi::point_arm(kappa, extent)` and `ssi::certify::certify_branch`
sizes its uniqueness tube from the same number (`tube_ladder`). So a
short plane × NURBS `Intersection` edge between large faces reads its
transversality the way the analytic arm read it before U-lever: a 2δ
edge at dihedral δ meters δ².

The two uses differ. The transversality is the wedge and should take
`reach`; the tube is a localisation radius along the edge, which an
over-stated extent could widen past a neighbouring branch, so it likely
keeps the edge's own extent. Splitting them is the work. No fixture is
known to reach the difference.
