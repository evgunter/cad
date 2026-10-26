---
id: f64-turn-rule-measures-headroom-against-f64-tau
kind: issue
title: At f64 the turn rule's headroom is read against f64 τ, not 2π, so Whole admits defects past the band
status: open
opened: 2026-10-26
priority: P3
cost: E
---


Found by the review of PR 5724 (`pred/turn-rule-follow-ups`), whose
probe `crates/geom/tests/review_turn_follow_ups_probes.rs`
(`conic_defect_bounds_and_verdicts_against_the_true_defect`, `#[ignore]`d)
reads every verdict against the arc length of the **true** defect
(`t1 − t0 − 2π`, 2π the real number).

`Period::turn` (`crates/geom-core/src/period.rs:305`) reads the headroom
as `self.length − extent`, and at `f64` the period is `f64::consts::TAU`,
`2.449e-16` short of 2π. The speed bounds now hold over the true defect
(`geom::carrier_speed::turn_defect`'s reach carries `DEFECT_ROUNDING`,
`crates/geom/src/carrier_speed.rs:75`/`:100`), but the headroom they
multiply does not: a window whose f64 headroom is `0` has a true defect
`2.45e-16` wide, which on the fast side of an ellipse is
`a·2.45e-16` metres.

Measured over 358k cases (after the fix pass):
- At `f64`, `Whole` reads a true defect up to `1.04ε` at `a = 100`,
  `ε = 1e-12`, and `1.36ε` at `a = 1e6`, `ε = 1e-9`.
- In the sub-ulp regime (`a·2.45e-16 > ε`, e.g. `a = 1e6`, `ε = 1e-12`,
  `extent = τ_f`) an overlap of up to `689ε` reads `Whole`.
- `Over` decided at a true length of `0.86 × 10ε` (sub-ulp) and
  `0.997 × 10ε` outside it.
- `Interval` encloses 2π and has none of these.

The likely fix is an allowance on τ itself, `hi·ulp(τ)` in metres,
folded into the headroom the rule reads (or into `Bounded`'s two
products), so `Whole` needs the headroom plus that allowance inside the
band and `Over`/`Short` need it past the band.
