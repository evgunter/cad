---
id: two-records-of-one-non-planar-surface-stay-two-at-import
kind: issue
title: step-import: two surface records of one non-planar locus (two CYLINDRICAL_SURFACE records of one cylinder) stay two records at import
status: open
opened: 2026-11-17
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
priority: P3
cost: M
---

Found by the dual review of `importgate/interp-layer` (PR 7091), which
closed `import-adoption-keeps-two-records-of-one-carrier` for planes.

## What

`crates/step-import/src/one_record.rs`'s `one_record` (its plane-face
grouping, `matches!(face.surface, Surface::Plane { .. })`) asks only
`PLANE` records whether they are one locus. Two `CYLINDRICAL_SURFACE`
records of one cylinder (a re-tiled bore), two coaxial cones or spheres of
one locus, stay two records into the body, and the edge between their
faces rests on the two records' pair (`topo::rest_locus`): two records at
rest are two surfaces (`crates/topo/README.md` `[topo.one-record]`).

## Fix

The same act for every analytic kind: group faces by facing (the record's
outward normal at a face point composed with its sense), read the locus at
the faces both ways through a reading that bounds a face's distance from
the other record's surface (the cover's `distance_range` about the
surface's own site, or the kernel's carrier-pair coincidence reading), and
join on Zero, refuse in band.
