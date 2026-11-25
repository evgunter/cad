---
id: plane-cone-trileans-lever-a-span-from-the-apex-and-a-face-at-its-own-reach
kind: issue
title: plane × cone levers its trileans from the apex over a span, but at the consumed region's own reach over a face
status: open
opened: 2026-10-11
priority: P3
cost: M
---


## What

`plane_cone_section` (`crates/geom-brep/src/intersect.rs`) levers its
trileans (`pn_aperture_*`, `pn_apex_section`, `pn_axis_normal`,
`pn_conic_type`) at `reach.lever_from(apex)`. Which length that is
depends on the reach the caller hands:

- **A face** (`Reach::Face`, the chord join's `section_reach` and the
  boolean frame's `FrameExtent::Cone`): its farthest distance from a
  point of it, `across`, whatever the apex. That is the region's own
  reach, and `cone_frame_rows::a_frustum_far_from_its_apex_is_levered_at_its_own_reach`
  pins it: levered at the reading point's distance from the apex
  (`≈ 100.5`) rather than the face's reach (`≈ 0.5`), `pn_conic_type`
  clears the band and the frame serves an ellipse the face's own extent
  cannot decide.
- **A span** (`Reach::Span`, `route_pose` and `offset_derive`'s
  `section_closed`): the carrier's farthest distance from the apex,
  which on a frustum far from its apex is that same overstated `≈ 100`.
  `route_pose` folds `lever_from(anchor)` for every anchored kind, and
  `offset_derive`'s cone lane reads the same span since PR 4646, so the
  two agree with each other and not with the face readers.

An overstated lever overstates every margin it meters, so a definite
sign is decided where the region's own reach leaves it in the band: the
row above's unsound serve, on the offset door's lane.

## Not done

PR 4646 kept each caller's lever (its review asked for main's values
where they are right) and documented `Reach::lever_from` per variant.
The fix is one convention for the plane × cone trileans: the consumed
region's own reach from a point of it, which a `Span` would supply as
`Reach::span_reach_from` read from a point of the span, or as the
edge's own extent (`edge_extent`, what `offset_derive` read before PR
4646). It moves `route_pose`'s lever for spheres and tori too, so their
arms' rows need reading first. A row: the far frustum's pose read
through `route_pose` over the face's generator span, which escalates
`pn_conic_type` exactly where the frame does.

Moved from GERMCERT to GERM on 2026-11-02, by `git mv`, when GERMCERT merged into GERM: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so GERM, GERMCERT, ROOTS and APEX came in under one budget.

## Within one Boolean, the frame and the chords (ORBIT, 2026-11-24)

Found by `orbit/plane-torus-one-lever`'s sweep for pairs whose
closed-form question is asked at more than one lever, which settled
the plane × torus case by having every reader take the section value's
pose (`geom_brep::PlaneTorusPose::closed_form`). Plane × cone has the
same shape inside one operation: the glue door reads the pair once at
the surface pair's reach (`boolean::join::surface_pair_reach`, held in
`SectionTable::plane_cone`), which the germ mint and the join's frame
take, while each chord reads `plane_cone_section` again at its own
divided face's reach (`chord_join::section_reach`'s cone arm, then
`plane_wall_case`'s `SectionReach::Cone` arm; and the planar side's
`bool_planar_chord_spec`). A pose whose `pn_apex_section` or
`pn_axis_normal` reads Zero at one of those levers and past the band
at the other is a line pair to the frame and a conic to the chord, or
the reverse. Not measured: the straddle wants a cone face whose own
reach differs from the surface pair's by enough to cross the band. The
plane × torus fix's shape applies: the chords take the held reading
instead of asking again.
