---
id: line-torus-coefficients-lose-a-shallow-crossing-to-their-formation
kind: issue
title: line_torus_roots forms p, q̂ and s with cancellation at a large torus, and the ladder certifies Miss on a shallow in-band crossing the formation lost
status: open
opened: 2026-11-25
priority: P2
cost: M
refs: [ray-torus-count-certifies-on-an-uncharged-discriminant, the-half-angle-ladder-certifies-in-band-configurations]
---



Split out of `ray-torus-count-certifies-on-an-uncharged-discriminant`
(its rows 4–5), which closed rung 1's half: `depressed_quartic_roots`
(`crates/geom-brep/src/quartic.rs`) now decides `Δ` and `D` held to their
own evaluation rounding (`sign_algebra`), so the count it certifies is
the `f64` coefficients' own. What remains is that the coefficients are
not the ray's.

## What

`line_torus_roots` (`crates/geom-brep/src/curve_face/line_quadric.rs`)
forms `m = |u|²`, `M = m + R² − r²`, `p = 2M − 4R²(1 − e²)` and
`s = M² − 4R²(m − n²)` by subtracting terms of size `R²` and `R⁴` whose
difference, near a tangency, is orders smaller. At `R = 1e6` the
formation's rounding moves the quartic's real roots by more than a
shallow crossing's depth, so a ray that crosses the tube by a few
nanometres can have coefficients with no real roots, exactly — and the
ladder certifies `Miss` on them, correctly for those coefficients.

Measured on the inside lane's near-tangent probe (rays aimed `depth`
inside or outside the tube at a random tangent point, at the default
band `(1e-9, 1e-8)`, 200k rays at `(R, r) = (1e6, 1e4)`, after the
rung-1 fix): of rays aimed 1e-9 to 1e-8 m inside the tube, 3.5% are
certified `Miss`; 1e-10 to 1e-9 m inside, 8%. The deepest two of the
200k are aimed 1.05e-8 and 1.21e-8 m inside, at the band's 1e-8 m
threshold, so these are in-band configurations certified (the trilean
violation `the-half-angle-ladder-certifies-in-band-configurations`
describes), not wrong answers outside the band. Before the rung-1 fix,
89 rays aimed past 1e-8 m inside were certified `Miss`, the deepest at
2.4e-7 m. The review of PR 4607 saw the same at
`(3e4, 300)`: 27 rays at depth 3.1e-11 to 3.3e-10 m.

## What was tried

Charging the formation through the same mechanism: `p`, `q̂`, `s`
formed as `geom_core::Rounded` from the ray's inputs, and their bounds
carried into `Δ` and `D` by `Rounded`'s first-order propagation. It
closes the in-band `Miss`es and is far too wide: at `(1e6, 1e4)`, 73% of
rays aimed 1e-7 m inside the tube read uncertain (against 22% with
the evaluation charge alone), because the bound sums `|∂Δ/∂s|·err(s)`
and the other partials, each orders above the cancelled `Δ` they
combine into. So it was not kept.

## What would close it

A recentred formation rather than a wider charge: form the coefficients
about the ray's point nearest the tube's core circle (or about the
tangent point the ray approaches), so that `M` and `m − n²` are read
from small quantities and do not cancel; then charge what is left,
which should be of the order of the crossing's own rounding. The
`R ≤ 1e3` rows show no loss, so the measure of success is the large-`R`
in-band `Miss` share without the uncertain share growing at depth.
