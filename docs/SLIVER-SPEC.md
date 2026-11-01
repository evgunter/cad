# SLIVER: a knot a few ulps off a bisection point no longer stalls the monotone arm

Unit: `work/clear/injectivity-monotone-arm-reads-a-sliver-knot-span-it-cannot-bound.md`
(P0/M). Program: CLEAR. Branch: `clear/sliver`. Review: a single FULL
review with mutation. Diagnosis first.

Read the row in full.

## What is true when this lands

1. **The mechanism is confirmed by execution.** Run the row's probe:
   `rect_jet` on the 3.5 rad arc-spine wall, at the cell
   `v in [0.375, 0.4375]`. Show the hull the sliver span produces
   against its neighbours, and that it does not shrink with depth. If
   the mechanism is not the row's, stop and report what it is.
2. **The fix is sound, and it is made at the producer.** The row names
   two shapes:
   - (a) `rect_jet` drops a span narrower than the cell's rounding, or
     bounds it by its neighbour. This relies on the jet being continuous
     across a knot of multiplicity at most the degree.
   - (b) the split snaps a bisection point to a knot within a few ulps
     of it.

   Weigh both in the PR body, plus any third shape you find. Pick one
   and argue its soundness:
   - (a) must still enclose the derivative over the whole cell,
     including the sliver span, and must handle a knot of full
     multiplicity, where the jet is discontinuous.
   - (b) must keep the cells a cover and keep depth bounded.

   Every decision is a funnelled decision at a named site with an audit
   row, or is structural. No new ε without a verdict that needs it:
   revert any concession alone and diff the verdicts.
3. **Rows. Each can go red.**
   - The arc-spine loft walls at 3.2, 3.4, 3.5, 3.6 and 6.0 rad (each
     wall against itself) answer within the default budget at both
     scalars, as the clean curls do.
   - The tour's `skinned` `s_duct` walls against themselves hold.
   - A full-multiplicity knot, where the jet really jumps, is still
     enclosed: make a row where dropping the span would be unsound, and
     show that it stays sound.
   - Every existing self-intersection and pipe row keeps its verdict
     class. List any moved receipt.
   - The budget rows stay within budget.
4. **Mutation.**
   - The fix reverted reds the curl rows.
   - The decision made lax (any span dropped, or any knot snapped
     regardless of distance) reds the full-multiplicity row.

## Ground

- `crates/geom/src/surfaces/nurbs.rs` (`rect_jet`, `restricted_jet`),
  `crates/topo/src/injective.rs` (`monotone`, and the split), and the
  sweep tests.
- CARVE's scratch is `carve_embed_scratch_arc_pairs` with
  `CARVE_EMBED_CURL`, in `crates/sweep/tests/carve_embed_scratch.rs` on
  `carve/embedding-certificate`. Read it, but do not merge that branch.
- REFINES (`clear/refines`) is in flight in `clearance.rs` and the same
  test files. Merge main as it lands.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 min.

## Report

≤100 lines, covering:
- the confirmed mechanism, with values;
- the shapes weighed, the one chosen, and its soundness;
- the rows and their receipts;
- the mutants;
- the PR number and its CI result.
