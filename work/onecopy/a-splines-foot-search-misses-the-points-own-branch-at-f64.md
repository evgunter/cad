---
id: a-splines-foot-search-misses-the-points-own-branch-at-f64
kind: issue
title: A spline's foot search, at f64, can polish a point on the carrier to a far branch from every seed but its own, so on_carrier escalates where On is the answer
status: open
opened: 2026-11-03
priority: P3
cost: M
---



Found by `onecopy/foot-certificate` while building the f64 far-branch row (`geom::curves::on_carrier::rows::a_foot_on_a_far_branch_at_f64_is_the_certificates_to_read`).

On a cubic spline of five knot spans, take a point on the carrier at `t = 0.35`. `Curve3::foot` from any seed but `0.35` returns a stationary point near `t = 0.03`, 0.05 away. `on_carrier` then escalates (`curve_carrier_piece_clear` in band): it is sound, it gives no `On` at the far foot and no `Off`, but the answer is `On`.

`foot` keeps the best of the seed and four samples per knot span (`SPLINE_SAMPLES`), then Gauss–Newton. Here every sample lies nearer the far branch's basin. A search of 300 random cubics of five spans, each with 99 points on the carrier and 21 seeds, gave 26,760 such feet in 726,100.

Fix: seed the polish from more than the single best sample (each local minimum of the sampled gaps), or descend from the best few and keep the least gap. Then re-measure the search above.
