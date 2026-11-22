UNIT 4c STAGE-2 REPORT, part 2 of 2.

LOCAL RUNS (CI is the record)
- fmt; run-gates.sh; the doc gate's lints on mesh; clippy mesh (all features, all targets).
- mesh whole, default profile: 523/523 after the merge. Changed rows (split_rulings, ruled_walk, trimmed::, curved::, eps inventory) at 1e-6 and 1e-12: green.
- mesh --features budget: green after the inventory re-pin.
- tools/tess-meter: 36/36. tess_budget_sweep against the baseline with tess-lint: clean.
- stl 18/18; viewer display_budget + review_pick_r2 (--ignore-default-filter): 13/13; demos/tour --cargo-profile ci-tour: 106/106.
- mesh + sweep + step-import in release: same results on base and head except sweep's 6 reds, which are identical on base (main's, release; see below).
Not run: clippy on geom-brep (untouched), the viewer with --features app, demos/tour clippy (no public signature changed), the other two tools roots.

MAIN'S REDS, NOT MINE (identical on base 35570b0552 in release): sweep torus_doors_join::a_rod_across_the_outer_equator_builds_at_its_volume, cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_1_across_the_rim, germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint, germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs, plus the two mass_props_are_thread_count_invariant rows you listed. I have not checked them in debug.

WHAT SLOWED ME
- Release builds of sweep's test binary (~15 min each, twice for the digests).
- The apex's two polygon copies: they made the walk's first shared-side test fire falsely, and the fix led me to drop that test.

=====
## The defect

A cylinder or cone face whose straight side holds a vertex (`walk::splits_a_straight_side`; booleans and `split_edge` make these) is the chart lane's. That lane laid lattice rows such a face does not need and sized it at its unproved headroom. A split therefore multiplied the face's triangles by up to 196× in the suites' bodies, and by 17,600× on a 0.01° cone. It also refused faces the split did not make harder: `DevelopedMerge` where two ruling points fall within the development's 27-bit cut, `FoldedTriangle` beside a split a hair from an end, and `Triangulation` on a planar neighbour narrower than δ, whose rim the routed face chorded at half δ.

## The cure

The chart lane walks such a face (`trimmed::walked`, `crate::ruled_walk`). There is no lattice and no triangulation step: its triangles join its two rows (its rims, or a rim and the apex) in order of angle, and each split side's points are fanned from the corner opposite that side. Each triangle is still read as every triangle of the lane is: its winding resolved and outward, and its certificate within δ. The walk is how the triangles are made; the certificate is what holds them.

The face sizes at δ (`trimmed::face_target`), by a theorem now stated at `sizing::sagitta_step` and `sizing::cone_grid_step`:
- the walk's front edge keeps `max(a_i, b_j) ≤ min(a_{i+1}, b_{j+1})`, so every triangle's corners lie within one chord of one row in angle;
- a triangle on rulings within `Δu` lies in the hull of the surface between the end rulings (on a cone, a scaled hull of one arc, the apex a row of one point), so its deviation is that column's sagitta;
- a side's points lie on its ruling, so the triangles they fan divide one walked triangle in its plane.
The worst reading is a chord of the largest rim against the next row, just under δ by construction (0.998 δ measured), since the rims are chorded at the solved step.

The walk needs no shared-side cure. A walked edge spans at most one rim chord, and two non-adjacent points one neighbour holds both of are the face's whole span apart (`ruled_walk`'s "No edge of four triangles"). A face whose loop is neither shape, such as a cone face through its apex onto the other nappe or one whose apex vertex is undecided, keeps the CDT and its answer.

## What moved

Patch digests, main 35570b0552 against this branch before the 7054 merge, over `mesh`, `sweep` and `step-import` (release, `CAD_FUZZ_SEED=1`):

| class | patches | moved | triangles before → after |
|---|---|---|---|
| walked faces | 121 | 121 | 53,142 → 1,436 |
| their neighbours | 475 | 178 | 2,792 → 2,390 |
| every other face | 80,241 | 0 | — |

- No face that meshed refuses, and every moved patch lost triangles.
- The 178 neighbours are planar caps whose shared rim is chorded again as the unsplit body chords it.
- The moved patches come from four tests: `curved`'s multiply-carried iso side and split-and-placed wedge (40 → 28, re-pinned), `sweep`'s `a_box_along_an_inflection_ruling`, and `reach_probes`' cone met on its axis.
- `tess-lint` against the tess-budget baseline: clean.

## The count relation

`split_rulings::a_split_side_adds_one_triangle_per_point`: eleven bodies, every line edge split at 1, 2 and 5 points and every line edge at once, placed obliquely, δ 0.1 to 0.001, 330 cells at each of the three ε. Each cell meshes, passes `check_mesh`, and is the body's (winding, sampled deviation, volume). Each face has the same face's triangles in the unsplit body plus one per traversal its loops gained (two per point on a one-face wall's seam).

`no_walked_triangle_has_an_edge_a_neighbour_makes`: the tightest bodies, 154 cells, the relation exact:
- a band cut by `mef` so that a one-chord face and a wide face each hold both of the other's sides;
- a D prism;
- a half cylinder and a half cone through the axis;
- a 0.5 rad wedge.

The residual pins are meshing rows:
- `a_ruling_split_a_hair_from_its_end_meshes`;
- `a_ruling_split_within_two_to_the_minus_27_of_its_slant_meshes`: the frustum seam at `10⁻⁹`, the 0.1° draft wall, a 2° cone of height 100 split `3·10⁻⁸` from its rim ends;
- `a_split_beside_a_large_cones_rim_end_meshes` and `a_cone_band_shorter_than_the_developments_resolution_meshes`, which were 7054's refusal pins on bodies the walk now takes.

## Mutants

| mutant | caught by |
|---|---|
| the walk off | 5 rows |
| a triangle spanning two chords | `ruled_walk`'s chord check, plus 5 rows |
| the side fan from the wrong corner | both `ruled_walk` rows, plus 6 |
| the headroom kept on a walked face | 4 rows |

## Sweep (§5)

- **Readers of `walk::splits_a_straight_side`:** `tessellate::lane_of` (routing, unchanged), `trimmed::walked` (new), and a `curved` test helper.
- **Sites that lay a lattice on a ruled face:** `trimmed`'s `lattice_rows` / `lattice_candidates`, reached now only by ruled faces with a ring or a trim carrier (which need interior points), and `chart_table`'s rows.
- **Blind spot:** a ruled face with a ring or trim carrier and a split side keeps the CDT. Measured on T1 at δ 0.02: 300 triangles against 299, so the split costs one triangle there, not a multiple.
- **Second pass at the gap:** the digest diff over three crates found moved patches only on walked faces and their neighbours.
- **Not searchable here:** a both-nappe cone face (no body built here makes one).

## What still refuses

- A cone face with a ring or trim carrier whose run of ruling points is anchored on a needle (`a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind`, open; its pinned bodies are now walked, so it has no pin until a non-rectangle row exists).
- The planar thin-arc refusal beside a CDT face (`a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta`).

## Filed and closed

- Closed: `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`, `a-ruling-split-a-hair-from-its-end-folds-a-triangle`.
- Dated note on the needle item. Nothing new filed.
=====
