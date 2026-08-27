# Review probes — GUI-0 R2 (PR #1094, head f511a00)

Three mutants executed against the shipped 26-test suite plus the R2
consumer suite (each applied, run, reverted; recorded here so the runs
are reproducible). Runner: this review lane's container, headless,
`cargo test -p viewer --test all`.

## Mutant 1 — reversed winding at the scene seam
`crates/viewer/src/scene.rs`, `SceneMesh::build`: reorder each
triangle's corners `[a, b, c]` → `[a, c, b]` after computing the
normal (so normals stay outward while the emitted winding flips —
the case a normals-only check cannot see).
Result: `scene_build::the_triangles_wind_outward_and_enclose_the_right_volume`
**FAILS** (enclosed volume goes negative). 5 of the 6 other scene rows
pass, including `a_finer_delta_never_coarsens_the_mesh` (its volume
assertion is on |error|, so it survives — the winding row is the one
carrying this claim). Claim 1's winding arm holds.

## Mutant 2 — lost field-of-view factor in the pan rate
`crates/viewer/src/input.rs`, `world_per_px`:
`2.0 * camera.distance() * (camera.fov_y() * 0.5).tan()` →
`2.0 * camera.distance()`.
Result: `input_mapping::a_pan_keeps_the_point_under_the_cursor`
**FAILS** ("a 137 px drag moved the world point 331.6 px"); the R2
random-state twin fails too. Claim 1's pan arm holds.

## Mutant 3 — dropped aspect division in the projection
`crates/viewer/src/camera.rs`, `projection_matrix`:
`[t / aspect, 0, 0, 0]` → `[t, 0, 0, 0]`.
Result: **all 13 shipped `camera_ops` rows PASS**, including
`framing_puts_the_whole_scene_inside_the_frustum`. The containment
assertion is structurally one-sided: the fit backs off by the SMALLER
half-angle, so the scene occupies at most `1/max(1, aspect)` of the
wider NDC axis, and scaling that axis by `aspect` (which is exactly
what dropping the division does) can never push a fitted scene out of
`[-1, 1]` — at any aspect, from either side. What DOES go red is
`input_mapping::a_pan_keeps_the_point_under_the_cursor` (1600×1000
viewport, moved 219.2 px for a 137 px drag), plus the R2 random-state
pan row. So the projection IS pinned against this break, but by the
pan-as-property row, not by the framing row the PR body credits.

## Constructive probe — silent misfit at pathological aspect
Not a mutant; the shipped code. `Camera::framing(plate, aspect)` for
thin aspects, then projecting the eight corners:

| aspect | distance (max 3.6277) | worst \|ndc x,y\| |
|---|---|---|
| 0.05  | 2.0148 | 0.864 |
| 0.02  | 3.6277 (clamped) | 1.199 |
| 0.01  | 3.6277 (clamped) | 2.397 |
| 0.005 | 3.6277 (clamped) | 4.794 |

Below aspect ≈ 0.023 the framing distance hits the zoom band's
ceiling (`MAX_DISTANCE_FACTOR = 100`) and `Camera::new` clamps it
silently, so the "fit" no longer contains the scene and nothing
refuses. Findings section of `review-report-r2.md` carries the
assessment.
