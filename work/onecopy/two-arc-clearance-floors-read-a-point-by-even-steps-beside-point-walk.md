---
id: two-arc-clearance-floors-read-a-point-by-even-steps-beside-point-walk
kind: issue
title: Two circle-arc clearance floors read a point against an arc by 32 even steps beside Curve3::point_walk
status: open
opened: 2026-11-09
---


Found while routing `Span::window_near` through `Curve3::point_walk` (row `the-typed-refusal-reason-leaves-an-unused-second-spelling-and-unpinned-reads`, item 2).

`point_walk` is the walk that reads a point against a span by its pieces (`geom` `curves/on_carrier.rs`). Two circle-arc clearance floors in `topo` still read a point against an arc's pieces their own way. Each takes the least distance from `m` to the arc's points at `CLEARANCE_SAMPLES` = 32 even steps, less half the arc length of one step:
- `boolean/solid_contain.rs` `edge_clearance`, its `Curve3::Circle` arm;
- `boolean/sphere_region.rs` `SphereFaceRegion::clearance`.

That is `point_walk`'s floor, `gap − reach`, taken over a fixed cover of pieces instead of a halving one. Neither 32 has a stated reason. Both are floors (a length), not decisions. So routing them through the walk needs a stated resolution: for example, halve until a piece's reach is within the escalation, or within the gap the caller needs.

A third reading halves, but over its own pieces: `topo` `splitting/pieced_arc.rs` `PiecedArc::pieces_contact`. It reads a point against a spiric or spline arc by `bisect` over the arc's own piece balls (`self.ball`), with its own depth cap `MAX_DEPTH`, and decides clear, on and leaf on the caller's band. It is a contact reading, not a floor. Whether it can ride `point_walk`, whose pieces carry a speed-bound reach and no ball, is part of this row. Found in PR 6629's review.

Sweep: `const [A-Z_]*(PIECES|SAMPLES)` across `crates/`, plus `bisect(` callers. The other hits cut a span to box it, or to hull a section (`injective::PIECES`, `section_bounds::PIECES`). They read no point.
