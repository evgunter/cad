---
id: copysign-stands-in-for-the-turn-side-and-hulls-at-a-decidable-tie
kind: issue
title: profile's fillet turn-side and arc-side copysigns hull at Interval where a value-level decision exists
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

- **`path::fillet_arc_carrier`** (`crates/profile/src/path.rs`):
  `let sgn = T::one().copysign(trims.half_tan)` — the fillet's turn
  side, `σ = sign(tan(φ/2))`. A two-way geometric choice. At a
  point-zero `half_tan` (a straight-through corner) `copysign` hulls `σ`
  to `[-1, 1]` and the arc's centre is hulled to both sides of the
  path.
- **`sugar::fillet_bulge`** (`crates/profile/src/sugar.rs`):
  `sgn * half_chord / (radius + apothem.copysign(sgn * cross))` — the
  cross product's sign selects whether the apothem adds or subtracts
  (the major/minor arc choice, as the comment above `fillet_bulge`
  says). Same shape: at collinear tangent points the tie is a value
  zero.

A third `copysign` in this program is NOT this shape and should stay:
`sugar::tangent_point`'s `spoke.norm().copysign(rho)` transfers the
sign of a signed offset radius, which is the answer, and its tie is a
zero-radius offset — a genuine singularity of the division that
follows.

## Why it is filed rather than fixed

PROPS's sign-hull unit owns `orthonormal_basis` and nothing else. Both
sites above want a per-site measurement before a respell — does the tie
arise at a point enclosure in a real fixture, or only at a degenerate
corner the caller has already refused? — which is this program's call,
not a mechanical substitution.
