---
id: the-census-edge-face-cut-crosses-chords-and-conics-only
kind: issue
title: The census's edge-face cut crosses a line with a face's chord and conic boundary arcs only; a spiric or spline arc it may reach refuses Uncrossable
status: open
opened: 2026-10-13
priority: P3
cost: M
refs: [an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane]
---


Found by FUSE's sweep for line- and circle-only arms in the census
(`census-curved-record-lanes-read-circles-only`, branch
`fuse/census-reads-every-curved-edge`). Not reproduced: no `ci` row
reaches it, and none was built.

The census's edge-face overlap cut (`ef_overlap_cells` →
`edge_crossings` in `crates/topo/src/census.rs`) crosses a line
edge with each boundary edge of the planar face it rests on, by the
edge's `LoopEdge` row: a chord (`chord_crossing`), a circle or ellipse
arc (`conic_crossings`, through `conic_plane_candidates`). A spiric arc
is crossed only where `SpiricArc::clears_segment` clears the whole
segment, and a spline arc (`LoopEdge::Unrowed`) never: anything else
refuses `CrossRefusal::Unsupported(ContainError::Uncrossable)`, so a
census over a line edge recorded on a planar face whose boundary holds
a spiric arc (a torus cut parallel to its axis) or a spline one, and
which reaches that arc, refuses rather than reading the record.

The containment walk the same `LoopEdge` rows serve gained its spiric
row in `work/cleave/carrier-walk-has-no-crossing-row-for-spiric-or-spline-edges.md`
(closed); the cut here did not. The meeting search
`census::curved::meetings` (a bisection with a sup-speed clearing
bound) reads a line against any carrier kind and is one way in.
