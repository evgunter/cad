---
id: re-homing-a-ring-on-a-spline-wall-refuses-where-the-new-face-holds-a-rim-edge
kind: issue
title: Re-homing a ring on a spline wall refuses RingIslandUnread where the face a chord walls off holds one of the wall's rim edges
status: open
opened: 2026-11-13
priority: P2
cost: M
---


Found by the review of PR 6891; filed by the
`sectarms/spline-ring-winding` lane's fix pass.

## What

A fork with one prong wholly inside the loft prism's flat wall and one
across the wall's bottom rim (`x ∈ [0.3, 0.6]`, `z ∈ [−0.25, 1.25]`)
refuses `JoinError::RingIslandUnread { kind: Nurbs }` in `prism ∪ fork`
and `prism ∖ fork`. The rim prong's chords run from the wall's rim back
to it, on the outer lane, and the `mef` that closes them walls off a
face whose cycle is those chords and a stretch of the wall's own rim.
The inside prong's rings are bystanders of that face, and
`chord_join::path_ring_side` reads the whole new face's cycle through
`run_loop_arcs`, which on a spline wall reads an edge only where it is
a spline that is the wall's certified intersection with another
surface (`SplineArc::of_edge`). A rim edge is neither, so the arm
`(Carrier::Wall(_), _) => None` refuses.

Pinned by `sweep`
`a_partner_across_a_lofted_wall_is_joined_on_its_section_value::a_fork_inside_and_across_the_rim_of_a_lofted_wall_refuses_its_re_homing_by_name`.

## What a fix has to settle

A rim edge of the wall lies on the wall (it is the face's own edge, an
iso row of the net or a seam the loft certified), so it could be read
as a loop arc with that as its evidence, and the curve-pair table
reads a line or a spline against an iso piece alike. Two things then
need an argument, which is why the fix pass filed this and did not
build it:

- a path's far end is an outer-loop point, and the wall's rim is the
  chart's edge, so a piece that runs along `u = 0` or `v = 0` shares a
  run with a rim arc and says nothing; whether enough references
  decide on a face whose every outer vertex is a chart corner is
  unmeasured;
- `path_parity`'s iso arm reads a spline arc alone, and a rim edge on
  a `Line` carrier (a degree-one row) is a `LoopArc::Line`, whose arms
  are stated for a quadric's pieces.

## Done when

The row above passes the join in both runs and is pinned where it
stops next.
