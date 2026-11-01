# HEADING: the graph separator bisects the two faces' perpendiculars

Unit: `work/clear/departure-graph-separator-weighs-each-heading-by-its-cells-offset-along-the-edge.md`
(P0/E). Program: CLEAR. Branch: `clear/heading`, cut from
`clear/seamdiag` (SEAMDIAG, PR #6224, which carries the pinned rows).
Review: a single review with mutation.

Read the row in full; SEAMDIAG's diagnosis and measurement are there.

## Decided shape (CLEAR orchestrator)

Normalise each perpendicular before subtracting:
`m = unit(unit(perp(hx, t)) − unit(perp(hy, t)))`. Apply it in
`graph_of`, and in `plane_through`'s one-line-edge case, which has the
same shape. `heading` only chooses the separator; `sign_on` still
decides both signs, so nothing new needs arguing. Say so at the site.

## What is true when this lands

1. **The twisted seams hold.**
   `a_twisted_loft_refuses_departure_along_a_seam_at_both_scalars`
   (SEAMDIAG) flips to a row that holds at both scalars. Rename it, and
   give it a receipt.
   - All four seams of the tour's `twisted_loft` hold. SEAMDIAG
     measured depth 21–24 and about 0.2 s each.
   - Report the tour body's whole-body verdict and time, and
     `twisted_tube`'s.
2. **`plane_through`'s `[d]` case gets its own row**: one that refuses
   before the change and holds after, or an argument at the site for
   why no supported input reaches it.
3. **Nothing else moves.** `s_duct`'s seams are unchanged. Every
   self-intersection, pipe and cavity row keeps its verdict class at
   both scalars; list any receipt that moves.
4. **Mutation.**
   - Reverting either site reds its row.
   - A wrong-sign bisector reds a row, or `sign_on` refuses it typed.
     Say which.

## Ground

- `crates/topo/src/injective.rs` (`graph_of`, `plane_through`,
  `heading`), `demos/tour` (build it from its own directory) and the
  sweep tests.
- SLIVER (`clear/sliver`) and REFINES (`clear/refines`) are in flight
  in `injective.rs` and `clearance.rs`. Merge main as they land.
- Main is red on four blend-wording rows from #6200, filed under CHROME
  as P0. Do not fix them.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.

## Report

≤50 lines, covering:
- the change;
- the rows and receipts;
- the whole-body verdicts;
- the mutants;
- the PR number and its CI result.
