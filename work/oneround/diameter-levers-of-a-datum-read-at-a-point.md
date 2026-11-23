---
id: diameter-levers-of-a-datum-read-at-a-point
kind: issue
title: edge_extent_inf and edge_extent_sup lever datums read at a point by the span's diameter, not its reach from that point
status: open
opened: 2026-11-23
priority: P1
cost: H
---

Found by `oneround/unsourced-unions` (PR 7294). Its review found the router (`curve_pair::pose`) and the ring path's arc crossing (`ring_path::spline_crossings`) levering a datum read at one point by the span's diameter. That PR moved those two to the span's reach from the point (`edge_reach_bounds`, `cover::piece_reach_bounds`). The other callers below were left as they are.

**The class.** `edge_extent_inf` and `edge_extent_sup` bound the diameter `D` of an edge's point set. They take no pivot. A datum read at one point `p` of the span (a sine, a slope or a coefficient centred there) moves over the span by at most the datum times the span's reach from `p`, `R(p) ≤ D`.

- **From above** (`edge_extent_sup`), `D` bounds `R(p)` too. That is sound but loose: it refuses more than it needs to.
- **From below** (`edge_extent_inf`), it is unsound. A lower bound on `D` is not a lower bound on `R(p)`. A definite sign read at `D` can come from a datum that `R(p)` reads in band. In the router's review row the difference was `2` against `1` for a segment crossed at its middle.

`edge_extent_inf`'s own doc (`crates/geom-brep/src/extent.rs:466`) states this as its contract. It names "the root a face move's corner accepts as transversal" and "a section's crossing" as readings levered at `D`. Both are read at a point. So the doc's contract is the second thing to change, beside its callers.

**The callers that lever a datum read at a point**, one per line: where, what the datum is, which bound.

- `crates/topo/src/offset_axial.rs` `corner_arms`. A line's arm used to be `edge_extent_inf`. PR 7294 reads it as the reach from the corner (both bounds), so it is out of the class. It is listed for the record.
- `crates/geom-brep/src/curve_face/pose.rs:991` `line_cone_read`. The lead coefficient `a2/dd` of the quadratic centred at the apex's foot on the line, levered by `edge_extent_inf` of the line over `[lo, hi]`. What the coefficient moves by is the run's reach from the foot (at most its length). From below.
- `crates/topo/src/census.rs:399` `Span::extent`, read by `parallel_reading` (`:5476`). The cross product of two segments' directions, `pm_census_ee_parallel`, levered by the shorter's `edge_extent_inf` and acting on Zero (`gap_is_zero_as`). The sine has no point of its own. What the Zero claims is a separation over the pair's overlap. From below on a reading that acts on Zero, so the bound itself is also the wrong one (`extent.rs:482`).
- `crates/topo/src/replace_face.rs:4206` `corner_arm`, read at `:4605` and `:4620` (`lane.root(.., inc.extent, ..)`). A root's transversality, read at the root, levered by the carrier's `edge_extent_inf`. From below.
- `crates/topo/src/replace_face.rs:2849` and `:3280`. These hand `edge_extent_inf` of the old carrier to `derive_edge`, and through it to `offset_derive::section_of`'s pose readings (`section_closed`'s `extent`). This row has not settled whether those readings are taken at a point or over the whole section. The fix unit reads each one.
- `crates/step-import/src/build.rs:2404`. `root_on`'s transversality at a tangent edge's root, levered by `edge_extent_inf`. From below. At `:554` the extent is maxed with the faces' reach and the apex reach, a wedge lever rather than a datum at a point, so it is not in the class.
- `crates/geom-brep/src/certify.rs:5602` and `crates/geom-brep/src/edge_nurbs.rs:2152`. The spline lanes' per-sample transversality and the uniqueness tube, levered by `edge_extent_inf`. The transversality is read at each sample. The tube's extent is the whole span's, which is a diameter by its nature. From below.
- `crates/geom-brep/src/conic_bend.rs:82` `ConicDeparture::of`. The departure out of a vertex: the tangent there, scaled by `edge_extent_inf`, and the bend. Read at the vertex, so the reach from the vertex is what it moves over. From below.
- `crates/topo/src/boolean/contact_verify.rs:337`. The tangent schedule's angular margins at each sample, capped by `edge_extent_sup`. From above, so sound but loose.
- `crates/topo/src/ring_path.rs:1802`. The no-length piece reading, `split_ring_path_spline_piece`, levered by `edge_extent_sup`. This is a datum of the whole span (its length), not of a point, so it is not in the class. It is listed because the review asked for every caller in `ring_path`.
- `crates/topo/src/offset_derive.rs:792` `section_closed`. `edge_extent_sup` of the old carrier, used as the section's region bound from above. It is a span datum, not in the class.

Test-only callers (`#[cfg(test)]` modules and `crates/*/tests/`) read `D` itself as a fixture's premise. They are not in the class.

**What a fix needs.** Every in-class caller has a point in hand: the foot, the root, the sample or the vertex. It also has the carrier and span that `edge_reach_bounds` (with `ExtentBall::of_cover_from` from above) reads, so no new door is needed in `geom-brep`. In `topo`, `cover::edge_reach_bounds` and `cover::piece_reach_bounds` serve the same purpose. Each caller's direction comes from its verdict: a definite sign uses the bound from below, a Zero acted on uses the bound from above, and every sign uses both. The differential needs re-taking per caller, as PR 7294 did for the router.
