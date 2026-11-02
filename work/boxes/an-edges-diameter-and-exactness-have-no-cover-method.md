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
