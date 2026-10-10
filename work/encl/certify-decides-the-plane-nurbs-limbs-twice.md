---
id: certify-decides-the-plane-nurbs-limbs-twice
kind: issue
title: geom-brep: certify.rs re-decides the PlaneNurbs limbs' on-locus and hull values through check_residual after the lane already decided them
status: open
opened: 2026-10-10
priority: P3
cost: E
---



(Found by a designer on design-fork row 104; not verified.)

## What

`crates/geom-brep/src/certify.rs` calls `check_residual("plane_nurbs_on_locus" / "plane_nurbs_hull_sup", …)` on `limbs.on_locus_max` / `limbs.hull_sup`. It does so after the PlaneNurbs lane (`ssi/certify.rs`) has already decided both values at the same band. That looks like a duplicate decision and a second refusal route, probably unreachable.

## Repair shape

Confirm whether the second decision is reachable. If it is not, delete it, so that each limb is decided once and refuses by one route. Check the k-stream: the duplicate rows should disappear and nothing else should move.
