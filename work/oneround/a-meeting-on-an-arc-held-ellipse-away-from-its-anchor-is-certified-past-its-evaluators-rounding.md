---
id: a-meeting-on-an-arc-held-ellipse-away-from-its-anchor-is-certified-past-its-evaluators-rounding
kind: issue
title: curve_pair::meet certifies a meeting on an arc-held ellipse away from its anchor where the evaluator's own point rounds past the band
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [an-arc-held-ellipse-is-charged-a-centre-held-ones-resolution-and-hulls-its-phase-across-an-interval-seam-turn, an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre]
---


Found by the arc-held ladder of `oneround/conic-pair-readers`
(`crates/geom-brep/src/curve_pair/ladder_rows.rs`, judged by
`analysis/oneround/conic-pair-readers/oracle.py` at 60 digits against the
curve the stored numbers state), after every reader in `curve_pair` that
read the derived centre was re-posed from the stored point.

An ellipse held on its arc evaluates a point `θ` from its anchor as
`A + u·a·(cos θ − 1) + v·b·sin θ` (`Conic::point_at`, `crates/geom/src/curves/conic.rs`),
so the point rounds at about `u·a·(1 − cos θ)`: exact to its coordinates
beside the anchor, and at the semi-axis's scale a radian away. Where that
rounding passes the band's zero, `meet` reads the evaluator's point as
the curve and certifies on it: on the ladder at `f64`, with the
evaluator's own error at the pose (`|point_at(θ) − P(θ)|`) beside it:

| ε | a | pose | evaluator's error | answer, against the oracle |
|---|---|---|---|---|
| 1e-9 | 1e8 thin | θ = 2.0, line at 90° | 1.5e-8 | a meeting `1.5e-8` off the true one |
| 1e-9 | 1e8 fat/thin | θ = 0.5, line at 5° | 1.1e-9, 2.0e-9 | a meeting `1.3e-8`, `1.5e-8` off (the error over the crossing's sine) |
| 1e-9 | 1e9 fat | θ = 0.5, line lifted 1e-8 | — | `Ok`, no meeting, true gap `3.8e-10` |
| 1e-12 | 1e4, 1e5 thin | θ = 2.0, line at 5° | 1.2e-12, 6.5e-12 | a meeting off by `1.2e-11`; `Ok`, no meeting, true gap `4.2e-13` |
| 1e-12 | 1e6 thin | θ = 0.5, line at 30° | 1.1e-11 | a meeting `1.1e-11` off |
| 1e-12 | 1e8, 1e9 thin | 1000 m of arc from the anchor (θ = 0.054, 0.017) | 7.7e-12, 1.7e-11 | a meeting `1.7e-11` off; for the circle door `Ok`, no meeting, true gap `9e-13`, `8e-13` |

19 such readings in all (7 at 1e-9, 12 at 1e-12, every one at `f64`;
`Interval` reads none wrongly, refusing these), the circle door's alike.
Judged against the stored numbers' curve, as every reading must be: the
evaluator is not the curve away from the anchor, by its own rounding.

`carrier_resolution::resolution` charges an ellipse `u·a`
(`an-arc-held-ellipse-is-charged-a-centre-held-ones-resolution-…`), but
`meet`'s closed-form arms read a candidate's point and its gaps without
that charge where it is past the band. The fix is a meter of the
evaluator's running bound (`Conic::running_point`, or the height's
`Height::running_at`) on the readings that certify a meeting or its
absence, so these refuse; or an anchor re-chosen near the part in use
(the general anchor, `a-section-conic-is-anchored-at-a-general-point-of-its-arc.md`).
