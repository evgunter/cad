# TESS-SPLIT — the aspect-capped split schedule (binding spec)

Executes the SPLIT half of #320 as corrected by #547 (the
dominant ~4.1x factor; diagnosis of record:
cad-work/tess-320-recon.md) under the RATIFIED aspect policy
(TESS-BUDGET.md, PR #568): the schedule takes the cell-minimizing
point on the certified ellipse subject to a first-fundamental-form
3-D aspect cap at **A = 16** (a named spec-time constant with its
reasoning at the definition site). Pre-logged **M / NUMERIC**.
**DISPATCH GATE: TESS-SPAN merged** (same sizing functions —
this unit builds on the per-cell schedule). Deviations reported,
never absorbed.

## D-1: the point selection

`grid_steps`' AM-GM grouping `(muu+muv, mvv+muv)` is replaced by
the constrained minimizer: over the cell's certified ellipse
`muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`, choose (h_u, h_v)
minimizing the cell count subject to the 3-D aspect — the ratio
of the grid cell's edge lengths mapped through the FIRST
FUNDAMENTAL FORM (sampled where the Hessian bound is sampled;
state the sampling choice and its conservatism) — not exceeding
A = 16. Closed form where it exists; a guarded 1-D search over
the ellipse boundary is acceptable if closed form is unwieldy
(deterministic, derivative-free, iteration-capped — REPORT the
choice). Degenerate directions (muu or mvv ≈ 0, the ruled-wall
case that motivated the unit) must be exact, not limits of the
generic formula — decided predicates, never raw comparisons, for
any case split.

## D-2: scope and composition

Applies per-cell (TESS-SPAN's schedule is the substrate). The
chord-pass coupling follows whatever D-2 choice TESS-SPAN landed
— extend it consistently, do not re-litigate it. The certificate
is untouched: every chosen point is inside the SAME ellipse; the
two-grid-cells-per-triangle-axis budget, half-open knot cells,
grid determinism, T-junction/retry ladder, and δ split all bind
exactly as TESS-SPAN's D-3 states them.

## D-3: the meter and baseline

Post-fix the split ratio reads ~1.0 at cap-inactive cells; keep
the meter sighted per TESS-SPAN's D-4 precedent (the
counterfactual columns), and ADD the cap's own visibility: a
per-face indicator (or column) for cap-ACTIVE cells (where A
bound the optimum), so a future A re-tune has data. Baseline
re-cut WITHOUT --sizing-only, rationale in the commit, the
TESS-BUDGET ritual.

## Acceptance rows

1. Ruled/degree-1 walls (the universal case): measured cell
   reduction consistent with the baseline's split column
   (~4x class-wide); exact numbers reported per scene.
2. Aspect honesty: on leaf_a f2 (the 1×4905 poster child) the
   chosen grid's measured 3-D aspect (through the FFF) is ≤ 16;
   a planted A=∞ run reproduces the strip (executed, reported,
   reverted) — the cap demonstrably binds.
3. Certification: the planted-violation falsifier stays red on
   a schedule pushed OUTSIDE the ellipse (execute, restore).
4. Degenerate-direction exactness: a ruled wall's flat direction
   gets its exact arm (test pins it against the generic formula's
   limit — bitwise or with a stated bound).
5. Renders re-blessed through the pipeline; determinism
   two-process row; T-junction suite green (as TESS-SPAN rows
   4-6, repeated at this head).
6. Baseline re-cut + tess-lint green; the cap-active indicator
   populated and sane (spot-check named faces).
7. Cold clippy: CI scope + interval + budget/probe lanes.
   k-lint fires → report, never silence.
8. M9-5 coupling (the lily rebuild): if M9-5's spec-freeze has
   happened by this unit's merge, re-pin against ITS landed
   state per the #569 agreement; else the at-merge entry carries
   the fresh-state pointer for M9-5.

## Standing brief lines

As ASM-4-SPEC's, verbatim (OUTPUT DISCIPLINE; foreground rows;
poll harness-backgrounded output files; kill by recorded PID only;
local-scripts/ tooling; merge-before-open + re-merge on movement +
confirm checks START; invariant comments; commit+push per unit;
PR bodies from lane-private paths, never the shared scratchpad).
