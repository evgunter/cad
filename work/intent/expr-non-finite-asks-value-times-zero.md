---
id: expr-non-finite-asks-value-times-zero
kind: issue
title: refuse_non_finite asks value·0, which reads [0,0] for an unbounded Interval enclosure, so an expression whose value runs to infinity passes the final check
status: open
opened: 2026-10-13
priority: P2
cost: E
---


`refuse_non_finite` (`crates/editor-core/src/expr.rs:1903-1911`) decides
an expression's finiteness as `value · 0` through
`k_stats::check_unlogged` and admits on `Sign::Zero`. At a point scalar
that is right: `∞·0` is NaN, which is poison. At `Interval`, an unbounded
enclosure such as `[MAX, ∞]`, `[0, ∞]` or `[−∞, ∞]` times zero is the
finite `[0, 0]`. A scratch row on SYM-17's branch showed this, and none
of the three is poison. So an expression whose enclosure runs to infinity
passes the final check as finite.

This is the same flaw SYM-17's delta review found at
`RigidMotion::check_rigid`'s translation probe (`0·tᵢ`; MINOR-1 on
`sym/17-review-delta`). PR #4197 fixed it there by asking
`Real::is_bounded` (`crates/geom-core/src/real.rs:385`), a required
method that decides both endpoints at an enclosure.

The fix is likely `value.is_bounded()` in place of the `value·0`
decision. The doc above the function says no `Margin` door fits, because
`value · 0` carries `value`'s dimension. `is_bounded` is not a band
decision, so that objection lapses with it. Moving off the K sample
`expr_non_finite` (F18) changes what that sample measures; say what moved.

Found by SYM-17's final pass, in the sweep for `x − x` / `0·x`
finiteness spellings.
