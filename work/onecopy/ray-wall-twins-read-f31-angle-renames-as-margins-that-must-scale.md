---
id: ray-wall-twins-read-f31-angle-renames-as-margins-that-must-scale
kind: issue
title: ray_wall_margin_twins: the F31 angle renames (period_near_mark, azimuth_near_cut_side) are sign-only reads at Band::least() whose margins do not scale with the model, and the twins' claim reads them as if they must
status: open
opened: 2026-11-06
priority: P3
cost: E
---


Filed by `onecopy/main-reds-k-and-m10`, which re-pinned the row on it.

## What

`crates/sweep/tests/ray_wall_margin_twins.rs`,
`the_wall_arms_margins_scale_linearly_with_the_model`, asserts every
decisive margin `point_in_solid` records on a pipe at a metre is the
millimetre twin's times a thousand. Since PR 6482 (`onecopy/angle-reads`,
merge `a6f0b5e99c`) a curved aim's foot reaches the sink under the two
F31 names (`docs/predicate-dimension-audit.md` row F31), and neither
scales linearly:

- `period_near_mark` (`geom_core::Period::near`): 208 pairs off, e.g.
  `9.990e-1` — the ratio is 1. Its margin is `reference − t`, a
  parameter (an angle) difference, so it does not scale at all.
- `azimuth_near_cut_side` (`geom::Azimuth::near` through
  `stable_azimuth`): 56 pairs off, e.g. `6.811e2` — the ratio is about
  a million, so its margin reads a quantity of length squared.

Both are taken under `k_stats::unlogged("F31", ..)` at `Band::least()`,
a sign-only band: no decision of either depends on the margin's
magnitude, and the twins' verdict streams already match.

## The question

Whether the twins' claim covers a sign-only reading at all. Two ways
out:

1. The claim reads only margins decided against a band with a length
   (skip samples whose `band_zero` is `Band::least()`'s), and says so in
   its doc. Recommended: a sign-only reading has no scale for a lever
   to set, and its outcomes are already compared.
2. Lever the F31 margins to a length (the angle times the direction's
   length for `period_near_mark`; `stable_azimuth`'s cross term over the
   direction's length for `azimuth_near_cut_side`), so they scale.

## Pin

The row is pinned at what main measures, at 1e-9 and 1e-12 (and the
default ε): `["azimuth_near_cut_side: 56 pairs off, e.g. 6.811e2",
"period_near_mark: 208 pairs off, e.g. 9.990e-1"]`. Closing this row
takes the pin.

Reproduce: `CAD_TOLERANCE_EPS=1e-9 cargo test -p sweep --features probe
--test all -- ray_wall_margin_twins::`.
