---
id: pncad-py-comparable-enums-do-not-hash
kind: issue
title: pncad-py: every comparable enum crosses unhashable, so a tally by kind cannot use a set or dict
status: closed
opened: 2026-09-06
closed: 2026-09-08
---


Measured en route by LIB-B-FACE-FRAME, which hit it writing a tally by
carrier kind; banked rather than fixed, because it is a 23-site change
to the crate's whole enum surface and not a face-frame decision.

## What happens

```python
kinds = {ev.face_carrier_kind(body, f) for f in ev.all_faces(body)}
# TypeError: unhashable type: 'pncad.SurfaceKind'
```

`SurfaceKind.Plane == SurfaceKind.Plane` is `True`, and
`{SurfaceKind.Plane}` raises. Python's own contract is that a type
defining `__eq__` without `__hash__` is unhashable, and pyo3's
`#[pyclass(eq, eq_int)]` generates exactly that: `__eq__` and no
`__hash__` unless `hash` is asked for.

## The whole hit list

Every fieldless mirror in the crate, all 23 — the `pyclass(eq,
eq_int, ...)` sites in `crates/pncad-py/src/py/`, none of which asks
for `hash`:

`Advisory`, `ArcSide`, `ArcSweep`, `AxisSense`, `BooleanOp`,
`CapEnd`, `CheckId`, `CheckKind`, `Cmp`, `ContactClass`, `CurveKind`,
`EntityKind`, `FlushRung`, `MateRole`, `MateSide`, `MeridianEnd`,
`OpGroup`, `PlaneRelation`, `RimSupport`, `SegTag`, `Severity`,
`SplitHalf`, `SurfaceKind`.

Swept with `grep -rn "pyclass(eq, eq_int" crates/pncad-py/src/py/*.rs`
(23 hits) cross-checked against a runtime probe that instantiates each
top-level class's own class attributes and calls `hash` on the first
one that is an instance of it (23 hits, same set). **What that pattern
could not match**: a class whose only instances come from a door
rather than from a class attribute — the probe finds nothing to hash
for those, so a value type with the same `__eq__`-without-`__hash__`
shape would be invisible to it. `Denotation` is the one such class
this unit read (`py/readback.rs`, `#[pyclass(frozen, ...)]` with a
hand-written `__eq__`) and it is unhashable for the same reason.

## Why it matters

These are TAGS. Counting faces by carrier kind, keying a policy table
by `ContactClass`, deduplicating a list of `SegTag` — every one of
those is a set or a dict in idiomatic Python, and every one raises.
The workaround is a list of comparisons, which is what
`crates/pncad-py/tests/test_face_frame.py::test_every_face_of_an_extrude_is_planar`
had to write.

## The fix, and the one question in it

pyo3's `hash` option derives `__hash__` beside the `__eq__` these
already ask for; it is spelled alongside `eq`, and whether it also
needs `frozen` on a fieldless mirror (none of the 23 declares it
today) is for the fixing lane to establish rather than for this row to
assert. The question worth asking before doing it is
whether any of the 23 is deliberately unhashable — nothing in the tree
says so, and the uniformity of the miss (23 of 23, no site asking for
`hash`) reads as never-considered rather than decided.

## Closed

2026-09-08 by LIB-HASH. Every fieldless mirror in
`crates/pncad-py/src/py/` carries `eq, eq_int, frozen, hash` and
derives `Eq, Hash`; `{SurfaceKind.Plane}` no longer raises.

**The count was 24, not 23**: `AssertionDir`
(`crates/pncad-py/src/py/measure.rs`) arrived between the filing and
the fix, which is the argument for the guard the unit added rather
than a roster.

**`frozen` is required** — pyo3 0.29.0's `pyclass_hash` refuses `hash`
without both `frozen` and `eq` — and costs a fieldless mirror nothing.

**The question this row asked is answered: no.** Not one of the 24 is
deliberately unhashable; no doc comment on any mirror mentions
hashing, keys, or a reason to refuse them. The uniformity of the miss
read as never-considered and that is what it was.

`Denotation` — the one class this row's probe named as invisible to it
— hashes too, by hand, over the same `(tied, candidates)` its
hand-written `__eq__` reads.

The residue this row's blind spot pointed at is its own file:
`pncad-py-value-classes-compare-without-hashing` holds the nine value
classes that still compare without hashing, `Expr` and `MeasureExpr`
being the two that say why they do not.
