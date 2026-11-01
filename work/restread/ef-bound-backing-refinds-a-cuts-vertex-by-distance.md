---
id: ef-bound-backing-refinds-a-cuts-vertex-by-distance
kind: issue
title: ef_bound_backed re-finds a vertex cut's boundary vertex by its distance from the cut point instead of carrying the vertex the point-on-span reading cut at
status: open
opened: 2026-10-18
priority: P3
cost: E
---

## What

`crates/topo/src/census.rs` `ef_overlap_cells` cuts an edge at a face's
boundary vertex where `on_edge_interior_reading` (`Span::contact`)
reads the vertex on the edge's interior. That reading decides the
vertex's gap from its foot (`curve_carrier_gap`).

`ef_bound_backed` backs a cell bound. Where the edge holds no vertex
there, it calls `any_boundary_vertex_at`, which searches the face's
boundary for a vertex at the cut point by a second decision:
`pm_census_bound_vertex`, `|pw − q|` with `q = p0 + dir·s`. At a vertex
cut that is the same quantity recomputed from the cut point. It is a
second spelling of the reading that made the cut. The helper's doc says
so, and the two can part by a rounding at the band's edge. A parting is
loud (an unbacked bound, or a pushed escalation), so nothing passes
that should not. But a backed contact can be refused for a rounding,
and the rule "one point-on-span reading" has a second home.

## The shape to give

Carry the vertex in the cut (e.g. `CutAt::Vertex(Option<VertexKey>)`)
and back a vertex cut by that vertex. Keep `any_boundary_vertex_at` for
the edge's own end bounds, where no point-on-span reading was made
(that is a vertex-at-point question, the vertex-vertex pass's). Mind
coincident boundary vertices: two copies at one point give two cuts at
one `s`, and the bound must accept whichever copy backs.
