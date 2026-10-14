---
id: onpair-has-no-incidence-lane-on-a-spline-adjacent-pair
kind: issue
title: OnPair cannot meter incidence on a spline-adjacent pair, so plane × spline rims stay Chart(plane)
status: open
opened: 2026-10-13
priority: P2
cost: H
---


## Finding

Filed by TANG's `OnPair` lane (`tang/onpair-description`), from its review (m3).

`OnPair`'s certificate is incidence on both surfaces through each
surface's implicit residual (`geom_brep::certify`, `Resolved::OnPair`),
and a `Surface::Nurbs` or `Approx` surface has no implicit form
(`resolve` refuses it `Unimplemented`). So two doors keep a two-surface
locus on a plane × spline pair as `Chart(plane)` instead of the pair:

- the loft's cap rims (`sweep::swept::describe_face_rim_at_rest`): the
  cap's own profile against its spline wall;
- STEP adoption's conventional rung for any plane × described-NURBS
  edge (`step-import/src/adopt.rs`, `nurbs_rim` ← `nurbs_plane_pair`),
  which is not limited to cap rims.

That is a capability gap, not D2's classification: D2's `Chart` list
(wraps, iso lines, same-surface splits, stated spline isos) does not
name a cap profile against a spline wall, and the STEP arm re-opens a
per-door chart choice for the class. A smooth plane × spline seam has no
honest description today; a transverse one would be `Intersection`
through the plane × NURBS lane (`PlaneNurbs`), exempt at tier 3 by kind.

The way through is a foot-point incidence lane for `OnPair` on a
spline surface (the plane × NURBS lane's certified foot residual is the
model), after which both doors state the pair or the intersection the
rule reads.
