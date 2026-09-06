---
id: copysign-as-a-branch-in-disguise-hulls-where-a-decision-exists
kind: issue
title: copysign standing in for a two-way choice hulls at Interval where a value-level decision exists
status: open
opened: 2026-09-06
---


## The shape

`Real::copysign`'s `Interval` arm must return the two-sided hull
`[−sup|x|, sup|x|]` for ANY `sign` enclosure containing zero, including
the point enclosure `[0, 0]` — because an `f64` zero's sign BIT is
invisible in an enclosure and a one-sided choice would fail to contain
an `f64` replay that saw `−0.0`
(`crates/geom-core/src/interval.rs`, `copysign`'s own docs).

That is right wherever the transferred sign is the ANSWER — an odd
function's sign, a signed distance. It is wrong wherever `copysign` is
standing in for a two-way CHOICE between candidate computations: there
the tie is a value zero, not a sign bit, the choice is the same for
every point of a point enclosure, and the hull throws away a decision
that exists. `Real::select_le_zero` (added by PROPS's sign-hull unit,
`crates/geom-core/src/real.rs`) is the door for that case: its
tie-break keys on `d == 0` and it therefore DECIDES at a point tie,
hulling only over a straddle of positive width.

`Vec3::orthonormal_basis` was the first instance and is fixed. This
issue collects the rest.

## The hit list

Swept with `grep -rn "\.copysign(" crates/*/src` at
`23cc2deba` (see the PR for what that pattern cannot match).

**This shape — a branch in disguise, listed here:**

- `crates/sweep/src/revolve/axis.rs:147` —
  `r_max = r_max.max(frame.r(p).abs().copysign(margin))`. The clearest
  member: the site's own docs (`axis.rs:116-123`) say the conditional
  "candidate enters only when the margin is non-negative" is "folded
  **comparison-free** via `copysign`". At `Interval` a `margin`
  enclosing zero — including the point zero of a chord exactly on the
  candidate — hulls to `±|r|`, and the `max` then admits `+|r|`
  unconditionally. Sound (a superset) and needlessly wide, and it
  decides nothing at a point tie that `select_le_zero` would decide.
- `crates/profile/src/path.rs:2380` —
  `let sgn = T::one().copysign(trims.half_tan)`: the fillet's turn
  side, `σ = sign(tan(φ/2))`. A two-way geometric choice; at a
  point-zero `half_tan` (a straight-through corner) `copysign` hulls
  `σ` to `[−1, 1]` and the arc's centre is hulled to both sides.
- `crates/profile/src/sugar.rs:1340` —
  `sgn * half_chord / (radius + apothem.copysign(sgn * cross))`: the
  cross product's sign selects whether the apothem adds or subtracts
  (the major/minor arc choice, per the comment at `sugar.rs:1316`).
  Same shape: at collinear tangent points the tie is a value zero.
- `crates/sweep/src/blend/arms.rs:755` —
  `T::one().copysign((self.rim - apex).dot(axis) * axis.dot(self.axis))`:
  a ±1 nappe selector. Two-way choice keyed on a dot product's sign.

**Genuine sign transfer, keep:**

- `crates/topo/src/boolean/solid_contain.rs:2630` (`cbrt`'s
  `acc.copysign(x)`) — an odd function's own sign, and the magnitude is
  zero at the tie, so the hull is exact there.
- `crates/geom-brep/src/implicit.rs:174` and
  `crates/geom-brep/src/offset.rs:193` — the cone's nappe sign from the
  axial height `h`. The tie `h = 0` is the APEX, where no tangent plane
  exists and the module docs already promise poison; the discontinuity
  is a genuine singularity, not a decidable choice.
- `crates/geom-brep/src/props/curved.rs:1702` —
  `chord_a.min(chord_b).copysign(f)`: a signed membership distance. The
  site's own comment records that the hull is tight exactly where it
  fires (chord ≈ 0 at a span endpoint).
- `crates/profile/src/sugar.rs:1014` —
  `spoke.norm().copysign(rho)`: `rho` is a signed offset radius and its
  sign is the answer; the tie is a zero-radius offset, a genuine
  singularity of the division that follows.

**Not this shape, and not reachable at `Interval`:**

- `crates/geom-core/src/linalg/svd.rs:202` —
  `let alpha = -nrm.copysign(b[k][k])`: the Householder cancellation-free
  sign choice IS a branch in disguise, but the routine is `f64`-only
  (raw `is_nan`/`<=` comparisons on the same lines), so no `Interval`
  arm exists to hull. Listed so a future generic-over-`Real` SVD does
  not inherit it silently.

## Why it is filed rather than fixed

PROPS's sign-hull unit owns `orthonormal_basis` and nothing else; each
of the four sites above is another program's, and three of the four
have a tie that may be a genuine singularity rather than a decidable
choice — which is a per-site measurement (does the tie arise at a point
enclosure in a real fixture?), not a mechanical respell.
