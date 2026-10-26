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
