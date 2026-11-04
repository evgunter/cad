---
id: conic-speed-hi-reads-the-signed-larger-semi-axis
kind: issue
title: Conic::speed_hi and speed_lo read the signed semi-axes, so a circle or an unchecked ellipse stored with a negative one under-reads its speed
status: closed
opened: 2026-11-04
priority: P2
cost: M
parent: an-ellipses-general-lane-is-written-on-no-chart
closed: 2026-11-04
pr: 6524
---


Filed by PCERT (`pcert/ellipse-general-lane`, the ellipse's general
lane).

## What

`geom::Conic::speed_hi` documents itself as "the greatest speed `|P′|`
over the carrier, and a bound on `|P″|`: the larger semi-axis", and
`speed_lo` as the least speed. Both read the stored semi-axes SIGNED
(`crates/geom/src/curves/conic.rs`): a `Circle`'s are its `radius`, and
an `Ellipse`'s `semi_axes_by_size()` is `max`/`min` of `semi_u`,
`semi_v`. Neither `Curve3::circle` nor `Ellipse::new` refuses a negative
value, and the repo already treats a negatively stored semi-axis as a
real input at rest (`work/restfront/an-ellipse-stored-minor-over-major-passes-tier-3.md`,
closed: "the readers that took a stored semi-axis as ordered or signed
now read" magnitudes). So an ellipse stored `(−10, 1e-3)` reads
`speed_hi = 1e-3`, and a circle of radius `−r` reads `−r`.

`git grep -n '\.speed_hi()' crates` lists 59 production readers. Each
that charges rounding or bounds a derivative at that reach under-reads
such a carrier. In `crates/geom-brep/src/implicit.rs`:
`sample_rounding` (`let a = conic.speed_hi()`), `torus_curvature_bound`
(`a` bounds `|C′|`, `|C″|`), and `conic_residual_harmonics`' `noise`
(`rounding_charge(d.norm() + conic.speed_hi())`).

The projected lane's charge no longer reads it: `implicit::semi_reach`
reads `max(|a|, |b|)` for `quadric_harmonics`' `terms`,
`conic_cone_harmonics`' reach and `conic_axis_distance_floor`, and the
`semi-reach-signed` mutant there is red on
`ellipse_lane_rows::an_ellipses_coefficients_stay_within_their_charge_on_every_chart`
(the row "a negative semi-axis a hundred times the chart").

## Fix

Read magnitudes in `speed_hi`, `speed_lo` and `speed_at` (bit-identical
for every carrier a door builds, whose semi-axes are positive), then
retire `semi_reach` into it.

## Closed

PR 6524, closed on the invariant rather than by reading magnitudes
(review CC2, revised). A negative semi-axis is not another spelling of
the same edge: every door builds positive ones, STEP's are positive
lengths, and a signed one reverses the traversal. So the edge
certificate decides it once, typed, at the head of its span check,
before any reader of the carrier's speed:
`CertifyError::SemiAxisNegative`. The kernel mints none, so at a
build it is the caller's input and ends in its own lever; at rest it is
the stored culprit's defect, and at import a damaged file. Main ended
such a record as a span that does not run forward.
`Conic::semi_axes`, `speed_lo` and `speed_hi` state the positive
invariant at their definition and read it as is. `implicit::semi_reach`
and its mutant are gone, and the charge reads `speed_hi` again.

Rows:
- `pcurve_cache::ellipse_lane_rows::a_conic_stored_with_a_negative_semi_axis_refuses_typed`:
  an ellipse with either or both semi-axes negative, and a circle of
  negative radius, at f64 and Interval, with the three door endings.
  Red under `semi-axis-unchecked`. A zero semi-axis is a collapsed
  length, so it stays the span check's.
- `tests/ellipse_signed_semi_axis_gate.rs`, re-pointed from
  `StatedInterval` to the typed refusal.

The readers this item named (`sample_rounding`,
`torus_curvature_bound`, `conic_residual_harmonics` and the others
outside geom-brep) read only certified edges' carriers, so they
inherit the invariant. The swept list is in the PR body.
