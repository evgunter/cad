---
id: turns-has-two-answers-to-an-integer-past-the-mantissa
kind: issue
title: angle.rs answers an integer too large for f64 twice — Turns::sin_cos refuses past 2^51, Angle::from_turns encloses past 2^53
status: open
opened: 2026-10-21
---

## What

Found by FRAME's `frame/exact-frames-are-turns` (PR 5573), which added
`Angle::from_turns` in `crates/geom-core/src/linalg/angle.rs`.

- `Turns::sin_cos` reads the turn in units of `1/(4·den)` and embeds
  `at` and `4·den` with `from_f64(… as f64)`. It refuses (`None`, and
  the caller falls back to the radian) past `den > 2^51`, so `4·den`
  stays exact.
- `Angle::from_turns` embeds `num` and `den` through the private
  `integer()`. That is one literal up to `2^53`, and past it a
  `hi·2^32 + lo` split whose one rounding is outward at `Interval`.

So two rules in one file answer "this integer has no `f64`". They are
not the same small change. `sin_cos` needs `4·den` exact and then
divides by it, so sharing `integer()` would change which turns it
reads exactly and which it reads through the radian, and that wants
its own pins. The open question is whether `sin_cos` should enclose
past `2^51` the way `from_turns` does, or whether a denominator that
large should be refused once, at `Turns::new`.
