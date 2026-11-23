DELTA REPORT, PR 7526 at a79bf304d5 (reviewer lane; merged main = merge base dd8e70198d, whose four `mesh/src` files I put under the head's tests)

## Verdict: APPROVE-WITH-FIXES
No MAJOR. The first review's MAJOR is cured, nothing that meshes on merged main refuses on this head in anything I ran, and only cylinder patches move. The fixes owed are one structural MINOR and five statements that are not true as written. Conditional on hosted CI, which has run nothing on this head.

## Claims
1. **HELD.** The 8 cells and main's 2 mesh on the head, `check_mesh` and the body's geometry holding, at ε 1e-9 and 1e-12 (10 of 10) and at 1e-6 on the 5 that build.
   - The class, 666 cells of my own at ε 1e-9: the 180-cell grid over seven scale/δ rows (r 1e-3 to 100, δ/r 1e-1 to 1e-4), 264 heights × gaps, 150 random three-hole walls (a notch beside a hole on every third, tall walls h 8 to 30, δ to 1e-4). 0 refusals, 0 bad meshes on the head; no head-only refusal against merged main. The correction fired on 35 cells, p = 4 on 34 and p = 2 on one; none needed more. I found no non-converging body.
   - The correction is the spline row's branch with one predicate widened (`refine_lane` in `TrimmedFace::emit`); past the budget it reaches `verdict.refusal`. Typed refusal at budget end seen under the development-metric mutant (`CertificateExceeded`), not on the shipped head, where nothing spends the budget.
   - **The skip: CORRECTED, five cells not four.** At ε 1e-6 the row prints `SKIPPED` for r=1 h=1.33 at gap 1.5 and 1.6 and for all three r=1e-3 cells; all five are the boolean's `Escalated`. It is topo's, so main's too (my first-pass probe died on the same cuts on both trees). So at 1e-6 the row holds 3 correction cells and main's 2. Two leaks: `two_holes` maps a MESHER refusal of its one-hole column probe into the same `Err`, so off the default ε that would be skipped too; and "loud" is stdout of a passing test, which nextest hides unless asked. Re-posing so all ten build at every ε would be the better shape.
2. **HELD.** Frozen head (my first-pass log) against this head, `mesh` at 1e-9: all 8,105 paired cylinder patches identical; the only change is the 8 two-hole refusals gone; torus (2,739) and sphere (238) patches move, which are main's PR 7498 (they are identical between merged main and this head). With the correction off, the grid reads 10 refusals and 242 cells at p = 0, the same cells the correction fires on. Against merged main, `mesh` and `sweep`'s 66 modules at 1e-9: only cylinder patches move (1,005 and 1,122, my probes among them); 0 head-only refusals; `sweep`'s 14 refusal rows the same on both.
3. **HELD, with a caution on what it asserts.** The identity is Euler's and I get the same derivation; it held as an equality on every cylinder face of all 666 cells and both suites' rows. But the equality itself is true of any triangulation whose inserted points are counted off the patch, lattice nodes included: with rows laid again `crossing_rods_keep_no_side_a_neighbour_can_make` stays GREEN. What says "no rows" is `p == 0` and the two-hole row's `p == 4`, not the equality. So in `reach_probes` the identity replaced a cap that could go red with an assertion that cannot, short of a dropped triangle; the cure's cost there is now held by nothing. Unstated edge: a pinch or a vertex met again on one branch is one CDT vertex counted twice in `n`; no body of the rows has one.
4. **Ruling HELD; the table CORRECTED.** With the cylinder's metric swapped for `k = 1/r`: all three rowless rows red; the one-hole column probe itself refuses at 4 of my 7 scale/δ rows; 25 of 132 height cells refuse at δ 1e-2; but all 108 grid cells of the three rows that have a column MESH. So "180 of 180 two-hole cells refuse" is not what I get; a single bound (0.0276) for 180 cells reads like the column probe refusing inside `two_holes`, not 180 walls. The direction stands: the present metric refuses none of 666, the development refuses some. A height that flips a mesh into a refusal with the correction on: I could not find one (264 height cells). The cylinder arm does read `trimmed_metric` → `cell_metric` at `nv = 1` as the docs say.
5. My first-review findings, below. Items not mine: the notched bound, the split-point bound, the mutual import and `TARGET_SHARE` are gone as described; the `13 * n` cap is gone (see claim 3).
6. **HELD.** `d9_mesh_goldens` with the two new bodies green at three ε; on merged main's mesher it and `route_rows` go red. Viewer: head 13/13; merged main's mesher reads `cut_cylinder` 4,520/7,816 and 81,208/84,504 and the tally (451,512, 147,844, 16,596, 8,142), its own pin, against the head's (424,728, 138,712, 11,004, 5,130). Every moved number has a base run and is this change's.
7. **Mostly true; three sites not.** Under Findings.
8. **Two of three true as far as I read; one CORRECTED.** The rods row: 0.999 across 1 at 0.6 refuses `Triangulation` at δ 0.5 to 5e-3 and MESHES at 1e-3; at 0.8 it refuses at 0.5 to 2e-2 and meshes at 5e-3 and 1e-3 (head; first pass had 0.8 the same on main). Its title and body say "at every δ". The cone/spline row: read only. The `powi` row: the test does call `powi`; the bit difference is not re-measured by me.
9. **True but for:** "four of the two-hole cells" (five); "180 of 180" (claim 4); "rows laid again → rows 1, 2 and 3 red" is true only of the lane's site (below); "every job green except the two" on the old head omits that its `lint` job stopped at the gates step, so rustfmt, clippy and rustdoc were SKIPPED there and have never run hosted on this PR.
10. **Mutants, mine.** Correction off: row 3 red, rows 1 and 2 green. Rows laid again in `AnalyticRow::lattice_rows` alone: all three rowless rows GREEN, only `chart_table::tests::the_cone_arms_rows_stand_a_certified_chord_apart` red. Rows laid again at `tessellate_trimmed`'s branch too: rows 1, 2, 3, the goldens and `route_rows` red. Development metric: rows 1, 2, 3 red. Merged main's whole mesher: rows 1, 2, 3 red.
11. **Hosted CI on this head: no job has run.** Run 45175373840 is queued; `merge ref` succeeded; two cache jobs skipped. Locally, debug: `mesh` whole with the slow set 562/562 at ε 1e-9, 1e-6 and 1e-12; `sweep`'s meshing modules 601/602 (`bisector_nodes::the_near_node_poses_at_the_witness_band`, red on merged main's mesher too); the viewer's two files 13/13. Not run by me: fmt, clippy, gates, rustdoc, tools, python, step-import, the tour.
12. Nothing else broken that I found. I read the unit's diff against the merge base whole; of main's own changes only the metric split.

