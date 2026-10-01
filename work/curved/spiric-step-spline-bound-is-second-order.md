---
id: spiric-step-spline-bound-is-second-order
kind: issue
title: The export-only spiric spline's sagitta certificate is second-order, so §5's node cap cannot state the kernel's eps
status: open
opened: 2026-09-18
---


## What

`docs/CURVED-SPIRIC-SPEC.md` §5 certifies the export-only cubic
`B_SPLINE_CURVE_WITH_KNOTS` by the node interval's sagitta:
`sup|D| ≤ h²·(M_s + M_P)/8` with `M_P = sup‖C″‖` in closed form and
`M_s = sup‖spline″‖` from the control-net difference hull, and takes
the least power-of-two node count whose bound is `≤ ε/4`, capped at
1024. That bound is **second order in `h`**; a cubic interpolant's
true error is **fourth order**, so the certificate's looseness grows
as `h²` and the cap is reached long before `ε/4` is.

Measured on the unit's own fixture — the sectioned vessel's cavity
(`R = 0.09375`, `r = 0.0703125`, `|d| = 0.0078125`, span
`1.7822450157733059`, `sup‖C″‖ = 1.3149371083070118e1` from
`geom::spiric_curvature_sup`), at `ε = 1e-9`:

| node intervals | stated bound (m) | densely sampled true sup (m) |
|---|---|---|
| 4 | 3.286408076243776e-1 | 7.398681236336576e-5 |
| 16 | 2.0533108380551935e-2 | 3.050074002546582e-7 |
| 64 | 1.2832897014678046e-3 | 1.191334358837479e-9 |
| 256 | 8.020548395619982e-5 | 4.651487459317821e-12 |
| 1024 | 5.012842262733273e-6 | 1.675804795143954e-14 |

`ε/4 = 2.5e-10`, so the arm refuses
`UnsupportedCurve { kind: "spiric: export tolerance not met" }` at
every CI ε cell (`1e-9`, `1e-6`, `1e-12`) on this body, while the
geometry it refuses to state is already accurate to `1.7e-14` at the
cap. The schedule would need ≈ 2·10⁴ node intervals to STATE `ε/4`
with this bound. PR-1b ships §5 as ratified and
`sweep/tests/spiric_rim.rs`'s row 10 pins both sides: the refusal at
the kernel's ε (which is also what kills the "node cap ignored"
mutant) and the full arm at `uncertainty_m = 1e-4`, where the bound
can be stated.

## Candidates, none of them this unit's

- **A fourth-order certificate.** The classical cubic-spline bound
  `‖f − s‖∞ ≤ (5/384)·h⁴·‖f⁗‖∞` is end-condition specific and
  `geom::NurbsCurve3::interpolate_with_params` builds the A9.1
  averaged-knot global interpolant, so the constant is not citable as
  written; it also needs `sup‖C⁗‖` for the spiric, which nothing
  derives yet. This is the fix that makes the ratified schedule work
  at ~10¹–10² nodes.
- **A schedule-plus-envelope certificate**, the shape every other
  certificate in this kernel uses: a dense sub-sample of `|D|` inside
  each node interval plus the sagitta term at the SUB-sample spacing
  `δ`. Rigorous and available today, but `δ ≈ √(8ε/(M_s + M_P))` puts
  the sample count at ~2.6·10⁵ per edge at `ε = 1e-9` and ~2.6·10⁷ at
  `1e-12`.
- **A sharper `sup‖C″‖`.** `geom::spiric_curvature_sup`'s triangle
  inequality misses an exact cancellation: from
  `f″ = −r·(ρ·cos v/f + r·sin²v·d²/f³)` (the `ρ²/f³ = 1/f + d²/f³`
  substitution) the bound is
  `r·((R − r)/f_min + r·d²/f_min³) + r`, which is `2r` exactly at
  `d = 0` where the current form diverges as `f_min⁻³`. On this
  fixture that is `1.7e-1` against `1.3e1`, a factor 76 — worth
  taking for the mesh chord schedule it also feeds, with the chord
  counts and render baselines it moves. It does NOT rescue §5: the
  cap would still need ~2·10⁴ intervals.

Raising the cap is not a candidate: 2·10⁴ control points for one edge
is not an export.

## Confirmed by both arms of the v6 dual (2026-09-19, PR #2861)

Both reviewers reproduced the table above independently and the item
survives every check they made of it.

- **The bound table, to the ulp.** R1: `3.2864080762437764e-1`,
  `2.053310838055194e-2`, `1.2832897014678048e-3`,
  `8.020548395619983e-5`, `5.012842262733274e-6` at 4/16/64/256/1024,
  and `sup‖C″‖ = 1.3149371083070118e1`. R2 the same to print
  precision, adding that the bound/true ratio is `≥ 4.4e3` at every
  node count — **sound, and second-order-loose exactly as filed**.
- **The sampled true sup at 1024**: R1 `1.8179817278190188e-14`, R2
  `1.82e-14`, against this item's `1.675804795143954e-14` — the same
  number at a different sampling grid's rounding. R1's ratio
  bound/true: `2.76e8`.
- **`M_P` is the whole looseness.** Both measured `M_s ≈ 0.283`
  against `M_P·Δt² = 41.8`.
- **The candidate sharper `sup‖C″‖` is right.** Both re-derived
  `f″ = −r(ρ·cos v/f + r·sin²v·d²/f³)` and the `ρ/√(ρ²−d²)`
  monotonicity; R1 checked it against the shipped form over the whole
  period to `5.6e-17` and confirms it is exactly `2r = 0.140625` at
  `d = 0` where the shipped form gives `11.109`. Value
  `1.7285679395136944e-1` vs `1.3149371083070118e1` = **76.07×**,
  against a sampled `sup‖C″‖` of `7.46e-2` (so still a valid 2.3×
  over-estimate). **It does not rescue the gate**: R2 computed the
  1024 bound under it as `9.9e-8`, still ≫ `2.5e-10`.
- **What it would move.** `curvature_step = √(8δ/M)`, so the step
  grows `√76 ≈ 8.7×` and `MAX_ANGULAR_STEP = π/4` does not clip it
  (R1: 0.215 rad at δ = 1e-3). Whether any chord count or render cell
  actually moves is **unmeasured by either arm**: no spiric-bounded
  face tessellates at this head, so the consequence is TESS's ground
  by announcement rather than a measured delta.
- **The cap's raising direction is cheap to read after all**, which
  this item's first write-up got wrong: one extra doubling suffices.
  bound(2048) = `1.2532105596091274e-6`, so any `uncertainty_m` whose
  `ε/4` lands in `[5.013e-6, 2.005e-5)` is refused at 1024 and met at
  2048 — R1 executed `8e-6` and `6e-6` refusing and `2.1e-5`
  succeeding; R2 the same at `1.9e-5`. PR-1b's
  `the_node_cap_refuses_one_doubling_short_of_the_tolerance` is the
  row, and the "≈2·10⁴ intervals" pricing is withdrawn.

Orchestrator's ruling at adjudication: the arm ships as ratified, this
item stays open on CURVED, the sharper bound is its own small unit
(it moves `spiric_step`, and TESS's ground by consequence), and a
fourth-order certificate for the `ε/4` gate is an `[ev]` question only
if the export is ever wanted at the kernel's own ε.
