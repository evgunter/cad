---
id: two-coplanar-ellipses-have-no-meeting-method
kind: unit
title: The curve-pair reader refuses two coplanar ellipses that are not one curve, and refuses a near-tangent coplanar circle and ellipse instead of placing the touch
status: open
opened: 2026-10-14
priority: P1
cost: M
refs: [the-curve-pair-reader]
---

Found while building the curve-pair reader (`work/fuse/the-curve-pair-reader.md`, stage (b)).

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
