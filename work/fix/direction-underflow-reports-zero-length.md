---
id: direction-underflow-reports-zero-length
kind: issue
title: the underflow twin: a direction under ~1e-162 is refused as zero length, which it is not
status: review
opened: 2026-09-04
branch: fix/direction-underflow
---


## The finding

`geom-core`'s `Vec3::normalize` documents a symmetric pair of failure
modes (`crates/geom-core/src/linalg/vec.rs:238-242`): components above
~1e154 overflow `norm_squared` to +∞, components below ~1e-162
underflow it to 0. PR #1738 closed the overflow end — `unit()` and
`UnitVec3::new` both ask `is_finite_length` before deciding — and left
the underflow end reporting a false cause.

Measured at #1738's head, a linear `Node::Pattern` with direction
`[1e-180, 0, 0]`:

```
norm_squared = 0.0   norm = 0.0
DegenerateDirection { role: "pattern direction" }
  → "the pattern direction has zero length"
```

The direction is not zero. It is a perfectly good direction whose
squared length fell out of the bottom of the format, and the user is
told the one thing about it that is false. `is_finite_length(0.0)`
passes — zero IS finite — so the new gate does not see it, and
`decide` then answers `Zero` definitely rather than in-band, so it
does not escalate either.

## Why it is worth a unit and not a shrug

It refuses, so nothing silent is minted and this is not the
coincident-instance hole #1738 closed. What it is, is exactly the
defect `NonFiniteDirection` was minted to prevent one file over: a
refusal whose stated cause is not the true one
(`memories/refusal-text-is-not-cause.md`). A user reading "zero
length" checks their direction, finds it nonzero, and has been sent
the wrong way; the true recourse is the same as the overflow arm's —
scale the geometry into the session's range.

The asymmetry lands on BOTH doors, because both now share one
predicate: `topo::UnitVec3::new` (SEAT-DV, PR #1564) and
`editor-core`'s `eval::wire::unit()` (PR #1738).

## Shape of the fix (not taken here)

The question is whether "a length that underflowed" is a third fact
beside zero and non-finite, or whether the two out-of-range facts are
one fact with two signs. That is a decision, not a diff — which is why
this is filed rather than fixed in #1738:

- a third refusal arm, symmetric with `NonFiniteDirection`, naming
  underflow and carrying the same "scale the geometry" recourse; or
- one out-of-range arm that both ends reach, with the zero arm
  reserved for a direction that is actually zero.

Either way the discriminator is arithmetic that already exists:
`norm_squared == 0` while the components are not all zero.

## Blind spot

Measured on the `f64` lane through the pattern door only. Not measured
at the interval scalar, where an enclosure straddling the underflow
threshold may escalate instead of deciding `Zero`, and not measured
through `UnitVec3::new`'s datum door, which shares the predicate and
is expected to behave identically but was not executed.

Found by #1738's style review, which executed the arithmetic.

## What landed

Branch `fix/direction-underflow`.

**The fork is taken: a THIRD refusal arm**, symmetric with the
non-finite one, not a single out-of-range arm both ends reach. The
reasoning:

- **The recourses differ, and that is what a refusal is for.** A
  genuinely zero direction means *give me a nonzero one*; an
  underflowed one means *scale the geometry into the session's
  range* — the overflow arm's recourse. Collapsing them produces a
  message that is right about the range and silent about which of two
  quite different mistakes the author made.
- The `Zero` arm keeps its meaning for a direction that IS zero,
  which is a real and common authoring error and deserves its own
  sentence. It keeps a second meaning too, measured here: a length
  the format holds perfectly well that the BAND calls zero (`1e-30`).
  That refusal is the tolerance's and a smaller ε changes it; the
  underflowed one is the format's and no ε changes it.
- It is the shape the overflow end already took (PR 1738, PR 2356),
  so the two ends of one documented symmetry get symmetric treatment.

**One gate, one arm, as scoped.** PR 1987's collapse still holds:
`topo::UnitVec3::new` and `editor-core`'s `eval::wire::unit()` are two
calls to `topo::query::decide_unit_direction`, so both doors are fixed
by one gate there. `UnitVec3Error::UnderflowedLength` is the kernel
arm; `NodeErrorKind::UnderflowedDirection { role }` is the evaluation
layer's, mapped in `wire::refusal`, tagged `underflowed_direction` at
the Python door.

**The discriminator, in the value channel.** `Real` has no comparison
surface, so `norm_squared == 0` cannot be spelled at the generic
scalar. `geom_core::is_underflowed_length(len, witness)` asks it the
way `is_finite_length` asks finiteness — two divisions, no bracket
read, no threshold — where `witness` is the largest absolute
component. The norm brackets that witness, so both ratios are bounded
UNLESS the norm came out exactly zero; then `witness / len` is `±∞`
for a vector with a direction and the scalar's poison for the zero
vector, which is the separation.

**Both declared blind spots closed by measurement**, and one of them
was wrong as filed:

- The **interval scalar does not escalate**. A `1e-180` component
  squares to `[0, 1e-323]`, the norm comes back `[0, 3.1e-162]`, and
  the whole enclosure sits inside the band — so it decides `Zero`
  DEFINITELY and refused `DegenerateDirection` at the merge base,
  exactly as `f64` did. The gate is a point-scalar gate (its ratio is
  an unbounded enclosure, not poison), so the enclosure lane's answer
  is unchanged and pinned.
- The **datum door** was executed for the first time and behaves
  identically at both ends.