## Findings
- **MINOR, DEMONSTRATED.** `trimmed.rs` `tessellate_trimmed` (the `(columns, Vec::new())` branch) and `chart_table.rs` `AnalyticRow::lattice_rows`: "a cylinder lays no row" now has two homes, and the lane no longer calls the one every doc and the PR name as the fact's home. A mutant at that home reds no lane row.
- **MINOR, DEMONSTRATED.** `rowless_cylinders.rs` `two_holes_across_…` doc, the unit file and the PR: five cells skip at ε 1e-6, not four.
- **MINOR, DEMONSTRATED.** `chart_table.rs` `trimmed_metric` doc and the PR: "180 of 180 two-hole cells refuse" under the development is not reproduced; 108 grid cells mesh there.
- **MINOR, DEMONSTRATED.** `work/tesslane/two-rods-a-hair-apart-…`: "at every δ" is false at fine δ.
- **NOTE, DEMONSTRATED.** `reach_probes.rs` `crossing_rods_…`: its new assertion is green with rows laid.
- **NOTE, BY-INSPECTION.** `common/rowless.rs` `two_holes`: the skip's `Err` also carries a mesher refusal of the column probe.
- **NOTE, BY-INSPECTION.** `trimmed.rs:676`: cites `AnalyticRow::metric`, which PR 7498 renamed to `trimmed_metric`/`cell_metric`.
- **NOTE, BY-INSPECTION.** `work/tesslane/a-cone-and-a-spline-cylinder-…` says "4–223 times"; the table and `lattice_rows`' doc say 1.2 to 223.
- **NOTE, BY-INSPECTION.** The module doc's "one round has cleared every such face measured" and `lattice_rows`' counts are measured claims with no guard beyond row 3's ten cells; marked measured, which is what they owe.

## Fate of my first review's findings
- MAJOR (two-hole walls): fixed, differently from the rows back, and I accept it.
- Tilt bound, holed bound: fixed (gone; prose marked measured, the `cos^-3/2` form stated).
- "14 faces denser": fixed (20, and my runs give 13 + 6 per ε by key, the PR's 14 by row).
- Main's sibling refusals unfiled: the two-hole cells are in the unit file and now mesh; the rods are filed, with the error above.
- `[mesh.sizing]` clearance clause: declined with a reason I accept (it says where a node is laid; no node, nothing to say).
- Stale sentences (`trimmed.rs` emit comment, `curved.rs:1096`, `UNPROVED_HEADROOM`, the inverted metric sentence, `id_repeats_apart`): fixed at each site.
- The unguarded measured claim at `lattice_rows`: fixed (ε coverage stated correctly, 1.2 to 223, the refusal class named with its row).
- The metric as an accident: addressed (stated, argued, measured), table overstated as above.
- `nu` / `pole_side_columns` on the cylinder: fixed, at the cost of the two-homes MINOR.
- The cone/cylinder split by kind: filed, not built; accepted.
- The `13 * n` cap pinning the implementation: replaced, by something that pins nothing there.
- Row 4 as a golden: fixed (moved into `d9_mesh_goldens`).
- `TARGET_SHARE`, the private FNV: fixed (both gone).
- The viewer pin's "fewer rays land": fixed.

## Executed / read only
Executed: 666 probe cells on the head, the same inside merged main's `mesh` run; per-patch digest logs for `mesh` × three ε on the head, `mesh` and `sweep` at 1e-9 on both trees, compared with each other and with my first-pass log of the frozen head; both viewer files on both trees; four mutants (correction off; rows at the chart's site; rows at both sites; development metric); the rods at 0.6 and 0.8 over seven δ; the two-hole row with output at ε 1e-6 and 1e-12. All local edits removed; nothing committed, pushed or posted.
Read only: the brief, the PR body and check runs (job steps of the old head's run), the unit's diff, the three filed rows, the work-file notes. Not read: the PR's thread, any `analysis/` branch, any other review. Not done: `sweep` at ε 1e-6/1e-12, merged main's `mesh` at those ε, the implementer's 900-cell sweep, the "16 rows red" counts, `trimmed.rs` end to end.
