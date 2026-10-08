---
id: great-arc-parity-is-read-twice-with-two-degeneracy-rules
kind: issue
title: Crossing parity along a path is read twice, by ring_path's path_parity and sectors' cone_side, under two different degeneracy rules
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [pairs-beside-an-unread-partner-keep-mains-rows]
---



Filed by PR 4289's review (Q1).

## What

Two readers decide whether a point lies inside a closed curve on a
sphere by the parity of a great-circle path's crossings:

- `crates/topo/src/ring_path.rs` `path_parity` reads a path against
  circle pieces, segments and rulings, counting each meeting point
  strictly inside both spans (`split_ring_path_in_span`, the distance
  to a span end times the slope the piece crosses at). A graze inside
  the band escalates, and the caller asks the next path.
- `crates/topo/src/boolean/sectors.rs` `cone_side` reads an arc from a
  direction to a reference inside one face, against a vertex's face
  sectors, which are great-circle arcs. A face is passed over only on a
  decided reading (`in_sector`). A reading zero or in band passes over
  the reference, and the next is tried.

Both are the same instrument: a crossing count along a great-circle
path, with the side at one end known. Their degeneracy rules differ,
and so do their margins. So a fix to one (PR 4289's review found two
exemptions in `cone_side` that held only for an exact zero) does not
reach the other.

## The shape to give

One parity primitive over great-circle arcs, with one rule for passing
a crossing over (decided only) and one rule for a degenerate path (try
the next). Both callers read through it. Each keeps its own
geometry: circle pieces and rulings for `ring_path`, and face sectors
at a vertex for `cone_side`. Weigh first whether the circle-piece case
generalises the sector case cleanly, since a sector is an arc of a
great circle.
