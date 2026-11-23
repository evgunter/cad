ISO-RUN-DENSITY — STAGE 2 REPORT. Head (git ls-remote): 553d4d6310a82bca1a7b1954c1b40f0c5e2c5ead on tesslane/iso-run-density, merged with main 032e8b71c7. Frozen: I build nothing further.

Summary: the unit is "the chart lane's cylinder arm lays no rows" (a fact of the chart in chart_table::AnalyticRow::lattice_rows). The route fact is back out by forward commits. The target is unchanged. The pre-push list is green on the merged head; details in the body.

Things the pair should know:
- The face-by-face digest comparison was taken against the PRE-MERGE base (7096's head de1ed37d38), not the merged main. Main's mesher changed since (7096's last commit among others). I re-ran the pins and the whole mesh suite at three ε on the merged head, not the digest diff.
- Pre-existing, not this unit's: review_pick_r2 was already off its pin on the base, by 6 grazes. sweep bisector_nodes is red on both trees. The tools/k-lint and tess-meter Cargo.lock files are rewritten by a local run, and I did not commit that.
- Blinding: no trailer, model or vendor name in any commit I made (from 35d2eeb0d2 on). My first two commits of this session (the probe commit 5e… and "iso-bounded faces: one route fact…") also carry none.
- Not done: the tour census, by your ruling. The sweep digest diff was run only at the default ε.
Filed: nothing new. The cost the unit adds is pinned against the already-filed shared-side item.

=====
## The chart lane's cylinder arm lays no rows

Unit 4b of the chart lane (`work/tesslane/a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line.md`), as re-scoped on 2026-11-23: the trimmed lane lays no lattice rows on a cylinder face. This is a fact of the chart (`chart_table::AnalyticRow::lattice_rows` answers none on a cylinder), not a fact of the route. Every chart-lane cylinder face that is not walked is triangulated on its boundary points alone, each triangle held by its certificate, at the lane's present target (`UNPROVED_HEADROOM`·δ, unchanged).

### What changed
- `chart_table::AnalyticRow::lattice_rows`: the cylinder lays no row. `lattice_steps`: the cylinder's row step is gone (it had no reader left); the cone keeps its graded rows. Module and `metric` docs say so.
- `trimmed`: the cylinder's #678 `debug_assert` is deleted. It asked about a single interior column beside a repeated id, and with no lattice node that shape cannot arise. `id_repeats_apart` now lives in the tests module that reads it. `UNPROVED_HEADROOM`'s doc names the boundary-point triangulation.
- The route is main's. Stage 1's route fact (`Trim`/`Lane::IsoBounded`, `walk::runs_on_iso_lines`) was built and taken back out by forward commits, once the measurement below showed the rule needs no class.

### Why: stage 1 (measured, debug, three ε)
On notched and holed pipe walls (210 cells) and wedged-ball sphere faces (60 cells), against the rectangle lane's count summed over the iso-rectangles the face divides into:

| | cylinder walls | sphere faces |
|---|---|---|
| today (rows, headroom) | 1.2 / 22 / 223 × (min / median / max) | 1.65 / 1.96 / 4.4 × |
| rows off, headroom kept, no refinement | 1.00 / 1.40 / 1.50 ×, every cell meshes | (no rows to drop) |

The sphere's gap is the headroom alone. Sizing the chart lane at δ is now its own question, lane-wide, before Ev; it is not this unit's.

### What moves (face by face, a per-patch digest log, this unit's mesher against its pre-merge base, PR 7096's head `de1ed37d38`'s mesher; main has since merged)
- `mesh` suite at ε 1e-9, 1e-6 and 1e-12: 58,360 / 55,540 / 55,964 faces compared. **Only cylinder faces move: 391**, from 1,013,583 to 31,999 triangles (median 0.16× per face). Every plane, cone, sphere, torus, spindle and NURBS patch is bit-identical, so no rim-sharing neighbour moves (the target and the chords are unchanged).
- `sweep`'s 61 meshing modules: 31,819 faces compared; 1,361 cylinder faces move, from 2,189,525 to 172,795 triangles (max ratio 1.018); every other kind is bit-identical. 416 keys did not pair between the two runs (their body digests differ run to run), so those faces are not compared.
- **No face newly refuses or folds**, in either suite or at any ε. `sweep`'s one failure (`bisector_nodes::the_near_node_poses_at_the_witness_band`, a boolean refusal) fails on the base too.

**The unit makes 14 faces denser.** The crossing rods' saddle faces (`reach_probes::crossing_rods_keep_no_side_a_neighbour_can_make`: 12 faces, up to 2.2× at δ 0.1–0.5) and one face in each rod-sliver row. Measured with an attempt log on both trees: the shared-side cure adds the points, and no fold or refinement does. With rows, a saddle face's first attempt held 12–16 lattice nodes and cured 4–12 sides. Without rows, its first triangulation is its `n` boundary points' `n − 2` triangles, nearly every one with a side the face across can make (δ 0.1: `n = 60`, 58 of 58 cured, then 4; 182 triangles against 82–92). This is pinned as a stated cost in that row: each cylinder face ≤ 3.25n, measured 3.03–3.14n. The rule that decides such a side is `the-mesh-cures-a-side-two-faces-can-both-make-three-ways`.

### Rows (`crates/mesh/tests/rowless_cylinders.rs`, slow set)
Each is a relation to the plain wall of the same pipe, meshed in the same run on the rectangle lane at δ/2. That is the chart lane's own target, which this unit does not change and which is before Ev lane-wide; `TARGET_SHARE` says so.
1. Notched and holed walls at δ 1e-2 and 1e-3: ≤ 1.1× (holed ≤ 1.4×). Measured 0.875–1.062 and 1.24–1.375.
2. A wall cut on a tilt φ ∈ {0.05, 0.3, 0.8}: ≤ 1/cos φ. Measured 1.000, 1.03–1.04, 1.36–1.375.
3. The same notched walls with every line edge split, placed obliquely: at most one triangle more per split point.
4. Byte digests of a notched, a holed and a tilted wall at δ 1e-2: identical at ε 1e-9, 1e-6 and 1e-12.
Mutants: rows laid again turns 1, 2 and 4 red (18×, 10×, the digest). An ε-reading metric turns 4 red at 1e-9 and leaves it green at 1e-12. Row 3 pins the split relation, so the rows mutant leaves it green by design.

Moved pins, each argued:
- `d9_mesh_goldens`: `tilted_above` and `tilted_below` (the oblique-cut cylinder halves) at both δ.
- `route_rows`: the "pipe cut on a tilt" bytes.
- `viewer` `display_budget`: `cut_cylinder`'s probes at both δ (4,520 → 656 and 81,208 → 2,912 largest-probe triangles; same δ, same stop).
- `viewer` `review_pick_r2`: the tally (424,728, 138,718, 11,004, 5,130). The merge base reads (451,512, **147,850**, 16,596, 8,142), which is already off its own pin (147,856) by 6 grazes, before this change.
- `trimmed::tests::a_shared_side_the_budget_leaves_refuses_typed` reads δ 5e-2 and 2e-2. At 5e-3 the rod sliver's first attempt now keeps no shared side. The row is for the retry budget's typed refusal. The mutant that rebuilds instead of refusing at the last attempt turns it red at 5e-2.

No cone or torus notch exists to measure: no door builds one (the boolean refuses a box through either wall).

### Census of the original class (measured, a route log over `mesh`, `sweep`, `step-import`)
- Cylinder or sphere faces bounded by iso lines alone that are no rectangle: `mesh` only (7 tests), `sweep` 0, `step-import` 0.
- Holed iso walls: `mesh` 22 routings, `sweep` 4.
- Chart-lane cylinder faces of any kind, which this rule touches: `sweep` 1,368 routings, `mesh` 504.
- The tour was not censused.

### §5 sweep
Pattern: every caller of `AnalyticRow::lattice_rows` / `lattice_steps` / `lattice_candidates` (`grep -rn 'lattice_rows(\|lattice_steps(\|lattice_candidates(' crates/mesh/src`):
- `trimmed.rs` (the trimmed lane's analytic arms): fixed, the cylinder lays none; the cone keeps its graded rows, which it needs (a probe that took them off cones saw cones refuse).
- `curved.rs::grid_counts`: not this unit's; the rectangle lane already lays no rows on a ruled chart and reads `lattice_steps` for the torus only.
- `chart_table.rs` tests: the cylinder row now asserts none.

Blind spot: a face that is geometrically a cylinder but stored as a spline (a lofted circle's wall) takes the NURBS lane's `per_cell_candidates`, which this pattern does not match. It still lays its Hessian-budget rows. That is the spline lane's own sizing, not an analytic chart's rows, so it is not this unit's.

### Pre-push (local, debug)
On the head merged with main `032e8b71c7`: fmt; `scripts/run-gates.sh`; `cargo clippy --workspace --all-targets --all-features -D warnings`; viewer `--features app`; the viewer's slow rows (`display_budget`, `review_pick_r2`: 13/13); `mesh` whole with the default profile, 551/551 at ε 1e-9, 1e-6 and 1e-12. Before the merge (on `177f237756`): `mesh` rustdoc with the gate's flags, `pncad-py`, and the three `tools` roots (clippy and tests). `sweep`'s meshing modules ran on this mesher before the merge, 595/596, with the base's own red. Running the tools roots rewrote their committed `Cargo.lock`s (`k-lint`, `tess-meter`); those edits are not committed here.
=====
