---
id: certification-contains-admits-an-infinite-probe
kind: issue
title: Certification::contains answers true for ±inf on an unbounded enclosure, where the enclosure is a set of reals and the retired inari backend answers false
status: open
opened: 2026-09-29
priority: P2
cost: E
design: true
---


## Finding

Found by `crates/geom-core/tests/certification_door_differential.rs`
(`contains_is_its_reference_over_every_operand_and_probe`), whose
reference reads the door's doc — "whether `x` lies in the enclosure" —
over the set of reals the enclosure stands for, so `±inf` lies in none:

```
contains([-inf, -1.7976931348623157e308]@Dac, -inf): membership
  left: true      (the door)
 right: false     (the reference)
```

`Certification::contains` (`crates/geom-core/src/interval/certification.rs`,
the `contains` body) is `self.is_certified() && self.0.contains(x)`, and
`DInterval::contains` (`interval-transcendentals/src/interval.rs`) is
`!nai && !empty && lo <= x && x <= hi`, which admits `x = ±inf` whenever
that side of the enclosure is unbounded (`[0, +inf]`, `entire`). Its own
doc says "the real number `x`". The kernel says elsewhere that `±inf`
is not a real number and has no enclosure (`interval.rs` module doc,
"Non-real inputs"; `Certification::point` refuses `±inf` because "an
infinite point would launder overflow into data"). inari 2.0,
the backend this one replaced, answers `false`: its `Interval::contains`
is `rhs.is_finite() & …` and its doc reads "±∞ and NaN are not real
numbers, thus do not belong to any interval".
`interval-transcendentals/docs/semantics-diffs.md` does not list the
divergence.

With the reference's `p.is_finite()` clause removed, the differential
agrees with the door on every case, so this is the only
disagreement across the five doors.

## The choice

- (a) the door (or `DInterval::contains`) refuses a non-finite probe, as
  inari did; the differential lands as written.
- (b) the contract admits `±inf` as a member of an unbounded side; the
  door docs (and `DInterval::contains`'s "the real number `x`") say so,
  the divergence goes into `semantics-diffs.md`, and the reference drops
  its `p.is_finite()` clause.

No caller was surveyed for a probe that can be infinite.

