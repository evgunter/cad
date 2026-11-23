---
id: stored-solved-values-whose-check-differs-in-kind-are-not-behind-the-door
kind: issue
title: Stored solved values whose check differs in kind from on-within-the-band are not behind the solved-value door
status: open
opened: 2026-11-23
priority: P1
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


Found by `a-solved-value-the-body-stores-is-admitted-by-its-check`. The door it built, `geom_core::solved::admit`, takes an `f64` candidate and a residual at `T`, admits on a reading in the band's zero, and is bounded `Certifies` (the `geom-core` README, DL5.2). Two storing sites moved onto it (`curve_face::composite::settle`, `curve_face::spline_face::settle`). The sites below store a solved value too, and each one's check differs in kind from "its point reads on within the band", so the README's rule does not settle whether the door holds them. They are listed `owed` to this row in `scripts/gates/from-computed-sites.tsv`.

- **`topo::replace_face::carrier_foot`** (an edge's re-anchored end). Newton's foot from `SplineFoot` (`f64`, `Projection3::t`), reached at `f64`, `Interval` and `Sym` (`SplineFoot::certified` is bounded `CertifiedBounds`; a dual holds `NoLane`). The check is in the caller (`replace_face`'s re-anchor walk): `gap_within_eps(carrier.eval(t_new).distance(point), tol, band)`, the margin `ε − gap` passing unless decided negative. It differs three ways: the tolerance is ε (what a corner is held to), not the band's zero; a gap at ε passes (Zero passes); and a failure is read further, as `ReanchorCollapse` or `ReanchorPastCarrierEnd` from where the clamp stopped, beside the analytic arms (`param_near`) that share the same check. What the door would need: a second form that admits within a tolerance the caller states, returning the gap on refusal so the caller's end reasoning runs, entered from the lane (`SplineFootOf<T>`, where `Certifies` holds) since the walk is generic over `Decide`.
- **`geom_brep::edge_nurbs::measured_foot`** (two calls): the chart foot a plane x NURBS pcurve image is built on. Checked by the image's limbs afterwards, never at the foot. Bounded `Real`.
- **`topo::pcurves::mint::computed_foot`**: a measured chart foot a candidate iso line (`Pcurve::IsoLine`) is built from; the candidate is judged whole at the probe (`probe_pick`), not the foot's point. Reached only where `T::fitted_lane()` is `Some`, so no dual at run time, but bounded `Real`.
- **`geom_brep::pair_section`'s `polished`** (three calls): a Newton foot kept by an `f64` window (`moved <= pad + band.escalate()`), a seed that `foot_along` then decides at `T` (`SITE_FOOT_ALONG`, Zero in band). Bounded `Decide`.
- **`geom::Curve3::polish`** (`on_carrier.rs`): the polish iterate re-centred at an enclosure scalar (`T::from_computed(mid_param(lo, hi))`); `Foot::on_carrier` then admits the foot by its gap in the band's zero (`OnCarrier::On`), which `curve_pair` and `topo::boolean::reduce` store (`split_other_at_point`). Same kind of check as the door, but a second admit, at `Decide`: a `Dual<Interval>` reaches the re-centring and takes a zero tangent there.

Decide per site: the door's second form, a move of the check into the lane, or a closed form at `T`.
