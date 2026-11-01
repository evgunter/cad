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
- Since PR 4358, `cone_side` has a third rule. Where a reading would
  pass the reference over, the face is read as not crossed if the arc
  and its sector lie decidedly apart in the face's plane: a line through
  one of the four bounds (the sector's two, the arc's two ends) has
  the other wedge strictly on its far side (`sectors::apart`,
  `bool_cone_apart`). It reads no heights over the plane, so it decides
  an arc lying in the plane, which `ring_path` would treat as a graze.

Both are the same instrument: a crossing count along a great-circle
path, with the side at one end known. Their degeneracy rules differ
(two in `cone_side` against one in `path_parity`), and so do their
margins. So a fix to one (PR 4289's review found two
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
