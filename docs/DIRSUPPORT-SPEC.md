# DIRSUPPORT: the distance arm separates two cells along a direction, not box against box

Unit: `work/clear/distance-test-separates-boxes-not-directions.md`
(P0/M). Program: CLEAR. Branch: `clear/dirsupport`. Review: a single
FULL review with mutation. This is a new certificate test on the
soundness path.

Read the row in full; the PRICE designers' measurements are there.

## What is true when this lands

1. **A non-touching pair is separated by a support along a direction**
   wherever the axis-aligned box test cannot separate it.
   - The test, as the designers proposed: take a unit direction `n`, for
     example the cell's centre normal or the normalised difference of the
     two cells' centres. Enclose `⟨P, n⟩` over each cell by the hull of
     `⟨P_i, n⟩` over that cell's restricted, dehomogenised control points.
     Separate the two cells if the enclosures are disjoint by more than
     the bound.
   - Analytic carriers use their exact support along `n`, or keep the
     box.
   - Weigh the choice of `n` in the PR body. `n` need not be certified,
     since only the support decides. Say why.
   - **Soundness:** the support is a convex-hull bound, valid only with
     every weight certified positive, otherwise the test falls back to
     the box; the restriction must be exact to the cell; and the gap
     decision is a funnelled decide at a named site with an audit row.
   - Reuse `rect_net` (ZEROSIDE) and the JETFRAME home where they apply.
     Merge main as they land.
2. **Rows. Each can go red.**
   - The Klein loop's gap pairs (3v3×6v1, 4v1×5v1) answer within the
     default budget, with pair counts reported. The designers measured
     the box test needing 2^7 cells per axis, against 2^5 for the
     support.
   - **Rotation:** a near-parallel wall pair, rotated 45° about an axis,
     costs the same number of pairs as the unrotated pair, within a
     stated factor.
   - **A pair at a gap smaller than the bound** is still refused or
     certified as before, never separated falsely.
   - **Every self-intersection, pipe, cavity and clearance row** keeps its
     verdict class at both scalars. List the moved receipts.
   - Report the whole-body numbers for `lily_leaf_a` and the Klein loop.
3. **Mutation.** Each of these must red a row:
   - the support reverted to the box;
   - the weight check dropped;
   - the hull over a subset of the control points;
   - the gap compared without the bound.

## Ground

- `crates/topo/src/clearance.rs` (the distance arm, `Bound::pair`, the
  walk) and `crates/geom/src/surfaces/nurbs.rs` (`rect_net`);
- the tour (`klein`, `lily`) and the sweep tests.
- JETFRAME (`clear/jetframe`) is in flight in `nurbs.rs`, and ZEROSIDE
  (#6234) adds `rect_net`. Merge main as they land.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground. Watch the disk.
- CI is the verification of record. Allow 25 min.

## Report

≤100 lines, covering:
- the test and its soundness;
- the choice of `n`;
- the rows and receipts;
- the rotation row;
- the whole-body numbers;
- the mutants;
- the PR number and its CI result.
