---
id: a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm
kind: issue
title: a sphere face whose tilted circle passes near a chart pole refuses at the trimmed lane's sphere arm
status: open
opened: 2026-10-28
---


Filed by TESS's sphere-arm unit
(`sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane`).

**Measured.** The trimmed lane's sphere arm bounds each boundary
chord's UV step by its lattice steps, from a closed-form speed bound
of the circle's image (`mesh::chords::sphere_uv_speeds`):
`|u′| ≤ |p′|/ρ`, which grows as `1/cos v` toward a pole. The chord
schedule is uniform in the carrier parameter, so a circle passing a
pole at polar distance `d` is chorded everywhere as finely as its
closest approach needs, about `1/d` times the chords an arc a row away
needs. The arm refuses typed where `d < NEAR_POLE_ROW_SHARE · h_v`
(`mesh::trimmed::NEAR_POLE_ROW_SHARE` = 1/16 of a lattice row):

```
UnsupportedCurve { note: "trimmed sphere face bounded by a tilted circle
  whose arc passes nearer a chart pole than the sphere arm's lattice
  resolves — …" }
```

Witnesses, each building sound at its closed form and pinned at this
refusal:

- `crates/sweep/tests/tilted_sphere_pair.rs`,
  `a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed`:
  two unit balls whose radical circle passes 1e-5, 1e-4 and 3e-4 from
  the first ball's pole, meshed at δ = 1e-2;
- `crates/sweep/tests/spheres_crossing_off_every_edge.rs`,
  `a_trimmed_face_cut_beside_its_pole_builds` (`Want::NearPole`): a
  ball of radius 0.01 whose circle passes 1e-3, 1e-5 and 1e-7 from the
  trimmed unit ball's pole, at δ = 5e-3.

**What a fix owes.** A chord schedule or a chart that follows such a
circle without the uniform `1/d` cost: a schedule graded toward the
closest approach (the chord pass's schedule is uniform by design and
shared by both faces, so this is a change to that contract), or a
face-local sphere chart whose poles are off the face, in which the
circle is nowhere near a pole and the polygon is read from the chord
points' 3-D positions rather than the stored image.
