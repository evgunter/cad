---
id: mesh-boundary-polylines-have-no-python-door
kind: issue
title: Mesh::boundaries has no Python door
status: closed
opened: 2026-09-09
closed: 2026-09-09
parent: LIB-GAPS-1
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

## Closed (2026-09-09, LIB-GAPS-1)

`Mesh.boundaries` answers `list[list[int]]` — one polyline of position
indices per model edge, in the kernel's edge order, in the same opaque
alphabet `Mesh.triangles` speaks. A Python consumer holding a `Mesh`
can now draw the model's edges, which is what a wireframe or a
hidden-line view is made of.

No value class: `BoundaryPolyline`'s three other fields are arena keys
the curation keeps unnameable, so a class would carry the indices and
nothing else. The inversion a drawing consumer wants is already on the
other side — `NodePick.boundary_names` answers one name per polyline in
this same order, entry for entry.

The polylines are answered whole rather than one at a time as
`Mesh.patch` is, because there is no concatenated spelling for them to
be separable FROM: a run of indices means nothing joined to the next
edge's.

The `MEMBERS_NOT_BOUND` row is GONE rather than moved: the member rule
accounts a bare-`pub` field by a same-named attribute on the Python
namesake, and `Mesh.boundaries` is that spelling, so a roster row for
it is stale by `test_the_member_rosters_decay`. `B-MESH-BOUNDARIES`
left `FAMILIES` with it.
