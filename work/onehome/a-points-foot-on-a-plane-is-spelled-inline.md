---
id: a-points-foot-on-a-plane-is-spelled-inline
kind: issue
title: A point's foot on a plane is spelled inline in five places with no geom home
status: open
opened: 2026-11-02
priority: P2
cost: E
---

Found by the sweep of the `onehome/geom-readings` lane (the axis-foot shape `o + a·((p − o)·a)` has a plane twin, `p − n·((p − o)·n)`), against main `44577259fb`.

A point's foot on a plane through `o` with normal `n` is written inline at:

- `crates/topo/src/census/depth_gap.rs` `least_point`;
- `crates/sweep/src/blend/reach.rs` `sheet_fn` (the `Plane` arm, after normalizing `n`);
- `crates/geom-brep/src/pair_section/plane_spline.rs` `pivot`;
- `crates/viewer/src/splitplane.rs` `preview`;
- `crates/sweep/src/blend/arms.rs` `trace` (the cylinder arm's `on_axis − tau·((on_axis − rim)·tau)`, a foot on the plane through `rim` square to `tau`).

Some take `n` as unit and one normalizes first, so they agree only on a unit normal. It is a one-carrier-and-a-point reading (`[one-way.readings]`), so its home is `geom`: beside `geom::Line::foot`, a plane's foot (on `Surface::Plane`, or a small plane view as `geom::Line` is for lines), with these sites repointed.
