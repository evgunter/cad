---
id: trimmed-lane-inner-rule-has-no-composite-arm
kind: issue
title: The trimmed lane's inner v rule is exact only for p_v <= 4, and a chart past it refuses at TRIM_NC_WINDOW with no composite arm
status: open
opened: 2026-10-23
priority: P3
cost: M
---


Filed by `trimmed-quadrature-composite-rounds`, whose unit gave the
trimmed lane's OUTER `u` rule a composite arm
(`geom_brep::props::quad::chord_polygon_flux_composite`) and kept its
INNER `v` rule exact. The inner rule integrates
`G_f(u, v) = ∫_{v₀}^{v} f(u, s) ds` with the closed Newton–Cotes rule of
order `3p_v − 1` per `v` span (`column_integral`), and
`newton_cotes_weights` tops out at 12 intervals, so the inner rule
covers `p_v ≤ 4`. `trimmed_patch_face_rounds` refuses a chart with
`p_v ≥ 5` typed at `TRIM_NC_WINDOW`, pinned by
`q6_a_chart_past_the_newton_cotes_window_refuses_typed` (its degree-(1, 5)
half).

No shipped construction mints such a chart: the fitted offset charts are
bicubic (`OFFSET_FIT_DEGREE = 3`) and the loft/sweep walls are degree ≤ 2
in `v`. A STEP-imported trimmed NURBS of degree ≥ 5 in `v` would reach it.

What the arm would be: the inner integral by a composite rule with its
own certified remainder (the midpoint rule's `h³/24·f_vv`, `f_vv` through
`PatchGrid`'s `v` ladder, which already carries `S_vv` and would need
`S_vvv`), and the outer arm's `φ''` read through the same inner rule for
`G_uu`. The outer composite arm's remainder reads `G_uu` through the
EXACT inner rule (`flux_uu` under `column_integral`), so an inexact inner
rule needs a remainder there too.
