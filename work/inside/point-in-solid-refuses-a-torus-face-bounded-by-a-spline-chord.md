---
id: point-in-solid-refuses-a-torus-face-bounded-by-a-spline-chord
kind: issue
title: point-in-solid refuses a torus face bounded by a spline chord (PartialTorusFace), so a body whose torus face a chord trims stops at the piece sort
status: open
opened: 2026-11-11
priority: P2
cost: M
---


Filed by SPIRICARM's rotational-lane stage 3 (`spiricarm/rotational-lane-torus`),
the torus twin of `point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord`.

## What

Once a chorded torus face measures through the rotational lane, a
boolean whose result keeps one passes the result gate and can stop at
the result's piece sort, where point-in-solid reads no torus face a
spline chord trims:

```
Pieces(Probe { shell: ShellKey(1v1), source: PartialTorusFace { face: FaceKey(6v3) } })
```

`crates/sweep/tests/germ_tilted_rod.rs`
`other_spins_and_steeper_tilts_refuse_at_their_germs`: the fifth op of
every pose (the half donut against the tilted rod) stops there, pinned
as `"pieces on a torus face"` at the compiled ε.

## Done when

point-in-solid reads a ray against a chorded torus face (its trim test
on the face's projected images), and the row moves to a body held to
an independent volume.

## More evidence (ORBIT, `orbit/tilted-torus-rim`, 2026-11-23)

The section value now marches a plane tilted against a torus's axis
(`geom_brep::PlaneTorusPose`), so a plane face crossing a torus face
obliquely leaves the torus face trimmed by a marched (spline) edge:

- `crates/sweep/tests/germ_torus_doors.rs`
  `subtract_and_intersect_reach_the_doors_union_does`: the
  near-perpendicular bar's `B ∖ A` (the bar less the donut) stops at
  `Pieces(Probe { …, source: PartialTorusFace { … } })`, pinned there;
  its `A ∖ B` and `A ∩ B` build.
- `crates/sweep/tests/germ_coplanar_conic.rs`
  `every_op_refuses_or_answers_its_closed_form`: the outer-equator box
  fixture's three ops build to their closed-form volumes, and
  `point_in_solid` on each result refuses `PartialTorusFace` at every
  witness, pinned as an outcome.

Further sites pinned by PR 7617, each skipping the piece sort's
`PartialTorusFace` for ops past ∪ under a floor that one such op builds
and is held to its truth:

- `crates/sweep/tests/germ_interior_oval.rs` `the_corner_bar_builds_its_lenses`;
- `crates/sweep/tests/germ_plane_torus_frame.rs`
  `the_donuts_corner_bar_builds_at_its_end_squares` and
  `a_general_pose_builds_to_its_volume`;
- `crates/sweep/tests/germ_torus_doors.rs`
  `subtract_and_intersect_reach_the_doors_union_does` (the near bar's
  `B ∖ A`);
- `crates/sweep/tests/germ_torus_rods.rs` (the fuzz reads `p` nowhere on
  such a body and holds it by its volume identities).
