FIX PASS REPORT, message 4 of 4 — PR BODY, part 3 of 3 (continues directly; ends at the ===== line).

## What moved

- **D9 goldens**: `tilted_above` and `tilted_below` at δ 0.1 and 0.01, four of forty digests, the same at ε 10⁻⁶, 10⁻⁹ and 10⁻¹². Each body's two trimmed cylinder faces: at δ 0.1, 1 of 8 nodes not laid, 25 triangles before and after, worst certificate 0.979 cell bounds both; at δ 0.01, 2 of 105 not laid, 209 → 207 triangles, 0.963 both. The other 36 digests, every planar and rectangle-lane body among them, do not move.
- **`reach_probes`**: `a_torus_wall_holding_a_spiric_ring_meshes_on_the_torus_row` is `Meshed` in all six ops at δ 10⁻³ (was `NoArea(Torus, 2)` on three); `a_cone_face_met_on_its_axis_refuses_unresolved_at_fine_delta` is new and pins the nine cells above.
- **`docs/tess-budget-data`** (spline faces only, none moved), the tour's pins (none moved), `sweep` (no row moved): see "Run".

## The item's bodies

- The torus ring (the donut and B2, δ 10⁻³): meshes, every winding resolved (`reach_probes`, above).
- The tilted pair (the box over x ∈ (0.5, 3) against the y-poled ball, z ∈ (0, 2) and (−2, 2), δ 0.04 and 10⁻²): all six ops mesh and the mesh is the body's (`winding_rows::a_node_within_rounding_of_a_loop_vertex_is_not_laid`).
- The whole frustum refuses `UnresolvedWinding` on its plane cap at δ 10⁻⁸, naming a triangle 0.64 of its allowance high, and meshes at 2·10⁻⁸, 10⁻⁶ and 10⁻³; the cut one meshes with its 226 thin triangles (`winding_rows`).

## Mutants

Each of the first five was run on the whole mesh suite in the first pass; the last two on their rows in the fix pass.

| mutant | red rows beyond the head's own |
|---|---|
| clearance 0 | `winding_rows::a_node_within_rounding_of_a_loop_vertex_is_not_laid` and `reach_probes::a_torus_wall_holding_a_spiric_ring_meshes_on_the_torus_row` (both `UnresolvedWinding`), the lane row (then `…_the_join_refuses`, now `trimmed::a_lattice_laid_without_clearance_cuts_a_triangle_the_lane_refuses`), `a_node_is_laid_exactly_where_its_box_meets_no_side`, `a_node_beside_a_crease_is_not_laid_and_a_boundary_point_snaps` |
| clearance applied to ear and refinement points too | `m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta`, `trimmed::an_ear_the_budget_leaves_refuses_typed`, `winding_rows::a_band_narrower_than_its_clearance_meshes_within_its_certificate`, `reach_probes::a_spiric_bounded_torus_wall_meshes_on_the_torus_row`, `reach_probes::a_ringless_cone_face_near_its_apex_meshes_an_edge_of_four` (row names as of the base before PR 6965) |
| the verdict's scale doubled | `validate::a_winding_is_resolved_above_eight_epsilons_of_its_coordinate_scale`, `winding_rows::the_whole_frustum_…` (at 2·10⁻⁸: `UnresolvedWinding`) |
| the verdict's scale halved | the same unit row, and `winding_rows::the_whole_frustum_…` (at 10⁻⁸: `Ok`) |
| the join skipped on a memo hit | `winding_rows::the_whole_frustum_…` alone (the second run returns `Ok`) |
| `emit` reading a bare sign | `trimmed::a_lattice_laid_without_clearance_cuts_a_triangle_the_lane_refuses` (`Ok(238)`) |
| an apex side searched on its other end's ruling only | `trimmed::a_side_from_the_apex_is_searched_at_every_azimuth` |

## By ε

- `winding_rows::the_whole_frustum_…`: the rim is 20ε, so its window is δ 10⁻⁸ at the default and 10⁻⁵ at ε 10⁻¹², refused and meshed either side at both. At ε 10⁻⁶ the window is 10⁻¹¹, past what a row meshes: the refusal cells print and skip, and the body is meshed at 10⁻³ and 10⁻⁵.
- `winding_rows::the_cut_frustum_…`: meshes, and its mesh is the body's, at every ε; the count of 226 is the default row's.
- `reach_probes::a_cone_face_met_on_its_axis_…`: the same nine cells refuse at every ε.
- The other added and changed rows expect the same at every ε, and pass at 10⁻⁶ and 10⁻¹².

## Sweep (§5)

The class: a decision read from the sign or the exact coincidence of rounded coordinates, with nothing saying whether f64 resolves it. Grepped in `crates/mesh/src` and `crates/stl/src`: a product (`dot`, `cross`, area, `shoelace2`, `of_signed_area`, `signum`) compared with zero, and every comparison with `0.0` in the lanes.

