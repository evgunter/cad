# FRUSTUM: a cone face's window ends where the face does

Unit: `work/clear/cone-window-reaches-the-apex-past-a-frustums-face.md`
(P1/M). Program: CLEAR. Branch: `clear/frustum`. Review: a single
FULL review with mutation.

Read the row in full. Its decided shape is below; do not re-open it.

## Context

`window_of` (`crates/topo/src/clearance.rs`) widens a cone face's `v`
range to `min(…, 0)` every time, so the window reaches the apex even
when the face stops short of it. A frustum is a cone face that does
not reach its apex. On a frustum:
- the cells at the apex can never be cleared by `cell_region`;
- the two halves meet in space there;
- so the walk exhausts `Budget(Pairs 65536)`.

Affected bodies:
- the cavity of `sf2b_interval_probe::interval_offset_charts_together_cone_frustum`;
- a frustum operand with top radius 3/64.

The pinned row is
`a_cone_frustums_cavity_exhausts_the_budget_at_its_apex` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`.

## What is true when this lands

1. **The window ends at the face.** When the face does not reach the
   apex, `window_of` ends the cone window at the face boundary's `v`
   range, not at the apex. A face that does reach the apex keeps
   today's window.
   - "Reaches the apex" is decided soundly:
     - structurally from the face's loop (a vertex at the apex, or a
       degenerate edge there); or
     - at a funnelled site with an audit row, on the boundary's
       certified `v` extent.
   - The window must still contain every point of the face, so it is a
     superset of the face's chart extent. Argue this in the PR body.
2. **Both frustums hold at both scalars:** the cavity and the 3/64
   operand. Re-pin the row as Holds, rename it, and give it a receipt
   well inside the budget.
3. **Nothing else moves.**
   - Every other self-intersection and pipe row keeps its verdict
     class at both scalars.
   - List any receipt that moves, with why. A cone face that reaches
     its apex (the cone row, the kiss) must be bit-identical.
   - Clearance rows that read cone windows (the m10_5/m10_6 suites and
     topo's clearance rows) keep their verdicts. List any moved
     receipt.
4. **Mutation.** Make the reach-the-apex decision lax (always "does not
   reach"), and also its reverse. Each must red a row.
   - The lax direction must red a cone that DOES reach its apex. If
     no such row exists, add one.
   - Also mutate the window to end short of the face's extent; that
     too must red a row.

## Ground

- `crates/topo/src/clearance.rs` (`window_of`) and the sweep tests.
- `docs/predicate-dimension-audit.md`, if a site is added.
- Read `docs/prompts/implementer-discipline.md` in full.
- Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record.

## Report

≤80 lines covering:
- the decision and its soundness argument;
- the receipts before and after;
- any rows that moved;
- the mutants;
- the PR number and its CI result.
