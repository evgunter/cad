INTERIM, delta re-review of PR 7526 at a79bf304d5 (reviewer lane). No MAJOR so far. Verdict so far: leaning APPROVE-WITH-FIXES; merged-main runs and the mutants are still to come.

Executed on the head so far (debug, default features):
- My own probes, 666 cells, ε 1e-9: the first review's 180-cell two-hole grid extended to seven scale/δ rows (r 1e-3 to 100, δ/r 1e-1 to 1e-4), a height sweep (33 heights from 0.5 to 4 × four gaps × two δ), and 150 random walls with three holes (every third with a notch beside the last hole, every tenth a tall wall h 8 to 30, δ to 1e-4). **0 refusals, 0 bad meshes, 0 unbuilt; the identity T = n − 2 + 2h + 2p holds as an equality on every cylinder face of all 666.** The correction fired on 35 cells (p = 4 on 34, p = 2 on one); no cell needed more than that.
- `mesh` whole with the slow set at ε 1e-9: 562/562 of its own rows (565 with my three).
- `sweep`'s 66 mesh-touching modules at 1e-9: 601/602, the one red `bisector_nodes::the_near_node_poses_at_the_witness_band` (main's).
- viewer `display_budget` + `review_pick_r2`: 13/13 on the head's pins.

By inspection so far, to be confirmed by run:
- The correction is the spline row's own branch with one predicate widened (`refine_lane` in `TrimmedFace::emit`); past the budget it falls to `verdict.refusal`, which is typed.
- NOTE candidate (style, not a blocker): the "no rows" fact now has two homes. `tessellate_trimmed` hard-codes `(columns, Vec::new())` for a cylinder and no longer calls `AnalyticRow::lattice_rows` there, so a "rows laid again" mutant planted in `lattice_rows` alone would red only `chart_table`'s unit row, not the lane. I will plant it at both sites and say which rows red.
- NOTE candidate: `trimmed.rs:676` cites `AnalyticRow::metric`, which PR 7498 renamed away.
- NOTE candidate: the two-hole row's skip also swallows a MESHER refusal of the one-hole column probe inside `two_holes` at a non-default ε (it maps `mesh::tessellate`'s error into the same `Err` the skip prints).
- The filed rods row says 0.999 across 1 at 0.8 refuses "at every δ"; my first-pass run had it refuse at δ 0.3 and 5e-2 and MESH at 5e-3 and 1e-3 on both trees. I will re-run it on this head with 0.6 beside it.

Hosted CI on this head: nothing has run (`change filter` and `merge ref` queued, two cache jobs skipped).

Next: `mesh` at ε 1e-6 and 1e-12 on the head; merged main's mesher (merge base dd8e70198d's four `mesh/src` files under the head's tests) for the per-patch differential over `mesh` and `sweep` and the two viewer pins; then the mutants (correction off, rows laid again at each site, the metric swapped for the development) and the skip at 1e-6.
