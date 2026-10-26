---
id: curved-copies-filling-one-volume-read-as-undecidable
kind: issue
title: Copies whose coincident faces are curved, or planar with an arc rim, are read as undecidable, not as interference
status: open
opened: 2026-10-26
priority: P1
cost: H
refs: [two-copies-filling-one-volume-certify-at-rest]
---


Found by `two-copies-filling-one-volume-certify-at-rest`'s sweep (refs
that row).

## What

Two copies of one revolved cylinder (radius 1, height 1, a full turn),
both pinned at the identity, fill one volume. `assemble` refuses them as
could-not-look, not as interference:

- the two discs of each pair of caps are planar, on one plane, facing
  the same way, but each is bounded by an arc. The planar face-pair arm
  (`census.rs` `planar_face_pairs`, `same_side_overlap`) reads Door 1
  `SameOriented` and then the region door refuses the trim
  (`ChartRegionError::NonPlanarTrim`, "Harmonic trig channels are not
  exact-structural zeros"), so no `SameSideInterference` is decided;
- the two cylindrical walls are on two `SurfaceKey`s, and the conformal
  arm (`sweep_conformal_patches`) reads only same-key OPPOSED curved
  pairs;
- so arm 1 of `sweep_cross_solid_backstop` refuses every curved pair
  within reach (`CensusUndecidable`, "a curved face of one is within
  reach of the other"), and arm 2 refuses the solid pair behind it.

Since stage 5 C an undecidable verdict is could-not-look: loud, but it
never refuses at `Error` and no assertion can say the overlap is
intended. So the class is the same as the one the planar row closed:
an overlap of two copies' material that is not reported as interference.

## What closing it takes

A same-side reading for the faces the planar one cannot see: a region
door that reads an arc-bounded planar trim (or `planar_interiors_meet`
built for arcs, which it refuses today as `InteriorsUnread::Unbuilt`),
and an aligned-sense curved pair meeting over an area, read as
`SameSideInterference` rather than left to arm 1's proximity refusal.
The probe: the two pinned cylinder copies above, in
`crates/editor-core/tests/asm_r2b_assembly.rs`'s fixtures
(`fixture::revolve_about` of a unit square about `along_y`).
