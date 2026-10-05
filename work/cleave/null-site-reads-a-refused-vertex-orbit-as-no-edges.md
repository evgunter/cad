---
id: null-site-reads-a-refused-vertex-orbit-as-no-edges
kind: issue
title: null_site reads a vertex whose orbit the door refuses as a vertex with no edges, and drops that vertex's null ties without a word
status: open
opened: 2026-10-03
priority: P3
cost: E
refs: [vertex-orbit-reads-no-start-vertex]
---


## What

`chord_join::null_site` (`crates/topo/src/chord_join.rs`) collects a
crossing site by following null edges out of each vertex. It reads
`body.edges_of_vertex(v).unwrap_or_default()`, so for a stale vertex
or an orbit the door refuses it gets an empty list. It then drops
every null tie that vertex has and returns a site short of them. Its
two callers each decide from that site:

- the chord copy in `chord_join.rs`, the `tied.contains(..)` match
  that picks the chord's direction;
- `boolean::join::locus_at_site` (`crates/topo/src/boolean/join.rs`),
  which tests whether an `OnEdge` germ's edge is incident to its site.

Both refuse typed when the short site misses an end, but under the
wrong name: "tied to neither or both", "not incident to its site". A
short site that still holds the end it was asked about answers as if
nothing were wrong.

`vertex-orbit-reads-no-start-vertex` (topo) widened this. The orbit
walk now refuses any walk that leaves its vertex, so the door answers
`None` on more torn bodies. Before that change a torn walk listed
another vertex's edges, and the site picked up that vertex's null
ties instead.

## The shape to give

Make `null_site` return `Option<Vec<VertexKey>>` (or the caller's
error type), so that each caller refuses with its own corrupt-operand
error when a vertex of the site cannot be read. That is a signature
change in `chord_join.rs` and `boolean/join.rs`. It was filed rather
than done because neither file is topo's ground.
