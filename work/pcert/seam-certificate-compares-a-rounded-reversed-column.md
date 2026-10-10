---
id: seam-certificate-compares-a-rounded-reversed-column
kind: issue
title: the seam certificate's backward branch compares the carrier against reversed_column's ROUNDED knot reflection bitwise, so the hull it certifies is against a column an ulp off the chart's
status: closed
opened: 2026-10-10
priority: P3
cost: E
branch: nurbs/row-space-reflection-exact
closed: 2026-10-10
pr: 4479
---

(NURBS lane, found by the rounded-reflection sweep of
`work/nurbs/row-space-reflection-compares-rounded-knots.md`. Read, not
measured.)

## Finding

`geom_brep::reversed_column` (`crates/geom-brep/src/nurbs_iso.rs`)
rebuilds a column's knots as `(a + b) - k`, one or two roundings per
knot. On a vector whose reflection is not representable (`[0, 0, 0.1,
1, 1]` reflects to `fl(1 − 0.1) = 0.9`, which is not `1 − 0.1` in ℝ)
the curve it returns is NOT the column run back: its knots are an ulp
off the exact reflection, so its point at `t` is not exactly the
column's at `a + b − t`.

The seam certificate's backward branch in
`crates/geom-brep/src/pcurve_cache.rs` (the `pcurve_iso_seam_sense`
decision, then `crate::nurbs_iso::reversed_column(&b)`) reverses the
chart's traversed column that way and compares its knots with the
carrier's under `!=`, then bounds the hull of `b − c`. Two
consequences:

* A carrier `reversed_column` built (`sweep::loft`'s one-segment wrap
  strut, `topo::offset_derive`'s section sense) matches bitwise, the
  hull reads 0, and the certificate states a gap of 0 against a column
  that is an ulp of knot shift away from the chart's. The knot
  perturbation's contribution (≈ ulp(k) · |C′|) is in no term.
* A carrier on the EXACT reflection (a STEP import, any producer that
  does not route through `reversed_column`) is refused as "not the
  chart's own column".

Both are confined to knot vectors whose reflection is not
representable; `KnotVector::mirror_symmetric`'s docs list the common
ones (thirds, `0.1/0.9`, `0.3/0.7`), and dyadic knots are not among
them. Whether a shipped producer builds such a wall is not measured.

## Shape

Do not rebuild the reflection. Decide the shared spline space with
`KnotVector::is_reflection_of` (exact two-vector reflection on `two_sum`
pairs, beside `mirror_symmetric`) on the un-reversed column's knots
against the carrier's, and compare the control nets with one read
backwards. `reversed_column` then has the same question to answer for
its producers: refuse an inexact reflection (as
`NurbsSurface::reversed_v` requires `mirror_symmetric`), or state that
its output is a different curve.


## Closed

Landed with `work/nurbs/row-space-reflection-compares-rounded-knots.md`
(PR 4479), not by refusing an inexact reflection: refusing would have
stopped `loft_body` building a one-segment loop through four equally
spaced sections at degree 1 (`1 − fl(1/3)` is not an `f64`), and
through 6, 7 or 8 sections at several degrees, all of which build on
main.

- `reversed_column` reflects through 0 (`KnotVector::negated`, exact
  for every vector): on `[−b, −a]` its point at `t` is the column's at
  `−t`. Its doc says so, and why the reflection about `[a, b]` is not
  offered.
- `seam_envelope` no longer rebuilds a reversal: run back, it decides
  the shared space with `KnotVector::is_reflection_of` and reads the
  traversed row's net backwards, its parameter map is `S − t` with
  `S` the reflection's sum, and its domain check reads `v` against the
  row's domain and `t` against the carrier's.
- `sweep::loft`'s one-segment strut is stated on `t ∈ [−1, 0]`,
  `v = −t`; `topo::offset_derive`'s reversed section can no longer
  refuse.
