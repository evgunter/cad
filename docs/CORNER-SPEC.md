# CORNER: the injectivity check across a curved corner

Unit: `work/clear/departure-reads-no-curved-corner.md` (P1/H).
Program: CLEAR. Branch: `clear/corner`. Review: DUAL, as a concurrent
pair (H) on a frozen head. This is a soundness arm.

## Context

`topo::injective` proves f : M → ℝ³ injective (`docs/ERROR-DESIGN.md`,
the *Self-intersection scope* clause, ratified on #5866). Its departure
arm certifies a touching pair across a shared edge.

**What CURVEDEP (#5991) built.** `present_curved` presents a cell as the
region on one side of the edge's chart curve. Its rows are in
`crates/sweep/tests/self_intersection_proves_injectivity.rs` and the
`injective` unit rows. The column argument needs the curve to be the
only boundary in the rectangle R. So a cell that holds a vertex where a
curved shared edge meets another edge reaches no presentation, and
`beside_curve` refuses it `Structural`.

**What that blocks.**
- Every open curved arc refuses: the kernel-built oblique cut, the
  boolean tee, and a full-revolve vessel's half-rims.
- SHELL-4 (parked on this row): its vessel fixture's cavity meets disc
  and cylinder along open semicircles.

## What is true when this lands

1. **Departure certifies a touching pair across a convex corner whose
   sides include a curved edge.**
   - The row names one candidate: the region on the face side of the
     curved edge's closed form continued past the vertex, which holds the
     face's part of the cell when every other boundary curve in reach
     lies on its face side, decided at the vertex.
   - Weigh it in the PR body against at least one alternative of your
     own, for example a presentation that is a corner region bounded by
     both edges' chart images.
   - Pick one, and argue its soundness against claim (1) of the clause:
     the touching cells must be injective on their union. Name the
     argument's steps, and say for each which decision enforces it.
   - Each step is a funnelled interval decision at named sites, with
     audit rows. A cell it cannot decide splits, and at the floor it
     refuses typed.
2. **It closes no other gap.**
   - A tangent (wedge-0) corner still refuses `SecondOrder`.
   - The equal-radius tee's node-plane points, where no first-order
     separator is strict, stay refused, and the refusal is typed. That is
     `the-second-order-arm-certifies-a-declared-wedge-0-edge`'s ground
     (CLEARTAIL).
3. **Rows. Each must be able to go red; prove it by mutation.**
   - `a_kernel_built_oblique_cut_refuses_typed_at_its_curved_corners`
     now holds; re-pin and rename it. Both of the CURVEDEP review's
     probes hold too:
     - the `prism_at` rod minus a slab tilted π/5;
     - `extrude(profile::circle)` cut at π/7.
   - SHELL-4's vessel operand, `vessel(1.0, 2.0)` from
     `crates/sweep/tests/common/shell_operands.rs`, holds as a body.
     Its discs meet the cylinder along open half-rims.
   - The boolean tee `crossing_equal_rods`: report what it reads. If it
     holds, re-pin it. If it refuses only at the node plane, pin that
     typed refusal and say why.
   - A folded variant of one curved-corner body reads `Violated`, with
     a witness off the corner.
   - **The lax-guard rows.** CURVEDEP's review blocked on these. For
     every soundness guard the new arm adds, a row goes red when that
     guard is made too lax. List each as `mutant → row`, all executed.
     Where no supported input can reach a guard, state the argument at
     the guard instead. Also add a `Holds` row that pins
     `certified ≥ 1` from the corner arm.
   - Every existing row in `self_intersection_proves_injectivity.rs`
     and every `injective` unit row keeps its receipt bit-identical. A
     moved receipt is a finding: stop and report it.
   - The large-body budget rows stay within budget: helix, thin shell,
     torus.

## Ground

- `crates/topo/src/injective.rs`, `docs/predicate-dimension-audit.md`,
  and the two test files.
- No other CLEAR unit is in flight in `injective.rs`.
- Run `python3 scripts/work.py territory --base origin/main`.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 minutes.

## Report

≤150 lines, covering:
- the presentation chosen and why, with its soundness argument;
- the rows and the `mutant → row` table;
- the budget numbers;
- the PR number and its CI result.
