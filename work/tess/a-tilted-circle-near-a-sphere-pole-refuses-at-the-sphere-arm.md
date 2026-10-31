---
id: a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm
kind: issue
title: a sphere face whose tilted circle passes near a chart pole refuses at the trimmed lane's sphere arm
status: open
opened: 2026-10-28
priority: P3
cost: M
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

**The cap (since PR 5920's fix passes).**
1. The first cut refused at a tuned `d < h_v/16`. Its review measured
   that as both too eager (at `d = 1e-3` the face meshes clean with
   the share at zero) and growing with δ.
2. A second cut capped one edge at the whole sphere's lattice count.
   That was perverse in δ, refusing the unit balls at `(0, 0, 0)` and
   `(1, 0.8, 0)` at δ ≥ 2e-2 while they mesh finer.
3. A third held the schedule to `sizing::MAX_COUNT` alone, and the
   second review found the hang that leaves: the face's CDT took 26 s
   per attempt at `d = 3e-4` (debug), quadratic in its incremental
   insertion.

**What ships.**
- **The CDT is bulk-loaded** (`TrimmedFace::insert_points`, spade's
  `bulk_load_cdt`, `O(n log n)`). In release the tilted pair at `d =
  3e-4` meshes or refuses in 0.2–0.3 s.
- **A stated per-edge work budget.** `trimmed::MAX_SPHERE_EDGE_CHORDS`
  (2¹⁶) bounds the chords the sphere's tightening may ask of one edge;
  past it the arm refuses `SphereTrimShape { cause: ChordBudget }` at
  once. Its doc states the measurement: 3.7·10⁴ boundary points mesh in
  0.2–0.3 s, and 5.7·10⁵ take 18–62 s per attempt even bulk-loaded.
  It is a count, so a coarser δ, which asks fewer chords, never refuses
  where a finer one meshes.
- An arc whose speed bound fails (`ρ² ≤ 0`) refuses `ThroughPole`.

Release wall times on the tilted pair (the radical circle `d` from the
pole), after both changes:

| d | δ 1e-2 | δ 4e-2 |
|---|---|---|
| 1e-3 | meshes, 46 ms | `FoldedTriangle`, 26 ms |
| 3e-4 | `FoldedTriangle`, 0.18 s | `FoldedTriangle`, 0.32 s |
| 1e-4 | `ChordBudget`, 28 µs | `FoldedTriangle`, 1.7 s |
| 3e-5, 1e-5, 1e-7 | `ChordBudget`, under 30 µs | `ChordBudget`, under 10 µs |

What remains is the folds: near the pole the uniform lattice leaves a
triangle folded over the chord beside it, which the winding census
refuses (`FoldedTriangle`) when its grid-drop retries do not clear it.
At `d = 1e-3`, δ 4e-2, the fold has no grid corner to drop (all three
corners on the boundary); a chord refinement there, not a grid drop,
is what would cure it.

Pinned:
- `crates/sweep/tests/tilted_sphere_pair.rs`:
  - `a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed`:
    `ChordBudget` at 1e-4 and 1e-5, δ 1e-2;
  - `a_circle_near_the_pole_folds_at_a_coarse_delta`: 1e-3 meshes at
    δ 1e-2 and refuses `FoldedTriangle` at 4e-2;
  - `a_section_near_a_pole_meshes_at_every_delta`: the (1, 0.8, 0)
    pose at every δ from 0.1 to 1e-3.
- `crates/sweep/tests/spheres_crossing_off_every_edge.rs`,
  `a_trimmed_face_cut_beside_its_pole_builds` (a ball of radius 0.01
  whose circle passes the pole, δ = 5e-3):
  - 1e-3: every op draws;
  - 1e-5: three ops draw and three refuse `FoldedTriangle`;
  - 1e-7: `ChordBudget`.

**What a fix owes.** A chord schedule or a chart that follows such a
circle without the uniform `1/d` cost: a schedule graded toward the
closest approach (the chord pass's schedule is uniform by design and
shared by both faces, so this is a change to that contract), or a
face-local sphere chart whose poles are off the face, in which the
circle is nowhere near a pole and the polygon is read from the chord
points' 3-D positions rather than the stored image; either retires
the budget and the folds together.
