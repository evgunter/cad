---
id: a-rigid-map-still-refuses-the-bowed-approx-fixture-at-eps-1e-12
kind: issue
title: at eps = 1e-12 a rigid map still refuses the bowed Approx fixture: the rotated patch's offset fit floors above eps, so the re-fit fallback cannot answer
status: open
opened: 2026-09-28
priority: P3
cost: E
---

## Measured (ENCL rigid-map headroom lane, branch `encl/rigid-map-approx-headroom`, 2026-09-28)

`topo::transform`'s `map_approx` now answers a LIMB refusal of a sound
face's image by re-fitting the mapped description through the mint
door (`crates/topo/src/transform.rs`, `map_approx`). That closes the
near-ε class wherever the moved patch is fittable at ε. At the
`CAD_TOLERANCE_EPS=1e-12` row it is not always fittable.

Subject: `topo::fixtures::bowed_patch` (the shape
`bowed_offset_approx` mints), `d = ±0.05`, fitted by
`geom_brep::offset_fit::fit_offset` at the run's ε = 1e-12: 3 rounds,
64 cells, `hull_sup = 0.365·ε` — far from ε. Re-derived under 93 rigid
maps (four axes × 23 angles plus a translation):

| | d = +0.05 | d = −0.05 |
|---|---|---|
| worst `hull_sup` drift | ×3.10, axis (1,1,1)/√3 at 0.945 rad | ×3.09, same map |
| images refused at ε | 10 / 93 | 10 / 93 |
| of those, fresh fit of the mapped base refuses | 4 (`RefinementStalled`, best 1.047e-12 … 1.079e-12) | 4, same maps |

The four are axis (0.3, −0.4, 0.8) at 2.565, 2.700, 2.835 and 2.970
rad. At the default and 1e-6 rows the same shape drifts by at most
×1.0055 and every re-fit certifies in one round.

## What the drift is made of (one map, one trace)

Per-cell terms of `Composite::cell_bound` on the fit, unrotated vs
axis (1,1,1)/√3 at 0.945 rad: the `dist` term (the hull of
`X = Ẽ·Ẽ − d²·w̃²`, divided by `‖E‖ + |d|`) goes from ≤ 8e-14 to
≥ 2.5e-13 per cell, while `tau` barely moves. `X` is a dot product, so
its Bernstein coefficients are frame-invariant in ℝ; what moves is the
enclosure WIDTH of a cancellation of `d² = 2.5e-3`-sized products down
to ~1e-14, which depends on how the frame splits coordinates into
exactly- and inexactly-representable parts. A control-vector-norm
reading of `Y` and `M̃` in place of the componentwise box
(`offset_meters::norm_sup`) was tried as a scratch A/B and moved the
drift by nothing measurable (×3.0965 vs ×3.0977), so the box norm is
not the cause here.

This is plausibly the same enclosure-width family as
`work/props/f64-refinement-inside-an-enclosure-has-five-more-sites.md`
(Bézier-insertion width) — not verified; the trace above is of `X`'s
formation, not of the insertion.

## What is open

A rigid map of a body that validates at 1e-12 can still refuse,
`ApproxRecertify { source: RefinementStalled }`: the re-fit answers
drift, not a frame in which ε is unreachable. Closing it needs the
certificate's rounding width to stop depending on the frame (forming
`X` without the `d²` cancellation, or recentring per cell), which is
O3's construction — not the transform's.
