---
id: interval-multiplication-by-a-half-pads-where-division-by-two-is-exact
kind: issue
title: Interval multiplication by ½ pads an ulp below 2⁻⁹⁵⁹ where division by 2 is exact
status: closed
opened: 2026-11-09
priority: P4
cost: E
closed: 2026-11-09
pr: 6661
---


Found in the delta review of PR 6632 (`onecopy/bracket-mid`).

`geom_core::mid_param` and `geom_core::bracket_mid`
(`crates/geom-core/src/param.rs`) halve by multiplying by
`T::from_f64(0.5)`. At `Interval` that goes through `DInterval`'s
`Mul` (`interval-transcendentals/src/arith.rs`). Its directed corner
bounds check the product for exactness, and that check gives up for
an operand below about `2⁻⁹⁵⁹`, so the bound is padded one ulp
outward. `DInterval`'s `Div` detects an exact `/2` at that magnitude
and does not pad.

The reviewer measured 2M random intervals:
- `(a + b) / [2]` against `(a + b) · [½]`: 406 cases differ.
- `a + (b − a) / [2]` against `a + (b − a) · [½]`: 168 cases differ.

Every difference is one ulp, and every one is wider at `· ½`.

The result is sound, but a halving pays padding it does not need. Five
sites that PR 6632 moved from `/ T::from_f64(2.0)` onto the homes see
it: `geom-brep::implicit`, `pair_section::torus_arms`, `topo::shell`
(two sites) and `sweep::blend::reach`.

## Asked

Make `Interval` multiplication by an exact power of two (½ in
particular) exact at every normal magnitude, as division by 2 already
is, so the homes halve without padding. The reviewer points out that
`DInterval` already has a power-of-two scaling path that is exact above
the smallest normal; route a point-operand power-of-two product through
it. Pin it with a row that halves an interval whose endpoints lie below
`2⁻⁹⁵⁹` and asserts the result is the exact half.
