---
id: an-ellipses-parameter-is-inverted-outside-curve3-param-near
kind: issue
title: An ellipse's eccentric anomaly is inverted in emit_topo::param_along, not in Curve3::param_near, which answers None for an ellipse
status: closed
opened: 2026-10-01
priority: P4
cost: M
closed: 2026-10-26
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

## Closed (2026-10-26, sweep follow-up)

Both halves of "once the arm lands" hold on main (`d621f67e`):

- `Curve3::param_near` has an `Ellipse` arm
  (`crates/geom/src/curves.rs:1756-1768`), anchored at `near` as the
  circle's and spiric's are; only `Nurbs` answers `None` (`:1769`).
- `emit_topo::param_along`
  (`crates/editor-core/src/names/emit_topo.rs:5181-5191`) has no
  carrier arms of its own: it gates on `places_points` (`:4478`, itself
  a `param_near` probe) and reads `Track::place` (`:4153`), which is
  `self.carrier.param_near(p, anchor)` for every carrier.

The callers this row named as reading `None` as a refusal now treat
`param_near` as total over the analytic kinds
(`crates/topo/src/replace_face.rs:3782`, `:4497`, `:4574`:
"`param_near` inverts every analytic kind"). No hand inversion of an
ellipse's eccentric anomaly is left in a non-test path: `editor-core`'s
only `Curve3::Ellipse` matches are `emit_topo.rs:4059` (the period) and
`eval/measure.rs:515` (a point on the major axis), and the one other
`atan2` over an ellipse's frame, `crates/topo/src/boolean/join.rs:6753`,
is the independent round-trip oracle inside the test
`the_equal_radius_section_always_crosses_at_two_nodes`. The arm landed
in `aa94de17ff` ("shell: re-anchor a moved vertex on a spline or ellipse
carrier", PR 4117, merge `ae25cf7f0d`, 2026-10-06, read through the
GitHub API: this checkout is shallow). The commit that cut
`param_along` over to `Track::place` was not traced.

`work/issues/a-seam-anchored-param-read-straddles-the-cut-at-interval.md`
still lists `param_along`'s ellipse arm as a hand copy; that bullet is
stale for the same reason, and is left to that row's owner.
