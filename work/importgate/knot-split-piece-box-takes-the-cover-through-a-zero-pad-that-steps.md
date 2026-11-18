---
id: knot-split-piece-box-takes-the-cover-through-a-zero-pad-that-steps
kind: issue
title: The C0 knot split's piece_box reads its edge pieces' covers through aabb(0.0), which steps every side out an ulp before its meet, near and width tests
status: open
opened: 2026-11-18
priority: P3
cost: C
---

## Finding

Filed by EMBED's lane `embed/rational-window` (2026-11-18), from its
sweep of the workspace's `aabb(0.0)` readers once PR 7120's review
showed that `Aabb::padded(0.0)` is not the identity: it steps every
bound one ulp out (`next_down` / `next_up`), and `Cover::aabb` ends in
it. PR 7120 says so at both doc comments.

`piece_box` (`crates/step-import/src/knot_split.rs`) encloses an edge
piece over `[a, b]` by `carrier.cover_over(window).ok()?.aabb(0.0)`.
`parts` then reads that box three ways:
- `meets`: a cell's image within `pad` of the box;
- `near(&bx)`: the box within `ε_in` of the knot line;
- the width stop, `hi − lo <= pad` on every axis.

A step can flip each only at a tie, and only toward more work or a
refusal (`meets` holds more, `near` and the width stop hold less), so
no wrong placement follows. But a piece lying exactly on the knot line,
or exactly `pad` wide, reads one step off its own value. Measured: no
body; the review in PR 7120 measured the same step moving a clearance
window's halvings off a curve's bound and turning a holding body into a
refusal.

**The fix** is the cover's own bounds, `Cover::box_in(&BoxAxes::World)`
read at its brackets' ends, as `topo::clearance::boundary_of` reads them
since PR 7120.

The workspace's other production `aabb(0.0)` reader,
`geom_brep::ssi::quadric_readout`, takes the operands' meet as a readout
box (a conservative superset beside the domain's slab) and feeds no split
or decision. It is not filed. Every other `aabb(0.0)` and zero-pad
`face_box` / `edge_box` call in `crates/*/src` is in a test module.
