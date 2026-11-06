---
id: ray-wall-twins-fire-different-predicates-at-1e-6
kind: issue
title: ray_wall_margin_twins at ε 1e-6: the millimetre pipe takes curve_pair_span_end_chord, curve_pair_span_end_side and side_at_infinity_met_edge_represents, which the metre twin does not (since PR 5781)
status: open
opened: 2026-11-06
priority: P3
cost: E
---


Filed by `onecopy/main-reds-k-and-m10`, which re-pinned the row on it.
On FLUSH's slate because PR 5781 (`flush/aims-ties`, merge
`4e0ef4858b`) is where it began: bisected first-parent over main from
`90f5db7a7e`, where the row stopped on its old pin at 1e-6.

## What

`crates/sweep/tests/ray_wall_margin_twins.rs`,
`the_wall_arms_margins_scale_linearly_with_the_model`, at
`CAD_TOLERANCE_EPS=1e-6` stops on `the twins must fire the same
predicate set`: the millimetre pipe (radius 1e-3, a thousand ε) records
`curve_pair_span_end_chord`, `curve_pair_span_end_side` and
`side_at_infinity_met_edge_represents`, and the metre pipe records none
of them. The verdicts agree. At 1e-9 and 1e-12 both twins fire the same
set (and the row stops on
`work/onecopy/ray-wall-twins-read-f31-angle-renames-as-margins-that-must-scale.md`).

Whether the small twin taking a branch the large one does not is a
defect (an aim chosen by an absolute length) or a correct reading of a
pipe a thousand ε across is the question.

## Pin

The row is pinned at 1e-6 on this stop (`the twins must fire the same
predicate set` with `side_at_infinity_met_edge_represents`). No CI row
runs it at 1e-6: the nightly takes it at 1e-9 (`k-lint (dev-probe)`)
and at the default ε (`--all-features`).

Reproduce: `CAD_TOLERANCE_EPS=1e-6 cargo test -p sweep --features probe
--test all -- ray_wall_margin_twins::`.
