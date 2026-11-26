---
id: a-covered-arc-on-an-equal-cylinder-pair-reads-the-quadrics-lies-on
kind: issue
title: A covered arc on an equal cylinder pair reads the quadric door's LiesOn, not the factored read
status: open
opened: 2026-11-26
priority: P3
---


Filed by GERM U4c (PR 7818), from the designers' review of the factored
read's `Lies` answer.

## What

`reduce::curved_face_arm` asks `factored_crossing` only of an uncovered
edge (`!covered && matches!(surface, Cylinder)`,
`crates/topo/src/boolean/reduce.rs`). A covered arc on one wall of an
equal cylinder pair with meeting axes skips it, and its carrier-identity
rung reads `wall_crossing`, so `cross` against the partner cylinder: the
conic × cylinder pose's `LiesOn`, the quadric door's reading. C5
(`crates/geom-brep/README.md`, the factored read's paragraph) has every
span on such a pair read through the bisector planes and never handed to
the quadric door. U4c moves the uncovered edge (its own predicate,
`bool_factored_in_bisector`, decides "lies on the partner" in metres
through the plane's lever); the covered arc keeps the quadric's reading.

## Owed

The covered rung reads a covered arc on an equal cylinder pair through
the factored read too (its `Lies` answer, or its typed refusals), or C5
says why a covered arc may keep the quadric's `LiesOn`. A row: a covered
mitred rim on a crossing rod, read by the decisions the op took, as
`crossing_equal_rods::the_mitred_rim_lies_on_the_crossing_wall_through_its_bisector_plane`
reads the uncovered one.
