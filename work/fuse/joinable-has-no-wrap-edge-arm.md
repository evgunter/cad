---
id: joinable-has-no-wrap-edge-arm
kind: issue
title: The curved join has no arm for a wrap edge, where one face lies on both sides of the two edges
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## The finding

`topo::boolean::edge_join::joinable` (`crates/topo/src/boolean/edge_join.rs`)
reads the two edges' face pair and returns `None` when `f == g`: an
edge with one face on both sides (a wrap edge, a seam a face meets
twice). A valence-2 vertex between two such edges on one carrier is
neither joined nor refused, the gap the curved unit's "Residue the
build owns" named. The workspace `ci` suite meets none: a probe over
every boolean output in sweep and editor-core logged no valence-2
vertex with `f == g`.

## What the arm needs

The same predicate with the pair read as one face twice: both edges on
one locus of the face's surface with itself (a `Chart` seam of one
surface), regular at the vertex. The join itself is unchanged; the
closed case leaves a seam self-loop, whose vertex is conventional only
where nothing else ends there.
