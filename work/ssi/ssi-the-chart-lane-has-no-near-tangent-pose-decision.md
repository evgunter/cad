---
id: ssi-the-chart-lane-has-no-near-tangent-pose-decision
kind: issue
title: ssi: the plane x NURBS lane has no decision for a pose within the band of tangent away from any marched state, so it ends in the cell budget
status: open
opened: 2026-10-06
priority: P2
design: true
---


(Filed by the transversality-lever lane, 2026-10-06, from the design
fork on `work/ssi/ssi-transversality-at-a-point-is-spelled-three-ways.md`;
probes on `analysis/design-fork/transversality-lever-b` at 537fd016fc,
`crates/geom-brep/tests/rev3983_probes.rs`.)

## What

The plane × NURBS lane decides transversality at points it visits (the
march's states, the Hermite's ends, refinement's chord midpoint:
`decide_transversality`, `crates/geom-brep/src/ssi/march.rs` ~917) and
over regions it certifies (the boundary strip, limb 3's tube). It has
no decision for a pose of the pair that lies within the band of tangent
where no branch is marched. The ℝ³ lane has one for its cylinder ×
sphere door (`SsiError::PairTangent`, `ssi_cs_tangency`); the chart
lane does not, and the sweep (`crates/geom-brep/src/ssi/exhaust.rs`,
`sweep`, the `SsiError::CellBudget` arm ~856) spends its budget there
instead of refusing.

## Witnesses (ε 1e-9)

- `leverb_near_tangent_parabola`: the wall `z = x²/(2ρ)`, `ρ` 1 mm,
  cut by `z = δ`, `δ = d²/(2ρ)`, so the two lines lie at `x = ±d`.
  At `d` = 12, 16, 24, 40 and 100ε every run ends in the cell budget
  (on main, and under the shape-operator arm). The point decisions
  clear (`sin θ · ρ = d`), and the pose is `δ ≪ ε` from tangent.
- `leverb_near_tangent_cylinder`: the wall `z = x²/2` (radius 1 m at
  the crest), cut at `sin θ` 1e-4, the pose 5e-9 m = 5ε from tangent.
  It also ends in the cell budget.

## What a decision there must read

The gap to the tangent pose itself, not a curvature model of it. The
fork measured two models and both are wrong:

- **A sweep cell test** (`LEVER_GAP` on the probe branch: a cell within
  Kε of the plane whose gradient enclosure holds zero) cannot tell
  enclosure slack from a vanishing gradient. With the cell-cut
  derivative boxes it fires on the witness's flat wall (sin θ 1e-6,
  E 1 m), a transversal crossing the angle decision clears by 100×
  (lever B, round 4).
- **The sagitta margin** `sin θ · min(E, ½ sin θ · ρ)` refuses correct
  tiny features: `ssi_limb3_one_arc::the_fold_answers_its_two_arcs_paired_as_the_locus_pairs_them`
  (the fold at `L = 1.2w`) gets "too close to call" at every ε, and
  on a saddle wall (the near-degenerate hyperbola, `c = −1e-4`) it
  underestimates the gap by about a thousand times (lever A and B,
  round 4).

The decision is a design question: the gap, its enclosure, and its
story and recourse (the fork's reading: `PairTangent`'s, shared by the
two lanes, not transversality's).
