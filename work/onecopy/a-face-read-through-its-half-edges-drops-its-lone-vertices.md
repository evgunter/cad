---
id: a-face-read-through-its-half-edges-drops-its-lone-vertices
kind: issue
title: glue's face closures, lineage's face cell and the census's face boundary read a face's vertices off its half-edges, so a face's lone vertices are missing from all three
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6654
---


Found by `onecopy/face-placement`'s sweep for row
`a-shell-whose-only-off-plane-vertex-is-lone-is-unplaced`. That row's
site read a face's vertices from `BoundaryMember::Edge`'s `half.start`
and skipped `BoundaryMember::Isolated`. The home is
`BoundaryMember::vertex()` (`topo/src/live.rs`), which answers a lone
vertex too.

## The finding

Three more readers take a face's vertices off its half-edges, which a
lone-vertex loop does not have (`BoundaryMember::half_edge`'s doc: "the
one disposition of a lone-vertex loop for a reader of half-edges"):

- `topo/src/boolean/glue.rs` `touched`'s `closures`: "each face's
  closure in its input: the face, its edges and their ends", read as
  `Cell::Vertex(h.start)` over `body.half_edges()`. A lone vertex of a
  face is in its closure, so a reduction that placed a cell on it (two
  faces meeting at a face's lone vertex) is not a meeting, and the row
  is dropped.
- `topo/src/lineage.rs`'s `Cell::Face` snapshot: `points` from
  `h.start` over every half-edge whose loop is the face's, so a face's
  lone vertices are not in its signature. It walks no loop on purpose
  (the snapshot is taken before a door's checks), so the fix reads the
  face's loops' `LoopBoundary::Empty { vertex }` records, not
  `FaceView::boundary`.

- `topo/src/census.rs`'s `FaceGeo.boundary` ("every vertex on the
  face's outer loop and rings"), built from `unchecked_face_cycles`'
  half-edge starts, and the `vertex_faces` incidence read off it: a
  face's lone vertex is in neither.

Not checked: whether any of them can see a lone vertex today, i.e. whether a
boolean input or a lineage subject carries one. `separation.rs`'s
`lone_vertices` fixture builds a tier-1-valid body that does.

## Fix

Read each face's vertices through `BoundaryMember::vertex()` (glue,
census), or add the loops' empty-boundary vertices (lineage). Pin each with a body
holding a lone vertex.
