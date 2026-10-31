---
id: the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding
kind: issue
title: The torus's canonical composite bounds a carrier on a thin torus (R/r 19 to 520) at 40 to 800 times the carrier's own rounding: a conditioning limit shared by spline carriers
status: open
opened: 2026-10-26
priority: P2
cost: M
---



Filed by PR 5728's implementer lane, from the measurements of fork row 203's
fix (the one limb-2 reader framed in certification arithmetic).

**Finding.** With every carrier framed at the operand before it is
expanded (`crates/geom-brep/src/carrier_net.rs` `framed_carrier_net`),
the rows that still refuse limb 2 against a torus refuse as
`AnalyticRung3Refusal::KernelLimit` with a bound 40 to 800 times the
carrier's own enclosure width there, at ε = 1e-12 (and, for the largest,
at the default ε):

| row | R, r (R/r) | bound b (m) | width w (m) | b / w |
|---|---|---|---|---|
| `mate7a_torus_rest::*`, `germ_circle_torus::*` (lily stem) | 5, 0.06 (83) | 2.2e-12 to 3.2e-12 | 2.6e-14 | 80 to 120 |
| `verbs_tubewall_r2_probes::r2_inner_radius_is_the_callers_own_rounding` | 10, 0.2 (50) | 1.1e-12 | 3.0e-14 | 36 |
| `boolean::section_cert::section_cert_rows::*` | (rows' torus) | 9.6e-12 | 6.2e-14 | 150 |
| `verbs_tubewall_r1_probes::*` (outside the session box) | 1300, 2.5 (520) | 5.4e-9 | 7.0e-12 | 780 |

The width is the net's (`CarrierNet::width`), so the excess is the
composite's. The torus's canonical form
`(|p|² + R² − r²)² − 4R²(x² + y²)` (`crates/geom-core/src/spline/compose.rs:1121`)
cancels terms of size `R⁴` to leave one of size `R²r·d`, and the metre
conversion divides by `(m + r)·((ρ + R)² − r²)`
(`crates/geom-brep/src/pcurve_cache/projected.rs:1571`): the bound is
about `u·R²/r` times the composite's operation count (degree 24 for a
circle's degree-6 net, about 100 roundings) over the part's own
rounding. A spline carrier on a torus reads through the same composite,
so it shares the limit.

**Wanted.** A conditioning that does not grow as `R²/r`: the composite
written about the tube's core (`m − r`, the distance from the circle of
centres) rather than about the torus's centre, or a per-span anchor of
the composite at the span's own core point. Either is a geom-core
composite change; neither is designed.

**Not wanted.** Loosening ε; reading the bound at points.
