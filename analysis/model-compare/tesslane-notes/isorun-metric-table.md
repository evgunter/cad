ISO-RUN-DENSITY — item C, the metric table. Measured, debug, the correction ON, fuzz seed pinned, default ε; three trees are one tree with the metric switched. **I keep the present metric: both alternatives lose, and they are one alternative.**

| metric | two-hole cells (900: gap 1.0–3.0 × 4 phases × 3 heights × 5 scale/δ) | mesh suite | sweep's 61 modules |
|---|---|---|---|
| present: `k` = column step over window height, `nv = 1`, rounded to a power of two | **0 refuse**; 8 cells take one correction round of 4 points | 552/552 | 595/596 (bisector_nodes, main's) |
| the cylinder's development, `k = 1/r` (an isometry) | **180 of 180 refuse** `CertificateExceeded`, 0.0276 against δ 10⁻³, i.e. the correction's 6 attempts spent | 536/552, 16 red | 580/596, 16 red |
| main's metric before this unit (rows square in 3-D) | **180 of 180 refuse**, the same bounds to the last digit | 536/552, the SAME 16 red | 580/596, 16 red |

The two alternatives are the same metric: main's aspect is `(Δu/nu)/((Δv)/nv)` with `nv = ⌈Δv/(r·h_u)⌉`, so `(Δv)/nv ≈ r·h_u` and the aspect is `≈ 1/r` — which is the development. Their identical bounds and identical 16-row failure lists are that. Red there (both): d9's two digest rows, m5_pr11's two tilted-wall rows, reach_probes' notched/holed pipe wall, the ring stored a period away, crossing rods, quadric two rods, fitted_refusals, route_rows' k-stream, split_rulings' notched wall, and the four rowless rows.
Why: at `k ≈ 1/r` a column is `1/r` of the window's height in the CDT's own coordinates, so a wall reads as a tall strip and the triangulation prefers triangles spanning several columns — the shape whose span round the axis nothing bounds. At the present `k` a column the height of the window is a square cell, so it prefers rim-to-rim strips one column wide, which is the shape the target bounds. Triangles: not comparable, since the alternatives refuse on the bodies this unit is about; on the bodies they both mesh, no count can be read for them against the present metric's own.
This is now in the code at `chart_table::AnalyticRow::metric` (what the plane is, why, and this measurement), and the inverted doc there is corrected.

**Item H done**: on a cylinder the column count is computed directly for the metric and `pole_side_columns` is not called (no singular point to thin beside, no node to lay); the site says so. d9's goldens still pass, so no byte moved.
**Item I**: I took the doc option. `a_shared_side_the_budget_leaves_refuses_typed`'s doc now says a lane that rebuilt on its last attempt instead of refusing would trip `tessellate_trimmed`'s `unreachable!` ("the last attempt accepts or refuses") on the same cells, not this row's assertion.
Next: F (the PR body) and G (the pre-push on the merged head, with the pick tally re-measured beside a base run on merged main). Then I push the pass whole and send the fix report.
