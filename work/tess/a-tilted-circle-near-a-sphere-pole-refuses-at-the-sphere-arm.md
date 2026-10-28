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
needs.

**The cap (since PR 5920's fix pass).** The first cut refused at a
tuned `d < h_v/16`. Its review measured that as both too eager (at
`d = 1e-3` the face meshes clean with the share at zero) and growing
with δ. A second cut capped one edge's count at the point count of the
whole sphere's lattice; that was perverse in δ too, refusing the unit
balls at `(0, 0, 0)` and `(1, 0.8, 0)` at δ ≥ 2e-2 while they mesh
finer. The schedule is now held to `sizing::MAX_COUNT` alone, the cap
every count in the crate is held to (`ResolutionOverflow` past it). The
pair meshes at every δ from 0.1 to 1e-3
(`tilted_sphere_pair::a_section_near_a_pole_meshes_at_every_delta`).
An arc whose speed bound fails (`ρ² ≤ 0` on its closed-form lower
bound) refuses `SphereTrimShape { cause: ThroughPole }`.

What remains is cost and folds. Below `MAX_COUNT` the schedule can ask
millions of chords, and near the pole the uniform lattice leaves
triangles folded over the chord beside it, which the winding census
refuses (`FoldedTriangle`) after its grid-drop retries:

- `crates/sweep/tests/tilted_sphere_pair.rs`,
  `a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed`
  (radical circle 1e-5, 1e-4, 3e-4 from the pole, δ = 1e-2): all
  three build. At 3e-4 the mesh takes about 26 s in a debug build and
  refuses `FoldedTriangle`. At 1e-4 it takes about 254 s and folds,
  and at 1e-5 it asks millions of chords. All three are measured, not
  meshed in the suite, which pins the fold at the next row's cost.
- `crates/sweep/tests/spheres_crossing_off_every_edge.rs`,
  `a_trimmed_face_cut_beside_its_pole_builds` (a ball of radius 0.01
  whose circle passes 1e-3, 1e-5, 1e-7 from the pole, δ = 5e-3): at
  1e-3 every op draws, its volume checked. At 1e-5 three ops draw and
  three refuse `FoldedTriangle`, about 2 s each. At 1e-7 nothing is
  meshed in the suite (millions of chords).

**What a fix owes.** A chord schedule or a chart that follows such a
circle without the uniform `1/d` cost: a schedule graded toward the
closest approach (the chord pass's schedule is uniform by design and
shared by both faces, so this is a change to that contract), or a
face-local sphere chart whose poles are off the face, in which the
circle is nowhere near a pole and the polygon is read from the chord
points' 3-D positions rather than the stored image.
