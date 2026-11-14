---
id: ders1-at-an-enclosure-scalar-is-wider-than-the-hodograph-on-most-curves
kind: issue
title: NurbsCurve::ders1 at an enclosure scalar is wider than the same derivative through the hodograph on most curves (a median 2.8×, 20× on the sweep's cubic interpolants): should every caller read it that way?
status: open
opened: 2026-11-14
priority: P2
---



Filed by ONEHOME's sweep-walls-at-t lane (PR 6952), which added `NurbsCurve2`/`NurbsCurve3::derivative_at` (`crates/geom/src/curves/nurbs.rs`, beside `ders1`). It is the same derivative as `ders1`'s, read through the hodograph: the homogeneous net's differences against the degree `p − 1` row of the same knots (`geom_core::spline::basis::basis_funs_to_degree_in`), plus the quotient rule on a rational curve. `crates/geom/tests/curves/derivative_at_is_ders1s.rs` checks it against `ders1` (f64 within 8 ulps of `|C′|`; the `Interval` enclosures meet) on a rational two-span path through a double knot.

**The measurement.** At `Interval` of a point parameter, on three cubic interpolants a path sweep reads (a 2-turn helix through 65 points, a closed circle of radius 3 through 33, a 370° flat ribbon):
- `ders1`'s derivative is 3–17e-13 wide, relative 4–9e-14 (a few hundred ulps). The width comes from the derivative recursion (`ders_basis_funs_in` then the homogeneous sum), not from hulling across knots: only the end parameters sit on a knot.
- `derivative_at`'s is 1.3–5.7e-14, about 20× narrower on these three.
- Normalized, the tangents are about 5e-14 and about 6e-15. A frame minted from them reaches 2.6e-13 and 3.3e-14.

**Over random curves** (PR 6952's review: 600 curves of degree 1–5, knot multiplicities up to `p`, polynomial and rational, 4899 stations, against an exact rational oracle with no enclosure miss), the ratio of `ders1`'s width to `derivative_at`'s has a median of 2.8: 3–5× on polynomial curves, 1.1–1.3× on rational ones. In 366 of 13940 components `derivative_at` is the wider, by up to 2.3×. The 20× above is the sweep's cubic interpolants, not the general case.

**Today** only the path sweep's station mint reads `derivative_at` (`crates/sweep/src/skin.rs` `read_tangent`). Every other `ders1`/`deriv` caller is unchanged.

**Owed.**
- Decide whether `ders1`'s first-derivative channel (and `deriv`) should read the hodograph form everywhere. That would make one derivative evaluation.
- Its cost at `f64` and at `Dual`, and the bit-for-bit pins it would move (`ders1_is_eval_and_deriv_bit_for_bit`, `span_bit_identity_ext`), are the measurement to take first.
