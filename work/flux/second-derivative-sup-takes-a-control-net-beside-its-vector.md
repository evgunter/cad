---
id: second-derivative-sup-takes-a-control-net-beside-its-vector
kind: issue
title: nonrational_second_derivative_sup takes a control net beside its knot vector and re-mints its differenced level
status: review
pr: 4838
opened: 2026-10-10
priority: P3
cost: E
---


Found by the `coefficient-vector-pairing-survivors` sweep (NURBS).

`crates/geom/src/curves/second_derivative.rs`
`nonrational_second_derivative_sup(knots, control: &[Point3<f64>])` is
a public door taking a control net beside its vector, related by count;
inside, it differences each channel with `knots.difference_coeffs`,
builds the derivative vector itself (`KnotVector::clamped(
derivative_knot_slice, p − 1)`), and re-mints the level with
`kv1.with_coeffs(&q1).map_or_else(Interval::refused, …)` — a refusal
arm dead by construction.

Disposition: take the curve (or a pair minted by its caller), build each
channel with `KnotVector::with_coeffs_from_fn`, and read the second
level through `SplineCoeffs::derivative` → `SplineCoeffsBuf::pair` →
`derivative_domain_hull`; `KnotVector::derivative` is the vector it now
builds by hand. `KnotVector::difference_coeffs` then has no consumer
outside `crates/mesh/src/chords.rs` (filed on CHORD) and its own test,
and goes with the last one.

2026-10-11 (ENCL `degree-1-partial-has-four-spellings`, PR 4754): the
premise of this row's last sentence no longer holds. `KnotVector::difference_coeffs` still
has consumers beyond `chords.rs`: `props::quad`'s `PatchGrid::step`
line step and `geom_core::spline::net`'s `TensorCoeffs::diff_u`/`diff_v`,
which every tensor partial (`TensorCoeffs::derived_u`/`_v`) is built on. New evidence: the site's
`p < 2` guard plus `clamped(derivative_knot_slice, p − 1)` is
`KnotVector::derived`'s split by hand — `SpanConstant` refused as
`DegreeBelowTwo`, `Discontinuous` as `DerivativeKnotVector`. A match on
`knots.derived()` (or the pair's `SplineCoeffs::derived`) gives both
refusals their cause directly.
