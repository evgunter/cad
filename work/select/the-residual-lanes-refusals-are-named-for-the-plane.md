---
id: the-residual-lanes-refusals-are-named-for-the-plane
kind: issue
title: A sphere's, cylinder's or torus's refusal beside a spline arrives under the plane lane's names: CertifyError::PlaneNurbs, PlaneNurbsRefusal, PlaneNurbsLimbs, CertCheck::PlaneNurbs*
status: open
opened: 2026-11-12
priority: P3
cost: E
---

Filed by the fix pass of PR 6879 (`sectarms/spline-curved-at-rest`), from
both reviews of that PR. A class item: nothing is wrong in what is
computed.

## What

PR 6879 gave a sphere, cylinder or torus beside a spline wall the plane
lane's door (`edge_nurbs::residual_nurbs_limbs`), and the NURBS × NURBS
lane already shared its vocabulary. So three lanes speak through names
that say "plane":

- `CertifyError::PlaneNurbs(_)` (`crates/geom-brep/src/certify.rs`), and
  `from_plane_nurbs`;
- `PlaneNurbsRefusal`, `PlaneNurbsLimbs`, `PlaneNurbsIncidence`
  (`crates/geom-brep/src/edge_nurbs.rs`);
- `CertCheck::PlaneNurbsOnLocus`, `PlaneNurbsHullValue` and the rest of
  the `PlaneNurbs*` checks.

A sphere's chord that misses its image is reported as
`CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { .. })`. The words a
person reads are already neutral (`CertCheck`'s `Display` says "the
spline Intersection lane"); the identifiers a program matches are not.

## The rename

To the lane's name (`ResidualNurbs*`, or `SplineLane*` if the pair lane
is to keep sharing them: that is the one choice here). It touches:

- every `matches!` on those variants across `geom-brep`, `topo`
  (`validate.rs` `classify_certify`), `sweep`'s rows and `demos/`;
- the Python surface, if a tag there is derived from these names
  (`crates/pncad-py`, pydoor's ground). The reviews say the tags carry
  them; `rg PlaneNurbs crates/pncad-py` finds no spelling today, so
  read how a refusal's tag is derived before renaming, and take
  pydoor's word on any tag that moves;
- `docs/predicate-dimension-audit.md`'s stream names
  (`plane_nurbs_transversality` and its reported twin are the plane's
  own and stay; the `ssi_*` names are shared already).

Not done in PR 6879: a rename across four crates and the Python tags is
its own change.
