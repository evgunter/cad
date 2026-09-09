---
id: mesh-boundary-polylines-have-no-python-door
kind: issue
title: Mesh::boundaries has no Python door
status: open
opened: 2026-09-09
---



Found by LIB-MEMBERS's first run of the member rule
(`work/lib/datum-crosses-name-for-name-as-two-types.md`, ruling (D)),
and chartered in the census as `B-MESH-BOUNDARIES`.

`mesh::Mesh` (`crates/mesh/src/types.rs:26`) has three bare-`pub`
fields: `positions`, `patches` and `boundaries`. Python's `Mesh`
answers the first two — `Mesh.positions`, `Mesh.triangles`,
`Mesh.patch(i)`, `Mesh.patch_count` — and nothing in
`crates/pncad-py/src/` reads `boundaries` at all.

A `BoundaryPolyline` is the polyline a tessellation carries for each
model edge. Without it a Python consumer holding a `Mesh` can render
the surface and cannot draw the edges of the model it came from,
which is what a wireframe or a hidden-line view is made of. The STL
doors do not need it (STL is triangles), so the omission cost nothing
until something wanted to draw.

Invisible until now because `Mesh` is curated and `pncad.pyi` declares
a top-level `Mesh`, so rule 1 accounted the type whole and a field it
fails to project was nobody's roster — `Pose.sense`'s shape
(LIB-B-FACE-FRAME), one type over.

## What closing it looks like

A reader beside `Mesh.patch` answering the polylines — the same
opaque-vertex-index alphabet `Mesh.triangles` speaks, so no new type
is needed — and the `MEMBERS_NOT_BOUND` row moving to
`MEMBERS_BOUND_AS`, which empties `B-MESH-BOUNDARIES` out of
`FAMILIES`.
