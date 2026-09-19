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
