---
id: the-ruled-aims-on-axis-fallback-hulls-at-interval
kind: issue
title: The ruled aim's on-axis fallback hulls two directions at Interval
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [the-side-readers-aims-decide-exact-ties-at-a-documents-nominal]
---

`crates/topo/src/boolean/ruled_region.rs` `RuledFaceRegion::aim` steps
from its edges' middles, carries their mean onto the surface along its
parallel, and falls back to the surface's reference azimuth where that
mean is on the axis. The fallback is an evaluation, not a decision
(`rho.select_le_zero(..)`). At `Interval`, an enclosure of `rho` that
straddles zero selects the hull of both arms, so the aim's direction is
a box spanning the reference azimuth and the mean's own. This is the
same shape as the sphere aim's former pick of its deepest step, which
`flush/aims-ties` retired by stepping from one arc in walk order.

It is unmeasured. A face whose middles' mean lies on the axis at a
nominal (a full band whose rims start half a turn apart) is the pose to
build. The aim only steers, so a box aim costs a ray, not a side.

The ruled aim stays at its middles. Moving it to its edges' golden
sections steered `split_seam_vanishing_tilt`'s wall aim onto the seam
ruling
(`an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side`).
