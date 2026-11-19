---
id: an-edge-between-two-records-of-one-surface-one-record-does-not-join-refuses
kind: issue
title: step-import: an edge between two records of one surface that one_record does not join (a spline or spindle record, two records facing opposite ways) refuses at the build
status: open
opened: 2026-11-19
---


Found by the dual review of `importgate/snap` (PR #7135).

## What

`crates/step-import/src/build.rs`'s `served_pair` refuses an edge whose
two surfaces the pair section finds coincident
(`geom_brep::SectionError::CoincidentSurfaces`) not-yet-supported,
naming this row (`ONE_LOCUS_LANE`). `one_record.rs` makes two records of
one plane, cylinder, cone, sphere or torus one record where their faces
face one way, so the arm is reached only by two records it does not
join: a spline or a spindle record stated twice, or two records of one
round surface whose faces face opposite ways.

## What would close it

One record for a spline or spindle surface stated twice, and a reading
of two records of one surface facing opposite ways along an edge (which
bounds no valid solid there and should refuse as the file's).
