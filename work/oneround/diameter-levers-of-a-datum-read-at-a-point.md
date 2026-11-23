---
id: diameter-levers-of-a-datum-read-at-a-point
kind: issue
title: a datum read at a point is levered by a span's diameter or chord (edge_extent_inf/sup, or a two-point read through the point doors), not by its reach from that point
status: open
opened: 2026-11-23
priority: P1
cost: H
---

Found by `oneround/unsourced-unions` (PR 7294). Its review found the router (`curve_pair::pose`) and the ring path's crossing levers (`ring_path::spline_crossings`) levering a datum read at one point by a diameter or a chord. That PR moved them to the span's reach from the point (`edge_reach_bounds`, `cover::piece_reach_bounds`):

- the router's two levers;
- all three of `spline_crossings`' arms: the arc, the ruling/segment (whose chord was `ExtentBall::point(from).lever_from(to)`), and the iso piece (whose chord was `IsoPiece::travel().0`).

The second review found the last two outside this row's first sweep. The other callers below were left as they are.

**The class.** A datum read at one point `p` of a span (a sine, a slope, or a coefficient centred there) moves over the span by at most the datum times the span's reach from `p`, `R(p) ≤ D`, where `D` is the diameter. A diameter or a chord between the span's ends is not `R(p)`:

- **From above**, `D` (`edge_extent_sup`) bounds `R(p)` too. That is sound but loose: it refuses more than it needs to.
- **From below** it is unsound. A lower bound on `D` (`edge_extent_inf`, or the chord) is not a lower bound on `R(p)`. A definite sign read at `D` can come from a datum that `R(p)` reads in band. A segment met at its middle reaches half its chord.

`edge_extent_inf`'s own doc (`crates/geom-brep/src/extent.rs:524`) states this as its contract. It names "the root a face move's corner accepts as transversal" and "a section's crossing" as readings levered at `D`, and both are read at a point. So the doc's contract is the second thing to change, beside its callers.

**What the first sweep could not match.** It keyed on the names `edge_extent_inf` and `edge_extent_sup`, and missed two kinds of read:

- A chord or diameter read through the point doors carries no extent name: `ExtentBall::point(a).lever_from(b)`, `points_diameter`, `points_lever_from`.
- So does one stored by a helper and read elsewhere: `IsoPiece::travel`.

The second sweep (below) matched the point doors and `.lever_from(`. Even it cannot see two things:

- A lever stored in a record's field and read far from where it was made: `Piece::Segment.lever`, `BoundSupport::Chord`, `Leaving.reach`. Each has to be followed to its readers.
- Whether a region read from a pivot (`reach.lever_from(apex)`) has its pivot at the point the datum is read.

**In-class callers of `edge_extent_*`** (lines on `2a4d0cb82b`'s merged tree): where, what the datum is, which bound.

- `crates/topo/src/offset_axial.rs` `corner_arms`. A line's arm used to be `edge_extent_inf`. PR 7294 reads it as the reach from the corner (both bounds), so it is out of the class. It is listed for the record.
- `crates/geom-brep/src/curve_face/pose.rs:991` `line_cone_read`. The lead coefficient `a2/dd` of the quadratic centred at the apex's foot on the line, levered by `edge_extent_inf` of the line over `[lo, hi]`. What the coefficient moves by is the run's reach from the foot (at most its length). From below.
- `crates/topo/src/census.rs:398` `Span::extent`, read by `parallel_reading` (`:5473`). The cross product of two segments' directions, `pm_census_ee_parallel`, levered by the shorter's `edge_extent_inf` and acting on Zero (`gap_is_zero_as`). The sine has no point of its own. What the Zero claims is a separation over the pair's overlap. From below on a reading that acts on Zero, so the bound itself is also the wrong one (`extent.rs:540`).
- `crates/topo/src/replace_face.rs:4185` `corner_arm`, read at `:4585` and `:4600` (`lane.root(.., inc.extent, ..)`). A root's transversality, read at the root, levered by the carrier's `edge_extent_inf`. From below.
- `crates/topo/src/replace_face.rs:2829` and `:3260`. These hand `edge_extent_inf` of the old carrier to `derive_edge`, and through it to `offset_derive::section_of`'s pose readings (`section_closed`'s `extent`). This row has not settled whether those readings are taken at a point or over the whole section; the fix unit reads each.
- `crates/step-import/src/build.rs:2402`. `root_on`'s transversality at a tangent edge's root (`:2408`), levered by `edge_extent_inf`. From below. The extent at `:554` is maxed with the faces' reach and the apex reach, a wedge lever, so not in the class.
- `crates/geom-brep/src/certify.rs:5652` and `crates/geom-brep/src/edge_nurbs.rs:2152`. The spline lanes' per-sample transversality and the uniqueness tube, levered by `edge_extent_inf`. The transversality is read at each sample. The tube's extent is the whole span's, a diameter by its nature. From below.
- `crates/geom-brep/src/conic_bend.rs:82` `ConicDeparture::of`. The departure out of a vertex, the tangent there scaled by `edge_extent_inf`, and the bend. Read at the vertex, so the reach from the vertex is what it moves over. From below.
- `crates/topo/src/boolean/contact_verify.rs:337`. The tangent schedule's angular margins at each sample, capped by `edge_extent_sup`. From above, so sound but loose.

