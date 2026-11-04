# Your review: PR 6530 (`splice/spline-region-charge`), a single FULL review

Frozen head: `e2185db6c914b338def8391c11b61fe35ffa4909`. Item: `work/splice/spline-region-decides-off-the-carrier-on-an-uncharged-f64-box-floor.md` (P1). Evidence branch: `splice/review-6530`.

`topo::boolean::spline_region::read_capped` read a point Off its NURBS carrier when every Bézier cell's floor decided past the band. The floor is the distance to the cell's control-point box, cut at f64 (`refine_knots`, de Casteljau, out of homogeneous form), and nothing charged the cut's rounding. Phase 0 on main measured up to 133 false Offs in about 2000 in-band samples at band 1e-12, on rational sheets of size or placement ≥ 3.4e3. The worst was a point at distance 0 read off by four escalations. `Trim::copy` decided "clear" on f64 piece boxes at every scalar, Interval included, because its pieces were collapsed to the value channel.

The PR copies PCERT's `spline_incidence` charge into topo; a live PCERT branch edits that file, so it is not moved. It changes three things:
- each homogeneous control point carries a first-order bound through every blend. Insertion now runs through `refine_plan` + `apply_homogeneous`, not `NurbsCurve3::refine_knots`;
- `Patch::floor` replaces `hull_gap`: the box gap less the cut bound, less a read charge;
- `Trim`'s pieces carry the same bound, and an enclosure's piece is held at the bracket middle and charged its half-width.

The walk is split out verbatim as `carrier_walk`, which `read_capped` calls.

Claims to falsify:
1. **The copied charge is a sound first-order bound.**
   - The PR's fuzz checks single-span patches against a double-double cut. Knotted insertion is said to be "spline_incidence's bound, verbatim" and measured only there. Check the copy really is verbatim, and run a knotted-cut drift check against the topo copy itself: insertions of any multiplicity, wide weights, far placement.
   - Is the move "about the middle of the control box" charged (the `2u·|h|` start)?
   - Is the read charge `rounding_charge(2·(reach + |q − c| + |c|))` enough for the query's own subtraction?
   - Does switching from `NurbsCurve3::refine_knots` to the plan path change any cell (the same knots, the same order)?
2. **The trim's charge and its parity argument.**
   - The PR argues that adjacent pieces share their end points bitwise, so a query clear of every widened box has the same parity against the computed chain and the true one. Check the shared-end claim in code, including at an enclosure's bracket middles and at iso segments.
   - Check that the half-width charge for an Interval piece covers the true piece.
   - Try queries near a piece joint and near a widened box's corner.
3. **One-way.** Both charges only lower a floor or widen a box. Check no path narrows anything, and that a poisoned or non-finite bound refuses rather than reading Off or clear.
4. **No ending moved silently.**
   - The lane ran topo's slow set and 52 spline rows at three ε. It did not run sweep or the tour, which reach `read_capped` through booleans.
   - Run sweep's spline-face tests and the tour's teapot, lily and klein at 1e-6 and 1e-12, and report any ending that moved: an Off that became a refusal or Unread, or a build that became a refusal.
   - Measure the cost on one heavy spline row.
5. **The rows can go red.**
   - Re-run the three named mutants: `spline-region-charge-dropped`, `spline-region-trim-uncharged`, `spline-region-bound-halved`.
   - Try others: drop the read charge only; drop the half-width charge only; widen the box on one channel only.
   - Check each witness is at the production call: `carrier_walk` is exactly what `read_capped` runs.
6. **The sweep's filings** (charttail, hone, isotail, pcert, all P2) are fair. Is any of them reproducibly a false Off today, and so P1?
7. **Inherited reds.** Reproduce `every_door_that_reads_a_box_is_inventoried` on main. Then check whether this PR's charged boxes should themselves be on that inventory.
