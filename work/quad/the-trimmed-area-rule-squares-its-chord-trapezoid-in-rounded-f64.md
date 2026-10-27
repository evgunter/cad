---
id: the-trimmed-area-rule-squares-its-chord-trapezoid-in-rounded-f64
kind: issue
title: quad: chord_polygon_area's trapezoid-equals-rectangle step reads um, hu, slope and ell(um) in f64, so a first-order rounding term sits outside the mismatch pad
status: open
opened: 2026-10-22
priority: P3
cost: M
---


## Found (QUAD, `quad/integral-composite-midpoint`, 2026-10-22)

Turned up by that lane's `midpoint(` sweep; inferred from the code,
not reproduced.

`chord_polygon_area` (`props/quad/trimmed.rs`) integrates each sub-chord's
column as the rectangle `[p, q] × [v₀, ℓ(u_m)]` and pads the gap to
the trapezoid the chord really bounds by
`mismatch = |ℓ′|·h_u²·width(g)/8`. Its comment says the two have
EXACTLY the same area because `ℓ` is affine. That holds at the exact
midpoint with an exact `ℓ`. The code reads all of these as f64:

- `um = p.midpoint(q)`;
- `slope = (vb - va) / (ub - ua)`;
- `top = ell(um)` (`va + (um - ua) * slope`);
- `hu = q - p` (used in the pad; the column's own factor is the
  interval `pt(q) - pt(p)`).

So the rectangle's height is off the trapezoid's mean height by a
rounding term `|ℓ′|·δ_u + δ_ℓ`. The area is then off by about
`g·h_u·(|ℓ′|·δ_u + δ_ℓ)`, first order in the rounding and not
proportional to `width(g)`, so the `mismatch` pad does not cover it.
The column's cells are also cut on `[v₀, top]` from the f64 `top`.

**Weight.** Relative size about 1e-16 per chord. This rule feeds the
extent gate (a gauge whose widths are `O(h)`), so it is masked
everywhere it has been measured. It is latent unsoundness, not a live
wrong answer.

**Fix shape.** Read `top` as an enclosure (`ell` over `pt(um)`, or
over `cell_midpoint(p, q).1`). Then pad the column by
`hull(g)·width(top)`, or integrate the last cell up to the enclosure's
far end and widen by the near-end gap.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds. `chord_polygon_area`
(`crates/geom-brep/src/props/quad/trimmed.rs:1291`) reads `slope` at
`:1315`, and `hu`, `um` and `top` at `:1326`–`1328`, all in f64. The
`mismatch` pad is at `:1387`.
