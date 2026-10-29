# CURVEDEP: the injectivity check across a curved shared edge

Unit: `work/clear/departure-presents-no-curved-shared-edge.md` (P1/H).
Program: CLEAR. Branch: `clear/curvedep`. Review: DUAL, a concurrent
pair (H), on a frozen head. This is a soundness arm.

## Context

`topo::injective` (SELFX, #5955) proves f : M → ℝ³ injective. The
reference is `docs/ERROR-DESIGN.md`'s *Self-intersection scope*
clause, which Ev ratified on #5866. Its departure arm certifies a
touching pair across a shared edge. A face must present the edge as
an iso of its chart, or as a line on a plane. Where neither face can,
the arm refuses `Uncertified{Departure}` at depth 0. Examples:
- a cylinder cut by an oblique plane;
- a cylinder–cylinder T;
- a fillet's non-iso rim.

That blocks SHELL-4 on curved cavities.

## What is true when this lands

1. **Departure certifies a touching pair across a curved shared edge.**
   - The row names two candidates: the graph of the edge over a
     projection, or presenting a face by its pcurve-bounded region.
   - Weigh them in the PR body, pick one, and argue its soundness
     against claim (1) of the clause: the touching cells must be
     injective on their union.
   - It is a funnelled interval decision at named sites, with audit
     rows. A pair it cannot decide splits, and at the floor it refuses
     typed.
2. **It closes no other gap.** A tangent (wedge-0) edge still refuses
   `SecondOrder`.
3. **Rows, each of which can go red:**
   - the oblique-cut cylinder holds;
   - a cylinder–cylinder T holds;
   - a fillet against its neighbours holds;
   - a variant of each in which the curved junction folds over itself
     reads `Violated`;
   - every existing row in `self_intersection_proves_injectivity.rs`
     is unchanged;
   - the large-body budget rows stay within budget.

   Re-pin `a_curved_junction_refuses_typed_naming_the_departure_arm`
   to the new behaviour.

## Ground

- `crates/topo/src/injective.rs` and `docs/predicate-dimension-audit.md`.
- The parallel units: LOOPS shares the sweep loop, and CLEARTEXT the
  refusal types. Merge main as they land.
- Run `python3 scripts/work.py territory --base origin/main`.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 min.

## Report

≤150 lines, covering:
- the separator you chose and why, with its soundness argument;
- the rows;
- the budget numbers;
- the PR number and its CI result.
