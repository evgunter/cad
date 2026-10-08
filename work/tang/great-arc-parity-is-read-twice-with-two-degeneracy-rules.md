---
id: great-arc-parity-is-read-twice-with-two-degeneracy-rules
kind: issue
title: Great-arc crossing parity is read twice, by chord_join's sphere_path_parity and sectors' cone_side, under two different degeneracy rules
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

- `crates/topo/src/chord_join.rs` `sphere_path_parity` reads a path
  between two points on a sphere against circle arcs. A crossing is
  passed over when any of its four readings is decided against it. A
  reading in the zero band returns `None`, and the caller asks another
  path.
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
geometry: circle arcs on a sphere for `chord_join`, and face sectors
at a vertex for `cone_side`. Weigh first whether the circle-arc case
generalises the sector case cleanly, since a sector is a circle arc of
a great circle.
