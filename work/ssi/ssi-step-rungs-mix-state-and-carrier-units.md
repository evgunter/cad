---
id: ssi-step-rungs-mix-state-and-carrier-units
kind: issue
title: ssi: the march's step rungs read the ℝ⁴ state curve's curvature, not the carrier's, so a wall whose pcurve bends oversamples
status: closed
opened: 2026-10-03
priority: P1
cost: M
closed: 2026-10-03
pr: 3968
branch: ssi/step-units
---


Split out of `plane-nurbs-ssi-does-not-certify-a-curved-dome` (its
cause 3).

## What

`march` (`crates/geom-brep/src/ssi/march.rs`, the realized arm of the
step) priced every step rung off the ℝ⁴ state curve:

- **`h_fit`** used `κ3d = ‖d₂‖/speed²`. `‖d₂‖` is the state curve's
  curvature, which holds the wall pcurve's bending `(u₂, v₂)''` in
  parameter units, although only the plane chart generates the 3-D
  carrier. On the dome's level loop it overstated the carrier's
  curvature by exactly √2, so the loop took 2^(3/8) ≈ 1.30 times the
  samples it needs: 1335 instead of about 1030 at ε 1e-9, over
  `SSI_MAX_FIT_SAMPLES = 1200`.
- **`h_quad` and `h_cub`** were relative bounds in state units, turned
  into metres only by `h·speed`, so they read `2ρ·speed/κ_state`
  where the 3-D heuristic wants `2ρ/κ_3D`: too permissive by up to
  `1/speed`.
- The module docs described the rungs as `√(2δ/κ)` and `∛(6δ/‖d₃‖)`
  in ε, which the code had not done for some time.

## Fix

`LocalSystem::carrier_jet` (`crates/geom-brep/src/ssi/system.rs`)
returns the carrier's `[C′, C″, C‴]` along the approximant, read
through the chart `point` reads (`J_A·d₂_A + D²P_A(d₁_A, d₁_A)`, and
the matching third derivative). Every rung is stated on it in metres:
`κ = ‖C″⊥‖/speed²` for the fit rung, `H ≤ 2ρ/κ` and
`H ≤ √(6ρ·speed³/‖C‴‖)` for the relative ones. On the ℝ³ lane the
state is the point, so the rungs are the same numbers.

## Closed

PR 3968. The ℝ³ lane is bit-identical (38,802 steps' `h` bits). On the
ℝ⁴ lane the dome's level loop takes 1030 samples at ε 1e-9 and
certifies (was `FitSampleBudget` 1335); at 1e-12 it takes 5787 (was
7505). Two cuts the old over-count had held certifying now escalate
limb 2, recorded under cause 4 of the umbrella. The closure pair's
sibling units defect is `ssi-closure-pair-reads-both-charts-as-metres`.
