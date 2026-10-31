---
id: curved-copies-filling-one-volume-read-as-undecidable
kind: issue
title: Copies whose coincident faces are cones, tori or splines are read as undecidable, and the rim pairs of copies decided to interfere stay undecidable beside the finding
status: open
opened: 2026-10-26
priority: P2
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

## Narrowed

2026-10-27. Cylinders, spheres and arc-bounded discs are read. The
aligned face-pair arm (`census.rs` `planar_face_pairs`) reads two more
cases:

- an arc-bounded disc pair the region door refuses (`NonPlanarTrim`);
- through its curved half (`curved_same_side_pairs`), two faces of
  different solids, both on cylinders or both on spheres, on two keys.

Each pair is on one carrier by Door 1 (`carrier_pair_reading`, a margin
decided Zero, `SameOriented`, `Definite`). It shares interior by
`shared_interior_witness`, which probes each vertex, each edge's
midpoint and each meeting of two edges, and reads the faces' cones in
the tangent plane there. Such a pair is
`SameSideInterference`, and arm 1 of `sweep_cross_solid_backstop` no
longer refuses it. The two pinned cylinder copies above are one loud
interference finding at the editor's gate (pinned at the identity and
turned off the axes), and so are coincident, spun and cut balls
(`crates/sweep/tests/curved_copies_filling_one_volume.rs`,
`asm_r2b_assembly.rs`
`copies_of_one_cylinder_filling_one_volume_are_one_interference`).

What stays open here:

- **The rim pairs.** A disc of one copy against the other copy's wall
  are two carriers meeting along the rim. Arm 1 still refuses them
  `CurvedWithinReach`, so the editor's partition carries them as
  evidence on the decided pair's finding (eight of them on the
  cylinder copies). The finding is loud either way. A quiet one would
  need each to lie on its own overlap (`at_rest.rs` `partition`), so
  an assertion cannot quiet the copies until these are read.
- **Other curved kinds.** Coincident cone, torus or spline faces on two
  keys are not read: Door 1 has no exact coincidence rung for them in
  this arm, so arm 1 still refuses them.
- **Curved copies in band.** Two cylinder or sphere copies whose
  carriers Door 1 reads in band (not `Definite`), offset by a few ε, are
  not read by the curved arm, so arm 1 still refuses their walls
  `CurvedWithinReach`. A sliver past the band is decided
  (`curved_copies_filling_one_volume.rs`
  `halves_off_by_a_few_eps_are_decided_past_the_band_only`). One in
  band may hide an overlap and is loud, never interference. Its planar
  counterpart is `an-aligned-in-band-pair-is-never-read-as-interference`.

Filed beside this row:
`aligned-arc-or-curved-faces-that-only-abut-are-never-read-flush` (the
two half-bands of the coincident cylinders that share only a seam ruling
are its subject), and `coincident-curved-copies-overlap-is-unlocalized`.

## Re-prioritised (2026-10-27, ATREST orchestrator): P1 → P2

The row's core ask merged (#5853 / #5827). What stays is residue that needs machinery not yet built: exact coincidence or metric rungs for cone, torus and NURBS carriers, a curved inner edge's extremal points, and the in-band and rim-pair readings. Each case stays loud and could-not-look, never a wrong verdict, so it is P2 work rather than a P1 defect.

## 2026-10-29: the ladder carries a cone (SPLITDOOR)

`boolean::carrier_eq` has a cone arm (`splitdoor/curved-copy`). The census's curved same-side arm still takes only what `carrier_eq::exact_curved` names (cylinder, sphere), so coincident cone copies stay as this row describes.
