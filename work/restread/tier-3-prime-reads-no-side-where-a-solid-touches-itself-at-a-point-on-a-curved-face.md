---
id: tier-3-prime-reads-no-side-where-a-solid-touches-itself-at-a-point-on-a-curved-face
kind: issue
title: Tier 3′ reads no side where a solid touches itself at a point a curved face of it passes
status: open
priority: P1
cost: H
opened: 2026-10-21
---

Found by `tier-3-prime-reads-no-side-for-a-face-touching-a-vertex-of-its-own-solid`'s
lane (branch `restread/point-touch-winding`).

## What

**A silent pass, across many real Boolean results.** The self-touch
reading (`sweep_self_touch_points`, `crates/topo/src/census.rs`) reads
a solid's whole link where the solid touches itself at a point
(`Link::at_point`, `rest::one_material`). `Sector::of` reads planar,
line-bounded faces only. Where a curved face or edge is in the link,
the reading is skipped, so the point passes unread, as it did before
the reading existed. The census module doc lists the class as
genuinely undetected. A curved sheet crossing through such a point is
not seen, and neither are planar sheets crossing there: one curved face
anywhere in the link leaves the whole point unread, its planar faces
included.

The skip was ruled for now in that PR (orchestrator with Ev,
2026-10-25). Refusing these points instead
(`BoundaryCrossesItselfUndecided { why: Unreadable }`) would have
refused 19 sweep rows, 34 findings, every one curved.

## The 19 measured families (default ε, on the lane's branch)

- `a_pierce_held_in_a_curved_face`: a cylinder wall holding the
  pierce; a wall the ruling pierces twice.
- `a_ruling_lying_on_a_wall`: a prism edge on the tube's wall; a
  ruling across an ellipse.
- `a_ruling_through_a_box_corner_edge`: a post through the near wall
  piece; a wall through a box edge.
- `a_tube_ending_on_a_ball::the_probe_family_ships_no_wrong_body`.
- `bisector_nodes`: a rod against a bore; slanted bores pinched to
  themselves.
- `curved_notch_pinch::a_curved_notch_at_a_corner_pinch_builds_bodies_the_census_cannot_read_yet`.
- `pi_seam_and_kiss_through_the_boolean`: a dome sunk across the
  tube's seam rulings; a rod rim on the dome across its seam meridian;
  a tube through the dome's base; a turned rim on a wall bounded by an
  ellipse.
- `reach_wall_chord_rows::a_cube_touching_a_drum_at_a_corner_answers_its_closed_form`.
- `rest_zip_admission`: a reflex union at a tangent site; the tangent
  site's controls; the reflex site off the pins' poses.
- `verbs_ga_r2_probes::r2_a_box_with_on_carrier_rulings_builds_every_op_but_its_void_which_refuses_typed`.

All are in `crates/sweep/tests/`. Each builds bodies where one solid
touches itself at a point on a cylinder, cone, sphere or torus face,
or on a curved edge.

## The fix

A curved face's sector at the point is its tangent cone there: a
half-plane or plane of the tangent plane, bounded by its edges'
tangents. The winding reader needs the curved sheet's side read off
its tangent plane only where the sheets separate to first order. A
tangency (the sheets agreeing to first order) needs the second-order
reading the cross-solid curved lanes take. The cross-solid analysis
has the same gap (`TouchUnreadable`). The fix lands with these 19
families read and the skip removed.
