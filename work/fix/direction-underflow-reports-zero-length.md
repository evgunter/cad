---
id: direction-underflow-reports-zero-length
kind: issue
title: the underflow twin: a direction under ~1e-162 is refused as zero length, which it is not
status: open
opened: 2026-09-04
---


## The finding

`geom-core`'s `Vec3::normalize` documents a symmetric pair of failure
modes (`crates/geom-core/src/linalg/vec.rs:227-230`): components above
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
