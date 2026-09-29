---
id: certification-contains-admits-an-infinite-probe
kind: issue
title: Certification::contains answers true for ±inf on an unbounded enclosure, where the enclosure is a set of reals and the retired inari backend answers false
status: open
opened: 2026-09-29
priority: P2
cost: E
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

## Ruling (orchestrator, 2026-09-29)

A defect against a documented contract, not a design choice, so it is
fixed where it starts: `DInterval::contains` refuses a non-finite probe
(`x.is_finite() && …`, inari's spelling), and the door inherits it.
The backend's own doc says "the real number `x`" and `±inf` is not one;
inari answered `false` and nothing lists the divergence as intended
(`semantics-diffs.md`: "anything new the harness finds is a bug, not a
difference"); and the kernel elsewhere gives `±inf` no enclosure
(`point(±inf)`, `interval.rs` "Non-real inputs"). Fixing only the door
would leave every other `DInterval::contains` caller admitting it.

Caller survey (on the fix's branch): `DInterval::contains` has one
production caller, the `Certification::contains` door, and the door has
no production caller in the workspace; every test caller probes a
finite value (a constant, or an f64 result guarded by `is_finite()` or
drawn from a bounded range). Nothing relied on admitting `±inf`.
