---
id: edge-join-reads-a-joined-vertex-with-one-incident-edges-tangent
kind: issue
title: edge_join reads a joined vertex's pair at the vertex with the kept edge's tangent, a point read the whole-edge fold cannot take
status: open
opened: 2026-11-12
---


Filed by the wedge fold's stage 2 (`onedoor/wedge-fold-2`, PR 6869), from its owed list ("`edge_join`'s vertex-tangent read").

**Where.** `topo::boolean::edge_join`'s `joinable` (`edge_join.rs:728`, the `classify_dihedral(surf_f, surf_g, p, reach, band)` and the `tangent_second_order` beside it) reads the pair at the shared vertex `w`. On a tangent locus it meters the second order along `tangent_at(body, e2, d2, w)`, the kept edge's carrier tangent at `w`, and never the gone edge's.

**Why the fold cannot take it.** The question is whether the locus is regular AT `w`: two edges' interior stations can each read a crease, or a jet-determinate tangency, while the curve the pair cuts is singular at `w` itself. One example is two equal cylinders whose intersection branches meet tangentially at a point. `geom_brep::edge_wedge_over` reads an edge's interior stations and has no station at an end, so reading both incident edges' fold verdicts answers another question. The read stays a point read, through `classify_dihedral` and its token.

**The question left.** Which tangent a joined vertex's second order should be metered along. The joined edge runs on the kept carrier, which is the case for `e2`'s. The case for both is that a regular point should read positive along each incident edge, and refuse where they disagree. That is a design choice for this program.
