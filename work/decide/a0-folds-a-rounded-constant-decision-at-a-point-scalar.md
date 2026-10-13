---
id: a0-folds-a-rounded-constant-decision-at-a-point-scalar
kind: issue
title: A0's exact constant fold at Select/min/max takes the other arm from a point scalar's rounded comparison
status: open
opened: 2026-10-13
priority: P2
---


## What was demonstrated (DECIDE-10 fix pass, 2026-10-13)

DECIDE-10's fix pass made the decision read decline at an inexact
witness (`crates/geom-core/src/sym.rs`, `combine`'s `read`, behind
`Session::witness`): at `Sym<f64>` or `Sym<Probe>` the construction
compares a ROUNDED value, so the arm an exact sign picks need not be
the arm the construction took (review R1's MINOR-1 on PR #4929).

Rule A0's constant fold has the same defect at the same scalars, and
the read's fix does not reach it, because A0 is not the read. At a
`Select` whose decision form is a rational constant
(`sym.rs`, `combine`'s `SymOp::Select` arm, `if a0 && let Some(c) =
constant_value(a)`) and at `min`/`max` of two constant forms (the arm
above it, `constant_value(a)` and `constant_value(b)`), A0 takes the
arm the EXACT constant picks. The constant is exact arithmetic on the
literals' `f64` values; the point construction evaluated the same
literals in rounded `f64`.

The probe, at `Sym<f64>`, literals only in the decision:

```rust
let big = Sym::<f64>::from_f64(1e17);
let d = (Sym::<f64>::from_f64(0.5) + big) - big - Sym::from_f64(0.25);
// d.value == -0.25 (f64 takes when_le); exactly d = +0.25 (A0 takes when_gt)
let m = d.select_le_zero(z + w, z) - z; // z, w parameters
```

| `w` | rules | answer | `proven` | value | counts |
| --- | --- | --- | --- | --- | --- |
| `1e-15` | shipped | `Zero` | true | `1.1e-15` | symbolic 1 |
| `1e-15` | read shut | `Zero` | true | `1.1e-15` | symbolic 1 |
| `1e-3` | shipped | `Positive` | false | `1.0e-3` | numeric 1, theorems_disputed 1 |
| `1e-3` | read shut | `Positive` | false | `1.0e-3` | numeric 1, theorems_disputed 1 |

So at `w = 1e-15` the tier proves a zero of a function the point
construction did not evaluate (its value is `w`), with the read shut
as with it on.

## Reach

Only point scalars (`f64`, `Probe`), and only where the construction's
rounding moves a constant decision across zero — a comparison of
literals that cancel catastrophically. The constant decisions the
measured documents mint (axis-direction frames through
`Vec3::orthonormal_basis`, `a0-leaves-max-and-min-of-constants-opaque`)
are exact in `f64`. At `Interval` the claim holds: the construction's
comparison is a certified sign of the same real. No document is known
to reach it; the K replay at `Sym<Probe>` is where it would show.

## What a fix would have to do

Take the arm only where the point construction provably takes it: the
rounded comparison agrees with the exact sign. The walk holds forms,
not the node's `f64` value, so this needs the decision's evaluated
value at the walk (or a bound on the literal arithmetic's rounding),
or A0 declining at an inexact witness — which would reopen the
atom chains `a0-leaves-max-and-min-of-constants-opaque` closed, at the
point lanes only. Measure that cost before choosing.
