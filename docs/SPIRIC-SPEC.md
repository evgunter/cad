# SPIRIC: the injectivity check across a spiric edge on a torus

Unit: `work/clear/injectivity-reads-no-spiric-edge-on-a-torus.md`
(P1/H). Program: CLEAR. Branch: `clear/spiric`. Review: DUAL, a
concurrent pair (H), on a frozen head. This is a soundness arm.

## Context

`topo::injective` proves f : M → ℝ³ injective (`docs/ERROR-DESIGN.md`,
the *Self-intersection scope* clause). Its departure arm certifies a
touching pair across a shared edge when one face can present the cell
beside the edge. It reads:
- iso and straight edges;
- closed curved edges (CURVEDEP, #5991);
- convex curved corners (CORNER, #6045).

`reading` admits only the plane and the cylinder. The torus is out until
it is argued.

**The case.** Offsetting the klein elbow's meridian caps along their
normals moves them off the torus axis. Each cap then meets the torus in
a spiric section, a quartic. The plane side reads only lines and
conics, and the torus side reads only harmonic rows. So the pair is
`Structural` on both sides, and it refuses `Departure` at the root, with
0 splits, at both scalars.

It is pinned as `the_klein_elbows_cavity_refuses_at_its_spiric_caps`,
which blocks SHELL-4.

## What is true when this lands

1. **Departure certifies a touching pair across a spiric edge** between
   a plane and a torus.
   - The row names one candidate. Present the torus face beside the
     edge, as the zero set of `ψ = (S − o)·n` in the torus chart, read
     as a graph over one chart axis per cell, split at its turning
     points. The cap's own plane is then the separator, and the plane
     side needs nothing.
   - Weigh that candidate in the PR body against at least one
     alternative of your own. One example: present the PLANE side,
     with the spiric curve as an implicit quartic in the plane's
     chart.
   - Pick one, and argue its soundness against claim (1) of the clause:
     the touching cells are injective on their union. Name each step of
     the argument and the decision that enforces it.
   - Every decision is a funnelled interval decision at a named site,
     with an audit row. Any cell it cannot decide splits, and at the
     floor it refuses typed.
2. **It closes no other gap.**
   - A tangent (wedge-0) spiric contact still refuses `SecondOrder`.
   - Curved corners on a torus stay refused unless argued. The torus
     enters `reading` only for what this unit argues.
3. **Rows. Each can go red; prove it by mutation, PER PROOF STEP.**
   - The table is per step of the soundness argument: for each step,
     the guard that enforces it, and the mutant that makes that guard
     too lax. Each step needs a red row, or an argument at the guard
     for why no supported input reaches it.
   - Two previous arms (CURVEDEP and CORNER) shipped guards that their
     own tables missed. Both reviewers found them by asking what the
     proof's steps need, not what the code checks.
   - The klein elbow's cavity holds at both scalars; re-pin it.
   - A spiric section built directly, a plane cut of a torus off its
     axis, holds at both scalars.
   - A folded variant, where the plane crosses the torus again off the
     shared edge, reads `Violated` with a witness off the edge. If no
     kernel operation can build one, use a unit-level fold at
     `certify_pair`.
   - Every existing self-intersection and pipe row keeps its verdict
     class at both scalars, and every f64 receipt is bit-identical.
   - The large-body budget rows stay within budget.
4. **No new ε without a verdict that needs it.** Revert any concession
   alone and diff the verdicts. Fix at the producer, not the reader,
   where a fix belongs to a producer.

## Ground

- `crates/topo/src/injective.rs`, `crates/topo/src/clearance.rs`,
  `docs/predicate-dimension-audit.md` and the sweep tests.
- FRUSTUM (`clear/frustum`) also touches `clearance.rs`. Merge main
  once it lands.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 min.

## Report

≤150 lines, covering:
- the presentation chosen and why, with its soundness argument;
- the per-step mutant → row table;
- the rows and their receipts;
- the budget numbers;
- the PR number and its CI result.
