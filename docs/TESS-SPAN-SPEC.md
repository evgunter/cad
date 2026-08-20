# TESS-SPAN — per-cell NURBS sizing in the shipped lane (binding spec)

Executes the SPAN half of #320 as corrected by #547's measurement
(recon: cad-work/tess-320-recon.md): whole-patch sup sizing pays
~2.5x tour-wide (3.8x on the #320 poster child leaf_a), and the
per-knot-span-cell bounds ALREADY EXIST — certified and falsified
cell-by-cell — behind the budget feature (`nurbs_cell_bounds`,
crates/mesh/src/nurbs_cert.rs:293). This unit promotes per-cell
sizing into the shipped lane. Pre-logged **M-L / NUMERIC**.
**OUT of scope**: the SPLIT factor (grid_steps' AM-GM decoupling —
waits on the aspect-policy design conversation) and certificate
tightening (its own future unit, TESS-BUDGET.md §4). Side-lane
unit run by the ASM orchestrator on Evan's assignment. Deviations
reported, never absorbed.

## D-1: the promotion

The shipped grid schedule sizes each knot-span cell from that
cell's own certified bound instead of the whole-patch sup:
non-uniform grid lines land on knot-cell boundaries; within a
cell, steps come from the cell's ellipse (the SAME point-selection
rule as today — the AM-GM grouping stays until the split unit
rules otherwise); the per-triangle certificate becomes the
certificate of the cell containing the triangle
(trimmed.rs:388-395). Every cell's grid stays inside its own
certified region — the certificate is untouched; only the
schedule moves.

## D-2: the chord pass

`chords.rs:543-590` shares the whole-patch steps with every
adjacent edge's chord schedule. Implementer's REPORTED choice:
adopt per-cell steps at edges (tighter, more code — the boundary
discipline below binds) or keep whole-patch steps at boundaries
(safe, forfeits span gain there — quantify the forfeit with the
meter). Either way: state the choice, the reasoning, and the
measured consequence.

## D-3: binding invariants

- The two-grid-cells-per-triangle-axis budget survives.
- Half-open cells at knots (C¹ second-derivative jumps at
  measure-zero boundaries — the existing Taylor-remainder
  argument; cite it at the site).
- Grid determinism: row-major ids on the final kept set;
  T-junction and MAX_GRID_RETRIES discipline preserved
  (trimmed.rs:40-76) — the retry ladder must be reasoned through
  for non-uniform lines, not assumed.
- δ split (`delta_s = chordal * 0.5`, tessellate.rs:43) unchanged.
- MESH-PROBEGATE (#558) is in flight in the SAME FILES: re-merge
  duty on every main movement, conflicts resolved consciously.

## D-4: the meter stays sighted

Post-fix, `uniform_cells` (defined as "the shipped grid") remains
true by construction and the span ratio reads ~1.0. Re-derive the
column semantics so the meter still detects future regressions of
BOTH kinds (judgment, reported — e.g. keep emitting the
whole-patch-sup counterfactual as a column). Re-cut the committed
baseline WITHOUT --sizing-only (the full deviation pass), with the
rationale in the commit, per the TESS-BUDGET.md:194-210 ritual.
The tess-lint gate is diff-only and goes green by construction on
a shrink — the re-cut is what locks the gain.

## Acceptance rows

1. leaf_a triangle count at δ=2e-3 drops by ≥3x (the measured
   span share); report the exact number and the meter row.
2. Tour NURBS cells land within ~10% of the baseline's
   `span_cells` column (the promotion captured the measured
   slack, not a lookalike). [The CSV has no `span_cells` column
   since #738 — it was identically `grid_cells`, which is the
   column to read this row against.]
3. Certification: the cell-level falsifier extends to the SHIPPED
   lane — a planted per-cell violation goes red (execute the
   plant, restore).
4. Renders: hosted lanes green; changed render artifacts
   re-blessed THROUGH THE PIPELINE (never hand-picked), per the
   reproducibility contract.
5. Determinism: same document, two fresh processes, identical
   mesh (ids included).
6. The existing T-junction/retry suite stays green; any row that
   had to change is itemized with why.
7. Baseline re-cut lands with the deviation pass and rationale;
   tess-lint gate green.
8. Cold clippy: CI scope + interval + the budget/probe feature
   lanes (the unreachable!-under-probe-features class). k-lint
   fires → report, never silence.

## Standing brief lines

As ASM-4-SPEC's, verbatim (OUTPUT DISCIPLINE; foreground rows;
poll harness-backgrounded output files; kill by recorded PID only;
local-scripts/ tooling; merge-before-open + re-merge on movement +
confirm checks START; invariant comments; commit+push per unit;
PR bodies from lane-private paths, never the shared scratchpad).
