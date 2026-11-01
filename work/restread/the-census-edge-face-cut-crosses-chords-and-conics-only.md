---
id: the-census-edge-face-cut-crosses-chords-and-conics-only
kind: issue
title: The census's edge-face cut crosses a line with a face's chord and conic boundary arcs only; a spiric or spline arc it may reach refuses Uncrossable
status: closed
closed: 2026-10-14
branch: fuse/curve-pair-reader-c2
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
row in `work/splice/carrier-walk-has-no-crossing-row-for-spline-edges.md`
(closed); the cut here did not. The meeting search
`census::curved::meetings` (a bisection with a sup-speed clearing
bound) reads a line against any carrier kind and is one way in.

## Closed — the cut reads `geom_brep::curve_pair::meet` (fuse/curve-pair-reader-c2)

`census::edge_crossings` reads each boundary edge against the census edge's segment through `meet`, the one reading of two spans' meetings (C5), on every carrier kind. A straight edge's span is its segment between its two loop ends, and any other edge's span is its certified carrier's. A meeting on both interiors is a cut, a meeting at a boundary edge's end is the vertex cuts', and a shared run is pass 5's. A spiric or spline arc is read by the walk, so `CrossRefusal::Unsupported(Uncrossable)` is gone from the cut. `SpiricArc::clears_segment`, the ball screen and their rows went with it.

- `census::tests::a_spiric_boundary_is_cut_where_it_meets_the_edge_and_clear_elsewhere`: the cube edge straddling the spiric arc, which refused `Uncrossable` before, is cut on the oval.
- The spline row built a synthetic `LoopEdge::Unrowed` with a ball and no curve, a screen that no longer exists, and is deleted. A spline boundary is read by the walk like a spiric one.
