---
id: a-splines-on-carrier-exhausts-its-piece-budget-at-interval
kind: issue
title: A spline's on_carrier at Interval exhausts its piece budget at every box from ±1e-3 to ±0.05, logging some 5,800 piece escalations
status: open
opened: 2026-11-03
priority: P2
cost: M
---



Found by the design fork on `a-foots-parameter-is-read-without-its-certificate` (two designers), and measured again by `onecopy/foot-certificate`.

On the cubic hairpin of `geom::curves::on_carrier::rows::a_tie_across_a_hairpins_branches_is_the_certificates_to_read`, `Foot::on_carrier` at `Interval` returns `Unwalked(Exhausted)`. The point lies on the near branch, and the box is ±0.05 about it. The run logs 5,852 escalations. Which predicates they are was not broken down; the walk's own rows (`curve_carrier_piece_clear`, `curve_carrier_piece`) are the likely ones. The designers saw the same at every box from ±1e-3 to ±0.05.

The refusal is honest: nothing reads `On` at the far branch. But it costs:
- The walk runs to `geom_core::bisect::PIECE_BUDGET` in every leaf.
- The driver gets thousands of escalations where one says the same.
- At ±1e-3, where a narrower box should answer, the walk still exhausts.

Look at how `Curve3::clear_of` spends its budget on a point within the band of the span (`crates/geom/src/curves/on_carrier.rs`, `clear_of` / `point_walk`). A piece that holds the point is never cleared, and its halves are walked to the budget. A walk that stops at the first piece within the band, its reach below the band (the `curve_carrier_piece` row), would answer `Near` at once.
