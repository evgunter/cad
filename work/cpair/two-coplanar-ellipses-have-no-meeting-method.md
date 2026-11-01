---
id: two-coplanar-ellipses-have-no-meeting-method
kind: unit
title: The curve-pair reader refuses two coplanar ellipses that are not one curve, and refuses a near-tangent coplanar circle and ellipse instead of placing the touch
status: open
opened: 2026-10-14
priority: P2
cost: M
refs: [5227]
---

Found while building the curve-pair reader (`the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227), stage (b)).

Two poses of `geom_brep::curve_pair::pose` (`crates/geom-brep/src/curve_pair/pose.rs`, `conic_conic`) answer with a typed refusal where a closed form could answer:

- **Two ellipses in one plane that are not one curve** refuse `MeetFault::Unsupported`. The one-curve reading (`pose::one_curve`, PR 5058's Procrustes margin `one_curve_gap`) decides coincidence. Past it there is no metre residual: the second ellipse's implicit along the first is of degree two, but its distance has no closed form. Two routes are open:
  - a residual whose floor and ceiling per metre of distance are certified near the ellipse, for the subdivision's frame;
  - the certified lower bound on an ellipse's distance that `splitting::containment`'s `ConicArc::hit` already reads.

  The boolean's crossing layer answers `Unread` on this pose. A total reader still owes it an answer.
- **A near-tangent circle and ellipse in one plane** go to `certified_subdivision`, which refuses `Unsettled::Tangency` at a double root, where the circle pair's arm places the touch by the root gap. The subdivision refuses when a piece narrower than the band is neither clear nor monotone. The arm could instead place the touch on that piece by the degenerate formula, as the root gap does for two circles.

**Why P2.** `carrier_cross` answered `None` on this pose before it read `meet`, so its switch moved no verdict there; it answers `Unread`, and its caller keeps its frontier door. The census confirm is different. Its own walk (`census::curved::reads_meet`, retired in stage (c)) decided two coplanar ellipses that are not one curve. Reading `meet`, it refuses typed on both edges. The unit row `census::curved::tests::the_reading_answers_a_meeting_on_every_kind`, two ellipses at right angles meeting near `(a, a)`, `a² = 4/5`, moved from "meets" to that refusal. The topo and sweep `ci` profiles move no body-level verdict, so no body door is known to reach it. An edge-edge record on two elliptical rims in one plane would now refuse its confirm (`CarryRefusal::Unread` through `records::landings`).

**A follow-on unit of `the-curve-pair-reader`, owned by FUSE.** Stage (c) made the census confirm a caller of `meet` (PR 5162), so an edge-edge record on two coplanar elliptical rims now refuses typed where the census's own walk decided it. The unit builds the coplanar ellipse pair's method by one of the routes above, and pins it on the census row `census::curved::tests::the_reading_answers_a_meeting_on_every_kind`, which then reads "meets" again.

## Evidence: pass 5′ reaches it at body level (2026-10-16)

Tier 3′'s pass 5′ (`census::sweep_curved_edge_edge`) reads every pair of curved edges through `meet`. On `fuse/curve-pair-reader-c3` five `sweep` rows now fail tier 3′, each typed `CensusUnsupported { cause: ContactLane(NotCertifiable { what: "two ellipses in one plane that are not one curve" }) }` and on nothing else:
- `split_across_a_revolve_seam::a_tube_cut_across_its_axis_splits_into_two_annular_halves`;
- `split_across_a_revolve_seam::a_counterbore_and_a_cone_socket_split_across_their_axes`;
- `parallel_cylinder_join::the_rim_crossing_rods_build_in_every_op`;
- `planar_ring_arc_closure::a_prism_with_a_station_on_its_arc_rims_through_a_slab_builds_every_op`;
- `planar_ring_arc_closure::a_prism_with_arc_walls_through_a_slab_builds_every_op`.

The first three are among the 31 rows pass 5′ first turned red (`curve-pair-meetings-are-runs`). The last two are new on the c3 head, after main was merged in. So **Why P2**'s premise, that no body door is known to reach this pose, no longer holds. The row is P1: c3 does not land green until this unit does.

## The first bullet, done (2026-10-16, on `fuse/curve-pair-reader-c3`)

Route 2. Two coplanar ellipses that are not one curve go to the certified subdivision (`pose::coplanar_ellipses`):
- **The equation.** The first ellipse's level on the second, `F = X²/A² + Y²/B² − 1`, is of degree two in the first's angle.
- **Its metres.** They are those of geom's lower bound on a point's distance from an ellipse: `geom::ellipse_level_clearance`, which is `Curve3::clearance`'s ellipse bound, signed as `F`.
- **The ceiling.** The ceiling on `|F|` per metre of that bound is `4/s` within `s/4` of the ellipse, `s` the second's smaller semi-axis. The argument is in the function's docs.
- **The stretch rule** then decides the meetings.

The census row `census::curved::tests::the_reading_answers_a_meeting_on_every_kind` reads "meets" again. The five sweep rows in the evidence above pass. The geom-brep rows that pin it:
- `meet_rows::two_coplanar_ellipses_cross_where_the_level_changes_sign`;
- `meet_rows::two_coplanar_ellipses_apart_meet_nowhere`;
- `meet_rows::two_coplanar_ellipses_that_touch_are_never_read_apart`.

Route 1, a certified per-metre residual, was not needed: the subdivision reads only the ceiling, never a floor.

## Left open: the touch

The second bullet stays open, and is now the unit's whole scope.
- **Where it refuses.** A near-tangent coplanar circle and ellipse refuse `Unsettled::Tangency`, and two coplanar ellipses that touch refuse alike. At a touch, the level stays within the band over an arc much wider than the band (about `10⁻⁴` rad for the `(2, 1)` pair). So the subdivision either finds no definite sign to split at (`SignInBand`) or narrows a piece to the band (`Tangency`).
- **The fix is not natural to this route.** It would hand the in-band piece back as a candidate for the stretch rule to read. That changes `half_angle::certified_subdivision`'s contract, which topo's ellipse × torus, circle × torus, conic × quadric and ruling doors share.
- **Priority.** The row is back to P2: no known body row reaches it.

## Evidence: a body row reaches the touch (2026-10-28, `splitdoor/two-rims-coplanar`)

"No known body row reaches it" no longer holds. `crates/sweep/tests/a_ruling_lying_on_a_wall.rs` `two_rims_the_band_reads_coplanar_are_one_curve_only_within_it`, at `k` = 30 and 80, reaches the coplanar ellipse touch through the crossing layer. Two radius-`0.01` rods on one axis are cut below planes through the origin of slopes `100` and `100(1 + kε)`.
- **The pose.** The rims share their minor axis and differ in their major semi-axis by about `kε`. So in the planes' common plane one rim's level on the other is `F ≈ −2δ·cos²θ`, a double root at the minor-axis tips `(0, ±0.01, 0)`.
- **The stretch.** They stay within the escalation `10ε` of each other over `|cos θ| < 10/k`: about ±0.34 rad at `k = 30`, some 66 cm of rim.
- **The refusal.** The subdivision's first cut finds no definite sign in that sixteenth, and `pose::coplanar_ellipses` refuses `Unsettled::SignInBand`. The pass `carrier_cross::boundary_crossing` answers `Unread`, and every op refuses `CurvedPierceUnsupported` at `reduce.rs`'s curved sweep arm. Measured at ε = 1e-9 and 1e-12. At 1e-6 the basin meter refuses the seam ruling first.

Building the touch alone does not settle that row. There the glue door decides the two cut planes one carrier, and the rims' meeting is in band, so the ops owe `Escalated` (Ev on PR 5973; `work/splitdoor/two-rims-read-coplanar-refuse-unsupported-or-untyped.md`, "Decided").

## Evidence: the touch keeps a tangent-walls fixture from the walls' parting (2026-10-30, `splitdoor/sheets-part-conic`)

`crates/sweep/tests/sheets_part_on_slanted_rims.rs`
`two_slanted_rims_tangent_on_the_ruling_refuse_at_their_touch`:
- **The fixture.** An inner rod of radius 1 about `(1, 0)` is cut below `z = 0.3·y`, and an outer rod of radius 2 about the origin is cut above the same plane. The walls are tangent inside one another along `x = 2, y = 0`. The rims are two ellipses in one plane, tangent at `(2, 0, 0)`, the inner one inside the outer.
- **The read.** For the inner rim against the outer wall, and the outer rim against two of the inner rod's faces, `carrier_cross::boundary_crossing` answers `Touch`, measured at 1e-9. It does not answer `Unread`. `reduce.rs`'s `interior` then reads the edge `Unseen`, the curved arm defers it, and every op refuses `CurvedPierceUnsupported { operand: A, face: FaceKey(4v1), edge: EdgeKey(12v1) }` at 1e-6, 1e-9 and 1e-12.
- **Cut flat.** The rims are circles, the crossing reads `At`, and both ops build at their closed forms, tiers 3 and 3′ included.
- **What it blocks.** The two rims are coplanar here, so the touch is read before the walls' parting is asked. Two elliptical rims in two planes (the inner rod cut below `z = y·tan t_a`, the outer above `z = y·tan t_b`, `t_a ≠ t_b`) cross transversally at the same point instead. There `boundary_crossing` reads `At`, `sheets_part` reads an ellipse on both sides, and the ops build or refuse on other grounds (`sheets_part_on_slanted_rims::rims_crossing_on_tangent_walls_build_every_op`). Only the coplanar, tangent pose stops at this unit's touch. Once the touch places, the pin moves.

## Evidence: tier 3′ on a body that builds (2026-11-01, `splitdoor/rims-seam-orientation`)

`crates/sweep/tests/sheets_part_on_slanted_rims.rs` `rims_crossing_on_tangent_walls_build_every_op` asserts the escalation. In `A ∪ B` and `B ∖ A` of two rods tangent inside one another and cut by two planes, B's cut cap keeps two coplanar ellipses internally tangent at a vertex they share, `(2, 0, 0)`: B's rim, and the section arc of A's wall. Every edge-edge pair of them that the census reads answers `MeetFault::Touch` on `curve_pair_coplanar_sub_side` at about 2ε, including the pairs whose two edges meet only at that shared end. The census types it `CensusEscalated`. The bodies build at their closed forms, tiers 1–3, at ε = 1e-6, 1e-9 and 1e-12. Placing the touch at the pair's shared end would clear those pairs.
