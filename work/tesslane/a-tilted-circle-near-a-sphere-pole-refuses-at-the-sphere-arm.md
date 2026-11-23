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
of the circle's image (`geom_brep::Pcurve::speed_bound`):
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
  - `a_circle_near_the_pole_meshes_at_a_coarse_delta`: 1e-3 meshes at
    δ 1e-2 and 4e-2. At 4e-2 the boundary-only fold is a triangle with a
    side the other ball's face may make too, so the trimmed lane's
    shared-side cure (carried to the sphere arm by
    `work/tesslie/a-cone-face-near-its-apex-meshes-an-edge-of-four-triangles.md`)
    splits it, and the fold goes with it;
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

- 2026-11-14 — moved from TESS to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## As a ring (2026-11-14, TESSLANE unit 3)

A tube of radius 0.3 tilted 0.22 about `x` and turned 3 about `z`, on
`ball_poled_z(√2)`, rings a ball face about 0.006 rad from its pole. The
ringed ops (both unions, ball ∖ tube) refuse `FoldedTriangle` at
δ `2·10⁻²` and mesh at `5·10⁻³` and `10⁻³`; turned 0 about `z`, they mesh
at every δ. Pinned by
`reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause`.

## Evidence (2026-11-23, ORBIT `orbit/one-sphere-escape-cut`)

That branch retires the boolean's re-chart of a closed ball: every sphere escape is now cut in on the ball's own chart, so a section circle that lies beside the ball's own pole stays there. Before, the re-chart turned the pole onto the centre line, away from the circle. `crates/sweep/tests/spheres_crossing_off_every_edge.rs` `a_circle_beside_a_pole_builds` moves onto this row. Every op of the unit ball against a radius-0.01 ball whose circle passes 1e-5 rad from the unit ball's pole builds at its closed form, tier-3 valid. Both unions and the unit ball's difference then refuse `FoldedTriangle` at δ 5e-3, and the other three draw. At 1e-7 rad all six ops build and then refuse `SphereTrimShape { ChordBudget }`. Those poses drew before. The re-chart had been keeping such poses off this arm, and now nothing does, which may bear on this row's band.