Out of class (a datum of the whole span):

- `crates/topo/src/ring_path.rs:1814`, `split_ring_path_spline_piece`, the piece's length.
- `crates/topo/src/offset_derive.rs:796` `section_closed`, the section's region bound from above.

**The second sweep: the point doors and `.lever_from(`**, production code (not `#[cfg(test)]` modules, rows, fuzz or probe files): 53 sites, read for this row.

- **In class, fixed by PR 7294:** `ring_path.rs` `spline_crossings`' ruling/segment arm (the chord `from → to`) and its iso arm (`spline_wall.rs:215` `IsoPiece::travel`). The rows are `ring_path::crossing_reach_rows`; red at the chord and at half the reach.
- **In-class candidate, not yet read to its consumer:** `crates/sweep/src/blend/battery.rs:593` `extent_of`. `points_diameter` of the rim's samples levers "what is read along the edge", the chain's G1 at a shared vertex (`:1102`). The G1 is read at the vertex, whose reach can be shorter than the diameter. The verdict's direction is not yet read.
- **Not in class: a reach from the point the datum is read at.**
  - A departure's reach from its base or vertex: `boolean/fate.rs:503`, `:1320`; `boolean/overlap.rs:576`; `link.rs:2575`; `link/second_order.rs:32`; `bound_side.rs:113` (a bound's chord from its base vertex); `offset_together.rs:709`.
  - `offset_axial.rs:1095`, the frame's extent: the body's vertices read from the axis origin, levering alignment margins that are directions and carry no point. It is a reach of the region's vertices only, not its curved edges. That is a cover question, not this row's.
  - The splitting folds, read from their own query point: `splitting/join.rs:475`, `splitting/order.rs:306`, `splitting/containment.rs:728`, `:1567`.
  - `ring_path.rs:453`, a cone ruling's lever from its base.
- **Not in class: a wedge reach from the edge's middle.** `wedge_reach.rs:122` and `pcurves/decision.rs:293` read the faces' points from the edge's middle (the wedge levers this row excluded at the start).
- **Not classified here: a region read from a pivot.** Its pivot has to be checked against where the datum is read: `boolean/join.rs` (`:1488`, `:3208`, `:3225`, `:4238`), `boolean/section_cert.rs` (`:1247`, `:1314`, `:1393`), `boolean/trace.rs:357`, `geom-brep` `locus.rs:580`, `:615`, `intersect.rs:551`, `:2410`, `carrier_eq.rs:839`, `pair_section/torus.rs:147`, `pair_section/cones.rs` (`:180`, `:221`, `:459`, `:673`), and `props/curved.rs:6582`. Each names an apex, anchor, frame origin or foot. Most read a datum at that same point (a cone's pose readings from its apex), but this row has not checked each.

**Open, scheduled here: `spline_crossings`' arc arm has no row.** The arc arm reads the arc's reach from the meeting (`√2` at a half turn's middle). Its old arm, the arc length (`π`), and the diameter (`2`) survive every `ring_path` row and the eight slow ring suites.

- **What is known.** No pose found reaches the crossing reading with those levers deciding differently:
  - The review's: a unit half turn met at its middle by a straight spline tilted out of its plane. `meet` escalates on `curve_pair_meet_stretch`: two in-band meetings `3.2·10⁻⁶` apart, before the crossing is read.
  - PR 7294's: the same half turn met in its plane by a cubic bent to the circle, `(1 − v²/2 + s·v + K·v³, v, 0)`, at `K` from 50 to 500 and `s` from `5·10⁻³` to `10⁻²`, under the band `(10⁻³, 10⁻²)`. `meet` escalates on `curve_pair_walk_pair_gap`, `curve_pair_meet_window` or `curve_pair_meet_stretch`, or says nothing.
  - The same cubic against a straight piece does reach the reading and decides cleanly (`crossing_reach_rows`). So what stops the arc is `meet`'s conic-against-spline reader, not the lever.
- **What is not excluded.** A pose whose curves match curvature well enough to pass the stretch and window rules, and still meet at a sine between `ε/(2r)` and `ε/(√2·r)`.
- **The bound swap is unkillable at a circle piece**: its cover is exact, so its two bounds are one.

**What a fix needs.** Every in-class caller has a point in hand (the foot, the root, the sample or the vertex), and the carrier and span that `edge_reach_bounds` (with `ExtentBall::of_cover_from` from above) reads. So no new door is needed in `geom-brep`. In `topo`, `cover::edge_reach_bounds` and `cover::piece_reach_bounds` serve the same purpose. Each caller's direction comes from its verdict:

- a definite sign takes the bound from below;
- a Zero acted on takes the bound from above;
- every sign takes both.

The differential needs re-taking per caller, as PR 7294 did for the router.
