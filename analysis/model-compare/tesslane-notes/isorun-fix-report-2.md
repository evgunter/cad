ISO-RUN-DENSITY — FIX REPORT, part 2/2: the corrected PR body for 7526. Head a79bf304d5ff11b8c216839fdabefe6e391417dc.

=====
## The chart lane's cylinder arm lays no rows

Unit 4b of the chart lane (`work/tesslane/a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line.md`), re-scoped on its own measurement: **the trimmed lane lays no lattice rows on a cylinder face**, a fact of the chart (`chart_table::AnalyticRow::lattice_rows`) and not of the route. Every chart-lane cylinder face that is not walked is triangulated on its boundary points, each triangle read by its certificate, at the lane's present target (`UNPROVED_HEADROOM`·δ, unchanged). **A triangle the certificate refuses gets a point inside it and the face is triangulated again** — the correction this lane already runs on its spline row, now on the cylinder row too.

The file's first "Done when" (a lattice line through each boundary run, sized at δ) is superseded by that re-scope and by the two rulings recorded in it: the δ target goes to Ev lane-wide, not keyed to a class, and this unit changes no target.

### What changed
- `chart_table::AnalyticRow::lattice_rows`: the cylinder lays no row. `lattice_steps`: its row step is gone (no reader left); the cone keeps its graded rows.
- `trimmed`'s emit pass: the certificate-driven correction runs for a cylinder row as well as a spline row. Same insertion (the triangle's centroid where the CDT runs), same `MAX_GRID_RETRIES`, same typed refusal past it.
- **The plane the triangulation runs in** (`AnalyticRow::trimmed_metric`, PR 7498's `cell_metric` underneath): with no row the lane reads it at `nv = 1`, so `k` is the column step over the window's whole height, rounded to a power of two. Points enter at `(u/k, v)`: a column the height of the window reads as a square cell, so the triangulation prefers strips joining the two rims a column at a time — the shape whose span round the axis the target bounds. Measured against the two alternatives, which are one alternative (the cylinder's development `k = 1/r`, and the metric the lane read when it laid rows, whose rows were square in 3-D so whose `k` is `1/r` too): **both lose** — 180 of 180 two-hole cells refuse `CertificateExceeded` with the correction's budget spent, 16 `mesh` rows and 16 `sweep` rows red. Kept as it is; the doc at the site carries this.
- The cylinder's column count is computed for that metric alone; `pole_side_columns` is not called there (no singular point to thin beside, no node to lay). The #678 `debug_assert` is deleted: it asked about a single interior column beside a repeated id, and with no node that shape cannot arise. `id_repeats_apart` moves to the tests module that reads it.
- The route is main's. Stage 1's route fact (`Trim`/`Lane::IsoBounded`, `walk::runs_on_iso_lines`) was built and taken back out by forward commits once the measurement showed the rule needs no class.
- `[mesh.sizing]`'s clearance clause was read and stands as written: it says where a lattice node **is laid** (the box `BOUNDARY_CLEARANCE` of its own step must meet no side), and a face with no node lays none, so the clause has nothing to say about it and nothing to correct. The clause's reach shrinks; its content does not move.

### Why: the measurement (stage 1)
On notched and holed pipe walls (210 cells) and wedged-ball sphere faces (60 cells), against the rectangle lane's count summed over the iso-rectangles the face divides into (`r_dec`), debug, three ε:

| | cylinder walls | sphere faces |
|---|---|---|
| with rows, at the headroom | 1.2 / 22 / 223 × (min / median / max) | 1.65 / 1.96 / 4.4 × |
| rows off, headroom kept, no correction | 1.00 / 1.40 / 1.50 ×, every cell meshes | (no rows to drop) |

The sphere's gap is the headroom alone, which this unit does not touch. Sizing the chart lane at δ is now its own question, lane-wide, before Ev (`tesslane/certificate-decides`).

### What moves, face by face
A per-patch digest of this mesher against the mesher of the main it merged (`032e8b71c7`), the fuzz seed pinned, default ε:
- **`mesh` suite: 58,353 faces paired; only cylinder faces move, 354 of them** (390 digest entries), from 1,013,168 triangles to 31,686. Every plane, cone, sphere, torus, spindle and NURBS patch is bit-identical, so no rim-sharing neighbour moves: the target is unchanged, so the chords are too. Against the pre-merge base the same count reads 355 faces (391 entries); the one difference is the rod sliver's δ 5·10⁻³ cell, which `a_shared_side_the_budget_leaves_refuses_typed` no longer runs (below).
- **`sweep`'s 61 meshing modules: 32,235 faces paired; 1,361 cylinder faces move**, from 2,189,525 triangles to 172,795. Every other kind is bit-identical.
- **No face newly refuses or folds**, in either suite, at any ε.
- The same `mesh` differential at ε 10⁻⁶ and 10⁻¹² gives the same picture (55,540 and 55,964 faces paired, 391 entries each).

**The unit makes 20 faces denser**, each stated:
- 14 in `mesh`: the crossing rods' saddle faces (`reach_probes::crossing_rods_keep_no_side_a_neighbour_can_make`, 12 faces, up to 2.2× at δ 0.1–0.5) and one face in each rod-sliver row. Measured with an attempt log on both trees: the **shared-side cure** adds the points, and no fold or correction does. With rows, a saddle face's first attempt held 12–16 lattice nodes and cured 4–12 sides; with none it is its `n` boundary points' `n − 2` triangles, nearly every one with a side the face across can make, so the cure splits almost all of them (δ 0.1: `n = 60`, 58 cured then 4; 182 triangles against 82–92). The rule that decides such a side is `the-mesh-cures-a-side-two-faces-can-both-make-three-ways`.
- 6 in `sweep`, 112 → 114 triangles each: four in `bisector_nodes::a_rod_against_a_bore_builds_every_operation`, two in `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself`.

### The two-hole class, and the correction
A review of the frozen head found a wall the suites did not hold: **two small holes at mid-height with a column and a half to 1.6 columns of free wall between them**, centred half a column off a rim chord point, refuse `CertificateExceeded`. With no point between the holes the triangulation joins their facing corners across the gap, a side wider than the target's columns. Measured on the review's construction over 900 cells (gap 1.0–3.0 columns × four phases × three heights × five scale/δ rows): the head refuses 8 and no other cell (δ 10⁻², `h` 1.0 and 1.9; δ 10⁻³, `h` 1.33; the same at `r = 10⁻³`; gap 1.5 and 1.6, phase 0.5; certificates 1.08–1.26 δ). 1,216 cells more near the seam (holes, a notch beside a hole, two notches from opposite rims, three hole sizes) refused nothing.

**The class is main's too, with its rows**: merged main refuses 2 cells of the same probe that this head meshes (`r = 10`, δ 0.1, `h` 1.0 and 1.33, gap 1.5, phase 0.5, bound 0.10823 against 0.1 — the same bound ratio as the head's δ 10⁻² cells), and its other cells take 1.2 to 17 times this head's cylinder triangles (median 3.6). The rows move which cells the class lands in; they do not remove it.

With the correction: **all 900 cells mesh**; each of the 8 takes **one round of four points**, after which its worst certificate reads 0.48–0.49 δ; the other 892 cells, the whole `mesh` suite (58,640 faces) and `sweep`'s 61 modules (32,235 faces) are **bit-identical** to the head without it — the correction fires nowhere else, because nothing else refuses.

### Rows (`crates/mesh/tests/rowless_cylinders.rs`, slow set)
One identity replaces the four ratio bounds the review showed were fits. A constrained triangulation of a polygon with `n` vertices, `h` holes and `p` points inside holds `T = n − 2 + 2h + 2p` (a connected planar graph with `V = n + p`, `n` boundary edges, every inner face a triangle and `h + 1` outer: `E = (3T + n)/2`, and `V − E + (T + 1 + h) = 2`). On this lane `n` is the trim polygon's points — each traversal of an edge its chord points but the last, so a seam the loop walks twice counts twice, as the polygon holds two copies of it — no two-copy triangle is dropped on a cylinder, which has no singular point, and `p` is every point the lane inserted (the shared-side cure's and the correction's, there being no lattice). The derivation and the reading are in the file header and `common::rowless::Count`.

It is asserted as an **equality** on every cylinder face of every body, with `p` read off the patch, which makes one assertion carry three claims: "no rows" is `p = 0`, a split ruling is a boundary point more, and the correction costs exactly two triangles a point.
1. `a_cut_wall_is_its_boundary_points_alone` — the notched, combed and holed walls and the tilted walls (tilt 0.05, 0.3, 0.8, 1.25, 1.35; bars of half-width 0.3, 0.6 and 0.9 of the radius; a comb of six notches on one rim) at δ 10⁻¹, 10⁻² and 10⁻³: `p = 0`.
2. `a_notched_wall_with_split_rulings_is_its_boundary_points_alone` — the same walls with every line edge split at its middle, placed obliquely: `p = 0`.
3. `two_holes_across_a_column_and_a_half_mesh_by_the_correction` — the review's 8 cells (`p = 4`, one round) and main's 2 (`p = 0`).
`reach_probes::crossing_rods_keep_no_side_a_neighbour_can_make` holds the same identity in place of its former `3.25n` cap, with the cure's measured numbers marked measured.

Mutants, each shown: **rows laid again** → rows 1, 2 and 3 red; **the correction off** → row 3 red. A mutant that rebuilt on the last attempt instead of refusing would trip `tessellate_trimmed`'s `unreachable!` ("the last attempt accepts or refuses") rather than a row's assertion, which `a_shared_side_the_budget_leaves_refuses_typed`'s doc now says.

What the counts come to **against the plain wall** of the same pipe is measured and stated in prose, not asserted: a notched wall 0.88–1.06 times the plain wall meshed at the lane's target, a holed one 1.24–1.38, the wider bars 1.48–1.63 and 1.76–1.88, the comb 1.30 at δ 10⁻², a tilt `φ` about `(1 + cos^{−3/2} φ)/2` (1.00 to 5.35 for `φ` 0.05 to 1.35). The count follows the boundary's points, and those follow its length round the axis; `1/cos φ` was a fit and is gone.

Moved pins, each argued:
- `d9_mesh_goldens`: `tilted_above` and `tilted_below` at both δ. **`notched_wall` and `holed_wall` are added to that corpus** (it runs at the three ε), which replaces the private digests and second FNV the old row 4 carried.
- `route_rows`: the "pipe cut on a tilt" bytes.
- `viewer` `display_budget`: `cut_cylinder`'s two rows (largest probe 4,520 → 656 and 81,208 → 2,912; same δ, same stop).
- `viewer` `review_pick_r2`: the tally, re-measured on this tree with a base run on its merged main beside it, which reads its own pin there (so the 6-graze drift the old base carried is main's and is gone): rays 451,512 → 424,728, grazes 147,844 → 138,712, refused candidates 16,596 → 11,004, rays with a refusal 8,142 → 5,130. Fewer rays are AIMED, one per subsampled vertex, because the corpus's cylinder faces have fewer vertices.
- `trimmed::tests::a_shared_side_the_budget_leaves_refuses_typed` reads δ 5·10⁻² and 2·10⁻²; at 5·10⁻³ the rod sliver's first attempt on its boundary points keeps no shared side. The row's subject is the retry budget's typed refusal, which those cells still reach.

### Census of the class the file was written about
A route log over `mesh`, `sweep` and `step-import`: cylinder or sphere faces bounded by iso lines alone that are no rectangle appear in `mesh` only (7 tests), `sweep` 0, `step-import` 0; holed iso walls, `mesh` 22 routings and `sweep` 4. Chart-lane cylinder faces of any kind, which this rule touches: `sweep` 1,368 routings, `mesh` 504. The tour was not censused.

### §5 sweep
Pattern: every caller of `AnalyticRow::lattice_rows` / `lattice_steps` / `lattice_candidates` in `crates/mesh/src`, then `grep -rnI -i cylinder` over `crates/mesh/src`, `crates/mesh/README.md`, `docs/TESS-BUDGET.md` and the two cylinder items in `work/tesslane/`, filtered for `lattice|rows?|grid|lever|strip`. Hits and disposition:
- fixed: `trimmed.rs` (the lane's own rows; the `nu` ingredient comment; `UNPROVED_HEADROOM`'s doc; the fold cure's "drops its lattice corners"; `id_repeats_apart`'s doc), `chart_table.rs` (module header, `lattice_steps`, `lattice_rows`, `cell_metric`, `trimmed_metric`), `chords.rs` (the one rule's "every trimmed arm's but the cylinder's"), `curved.rs` (`grid_counts`' "rows every `lever·hu`");
- dated notes appended: `work/tesslane/the-trimmed-cylinder-lane-has-no-worst-triangle-theorem.md` and `the-trimmed-cylinder-arm-chords-its-boundary-to-no-lattice.md`, each of which describes a lattice that is gone; `work/tesslane/plan.md`'s 4b row;
- left alone as the curved lane's, not this lane's: `docs/TESS-BUDGET.md`'s one-row strips, `curved.rs`' `grid_counts` arms and pole-column prose.

Blind spot the pattern cannot match: a face that is geometrically a cylinder but stored as a spline (a lofted circle's wall) takes the NURBS lane's `per_cell_candidates` and keeps its Hessian-budget rows; so does a cone of half-angle near zero. Filed as `tesslane/a-cone-and-a-spline-cylinder-keep-the-rows-a-cylinder-face-no-longer-lays` (P3), expected to close with the lane-wide correction, with the measurement it owes written down.

### Filed by this unit
- `tesslane/a-cone-and-a-spline-cylinder-keep-the-rows-a-cylinder-face-no-longer-lays` (P3), above.
- `tesslane/two-rods-a-hair-apart-crossing-at-a-shallow-angle-refuse-triangulation` (P3): rods of radius 1 and 0.999 crossing at 0.6 or 0.8, `A ∖ B`, refuse `Triangulation` at every δ **on main and on this head alike**, found by review, not re-measured here.
- `green/issue555s-skewed-prism-reads-its-skew-through-powi` (P4): `issue555_subfloor_cap` builds its prism through `f64::powi`, whose bits differ between builds of the same source (measured: a vertex an ulp apart, each build stable run to run), so that body's mesh differs between trees for an input reason, not a kernel one. This is why 6 keys of that test do not pair in the differential above; the other unpaired keys are this PR's own new rows and the rod sliver's retired δ.

### Verification
All local, debug, on this head merged with main `dd8e70198d`: `cargo fmt --all --check`; `scripts/run-gates.sh`; `python3 scripts/work.py lint`; `cargo check -p mesh --tests`; `cargo clippy --workspace --all-targets --all-features -D warnings`; `mesh` clippy with `--features budget` and without; viewer `--features app`; `pncad-py`; the viewer's two slow rows (13/13, with a base run of them on that same main beside it, which passes its own pins); `sweep`'s 61 meshing modules (595/596 — `bisector_nodes::the_near_node_poses_at_the_witness_band`, red on main too, `work/green/main-red-the-near-node-poses-at-the-witness-band-again.md`); and **`mesh` whole with the default profile at ε 10⁻⁹, 10⁻⁶ and 10⁻¹²: 562/562 each**.

At ε 10⁻⁶ four of the two-hole cells are not built at all: the boolean's pierce decision lands in the run's band (`1.58·10⁻⁶` on the unit pipe, `−1.00·10⁻⁶` on the millimetre one). The row prints `SKIPPED` with the refusal and asserts that the cuts do build at the default ε, so the absence is in the log and cannot pass silently.

No hosted test job has run on any head of this branch yet; `work.py lint` is green. Main's lint job is red on `scripts/payload-rung-sweep.py --check` and has been since PR 7458, tracked on `work/boxfinish/log.md` — not this branch's.
=====
