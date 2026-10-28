---
id: a-rim-between-two-spline-walls-is-certified-against-a-fitted-image-not-its-exact-one
kind: issue
title: A rim shared by two spline walls is certified against a degree-1 image fitted through feet, though each wall carries it exactly as an iso arc, so lily wall 8 refuses at ImageMiss
status: open
opened: 2026-10-28
priority: P0
cost: M
---


Lily wall 8's next stop, since PR 5868 read her rim Transverse.

## What

`describe_intersection` certifies the result's rim, the shared `Circle`
between a NURBS wall of each body, through
`edge_nurbs::nurbs_pair_limbs` (`crates/geom-brep/src/edge_nurbs.rs`).
That lane builds each chart's image of the carrier by `chart_image`: a
degree-1 interpolant through foot points at the fixed `PXN_FIT_SAMPLES`
schedule. Limb 2 then bounds `sup |S(P(t)) − C(t)|` against that image.
It refuses with `PlaneNurbs(Limb { ImageMiss })`, by a margin of 9.0e-6 m
in the sweep row (5 stations) and 4.53e-6 m in the tour (9 stations).
At ε 1e-6 the miss falls in band and escalates (`PlaneNurbsHullValue`,
or the pair's transversality at sample 12).

The rim is not a fit. It is an exact `IsoArc` boundary row of each wall,
so the whole margin is the certificate's own approximation (the class of
`ssiedge/plane-nurbs-certificate-bound-does-not-refine-with-eps`).

## The precedent

`plane_nurbs_incidence` already holds the exact arm for the plane lane:
`exact_arc_rim` takes the image as stated, or picks it with
`pick_arc_rim`, and certifies it through `PcurveCache::certify`. The
answer is used where the fitted image's limb refuses
(`IncidenceRefusal::ArcRim`). `nurbs_pair_limbs` has no such arm on
either chart.

## What would close it

The NURBS × NURBS lane reads the exact image wherever one certifies, on
each chart. It falls back to the fitted image only where none does, and
lily wall 8 moves past the rim. This row does not take the fixed-schedule
question (SSIEDGE's design row), which stays open for edges with no
exact image.
