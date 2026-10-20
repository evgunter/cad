---
id: an-axis-mated-spin-at-interval-widens-the-copys-translation-by-the-measured-spins-dust
kind: issue
title: An axis-mated spin at Interval widens the copy's translation by the measured spin's dust
status: open
opened: 2026-10-20
priority: P3
cost: M
refs: [a-turn-by-the-nearest-quarter-turn-rounds, transform-retires-into-a-placement]
---

Found by INTENT stage 3 D's exact-turn lane. A placement's spin is set in `mate::value::settle` (`crates/editor-core/src/mate/value.rs`) by turning the coset's representative about the residual's line through `at` from its measured spin to the written one (`turn_to`). For a spin written as a fraction of a turn the written rotation is exact (`geom_core::Angle`), but the measured spin is read by `SolveScalar::solve_atan2` and undone by a rotation built from its sine and cosine.

At `Interval` an exact `(0, 1)` measures `atan2` as the dust `[-2e-323, 2e-323]` (measured: `Interval::zero().atan2(Interval::one())`), whose cosine is `[1 − 4.4e-16, 1]`. The undo's diagonal therefore carries that width, and `turn_to(..) * m` levers it by the copy's translation: an `Axis`-mated pip at `(0.5, 0.5, 1.04)` with an exact turn reads its `x` and `y` as `[0.49999999999999967, 0.5000000000000002]` at `Interval` (its linear part is exact, `f64` is exact). A plane mate's spin turns about the copy's origin and keeps the translation as it is, so the plane-mated and unmated rows in `intent_s3_d_placements::a_block_placed_upright_by_a_written_quarter_turn_stands_on_the_worlds_planes` are bit-exact at every lane; an axis mate's is not.

Both reds this was found beside (`m10_di_dual_corpus`'s tour die, the 1e-12 fillets) are green with this width left in.

**Done when** an axis-mated copy whose measured spin is exactly zero, and whose written spin is a fraction of a turn, is placed bit-exactly at `Interval`. Two readings, for the owner: read the measured spin's sine and cosine off the chart's `(y, x)` pair rather than through `atan2` and back, or have the box lanes' `solve_atan2` enclose an exact `(0, x > 0)` as the exact zero and skip a turn by an exact zero. Add the axis-mated case to that row.
