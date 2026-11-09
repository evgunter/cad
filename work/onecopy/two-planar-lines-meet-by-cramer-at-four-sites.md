---
id: two-planar-lines-meet-by-cramer-at-four-sites
kind: issue
title: Two planar lines' meeting parameters are spelled by Cramer's rule at four sites
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by the PR 6594 review's planar sweep, extended to `w.perp_dot(u)` shapes. The parameters at which two planar lines `p + t·r` and `q + s·u` meet are spelled by Cramer's rule, `(q − p)×u / r×u`, at four sites:
- `crates/profile/src/seg.rs` `line_line` (~`:1673`)
- `crates/profile/src/seg.rs` the ray cast's line arm (~`:2447`)
- `crates/profile/src/path.rs` the fillet corner's `t_ray`/`s_arr` (~`:4432`)
- `crates/topo/src/chart_region.rs` (~`:3383`)

Each decides the denominator under its own gate before dividing.

Fix: give the reading one home on `geom_core::Line2`, e.g. `Line2::meet(&self, other) -> (T, T)` returning both parameters with the denominator, and have the four read it. Each caller keeps its own gate on the denominator.

The sweep cannot see a meet whose cross products are named before the division (`let denom = …; let num = …;` in separate statements with renamed operands), or one spelled through `Vec3::cross` on lifted planar vectors.
