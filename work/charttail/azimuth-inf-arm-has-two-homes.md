---
id: azimuth-inf-arm-has-two-homes
kind: issue
title: The analytic charts' azimuth inf arm is spelled twice: chart_region's certified_arms and pcurve_cache's azimuth_lever_inf
status: open
opened: 2026-10-25
priority: P3
cost: E
---


Found building unit 2 of `period-headroom-margin-has-no-shared-home`
(branch `pred/period-turn-sites`).

The analytic charts' inf on the azimuth stretch over a window is now
spelled in two places:

- `topo/src/chart_region.rs` `certified_arms`: sphere `r·cos(max|v|)`
  (refused past `π/2`), torus `R − r`, cone `v_inf·sin α`, each gated
  by `chart_region_arm_inf`;
- `geom-brep/src/pcurve_cache.rs` `azimuth_lever_inf` (added by that
  branch for `pcurve_cache/projected.rs`'s check 2, which reads its
  sweep against one turn at `Bounded(inf, sup)`): the same three forms,
  clamped at zero, with the sphere's `|v|` capped at `π/2`.

`geom-brep` cannot reach `topo`, so the home is `azimuth_lever_inf`,
beside its sup twin `azimuth_lever`. `certified_arms` would read its
analytic arms there and keep its own gate and refusals. Its spline arm
is its own and stays.

**Not window-tight on the torus** (PR 5710's review, NOTE-2):
`azimuth_lever_inf` reads `R − r` for every torus window, the global
inf of `R + r·cos v`. A window away from the inner equator has a larger
inf (the least of `R + r·cos v` over its `v` range, or `R − r` where the
range reaches `v = π`), so a short sweep there may escalate where a
window-tight inf would decide it. `certified_arms` makes the same
choice deliberately ("a window-free bound cannot be wrong about a
branch"); one home would make it once.

## Moved (2026-10-26)

CHART folded into CHARTTAIL (Ev, in chat, 2026-10-26): CHART had closed
every P0 and P1 row, and its last rows are the chart boundary's tail.
Id and body unchanged.

## A third assembly (2026-11-14, TESSLANE)

`topo/src/pcurves_rings.rs` `Outer::arms` (the ring door's analytic reader, branch `tesslane/ring-placement`) assembles the analytic inf arms a third time. Its azimuth arm reads `azimuth_lever_inf`, which is now `pub` and exported from `geom_brep`. Its second arm is the polar radius (`pcurves::polar_arm`) or 1. A plane reads `(1, 1)`. Both arms are gated by `chart_region::gate_arm_as` on `ring_place_arm`.

So the azimuth formula is spelled twice, not three times, but the per-chart assembly (which arm per kind, the plane's, the gate) is now written in two places in `topo`.

The two disagree past a sphere's pole rows:
- `certified_arms` refuses `ArmUnbounded` where the window's `|v|` bracket tops `π/2`;
- `azimuth_lever_inf` clamps `|v|` at `π/2`, reads zero there, and `Outer::arms`' gate turns that into `Unbounded`.

At `f64` the two land on the same refusal. Under an interval scalar, `certified_arms` reads the bracket's top through `Bounds`, while `azimuth_lever_inf` takes `min` and `cos` at the scalar, so the edge cases differ. Neither reads the twin sheet's true arm `r·|cos v|` past the pole. One home would be `certified_arms` reading `azimuth_lever_inf` and `polar_arm`, with `Outer::arms` calling it. That needs `certified_arms` without its `Bounds` bound, which `scripts/gates/bounds-allowlist.sh` holds.
