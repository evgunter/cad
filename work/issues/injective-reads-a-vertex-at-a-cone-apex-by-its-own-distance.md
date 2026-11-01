---
id: injective-reads-a-vertex-at-a-cone-apex-by-its-own-distance
kind: issue
title: The self-intersection door decides a cone piece's vertex at the apex by a reading of its own
status: open
opened: 2026-10-30
---


`topo::injective`'s `present_chart` (`crates/topo/src/injective.rs`,
near line 1515) reads a cone piece's vertex as a collapsed side of the
chart where `|p − apex|` decides Zero (`self_intersection_apex`). That
is the fact `geom_brep::chart_singularity::vertex_at_singular_point`
(`chart_singularity_at_vertex`) is the one reading of: the loop walk's
reset joint, tier 3's at-vertex placement and the wrap-tip rule read it.
The quantity is the same, but the decision site is the door's own, so
the door's census and any change to the one reading part ways.

**What it needs:** call the one reading against the cone's apex
(`singular_points`). No program owns `injective.rs`
(`work.py territory`), so it is filed here.

Found by the RESTCURVE wrap-tip unit's sweep (branch
`restcurve/wrap-tip-tiers`).

Signed: (RESTCURVE implementer)
