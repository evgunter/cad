---
id: census-edge-pass-reads-no-line-conic-crossing
kind: issue
title: The census's edge passes never read a line crossing a coplanar conic boundary arc as an event
status: open
opened: 2026-10-08
---


Found by CONTACT-12 (branch `contact/12-ef-crossing-cuts`).

`snapshot` (`crates/topo/src/census.rs`) keeps only `Line` edges, so
pass 5 (`sweep_edge_edge`) never pairs a line edge with a circle,
ellipse or spiric edge. A line edge of one part that crosses a coplanar
conic boundary arc of another part's planar face is therefore no
`EdgeEdgeCross` of its own. The edge-on-face lane now cuts the line at
that crossing (`boundary_crossings`, `CutAt::ConicCrossing`). The cell
bound there has no rung (`ef_bound_backed` answers `false`), so a
declared seat whose straight edge crosses a cap's rim refuses with an
`UndeclaredContact` `EdgeFaceOverlap` that no declaration can answer.
That outcome is loud and typed, but the vocabulary does not fit it: the
event underneath is the crossing.

Whether this wants a line × conic crossing class, or nothing because
D10's INTENT stage 4 retires declared pairs, is the owner's call. The
lane-level gap (pass 5 sees no curved edge) stands either way.
