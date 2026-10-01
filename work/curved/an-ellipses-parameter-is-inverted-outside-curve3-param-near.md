---
id: an-ellipses-parameter-is-inverted-outside-curve3-param-near
kind: issue
title: An ellipse's eccentric anomaly is inverted in emit_topo::param_along, not in Curve3::param_near, which answers None for an ellipse
status: open
opened: 2026-10-01
priority: P4
cost: M
refs: [edge-pieces-are-named-by-their-ends]
---


## What

`emit_topo::param_along` (`crates/editor-core/src/names/emit_topo.rs`,
its `Curve3::Ellipse` arm) inverts an ellipse's eccentric anomaly by
hand, near the middle of the edge's interval, because
`Curve3::param_near` (`crates/geom/src/curves.rs:1206`) answers `None`
for an ellipse. The inversion has two homes in waiting.

## Why it was not moved

`param_near` has callers that read `None` as a refusal:
`topo::replace_face::plan_reanchors` (`replace_face.rs:2303`),
`sweep::blend::surgery` (`surgery.rs:2520`) and `topo::boolean::reduce`
(`reduce.rs:2495`). Giving the ellipse an arm there turns those
refusals into answers, a behaviour change in the kernel to be measured
on its own, not ridden in on a naming PR. Once the arm lands,
`param_along` drops its own and calls `param_near` for every carrier.
