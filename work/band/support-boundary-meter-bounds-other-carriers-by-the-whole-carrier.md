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
through `boxed_reach`: the certified axis-aligned box `geom::curves::boxes`
mints for that piece (`ellipse_arc_aabb` and `spiric_arc_aabb` over the
edge's window, `nurbs_curve_aabb` over the control hull), read for the
distance from the trim centre or the height along the axis. The box
holds the piece, so the read never passes an edge that reaches the
strip, and its refusal says it is a bound (`RingClearance { bounded:
true }`). It can still refuse an edge that clears:

- the box is AXIS-ALIGNED, so on a tilted ellipse or a support whose
  axis is not a world axis its corners overstate the reach;
- a NURBS edge's box is its whole control hull, not its window's.

An earlier spelling of this read bounded an ellipse's distance by the
ball of radius `major`, which is NOT sound: tier 3 does not order
`major` and `minor` (a STEP `ELLIPSE` stores its semi-axes as given),
and a swapped ellipse leaves that ball. The box has no such premise
(`boxed_reach_holds_every_point_of_its_piece` reads both spellings).

Reachable for the ellipse on a curved support
(`band_annulus_host_boundary::a_tilted_cut_reaching_a_cylinder_hosts_trim_between_samples_refuses`,
where the cylinder's axis is a world axis and the box's height range is
the arc's own). No row measures an over-refusal; a support whose axis
is tilted, or a STEP-imported plate with a spline outline near a bore,
is where one would show.

## What the taker owes

If a row shows the box over-refusing: read the ellipse's height in its
own frame (a sinusoid of the parameter, whose window extremes need the
window-membership home `arc-window-membership-has-three-spellings`
asks for), and a NURBS edge over its window (subdivide its control
polygon to the window first).
