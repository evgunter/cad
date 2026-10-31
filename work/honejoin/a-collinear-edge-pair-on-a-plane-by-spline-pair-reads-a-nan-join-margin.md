---
id: a-collinear-edge-pair-on-a-plane-by-spline-pair-reads-a-nan-join-margin
kind: issue
title: the join reads a NaN margin at a vertex between two collinear LINE edges on a plane × B-spline pair, and the import refuses JoinUndecided
status: open
opened: 2026-10-29
priority: P2
cost: M
---


(FOREIGN implementer, `foreign/c0-knot-split`.)

## Witness

`an_off_iso_line_on_a_spline_wall_imports_its_general_row`'s tetrahedron, with its straight diagonal `(t, t, 0)` stated as two collinear `EDGE_CURVE`s on one `LINE` that meet at `(0.5, 0.5, 0)`. That point is a vertex nothing else meets, between the slanted plane and the B-spline wall. The import refuses:

```text
step import: joining the edges of the solid at #94: whether two edges meeting at a vertex on one
curve are one edge is undecided (margin is invalid (NaN or a refused enclosure) against the
ambiguity band (1e-9, 1e-8)).
```

The file is legal, and its two edges are one edge. The join should join them.

## Where

`topo::boolean::edge_join::joinable`: both edges read `Locus::Transverse` (each adopts as `Intersection` of the pair), and the regularity reading `geom_brep::classify_dihedral(plane, nurbs, p, reach, band)` at the vertex returns an invalid margin. Tier 3's check 11 reads the same predicate at rest.

## Context

The FOREIGN C⁰-knot split first met this margin at a knot vertex between two spline sub-arcs on a plane × spline pair. That case no longer reaches the reading: `joinable` now refuses, structurally, to join two spline pieces meeting at a knot of full multiplicity (`crease_at`, Ev's PR 5913 ruling). The reading itself is unrepaired, and this witness reaches it with no spline edge at all.

Reproduce: take `tetra_step` in that test, add a `VERTEX_POINT` `m` at `(0.5, 0.5, 0)`, and state `oq` as two `EDGE_CURVE`s, `o → m` and `m → q`, on the one diagonal `LINE`. The wall's loop walks them forward, and the slanted plane's walks `m → q` then `o → m`, each reversed.
