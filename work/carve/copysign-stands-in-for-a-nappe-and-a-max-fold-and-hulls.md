---
id: copysign-stands-in-for-a-nappe-and-a-max-fold-and-hulls
kind: issue
title: sweep's nappe selector and revolve max-fold spell a two-way choice with copysign, which hulls at a decidable tie
status: open
opened: 2026-09-12
---


## The shape

`Real::copysign`'s `Interval` arm must return the two-sided hull
`[-sup|x|, sup|x|]` for ANY `sign` enclosure containing zero, including
the point enclosure `[0, 0]` — because an `f64` zero's sign BIT is
invisible in an enclosure and a one-sided choice would fail to contain
an `f64` replay that saw `-0.0` (`Interval::copysign`'s own docs,
`crates/geom-core/src/interval.rs`).

That is right wherever the transferred sign is the ANSWER — an odd
function's sign, a signed distance. It is wrong wherever `copysign` is
standing in for a two-way CHOICE between candidate computations: there
the tie is a value zero, not a sign bit, the choice is the same for
every point of a point enclosure, and the hull throws away a decision
that exists. `Real::select_le_zero` (`crates/geom-core/src/real.rs`,
added by PROPS's sign-hull unit) is the door for that case: its
tie-break keys on `d == 0`, so it DECIDES at a point tie and hulls only
over a straddle of positive width.

`Vec3::orthonormal_basis` was the first instance and is fixed. Swept
with `grep -rn "\.copysign(" crates/*/src`; what that pattern cannot
match is in the sign-hull unit's PR body.

## This program's two sites

- **`revolve::axis`'s radial fold** (`crates/sweep/src/revolve/axis.rs`):
  `r_max = r_max.max(frame.r(p).abs().copysign(margin))`. The clearest
  member: the site's own docs say the conditional "the candidate enters
  only when the margin is non-negative" is *folded comparison-free via
  `copysign`*. At `Interval` a `margin` enclosing zero — including the
  point zero of a chord exactly on the candidate — hulls to `±|r|`, and
  the `max` then admits `+|r|` unconditionally. Sound (a superset) and
  needlessly wide, and it decides nothing at a point tie that
  `select_le_zero` would decide.
- **`blend::arms`'s nappe selector** (`crates/sweep/src/blend/arms.rs`):
  `T::one().copysign((self.rim - apex).dot(axis) * axis.dot(self.axis))`
  — a `±1` nappe choice keyed on a product of two dot products. Two-way
  choice, value-zero tie.

## Why it is filed rather than fixed

PROPS's sign-hull unit owns `orthonormal_basis` and nothing else. The
`axis.rs` fold is the one where a respell is close to mechanical (the
`max` is already the selection); the nappe selector wants a measurement
first — whether its tie is a decidable choice or a genuine cone
degeneracy, which is the same question `geom-brep`'s two cone-nappe
`copysign`s answer with "singularity, keep the hull".
