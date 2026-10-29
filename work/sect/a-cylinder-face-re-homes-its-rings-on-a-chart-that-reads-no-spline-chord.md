---
id: a-cylinder-face-re-homes-its-rings-on-a-chart-that-reads-no-spline-chord
kind: issue
title: A cylinder face re-homes its rings on its chart (chart_ring_side), whose window walk reads no projected image: a chord cut from a fitted section, on a cylinder face holding a bystander ring, refuses RunImageNotYet
status: closed
opened: 2026-10-28
closed: 2026-10-29
priority: P1
cost: M
branch: sect/rehome-by-path
---


Found by SECT step 1b (`sect/cylinder-sphere-chord`, 2026-10-28), in
its sweep for readers of a ring-lane island with no spline arm.

## What

Step 1b closes a carried chord's ring-lane island by a path on every
curved face (`ring_path::Quadric::Cylinder`, `LoopArc::Spline`), but
ring re-homing after a chord's `mef` still reads a cylinder face on its
chart: `ChordJoiner::rehome_rings` takes `Homing::Chart` for every
cylinder, and `chord_join::chart_ring_side` walks the run's edges
through `face_azimuth_images`, whose window reads only a closed-form
span box. A chord on a fitted carrier has a projected image there, so
the walk ends `SplitJoinError::RunImageNotYet` (as
`pcert/a-spiric-run-edge-ends-the-window-walk-as-an-invariant` records
for the projected and general images).

None of the 54 runs of `crates/sweep/tests/cylinder_sphere_frame.rs`
reaches it: no chord there divides a wall face that holds a bystander
ring. A wall pierced elsewhere (a second ball, a rod through the drum)
before a carried chord divides it would.

## Done when

A cylinder face re-homes a ring by the path reading the sphere, cone
and torus faces use (`chord_join::path_ring_side`, which now reads a
cylinder and a spline arc), or the chart walk reads a projected image;
and a row divides a ringed wall by a carried chord.

## Closed

2026-10-29, by `sect/rehome-by-path`. A cylinder face re-homes its
rings by the path, as the sphere, cone and torus faces do
(`ChordJoiner::rehome_rings` takes `Homing::Path` for every curved
face; `chord_join::path_ring_side` reads a spline run edge as a loop
arc). The chart walk (`chart_ring_side`) and its three predicates are
deleted. The parity is path-independent on a cylinder, an annulus
(`ring_path`'s module docs), so no chart image of the run is read.

Measured: `crates/sweep/tests/a_ringed_wall_divided_by_a_carried_chord.rs`
(a drum against two balls as one operand, each piercing the wall) had
three of its six runs (the balls first) stop at `RunImageNotYet` from
`chart_ring_side`; all six now stop past the join, at the result
gate's volume or the spline edge join. On PR 6011's head, drum × donut
placement 5 stops there in all six runs and passes the re-homing with
this change. `chord_join::wall_ring_rows` searches a spline-bounded
wall island's bystanders against membership in the ball.

The cylinder's other chart reader on a ring-lane run, the island
winding a wall face's planar chord closes, is
`a-wall-islands-planar-closing-reads-its-run-on-the-chart`.

