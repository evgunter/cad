---
id: an-edges-diameter-and-exactness-have-no-cover-method
kind: issue
title: An edge's diameter, whether a reach is exact, and the pivot it is read at still match on Curve3: the cover has no method for them
status: open
opened: 2026-11-02
---



Found by `cover-edge-doors` (its sweep). The edge doors that bound a region now read the arc cover. These matches on `Curve3` remain, because each asks something the cover's methods (`support`, `box_in`, `distance_range`, `ball`, `aabb`) do not answer:

- **A diameter.** `geom_brep::extent::edge_extent_sup` (`crates/geom-brep/src/extent.rs`) is an upper bound on the farthest two points of an edge lie apart. Its arms are exact on a circle arc (`2|r|·sin(min(|Δt|, π)/2)`), and an ellipse arc is within a few percent. The cover's box diagonal or ball diameter loses up to about 4% on a 120° circle arc, and a ball can lose 2×. `edge_extent_inf` is the lower side, which the unit leaves alone.
- **Whether a bound is exact.** `geom_brep::extent::edge_reach_bounds` answers `Exact` on a line and a circle, and `Between` otherwise. In `sweep::blend::surgery`, `support_boundary_clearance` reads a line or a circle through `piece_along`/`piece_distance` and flags the range "the piece's own", and reads every other kind through `boxed_reach` (the cover), flagged "a bound". The cover gives the same numbers, but it cannot say they are exact.
- **Where a reach is read from.** `Reach::foot_on` and `Reach::reading_point` (`crates/geom-brep/src/extent.rs`) choose the pivot a span's lever is read at: a conic's centre, or a line's or a net's minimax point.
- **A face lane's edge reading** (the face unit's): `census::depth_gap::face_read` (`crates/topo/src/census/depth_gap.rs`) covers each boundary edge with a census `Piece` (a chord, a conic's ring, a spiric's torus). `census::in_plane` tells a line edge from a curved one. Both now match on the carrier directly, where they used to match through `EdgeBoxRule`.

These are the levers' and the gate's question (`cover-levers`, `cover-kind-match-gate`). Either the cover gains a method (a diameter, an exactness flag) or each one is exempted with a reason. Until then, the kind-match gate will flag every site above.

**Also left by the unit, with no production reader:** `geom::curves::boxes`'s arc boxes, `circle_arc_aabb`, `ellipse_arc_aabb`, `spiric_arc_aabb`, `conic_arc_aabb` and `nurbs_curve_aabb`, and `conic_arc_extent_along` with its `_bracketed` twin. Since this unit, only tests and probes call them (`crates/geom/tests/`, `crates/sweep/tests/spiric_rim.rs`, and `topo`'s spiric box rows). They are a second reading of an arc's region beside `Curve3::cover_over`, and each one dispatches on kind. Retire them, or keep them as test oracles.

## Evidence: an exactness invariant held by the caller (PR 6399's delta review, 2026-11-03)

`geom_brep::edge_reach_bounds` now takes its bound from above from the caller, but still returns `Exact(above)` for a line or circle by carrier kind alone. That `above` is the reach (and not merely a bound on it) is an invariant only the caller holds. Topo's caller hulls the vertices into it, so `above` can pass the arc's reach by a vertex's residual. Exactness wants a cover method, as this row says, not a kind check at the door. (BOXES orchestrator)
