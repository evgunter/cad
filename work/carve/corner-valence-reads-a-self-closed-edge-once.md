---
id: corner-valence-reads-a-self-closed-edge-once
kind: issue
title: corner_at, cap_incidence and the corner admission count a self-closed edge once at its vertex
status: open
opened: 2026-10-06
priority: P3
cost: M
---

## Finding (the class sweep of `self-closed-link-sharing-its-vertex-records-two-junctions`, and its review)

`walk_chains` (`crates/sweep/src/blend/battery.rs`) counts a
self-closed link at its one vertex twice, because the link arrives
there and leaves. So a self-closed requested link `s` beside another
requested link `o` at its vertex `v` is a corner. `o`'s chain is OPEN
and ends at `v`, and that end reaches predicate 6.

A resolvable `o` at a self-closed rim's vertex needs a degree-4 fan:
edges `{s, o, x}` and faces `{F1, P, Q}`. A lone `o` there is
co-surface and `resolve_link` refuses it. The vertex then lists three
DISTINCT edges, although its degree is four.

Every valence reader on that path counts distinct edges or faces
through `Body::edges_of_vertex` / `Body::faces_of_vertex`, which list
a self-closed edge (or a face reached twice) once. There are five:

- `battery::corner_at`: `valence = edges.len()` and
  `named = edges.filter(requested).count()`. On the fan above it reads
  `valence == 3` and takes the trihedral arm.
- `battery::corner_at`'s face count: `faces.len() != 3` before the
  normals are read.
- `battery::cap_incidence`: `let [_, _, _] = incident[..]`, the same
  distinct-edge arity.
- `admit::Corner::admit`: `let [f0, f1, f2] = faces[..]` over the
  deduped face orbit.
- `surgery`'s corner gather, after `ends.sort_by_key(CornerLinks::vertex)`:
  `incident.len() != 3` and `here != incident` over distinct edges.
  `resolve_rim` shadows it today.

**Traced outcome** (CARVE review of PR 4185): on the degree-4 fan,
`corner_at` can mis-tag the end (`BodyNotIntact` on an intact body) or
admit it as a trihedron. Nothing is carved from either reading,
because the closed one-link chain `s` reaches `resolve_annulus`, whose
vertex-orbit set equality always refuses `UnsupportedChain` first. The
outcome is a typed refusal with the wrong tag, not a wrong body. No
body the tree builds has a non-seam edge ending at a self-closed rim's
vertex, so no test reaches it today.

## Fix shape

Read the degree, not the distinct-edge count, at all five readers: a
self-closed edge in the fan counts twice. A fix that sweeps only some
of the five is a half-fix, so all five belong to this row. The
alternative is to refuse an open chain end at a vertex whose fan holds
a self-closed edge as a corner configuration with a tag of its own.
Add a test on a body once a door builds the shape.
