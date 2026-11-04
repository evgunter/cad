---
id: point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord
kind: issue
title: point_in_solid refuses a sphere face bounded by a spline chord (PartialSphereFace), so every cylinder x sphere result that builds is no legal operand
status: open
opened: 2026-11-04
priority: P1
cost: M
---


Filed by SPIRICARM's rotational-lane stage 2 (the sphere chart,
`spiricarm/rotational-lane-sphere`, 2026-11-04), once cylinder ×
sphere booleans build.

## Measured

With the sphere's chorded faces measured (`geom_brep::props::quad`
`rotational_cut_face_rounds`, `RotationalChartQ::Sphere`), every
cylinder × sphere boolean in `crates/sweep/tests/cylinder_sphere_frame.rs`
builds at its oracle's volume, passing tiers 2 and 3′ and the at-rest
certificate, but no result is a legal operand: its union with a far
brick refuses `BooleanError::Containment(PartialSphereFace { face })`.
`topo::boolean::solid_contain` (`face_geo`, the `Surface::Sphere` arm)
reads a sphere face that is not closed on its surface through
`sphere_face_region`, which crosses great circles with circle arcs
only, so a face bounded by a spline chord (its pcurve a projected
image) has no region there and the walk refuses it.

The same refusal stops a result's piece sort
(`BooleanError::Pieces(PieceSortError::Probe { source:
PartialSphereFace })`): both two-loop poses' `A ∖ B` in that file, and
each rod's `b − a` in
`crates/sweep/tests/the_door_reads_seam_regions_as_tier_3_does.rs`
(`unit b-a rod`, `unit b-a rodx`, `tilt b-a rodx`). `PieceSortError::Probe`'s
own doc said no row reached it.

Counts at the default ε, debug, on the branch: 52 of
`cylinder_sphere_frame`'s 54 runs build at their volume and refuse only
as an operand; the other 2 refuse at the piece sort. Every run of
`a_ringed_wall_divided_by_a_carried_chord` and
`verbs_cylsph_opening::a_transversal_pose_builds_in_both_poses` builds and
is no operand. The other ε rows were not run.

## Why P1

A normal boolean on normal geometry builds a body that no later boolean
can take, so a cylinder × sphere result is a dead end for authoring. It
is P1 rather than P0 only because the first boolean now answers.

## Done when

`point_in_solid` reads a sphere face bounded by projected chords (its
region off the stored pcurves, as the cylinder's `ruled_region` reads
its walls), and the rows above go SOUND: `cylinder_sphere_frame::at_its_volume`
and `no_operand_at_the_sphere` are where they are pinned.
