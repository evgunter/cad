---
id: a-cylinder-face-re-homes-its-rings-on-a-chart-that-reads-no-spline-chord
kind: issue
title: A cylinder face re-homes its rings on its chart (chart_ring_side), whose window walk reads no projected image: a chord cut from a fitted section, on a cylinder face holding a bystander ring, refuses RunImageNotYet
status: dispatched
opened: 2026-10-28
priority: P1
cost: M
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
