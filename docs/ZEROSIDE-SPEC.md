# ZEROSIDE: a spline side's zero against a plane is read off its own control points

Unit: `work/clear/departure-reads-a-spline-sides-zero-to-second-order-so-a-cap-corner-outruns-the-depth-budget.md`
(P0/M). Program: CLEAR. Branch: `clear/zeroside`, cut from
`clear/seamdiag` (SEAMDIAG, PR #6224, which carries the pinned rows).
Review: a single FULL review with mutation. It is a new enclosure on a
soundness path.

Read the row in full; SEAMDIAG's measurements are there.

## The recommended shape (SEAMDIAG)

When the carrier arm reads a linear separator ψ (a plane) on a spline
side, enclose ψ over that side by its value at the side's own iso
control points (the restricted iso net). By the convex-hull property,
that encloses ψ over the iso. When the iso lies in the plane, it reads
zero at any depth, with no new arm and no ε.

Weigh it in the PR body against the alternatives the row names: the
cleartail row's CORNER extension to two curved edges, and raising the
depth (not recommended). Pick one and argue its soundness.
- The enclosure must hold for a RATIONAL net. The convex-hull property
  holds of the projected points only when every weight is positive, so
  check the weights, and refuse otherwise.
- It must cover the side exactly over the cell's span, restricted by
  knot insertion or blossoming, not the whole iso.
- It must still be sound when the iso merely approaches the plane
  rather than lying in it.

Every decision is a funnelled decision at a named site with an audit
row, or is structural. No new ε without a verdict that needs it.

## What is true when this lands

1. **The cap corners hold.** SEAMDIAG's pinned rows flip at the default
   depth 40, at both scalars where the builder allows (the tour rows
   are f64-only):
   - `a_lens_loft_cap_refuses_departure_at_its_tip_at_both_scalars`;
   - the tour's `lily::cap_corner_departure::…`;
   - the tour's `klein::cap_corner_departure::…`.
   Rename each, and give it a receipt. Report the whole-body verdicts of
   `lily_leaf_a`/`b`/`c` and the Klein loop, with times.
2. **The enclosure is sound.** Add rows:
   - an iso that lies in the plane reads zero;
   - an iso that crosses the plane reads mixed;
   - an iso that lies strictly off the plane reads that sign;
   - a negative or zero weight is refused, or falls back.
3. **Nothing else moves.** Every self-intersection, pipe and cavity row
   keeps its verdict class at both scalars; list any receipt that
   moves, with why.
4. **Mutation.** Each must red a row:
   - the fix reverted;
   - the whole iso net instead of the cell's restriction;
   - the weight check dropped;
   - the hull taken over a subset of the control points.

## Ground

- `crates/topo/src/injective.rs` (`sign_on`, `zero_on`, `centred`,
  `jet_on`), `crates/geom/src/surfaces/nurbs.rs` (the restricted net,
  and any public door onto a NURBS iso's control net), `demos/tour` and
  the sweep tests.
- SLIVER (`clear/sliver`) is in flight in `nurbs.rs`'s `rect_jet` and
  in `injective.rs`. HEADING (`clear/heading`) is in `graph_of`. Merge
  main as they land.
- Main is red on four blend-wording rows from #6200, filed under CHROME
  as P0. Do not fix them.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 min.

## Report

≤100 lines, covering:
- the shape chosen, the alternatives, and the soundness argument;
- the rows and receipts;
- the whole-body verdicts;
- the mutants;
- the PR number and its CI result.
