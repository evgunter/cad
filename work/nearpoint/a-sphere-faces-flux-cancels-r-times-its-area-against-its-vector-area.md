---
id: a-sphere-faces-flux-cancels-r-times-its-area-against-its-vector-area
kind: issue
title: A sphere face's closed-form flux cancels R·Area against c·A⃗ by h/R, and its area's level difference cancels near a pole, so a thin cap's volume reads wrong and its certified enclosure straddles zero
status: open
opened: 2026-10-10
priority: P1
cost: M
---


Found by CLEAVE's `a-built-sliver-is-not-a-legal-operand` lane (branch
`cleave/sliver-not-an-operand`), measured on main `9192ca4d3a`, release,
ε = 1e-9.

## What

`geom_brep::props::curved::sphere`
(`crates/geom-brep/src/props/curved.rs:2672-2674`) measures a face as

```text
area = R²·Δu·(hi − lo)
flux = ±R·area + (c − o)·A⃗
```

Both have conditioning defects on a thin cap.

1. **The flux.** For a cap of height `h` the two flux terms are each
   about `R·A`. Their sum, `π h²(3R − h)` taken about a point on the
   cap, is smaller by `h/R`. So the result's error is the terms' rounding
   amplified by `R/h`.
2. **The area.** `hi − lo` is a difference of two latitude sines. Both
   are near 1 when the cap sits at the chart's pole, and the boolean's
   re-chart puts the pole on the centre line by design. The difference
   is about `θ²/2`, so its relative error grows as `u/θ²`.

DESIGN.md Q1 (`docs/DESIGN.md:1872`): "a formula whose error is
amplified by a vanishing quantity (…) is a defect, re-posed or
certified, never covered by K. Interval replay does not rescue it." This
is that class on the closed form both tier 3's check 7 and the role read
sum.

## Witness

The unit ball at the origin against `ball(50)` at `d = 51 − gap` on the
`z` axis, both `y`-poled. Take `a ∩ b` and `b ∩ a` (and `a ∖ b` at
`d = 49 + gap`); the sliver is the lens. The closed form is the
textbook lens, evaluated with mpmath at 40 digits:
`V = π(R + r − d)²(d² + 2dr − 3r² + 2dR + 6rR − 3R²)/(12d)`, R = 50, r = 1.
`A` is the kernel's area, which agrees with `2πr·h₁ + 2πR·h₂` to 1e-8.

| gap | true V (m³) | true V/A | kernel f64 V, a∩b / b∩a | certified V/A (role read) | next op |
|---|---|---|---|---|---|
| 2e-6 | 1.232e-11 | 5.0e-7 (50 Kε) | 5.727e-12 (−54%) / 1.777e-12 (−86%) | [−9.2e-6, 9.7e-6] / [−8.2e-6, 9.5e-6] | refuses |
| 3e-6 | 2.772e-11 | 7.5e-7 | 3.565e-11 (+29%) / 2.994e-11 (+8%) | straddles 0 | refuses |
| 5e-6 | 7.700e-11 | 1.25e-6 | 8.868e-11 (+15%) / 8.269e-11 (+7%) | straddles 0 | refuses |
| 1e-5 | 3.080e-10 | 2.5e-6 | 3.273e-10 (+6%) / 3.509e-10 (+14%) | decides `Outer` | reads |

The "next op" is `union` with a far brick, which refuses
`ShellWitnessExhausted`, first in band at
`bool_point_in_solid_infinity_enclosure`.

**Per face, at gap 2e-6.** This was read by instrumenting
`topo::props::rederive_about`, recentred about the shell's least corner
near `(−2e-3, 0, 1)`.

| face | flux enclosure (m³) | area enclosure (m²) |
|---|---|---|
| unit sphere (each of two) | width 2.1e-15 | width 2.1e-15 |
| `R = 50`, face 3v5 | [−3.58e-10, 3.39e-10] | [6.159973e-6, 6.159986e-6], 2.3e-6 relative |
| `R = 50`, face 4v5 | [−3.59e-10, 3.39e-10] | the same |

**The f64 reading is wrong too.** Check 7 reads its volume with
`volume_pad = 0`, because every face is closed-form, so the result gate
accepts `V/A` = 2.3e-7 and 7.2e-8 as definite off the wrong values. The
sweep suite's volume check (`spheres_crossing_off_every_edge.rs`)
passes only because its slack is `1e-15 × 5.2e5`.

## Not covered by

- **`sphere-rim-level-cosine-cancels-near-a-pole`.** That row is the
  rim-level DECISION meter's `√(1 − s²)`. This row is the measured flux
  and area.
- **FLUX's `sphere-flux-arm-carries-two-closed-forms-for-one-face-kind`.**
  Its Gauss–Bonnet arm keeps the same `R·Area + c·A⃗` shape, so the flux
  cancellation survives that unification.

## Done when

The witness at gap 2–5e-6 measures its volume within a few ulp of the
closed form at the shell's extent. The role read's certified `V/A` must
then decide (true values are 50–125 Kε), so the slivers become legal
operands. A family of caps of height `h ≪ R` (several `R`, poles on and
off the chart's pole) is pinned against `π h²(3R − h)/3`, at every ε
row.

Unblocks CLEAVE's `a-built-sliver-is-not-a-legal-operand`.
