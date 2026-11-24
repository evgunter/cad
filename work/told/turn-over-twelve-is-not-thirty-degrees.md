---
id: turn-over-twelve-is-not-thirty-degrees
kind: issue
title: turn/12 evaluates as the float tau over twelve, one float below thirty degrees
status: open
opened: 2026-11-21
priority: P2
cost: M
---


`turn` evaluates as `T::tau()` and `turn/12` as that float divided by twelve (`crates/editor-core/src/expr.rs`, `eval_inner`'s `K::Turn => Visit::Value(T::tau())`), which is `0x3fe0c152382d7365`, one float below the float nearest π/6 (`0x...7366`). Before `a-number-in-a-unit-is-rounded-once`, `30 deg` was `30 × fl(π/180)` and happened to be the same `...7365`; it is now the float nearest π/6, so `turn/12` and `30 deg` are one float apart. `180 deg`, `1 pi rad` and `f64::consts::PI` are one float (pinned in `quantity::tests::a_number_in_a_unit_is_its_decimal_times_the_exact_factor_rounded_once`), and `turn/4`, `turn/2` are exact multiples of the float π by a power of two, so they agree too.

A rotation by a constant fraction of a turn reads its sine and cosine from the fraction (`angle_of`, `geom_core::Angle::of_turns`), so the value channel is what differs. The question: should a constant fraction of a turn evaluate to the float nearest its exact value (the same door, with `turn` as 2π exactly), so that `turn/12` and `30 deg` are one float?

This mismatch is made by `a-number-in-a-unit-is-rounded-once`: before it, `30 deg` and `turn/12` coincided by accident (both the float below the one nearest π/6). What would fix it: evaluate `turn` and a constant fraction of it through the door, as the float nearest the exact fraction of 2π (`quantity`'s exact arithmetic, π as an enclosure), so `turn/12` is the float nearest π/6, as `30 deg` is.