- `trimmed::TrimmedFace::emit`, the fold census's `normal·outward > 0.0`: fixed (an unresolved triangle refuses at once; the sign is read of a resolved normal only).
- `trimmed::TrimmedFace::insert_points`, the exact-coincidence merge of a mesher's point: retired to a refusal.
- `trimmed::constrain_boundary` and `chain_crease_lines`, a point exactly on a constraint: retired to a refusal.
- `trimmed::Creases::snap` on a lattice node: gone; on a boundary point: stays, the page says so. `Creases::on`, an exact lookup of a node's coordinate among the creases: gone, replaced by the node's provenance.
- `trimmed::trim_orientation`, `curved` (`flip` from `of_signed_area`) and `planar` (`shoelace2 < 0.0`): a whole loop's orientation from its signed area. Not this unit: `shoelace2-sums-about-the-chart-origin-so-a-thin-face-far-up-a-chart-can-read-the-wrong-orientation` holds it, and a face whose loop is that thin now refuses at the join.
- `walk`'s `mid_az` (an azimuth from a loop's vector area): chooses a 2πk branch, no winding. Not this class.
- `stl::facets`, `len == 0.0`: kept as the writer's own refusal, its doc corrected.
- The curved and planar lanes' own triangles: no check of theirs was added; the join reads them, and on the rectangle lane it now refuses faces that meshed (the first section).

What the pattern cannot match: a sign read through a helper (`PolygonOrientation`, `Band`, spade's predicates), or a comparison whose operand is a renamed local. Second pass at that gap: every comparison with `0.0` in `trimmed`, `curved`, `planar`, `walk`, `cdt`, `tessellate` and `chords`, read one by one (about thirty lines); none decides a winding or a coincidence beyond the hits above. Not searched: `geom_brep`'s and spade's own predicates, which are exact or banded by their own contracts. The sweep is as of the base before PR 6965; the code that PR and later merges of main added was not re-swept.

## Filed

- Evidence added to `work/tesslane/a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md` (the funnel cell under the clearance, before and after PR 6965) and to `a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it.md` (the nine cells).
- New: `work/tesslane/the-winding-verdicts-scale-reads-corner-norms-not-what-an-evaluation-rounds-at.md` (P2): the page's "would not flip if each coordinate moved by its evaluation rounding" against a scale that reads corner norms and drops the anchor; both reviewers' margin measurements and the far-centred case, unmeasured. The scale is not changed here.
- New: `work/tesslane/a-point-placed-inside-a-developed-needle-triangle-can-land-on-its-side.md` (P3): the miss above.

## Run locally (hosted CI is the record)

On the fix-pass head. After the last merge of main (which brought certificate changes into `cert.rs`) the mesh suite (497 of 497), clippy for mesh, stl, the binding and the viewer, the gates, the tracker lint, fmt and the ε rows were run again; the other lines are from the head before that merge:

- `cargo nextest run -p mesh`, whole, default ε: 493 of 493. The added and changed rows (25) pass at ε 10⁻⁶ and 10⁻¹², the D9 table at all three; no digest moved in the fix pass.
- `bash scripts/run-gates.sh`, `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`, `cargo fmt --all --check`, `python3 scripts/work.py lint`: green.
- clippy `--all-features --all-targets -D warnings` for mesh, geom-brep, topo, stl; `-p viewer --features app` (which is what asked for the refusal's payload to be boxed); `-p pncad-py --features python`; `demos/tour`: green.
- mesh budget job (`clippy` and `cargo test -p mesh --features budget`): green, before the payload was boxed. `docs/tess-budget-data` does not move.
- tools job (k-lint, tess-lint, tess-meter: fmt, clippy, tests): green.
- `demos/tour` suite (`ci-tour`, `ci` profile): 97 pass, no pin moved.
- `cargo nextest run -p stl`: 18 pass. `-p step-import`: 329 pass. `-p pncad-py` (Rust rows): 132 pass.
- The Python wheel suite (`maturin build`, `unittest discover`): 959 tests, OK.
- viewer app rows: 1,551 pass; 3 fail for want of a wgpu adapter in the container (`gpu::tests::every_pass_builds_on_a_real_device`, `the_culled_passes_draw_every_face_that_faces_the_eye`, `drive::driven::a_picture_shows_the_window_with_the_part_drawn`).
- `cargo nextest run -p sweep`, whole, slow set included: 3,773 of 3,775; the two reds are `mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads` and `…_at_one_thread`, a mass-properties golden that calls no mesh code, on a branch that differs from main only in mesh, stl and the Python binding: main's, taken in with a merge of main, not re-checked on main itself.
- Not run: wasm32 clippy; the tour's nightly-only rows; the whole suite at ε 10⁻⁶ and 10⁻¹² beyond the rows named above. The clearance table and the first five mutants are the first pass's, taken before PR 6965 and before this fix pass.
=====
