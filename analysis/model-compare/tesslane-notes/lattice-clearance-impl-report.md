IMPLEMENTER tesslane/lattice-clearance — FINAL REPORT (message 1 of 4; the PR body follows in parts 2–4).

HEAD `b6993486b8b6cf916cfb12cb22d402a6cfcfb4a7` (confirmed by `git ls-remote origin tesslane/lattice-clearance`). Main is merged in, PR 6966 and PR 6965 included. Nothing is red in `cargo nextest run -p mesh` (489/489).

1. THE MEASUREMENT
- Clearance table (548 distinct analytic trimmed faces: every one the mesh suite builds plus 42 bodies of mine; in-process A/B, main's lattice against the clearance; taken on the base BEFORE PR 6965): none: worst 1.81 cell bounds; 1/16: 2,123 nodes not laid on 304 faces, 236 change count, worst 1.81, 2 faces newly refuse; 1/8: 4,074 / 387 / 289 / 1.93 / the same 2; 1/4: 7,984 / 400 / 312 / 1.98 / 7 newly refuse.
- Wrong in the spec/log: the log's 1.06 with none (suite faces reach 1.81); "nothing refuses at 1/4" (r = 5 tilted cylinder and a rod cut at 1.1 rad pass the headroom by 2–7%); the whole frustum at δ 3e-9 meshes on this base (log: cone face refuses); the moved-a-metre count is 9,330–9,732 under cert's norm scale, and the log's 6,610 matches a largest-|coordinate| scale (6,183–6,810 here).
- The two frusta otherwise reproduce exactly: cut, δ 1e-4: 226 lattice-cornered triangles at or under ε, least 5.56e-10 m; whole, δ 1e-8: 81,792 triangles, 6,780 unresolved, all in the plane cap; none at 2e-8.
- Spline arm: 0 nodes not laid on 703 faces at 1/16, 1/8, 1/4, 3/8 (2,007 at 1/2); it takes the rule and no spline mesh moves. My sentence in the body on WHY (iso-line boundaries a step from the first line) is a reading, not a measurement.
- The funnel cell (h = 2e-2 above, δ 5e-5) that refused under any clearance meshes once PR 6965 is merged; no cell is absorbed or re-pinned by me. NOT re-taken after that merge: the A/B census (its hooks were local and are out). What was re-run after it is the whole mesh suite, which is green.

2. WHAT CHANGED
- `crates/mesh/src/trimmed.rs`: `BOUNDARY_CLEARANCE` (1/16) read through `boundary_clearance()` by `TrimmedFace::lay` alone; `lay` (free fn), `Node { u, v, step }`, `Side` (chart segment vs rectangle; on the cone the developed segment vs the annular sector), `clip`, `radii_sq`; `lattice_candidates` and `per_cell_candidates` return nodes with their own step (a graded cone row's v-step is the smaller of its two gaps: MY reading of "its own step" on PR 6965's graded rows, flag it); module docs rewritten ("The lattice keeps clear of the boundary", "The retry loop").
- Retired: exact-duplicate drop → `Triangulation` refusal in `insert_points`; node-on-a-side retry → `Triangulation` at once in `constrain_boundary`/`chain_crease_lines` (`split_offender` gone); `Creases::cut_candidates` moves no node. Neither refusal is reached by any body (0 hits over the census under any clearance; 6 and 60 on main's lattice). They are kept as refusals, not removed, because the arm must answer and the page forbids a merge: that is a judgement, say if you want them otherwise.
- `crates/mesh/src/validate.rs`: THE ONE DEFINITION `resolved_normal` (+ `Winding`, `winding`, `first_unresolved`, public `check_winding`, `UnresolvedWinding`). Readers: (1) `tessellate.rs` placement fold, every lane and memo hits → `TessellateError::UnresolvedWinding { face }`; (2) `TrimmedFace::emit` via `validate::winding`; (3) `check_winding`, called by `common::check_mesh_acceptance` and `reach_probes::geometry` (MIN_SHAPE, `Miss`, `Got::NoArea` gone).
- `cert.rs`: `coordinate_scale` split out of `position_scale` (one spelling). `types.rs`: the variant, `Triangulation`'s text, `FacePatch` doc. `stl/src/lib.rs`: doc cites the promise; exact-zero refusal kept as the writer's own. `pncad-py`: tag `unresolved_winding`, roster, census. `nurbs_cert.rs`: one doc phrase.
- Tests: `tests/winding_rows.rs` (new, 4 rows), `validate::tests` (3), `trimmed::tests` (3 new, 3 adapted), `reach_probes` (torus ring Meshed ×6), D9 digests (4 of 40), slow set, `bare-rounding-sites.tsv`.
- Commits (mine, no trailers): 833e079ef3 both rules; 89f3a5eb00 constant to 1/16; ee3cb7578a tracker evidence; 87a5d57d39 digests, merged frustum row, slow set; f9df578f32 gate fixes; merges fa6a74f2a8 (6966's branch), 707d34590b, 834fcdf386, b6993486b8 (main with 6965: conflicts in trimmed.rs, types.rs, reach_probes.rs, nextest.toml, the cone row's file).
- NOT done: the two issue files' status is not flipped to closed (I left closing to you); `work/tesslane/log.md` and the unit file untouched.

4. RUN LOCALLY
- After the last merge: `-p mesh` whole 489/489; added/changed rows at ε 1e-6 and 1e-12 (16/16 each, D9 table included); `bash scripts/run-gates.sh` rc 0; `work.py lint` ok; fmt check clean; clippy mesh+stl and demos/tour clean.
- Before the last merge (head 834fcdf386): `-p sweep` whole 3,762/3,762; tour suite 97/97, no pin moved; tools job green; budget job green but the funnel cell; `-p stl` 18/18; pncad-py Rust rows 132/132; Python wheel suite 959 OK; doc gate OK; clippy geom-brep, topo, viewer app, pncad-py python: clean; viewer app rows 1,551 pass, 3 fail for no wgpu adapter in the container.
- Left to CI: everything above after 6965's merge except the mesh suite; viewer clippy post-merge (the container was killed under it); wasm32; the whole suite at the other ε rows; nightly tour rows.
- Mutants, each run on the full mesh suite: in the body.

5. HEAD: b6993486b8b6cf916cfb12cb22d402a6cfcfb4a7.

6. WHAT SLOWED ME
- The spec's table came from the designers' corpus; the stop was real and cost a round trip (handled well on your side).
- 4 vCPUs, 12–13 GB writable: the sweep suite alone took 2 h wall beside other runs, and the disk filled twice (incremental caches); one container kill at the end.
- A census script that re-invoked cargo picked up my in-progress edits mid-series; I re-took it as an in-process A/B.
- One slip of mine: a row meshed the frustum at δ 2e-11 at ε 1e-6 and hung; bounded.
- Blinding note: the commits' author line is the container's configured identity, the same as the commits already on the branch; no trailer or session line was added.
- I ran two gate scripts bare once while iterating; `run-gates.sh` was run whole before each push after that.

FINAL REPORT, message 2 of 4 — PR BODY, part 1 of 3 (the body runs from the first ===== line here to the last ===== line of message 4).

=====
# The trimmed lattice keeps clear of the boundary, and `tessellate` refuses a face whose triangle f64 cannot wind

Builds the two rules of `crates/mesh/README.md`: `[mesh.winding]` and, in `[mesh.sizing]`, "The lattice keeps clear of the boundary" with `trimmed::BOUNDARY_CLEARANCE` (1/16). Closes `a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area` and `the-stl-writers-doc-names-a-contract-nothing-states`.

Terms. A **trimmed face** is a curved face meshed in its chart: its boundary is the chord points it shares with its neighbours (the **trim polygon**, every loop), its interior vertices are a **lattice** (a rectangular grid of chart points sized from δ), and a constrained Delaunay triangulation joins them. A **cell bound** is the deviation the face's lattice was sized at, `UNPROVED_HEADROOM`·δ = δ/2 on an analytic chart, so a certificate may reach 2 cell bounds before the face refuses.

## The cone face near its apex

Measured on the base before PR 6965, one cell that main meshed refused under any clearance: the funnel cut `h = 2·10⁻²` above its apex, the half above, δ `5·10⁻⁵` (1.31 cell bounds on main; 2.33 at 1/16 with 15 of 15,252 nodes not laid). On PR 6965's certificate and graded rows, merged here, it meshes under the clearance: `reach_probes::the_unit_funnel_cut_near_its_apex_meshes_at_every_delta` passes, as does every other row of the suite. The table below is the earlier base's; the in-process A/B was not re-taken after that merge, and the suite's pins are what was re-run. The cone's rows are graded, so a cone node's own step along `v` is the smaller of its gaps to the lines beside it (`trimmed::lattice_candidates`).

## Measured first

Every trimmed face the `mesh` suite builds, and 42 bodies more (frusta and a funnel cut by a tilted plane, the hollowed vessel and elbow, C1, the donut and B2, the tilted pair, a rod and a turned bar, a tube on a ball, rods cut at tilt 0.5 and 1.1; six δ each), meshed twice in one process: with main's lattice and with the clearance, same polygon and retry loop. 548 distinct analytic trimmed faces with 826,099 lattice nodes; 703 distinct spline faces with 3,611,422.

| clearance | nodes not laid | faces touched | triangle count changes | worst certificate, cell bounds | faces main meshes that refuse |
|---|---|---|---|---|---|
| none | 0 | 0 | 0 | 1.81 | 0 |
| 1/16 | 2,123 | 304 | 236 | 1.81 | 2 (the cell above, two faces) |
| 1/8 | 4,074 | 387 | 289 | 1.93 | 2 (the same) |
| 1/4 | 7,984 | 400 | 312 | 1.98 | 7 |

At 1/4 the five more are plain cylinders cut by a steep plane, each a few percent past the headroom: `m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta` at r = 5, δ 1.5·10⁻³ (1.52 → 2.04), and a rod of radius 0.5 cut through the origin by the plane of normal (sin 1.1, 0, cos 1.1), both halves, at δ 2·10⁻² (1.11 → 2.04) and 5·10⁻⁴ (1.37 → 2.14). A suite-only run at 3/8 and 1/2, each test stopping at its first red, adds refusals on five and six more `reach_probes` rows.

Where this differs from the design fork's table (`work/tesslane/log.md`, 2026-11-14): that table read 1.06 cell bounds with no clearance over the designers' 486 faces and no refusal at 1/4. The suite's own faces reach 1.81 on main (the funnel near its apex), 1.75 (T1's ringed cone), 1.52 (the r = 5 cylinder), 1.30 (the holed pipe).

**The spline arm takes the rule and nothing moves.** On all 703 spline faces no node is within 1/16, 1/8, 1/4 or 3/8 of its own step of a side or a crease line (2,007 are at 1/2). No mesh, digest or budget row of a spline face changes.

**The two frusta** (`revolved[(0,0), (1,0), (r,1), (0,1)]`, r = 20ε), on main:
- Cut by the tilted plane, δ 10⁻⁴: 226 certified triangles with a lattice corner stand at or under ε (least 5.56·10⁻¹⁰ m, 4.4·10³ rounding allowances). As the log has it.
- Whole, δ 10⁻⁸: 81,792 triangles, 6,780 with an unresolved winding, all in the plane cap; none at 2·10⁻⁸, 10⁻⁷ or 10⁻⁶. As the log has it.
- Whole, δ 3·10⁻⁹: main meshes it (149,324 triangles, 22,756 unresolved in the cap). The log says the cone face refuses `CertificateExceeded` there.
- Whole, moved a metre off its axis, δ 10⁻⁸: 9,330 to 9,732 unresolved by direction, and 4,465 to 4,734 at 2·10⁻⁸. The log's 6,610 is what a scale of the largest |coordinate| counts (6,183 to 6,810 here). This PR reads the scale as `cert` already spells a position's (the corner's norm).

FINAL REPORT, message 3 of 4 — PR BODY, part 2 of 3 (continues directly).

## The clearance

`trimmed::lay` decides, once and before the retry loop, which lattice nodes are laid: a node is laid only where the box `BOUNDARY_CLEARANCE` of its own step each way, per chart axis, meets no side of any loop and no crease line. Nothing is merged or moved.

- **The step is the node's own** (`Node::step`): on an analytic chart the lattice's realized column step and its row's gap (the smaller of the two beside it where the rows are graded, the cone's); its band's on a spline chart, where a node on a cut line is laid by each band beside it at that band's steps.
- **Measured against** every side of the trim polygon, rings included (`TrimPolygon`, every loop). A period seam the loop runs along and a singular row are sides like any other: the column beside a seam ruling, or the row beside a pole, is laid when it is a clearance away.
- **In the plane the CDT runs in** (`Side`), because a flat triangle is a node on the line of a side where the triangulation holds that side straight. In the chart that is the chart segment against the node's rectangle. On the cone the CDT runs in the development: the side is the segment between its ends' developed points, which bows from the chart's chord toward the apex, and the box is the image of the chart rectangle, an annular sector. `in_the_development_a_nodes_box_is_its_sector` holds it to the sampled side and shows nodes that tell the two lines apart.
- **Ear and refinement points are not held to it**: each is strictly inside a kept triangle.
- **A face may lay no node.** A rod's wall between two parallel tilted cuts 0.2 down to 0.002 apart, δ 10⁻² to 10⁻⁴, meshes at every width (worst 1.96 cell bounds at 1/4, 0.67 where no node is laid): its triangles stand on boundary points and the ear cure's, and each is certified as any other. No face of the census refuses for having lost its nodes. Row: `winding_rows::a_band_narrower_than_its_clearance_meshes_within_its_certificate`.
- **Nothing but the constant and the goldens depends on the value.** The rows read `BOUNDARY_CLEARANCE` or a multiple of it.

## What retires

Measured over the census: with any clearance on, the exact-duplicate drop fires 0 times and the node-on-a-side retry 0 times (6 and 60 with main's lattice, on the donut and B2 and on `a_ring_stored_a_period_away`).

- **The exact-duplicate drop** (`TrimmedFace::insert_points`): a point of the mesher's entering at an existing vertex is no longer dropped. The arm cannot be deleted (the point has to become a vertex or not), so it refuses `Triangulation`. No body reaches it; `the_development_keeps_apex_copies_one_and_mesh_vertices_apart` reaches it with a hand-placed point.
- **The node-on-a-side retry** (`constrain_boundary`, `chain_crease_lines`): a side or a crease link realised through a point of the mesher's refuses `Triangulation` at once; the drop, the `split_offender` flag and its share of the retry budget are gone. No body reaches it.
- **The crease move** (`Creases::cut_candidates`): no node is moved onto a crease. A node within its clearance of a crease line is not laid, and the crease's own point on each grid line serves. The snap of a boundary point onto a crease stays (`CREASE_MERGE`).
- `TessellateError::Triangulation`'s text now says what raises it.

## The winding

One definition, `mesh::validate::resolved_normal`: a triangle is resolved when its cross product exceeds `cert::rounding_allowance` at `cert::coordinate_scale` (the largest corner norm, the spelling `cert::position_scale` already had) times its longest side. Three readers:

1. **The join**, `tessellate`'s placement fold: every triangle of every face, whichever lane made it and whether the patch is the lane's or the memo's, read against the positions it has in the mesh. A face with an unresolved triangle refuses `TessellateError::UnresolvedWinding { face }` (tag `unresolved_winding` in `pncad-py`).
2. **`TrimmedFace::emit`** (`validate::winding`): outward, folded or unresolved. A fold is cured as before; an unresolved triangle is no fold, and the face refuses at the join.
3. **`validate::check_winding`**, public, beside `check_volume`. `common::check_mesh_acceptance` and `reach_probes::geometry` call it, the latter in place of `MIN_SHAPE`, which is gone with `Got::NoArea`.

No ε is read. `FacePatch`'s doc and the STL writer's cite the promise. The writer keeps its refusal of a cross product of length exactly zero or not finite: it is the one case in which it cannot write a normal, and a hand-built `Mesh` reaches it.

## What moved

- **D9 goldens**: `tilted_above` and `tilted_below` at δ 0.1 and 0.01, four of forty digests, the same at ε 10⁻⁶, 10⁻⁹ and 10⁻¹². Each body's two trimmed cylinder faces: at δ 0.1, 1 of 8 nodes not laid, 25 triangles before and after, worst certificate 0.979 cell bounds both; at δ 0.01, 2 of 105 not laid, 209 → 207 triangles, 0.963 both. The other 36 digests, every planar and rectangle-lane body among them, do not move.
- **`reach_probes`**: `a_torus_wall_holding_a_spiric_ring_meshes_on_the_torus_row` is `Meshed` in all six ops at δ 10⁻³ (was `NoArea(Torus, 2)` on three).
- **`docs/tess-budget-data`** (spline faces only, none moved), the tour's pins (none moved), `sweep` (no row moved): see "Run".

## The item's bodies

- The torus ring (the donut and B2, δ 10⁻³): meshes, every winding resolved (`reach_probes`, above).
- The tilted pair (the box over x ∈ (0.5, 3) against the y-poled ball, z ∈ (0, 2) and (−2, 2), δ 0.04 and 10⁻²): all six ops mesh and the mesh is the body's (`winding_rows::a_node_within_rounding_of_a_loop_vertex_is_not_laid`).
- The whole frustum refuses `UnresolvedWinding` on its plane cap at δ 10⁻⁸ and meshes at 2·10⁻⁸, 10⁻⁶ and 10⁻³; the cut one meshes with its 226 thin triangles (`winding_rows`).

FINAL REPORT, message 4 of 4 — PR BODY, part 3 of 3 (continues directly; ends at the ===== line). Note outside the body: the mutants were run on the head before PR 6965 was merged, and 6965 renamed or replaced some `reach_probes` cone rows; re-check the row names in the second mutant's line against the merged tree before pasting.

## Mutants

Each run on the whole mesh suite.

| mutant | red rows beyond the head's own |
|---|---|
| clearance 0 | `winding_rows::a_node_within_rounding_of_a_loop_vertex_is_not_laid` and `reach_probes::a_torus_wall_holding_a_spiric_ring_meshes_on_the_torus_row` (both `UnresolvedWinding`), `trimmed::a_lattice_laid_without_clearance_cuts_a_triangle_the_join_refuses`, `a_node_is_laid_exactly_where_its_box_meets_no_side`, `a_node_beside_a_crease_is_not_laid_and_a_boundary_point_snaps` |
| clearance applied to ear and refinement points too | `m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta`, `trimmed::an_ear_the_budget_leaves_refuses_typed`, `winding_rows::a_band_narrower_than_its_clearance_meshes_within_its_certificate`, `reach_probes::a_spiric_bounded_torus_wall_meshes_on_the_torus_row`, `reach_probes::a_ringless_cone_face_near_its_apex_meshes_an_edge_of_four` |
| the verdict's scale doubled | `validate::a_winding_is_resolved_above_eight_epsilons_of_its_coordinate_scale`, `winding_rows::the_whole_frustum_…` (at 2·10⁻⁸: `UnresolvedWinding`) |
| the verdict's scale halved | the same unit row, and `winding_rows::the_whole_frustum_…` (at 10⁻⁸: `Ok`) |
| the join skipped on a memo hit | `winding_rows::the_whole_frustum_…` alone (the second run returns `Ok`) |

## By ε

- `winding_rows::the_whole_frustum_…`: the rim is 20ε, so its window is δ 10⁻⁸ at the default and 10⁻⁵ at ε 10⁻¹², refused and meshed either side at both. At ε 10⁻⁶ the window is 10⁻¹¹, past what a row meshes: the refusal cells print and skip, and the body is meshed at 10⁻³ and 10⁻⁵.
- `winding_rows::the_cut_frustum_…`: meshes, and its mesh is the body's, at every ε; the count of 226 is the default row's.
- The other added and changed rows expect the same at every ε, and pass at 10⁻⁶ and 10⁻¹².

## Sweep (§5)

The class: a decision read from the sign or the exact coincidence of rounded coordinates, with nothing saying whether f64 resolves it. Grepped in `crates/mesh/src` and `crates/stl/src`: a product (`dot`, `cross`, area, `shoelace2`, `of_signed_area`, `signum`) compared with zero, and every comparison with `0.0` in the lanes.

- `trimmed::TrimmedFace::emit`, the fold census's `normal·outward > 0.0`: fixed (three answers).
- `trimmed::TrimmedFace::insert_points`, the exact-coincidence merge of a mesher's point: retired to a refusal.
- `trimmed::constrain_boundary` and `chain_crease_lines`, a point exactly on a constraint: retired to a refusal.
- `trimmed::Creases::snap` on a lattice node: gone; on a boundary point: stays, the page says so.
- `trimmed::trim_orientation`, `curved` (`flip` from `of_signed_area`) and `planar` (`shoelace2 < 0.0`): a whole loop's orientation from its signed area. Not this unit: `shoelace2-sums-about-the-chart-origin-so-a-thin-face-far-up-a-chart-can-read-the-wrong-orientation` holds it, and a face whose loop is that thin now refuses at the join.
- `walk`'s `mid_az` (an azimuth from a loop's vector area): chooses a 2πk branch, no winding. Not this class.
- `stl::facets`, `len == 0.0`: kept as the writer's own refusal, its doc corrected.
- The curved and planar lanes' own triangles: no check of theirs was needed, the join reads them.

What the pattern cannot match: a sign read through a helper (`PolygonOrientation`, `Band`, spade's predicates), or a comparison whose operand is a renamed local. Second pass at that gap: every comparison with `0.0` in `trimmed`, `curved`, `planar`, `walk`, `cdt`, `tessellate` and `chords`, read one by one (about thirty lines); none decides a winding or a coincidence beyond the hits above. Not searched: `geom_brep`'s and spade's own predicates, which are exact or banded by their own contracts. The sweep is as of the base before PR 6965; the code that PR added to the lane was not re-swept.

## Filed

- Evidence added to `work/tesslane/a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md` (the funnel cell under the clearance, before and after PR 6965). No new row.

## Run locally (hosted CI is the record)

- `cargo nextest run -p mesh`, whole, default ε, on the head merged with main (PR 6965 in it): 489 of 489 pass. The added and changed rows pass at ε 10⁻⁶ and 10⁻¹², the D9 table at all three.
- `bash scripts/run-gates.sh`, `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`, `cargo fmt --all --check`, `python3 scripts/work.py lint`: green.
- clippy `--all-features --all-targets -D warnings` for mesh, geom-brep, topo, stl; `-p viewer --features app`; `-p pncad-py --features python`; `demos/tour`: green.
- mesh budget job (`clippy` and `cargo test -p mesh --features budget`), before the last merge: green but for the funnel cell, since cured. `docs/tess-budget-data` does not move: the meter records spline faces, and none moved.
- tools job (k-lint, tess-lint, tess-meter: fmt, clippy, tests): green.
- `demos/tour` suite (`ci-tour`, `ci` profile): 97 pass, no pin moved.
- `cargo nextest run -p stl`: 18 pass. `-p pncad-py` (Rust rows): 132 pass.
- viewer app rows: 1,551 pass; 3 fail for want of a wgpu adapter in the container (`gpu::tests::every_pass_builds_on_a_real_device`, `the_culled_passes_draw_every_face_that_faces_the_eye`, `drive::driven::a_picture_shows_the_window_with_the_part_drawn`).
- `cargo nextest run -p sweep`, whole, slow set included: 3,762 pass.
- The Python wheel suite (`maturin build`, `unittest discover`): 959 tests, OK.
- After the last merge of main, re-run: the mesh suite, the gates, fmt, tracker lint, clippy for mesh and stl and `demos/tour`, the ε rows. Not re-run after it: sweep, tour, tools, budget, the Python suite, viewer clippy (the container was restarted under it).
- Not run: wasm32 clippy; the tour's nightly-only rows; the whole suite at ε 10⁻⁶ and 10⁻¹² beyond the rows named above.
=====
