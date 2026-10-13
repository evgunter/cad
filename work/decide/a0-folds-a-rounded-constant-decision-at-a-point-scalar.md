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
rounding moves a constant decision across zero. That takes a comparison
of literals that cancel catastrophically: one rounded subtraction of two
`f64` constants keeps its exact sign, so it needs a chain of at least
two operations. The constant decisions the measured documents mint
(axis-direction frames through `Vec3::orthonormal_basis`,
`a0-leaves-max-and-min-of-constants-opaque`) are exact in `f64`. At
`Interval` the claim holds, because the construction's comparison is a
certified sign of the same real.

Two lanes run A0 at a point scalar:

- **The K replay at `Sym<Probe>`** (`editor_core::drive`). A false fold
  there reaches K telemetry as a `symbolic` sample.
- **The coincidence door's replay at `Sym<f64>`**
  (`crates/editor-core/src/coincide/replay.rs:67` for the lane,
  `:86-89` for the session at `SymRules::shipped()`, so A0 is on). This
  one is a verdict path. A row the replay proves becomes
  `Proof::Structural(Rung::MarginIdentity)` or
  `Proof::Structural(Rung::CarrierVerdict)` (`replay.rs:151,157`,
  `coincide.rs`'s `prove`). A proven row is one the
  `unproven-coincidence` check does not report, and that check is
  `CheckKind::Certified` (`crates/editor-core/src/checks.rs:135`). The
  replay is a deferring session, so its branches are the build's, but
  the margin forms the rungs ask for are built afterwards by walks
  under `shipped()`, and A0 takes the exact constant's arm there. So a
  false fold would certify a coincidence across the family that holds
  only where the construction took the other arm.

**Not shown reachable.** The replay makes every continuous document
variable a symbol (`EvalOptions::symbols`), so a constant decision form
there comes only from literals in kernel code or from a computed value
re-entered as a constant. The first kind are exact in the documents
measured. The second is already a known source of false proofs, with
its own rows (`scripts/gates/from-f64-sites.tsv`'s `launders(…)` and
`stored-carrier-data(…)` classes). No document or small probe through
`prove` is known to reach the fold. Reaching it needs a kernel
computation that compares cancelling literals inside a decision the
replay asks a rung about.

**Priority.** It stays P2, as an interval and error-propagation defect.
The coincidence reach is why it is not lowered to P3 as latent
unsoundness: if reached, it is a false certified verdict and not only
telemetry. It is not raised to P0, because no document is shown to
reach it.

## What a fix would have to do

Take the arm only where the point construction provably takes it: the
rounded comparison agrees with the exact sign. The walk holds forms,
not the node's `f64` value, so this needs the decision's evaluated
value at the walk (or a bound on the literal arithmetic's rounding),
or A0 declining at an inexact witness — which would reopen the
atom chains `a0-leaves-max-and-min-of-constants-opaque` closed, at the
point lanes only. Measure that cost before choosing.
