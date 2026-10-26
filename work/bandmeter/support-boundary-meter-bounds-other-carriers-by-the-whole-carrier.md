---
id: support-boundary-meter-bounds-other-carriers-by-the-whole-carrier
kind: issue
title: blend: the rim support-boundary meter bounds an ellipse, spiric or NURBS edge by an axis-aligned certified box, which can over-refuse
status: open
opened: 2026-10-01
priority: P3
cost: M
---


## Finding

`support_boundary_clearance` (`crates/sweep/src/blend/surgery.rs`)
reads a line or circle edge over its stored window exactly
(`piece_distance`, `piece_along`), and an ellipse, spiric or NURBS edge
through `boxed_reach`, off the certified axis-aligned box
`geom::curves::boxes` mints for it: an ellipse arc's over its window
(`ellipse_arc_aabb`), a spiric oval's over its WHOLE period
(`spiric_arc_aabb`), a NURBS curve's over its whole control hull
(`nurbs_curve_aabb`). An ellipse's HEIGHT along the axis is also read
exactly over its whole period (a sinusoid in any frame) and intersected
with the box's. Each range holds the piece, so the read never passes an
edge that reaches the strip, and its refusal carries `RingClearance {
bounded: true }`. It can still refuse an edge that clears:

- an ellipse's DISTANCE (a plane support) comes from the axis-aligned
  box, which overstates on a tilted ellipse;
- a spiric's and a NURBS edge's box covers the whole carrier, not the
  edge's window;
- an IN-BAND bounded margin goes out as `Escalated` with the
  two-tolerance recourse, which cannot settle a limit set by the box's
  width rather than by the tolerance.

An earlier spelling bounded an ellipse's distance by the ball of radius
`major`, which is NOT sound: tier 3 does not order `major` and `minor`
(a STEP `ELLIPSE` stores its semi-axes as given), and a swapped ellipse
leaves that ball. The box has no such premise
(`boxed_reach_holds_every_point_of_its_piece` and
`band_review_3715_swapped_ellipse_distance_ball_encloses` read both
spellings).

Rowed: an ellipse on a tilted-axis cylinder host, where the world box's
height alone over-refused and the exact sinusoid carves
(`band_annulus_host_boundary::a_tilted_cut_clear_of_a_tipped_cylinder_hosts_trim_carves`).
No row measures a distance over-refusal; a STEP-imported plate with a
spline outline near a bore, or a tilted bore crossing a cap's outer
cycle, is where one would show.

## What the taker owes

Make the in-band bounded ending honest (a recourse that names the
bound, not the tolerance). If a row shows the box over-refusing on
distance: an ellipse arc's distance in its own frame, and a NURBS edge
over its window (subdivide its control polygon to the window first) —
any window-membership read routed through the home
`arc-window-membership-has-three-spellings` asks for.
